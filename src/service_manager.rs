//! This module provides functionalities for managing Windows services.
//! It includes structs for service configuration and a `ServiceManager` for creating, retrieving, and managing services.

use widestring::U16CString;
use windows_sys::Win32::{
    Security::SC_HANDLE,
    System::Services::{
        CloseServiceHandle, CreateServiceW, OpenSCManagerW, OpenServiceW, SC_MANAGER_ALL_ACCESS,
        SERVICE_ALL_ACCESS,
    },
};

use crate::{
    common::{get_last_error, set_privilege},
    error::{CreateServiceError, OpenServiceError, ServiceManagerError},
    service::{ServiceErrorControl, ServiceHandle, ServiceStartType, ServiceType},
};

/// Configuration for a Windows service.
#[derive(Default, Clone, Debug)]
pub struct ServiceConfig {
    pub service_name: String,
    pub display_name: String,
    pub binary_path: String,
    pub service_type: ServiceType,
    pub start_type: ServiceStartType,
    pub error_control: ServiceErrorControl,
}

/// Manages Windows services, providing functionalities to create, retrieve, and control services.
pub struct ServiceManager {
    handle: Option<SC_HANDLE>,
}

impl Drop for ServiceManager {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            unsafe {
                CloseServiceHandle(handle);
            }
        }
    }
}
/// # Examples
///
/// ```rust
/// let service_manager = ServiceManager::new()?;

/// let service_name = "test";
/// let service_path = r"C:\Windows\system32\test.sys";

/// let service_handle = service_manager.create_or_get(ServiceConfig {
///     service_name: service_name.to_string(),
///     display_name: service_name.to_string(),
///     binary_path: "invalid path test".to_string(),
///     start_type: ServiceStartType::DemandStart,
///     service_type: ServiceType::KernelDriver,
///     ..Default::default()
/// })?;

/// assert_eq!(
///     service_handle.get_start_type()?,
///     ServiceStartType::DemandStart
/// );

/// service_handle.update_config(ServiceConfig {
///     display_name: service_name.to_string(),
///     binary_path: service_path.to_string(),
///     service_type: ServiceType::KernelDriver,
///     ..Default::default()
/// })?;
/// service_handle.start_blocking()?;

/// assert_eq!(service_handle.state()?, ServiceState::Running);

/// service_handle.stop_blocking()?;

/// assert_eq!(service_handle.state()?, ServiceState::Stopped);

/// std::thread::sleep(std::time::Duration::from_secs(2));

/// service_handle.delete()?;
/// ```
impl ServiceManager {
    /// Creates a new `ServiceManager` with access to the service control manager.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't open the service control manager.
    pub fn new() -> Result<Self, ServiceManagerError> {
        let handle =
            unsafe { OpenSCManagerW(std::ptr::null(), std::ptr::null(), SC_MANAGER_ALL_ACCESS) };

        if handle == 0 {
            return Err(ServiceManagerError::from((
                get_last_error(),
                "[ServiceManager::new] handle == 0".to_string(),
            )));
        }

        set_privilege("SeLoadDriverPrivilege".to_string()).map_err(|_| {
            ServiceManagerError::AccessDenied(
                get_last_error(),
                "[ServiceManager::set_privilege] failed".to_string(),
            )
        })?;

        Ok(Self {
            handle: Some(handle),
        })
    }

    /// Creates a new service with the specified configuration.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't create the service.
    pub fn create_service(
        &self,
        options: ServiceConfig,
    ) -> Result<ServiceHandle, CreateServiceError> {
        let scm_handle = self.handle.ok_or(CreateServiceError::InvalidHandle(
            get_last_error(),
            "[create_service] invalid service manager handle".to_string(),
        ))?;

        let service_name = U16CString::from_str(options.service_name.clone()).map_err(|_| {
            CreateServiceError::InvalidName(0, "[create_service] invalid service name".to_string())
        })?;
        let display_name = U16CString::from_str(options.display_name.clone()).map_err(|_| {
            CreateServiceError::InvalidName(0, "[create_service] invalid display name".to_string())
        })?;
        let binary_path = U16CString::from_str(options.binary_path.clone()).map_err(|_| {
            CreateServiceError::InvalidParameter(
                0,
                "[create_service] invalid binary path".to_string(),
            )
        })?;

        let handle = unsafe {
            CreateServiceW(
                scm_handle,
                service_name.as_ptr(),
                display_name.as_ptr(),
                SERVICE_ALL_ACCESS,
                options.service_type as u32,
                options.start_type as u32,
                options.error_control as u32,
                binary_path.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            )
        };

        if handle == 0 {
            return Err(CreateServiceError::from((
                get_last_error(),
                "[create_service] handle == 0".to_string(),
            )));
        }

        Ok(ServiceHandle::new(handle))
    }

    /// Retrieves an existing service by name.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't retrieve the service.
    pub fn get_service(&self, service_name: String) -> Result<ServiceHandle, OpenServiceError> {
        let scm_handle = self.handle.ok_or(OpenServiceError::InvalidHandle(
            get_last_error(),
            "[get_service] invalid service manager handle".to_string(),
        ))?;

        let service_name = U16CString::from_str(service_name)
            .map_err(|_| OpenServiceError::InvalidName(0, "invalid service named".to_string()))?;
        let handle = unsafe { OpenServiceW(scm_handle, service_name.as_ptr(), SERVICE_ALL_ACCESS) };

        if handle == 0 {
            return Err(OpenServiceError::from((
                get_last_error(),
                "[get_service] handle == 0".to_string(),
            )));
        }

        Ok(ServiceHandle::new(handle))
    }

    /// Creates a new service if it doesn't exist, otherwise retrieves the existing service.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't create or retrieve the service.
    pub fn create_or_get(
        &self,
        options: ServiceConfig,
    ) -> Result<ServiceHandle, CreateServiceError> {
        if let Ok(service_handle) = self.get_service(options.service_name.clone()) {
            return Ok(service_handle);
        }

        self.create_service(options)
    }
}
