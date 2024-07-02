//! This module provides functionalities for controlling Windows services.
//! It includes structs and enums for handling service configurations and states,
//! and functions for starting, stopping, pausing, and querying services.

use std::fmt::Display;
use widestring::U16CString;
use windows_sys::Win32::{
    Foundation::{ERROR_INSUFFICIENT_BUFFER, FALSE},
    Security::SC_HANDLE,
    System::Services::{
        ChangeServiceConfigW, CloseServiceHandle, ControlService, DeleteService,
        QueryServiceConfigW, QueryServiceStatus, StartServiceW, QUERY_SERVICE_CONFIGW,
        SERVICE_ADAPTER, SERVICE_AUTO_START, SERVICE_BOOT_START, SERVICE_CONTINUE_PENDING,
        SERVICE_CONTROL_PAUSE, SERVICE_CONTROL_STOP, SERVICE_DEMAND_START, SERVICE_DISABLED,
        SERVICE_FILE_SYSTEM_DRIVER, SERVICE_KERNEL_DRIVER, SERVICE_PAUSED, SERVICE_PAUSE_PENDING,
        SERVICE_RECOGNIZER_DRIVER, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STATUS,
        SERVICE_STOPPED, SERVICE_STOP_PENDING, SERVICE_SYSTEM_START, SERVICE_WIN32_OWN_PROCESS,
        SERVICE_WIN32_SHARE_PROCESS,
    },
};

use crate::{
    common::get_last_error,
    error::{ControlServiceError, DeleteServiceError, QueryServiceError, UpdateServiceError},
    service_manager::ServiceConfig,
};

/// Represents a handle to a Windows service.
#[derive(Default, Debug)]
pub struct ServiceHandle {
    handle: Option<SC_HANDLE>,
}

/// Defines the error control levels for a Windows service.
#[repr(u32)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum ServiceErrorControl {
    ErrorCritical = 0x00000003,
    ErrorIgnore = 0x00000000,
    #[default]
    ErrorNormal = 0x00000001,
    ErrorSevere = 0x00000002,
}

/// Defines the start types for a Windows service.
#[repr(u32)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum ServiceStartType {
    AutoStart = 0x00000002,
    BootStart = 0x00000000,
    #[default]
    DemandStart = 0x00000003,
    Disabled = 0x00000004,
    SystemStart = 0x00000001,
}

/// Defines the types of a Windows service.
#[repr(u32)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum ServiceType {
    Adapter = 0x00000004,
    FileSystemDriver = 0x00000002,
    #[default]
    KernelDriver = 0x00000001,
    RecognizerDriver = 0x00000008,
    Win32OwnProcess = 0x00000010,
    Win32ShareProcess = 0x00000020,
}

/// Represents the possible states of a Windows service.
#[repr(u32)]
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceState {
    Stopped = 1,
    StartPending,
    StopPending,
    Running,
    ContinuePending,
    PausePending,
    Paused,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Adapter => write!(f, "Adapter"),
            Self::FileSystemDriver => write!(f, "FileSystemDriver"),
            Self::KernelDriver => write!(f, "KernelDriver"),
            Self::RecognizerDriver => write!(f, "RecognizerDriver"),
            Self::Win32OwnProcess => write!(f, "Win32OwnProcess"),
            Self::Win32ShareProcess => write!(f, "Win32ShareProcess"),
        }
    }
}

impl Display for ServiceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Stopped => write!(f, "Stopped"),
            Self::StartPending => write!(f, "StartPending"),
            Self::StopPending => write!(f, "StopPending"),
            Self::Running => write!(f, "Running"),
            Self::ContinuePending => write!(f, "ContinuePending"),
            Self::PausePending => write!(f, "PausePending"),
            Self::Paused => write!(f, "Paused"),
        }
    }
}

impl Display for ServiceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = writeln!(f, "StartType: {:?}", self.get_start_type());
        let _ = writeln!(f, "State: {:?}", self.state());
        Ok(())
    }
}

impl TryFrom<u32> for ServiceType {
    type Error = QueryServiceError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            SERVICE_ADAPTER => Ok(ServiceType::Adapter),
            SERVICE_FILE_SYSTEM_DRIVER => Ok(ServiceType::FileSystemDriver),
            SERVICE_KERNEL_DRIVER => Ok(ServiceType::KernelDriver),
            SERVICE_RECOGNIZER_DRIVER => Ok(ServiceType::RecognizerDriver),
            SERVICE_WIN32_OWN_PROCESS => Ok(ServiceType::Win32OwnProcess),
            SERVICE_WIN32_SHARE_PROCESS => Ok(ServiceType::Win32ShareProcess),
            _ => Err(QueryServiceError::from((
                0,
                "invalid service type".to_string(),
            ))),
        }
    }
}

impl TryFrom<u32> for ServiceStartType {
    type Error = QueryServiceError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            SERVICE_AUTO_START => Ok(Self::AutoStart),
            SERVICE_BOOT_START => Ok(Self::BootStart),
            SERVICE_DEMAND_START => Ok(Self::DemandStart),
            SERVICE_DISABLED => Ok(Self::Disabled),
            SERVICE_SYSTEM_START => Ok(Self::SystemStart),
            _ => Err(QueryServiceError::from((
                0,
                "invalid service start type".to_string(),
            ))),
        }
    }
}

impl TryFrom<u32> for ServiceState {
    type Error = QueryServiceError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            SERVICE_STOPPED => Ok(ServiceState::Stopped),
            SERVICE_START_PENDING => Ok(ServiceState::StartPending),
            SERVICE_STOP_PENDING => Ok(ServiceState::StopPending),
            SERVICE_RUNNING => Ok(ServiceState::Running),
            SERVICE_CONTINUE_PENDING => Ok(ServiceState::ContinuePending),
            SERVICE_PAUSE_PENDING => Ok(ServiceState::PausePending),
            SERVICE_PAUSED => Ok(ServiceState::Paused),
            _ => Err(QueryServiceError::from((
                0,
                "invalid service state".to_string(),
            ))),
        }
    }
}

impl Drop for ServiceHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            unsafe {
                CloseServiceHandle(handle);
            }
        }
    }
}

impl ServiceHandle {
    /// Creates a new `ServiceHandle`.
    pub fn new(handle: SC_HANDLE) -> Self {
        Self {
            handle: Some(handle),
        }
    }

    /// Returns the current state of this `ServiceHandle`.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't retrieve the service status.
    pub fn state(&self) -> Result<ServiceState, QueryServiceError> {
        let status = self.get_status()?;

        ServiceState::try_from(status.dwCurrentState)
    }

    /// Updates the configuration of the service.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't update the service configuration.
    pub fn update_config(&self, options: ServiceConfig) -> Result<(), UpdateServiceError> {
        let handle = self.handle.ok_or(UpdateServiceError::InvalidHandle(
            0,
            "[update_config] invalid service handle".to_string(),
        ))?;

        let display_name = U16CString::from_str(options.display_name.clone()).map_err(|_| {
            UpdateServiceError::InvalidParameter(
                0,
                "[update_config] invalid display_name".to_string(),
            )
        })?;
        let binary_path = U16CString::from_str(options.binary_path).map_err(|_| {
            UpdateServiceError::InvalidParameter(
                0,
                "[update_config] invalid binary_path".to_string(),
            )
        })?;
        unsafe {
            if ChangeServiceConfigW(
                handle,
                options.service_type as u32,
                options.start_type as u32,
                options.error_control as u32,
                binary_path.as_ptr(),
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                display_name.as_ptr(),
            ) == FALSE
            {
                return Err(UpdateServiceError::from((
                    get_last_error(),
                    "[update_config] failed".to_string(),
                )));
            }
        }
        Ok(())
    }

    /// Returns the get start type of this [`ServiceHandle`].
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't get the start type.
    pub fn get_start_type(&self) -> Result<ServiceStartType, QueryServiceError> {
        let config = self.get_config()?;

        ServiceStartType::try_from(config.dwStartType)
    }

    /// Sets the start type of the service.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't set the start type.
    pub fn set_start_type(&self, start_type: ServiceStartType) -> Result<(), UpdateServiceError> {
        let handle = self.handle.ok_or(UpdateServiceError::InvalidHandle(
            0,
            "[set_start_type] invalid service handle".to_string(),
        ))?;
        let config = self.get_config().map_err(|_| UpdateServiceError::AccessDenied(get_last_error(), "[set_start_type] failed to get service config".to_string()))?;

        unsafe {
            let mut tag_id = config.dwTagId;
            if ChangeServiceConfigW(
                handle,
                config.dwServiceType,
                start_type as u32,
                config.dwErrorControl,
                config.lpBinaryPathName,
                config.lpLoadOrderGroup,
                &mut tag_id,
                config.lpDependencies,
                config.lpServiceStartName,
                std::ptr::null(),
                config.lpDisplayName,
            ) == FALSE
            {
                return Err(UpdateServiceError::from((
                    get_last_error(),
                    "[set_start_type] ChangeServiceConfig failed".to_string(),
                )));
            }
        }
        Ok(())
    }
    pub fn delete(&self) -> Result<(), DeleteServiceError> {
        let handle = self.handle.ok_or(DeleteServiceError::InvalidHandle(
            0,
            "[delete] invalid service handle".to_string(),
        ))?;
        unsafe {
            if DeleteService(handle) == FALSE {
                return Err(DeleteServiceError::from((
                    get_last_error(),
                    "[delete] DeleteService failed".to_string(),
                )));
            }
        }
        Ok(())
    }

    /// Starts the service and blocks until it is running.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't start the service.
    pub fn start_blocking(&self) -> Result<(), ControlServiceError> {
        self.control_blocking(ServiceState::Running, || self.start())
    }

    /// Stops the service and blocks until it is stopped.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't stop the service.
    pub fn stop_blocking(&self) -> Result<(), ControlServiceError> {
        self.control_blocking(ServiceState::Stopped, || self.stop())
    }

    /// Pauses the service and blocks until it is paused.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't pause the service.
    pub fn pause_blocking(&self) -> Result<(), ControlServiceError> {
        self.control_blocking(ServiceState::Paused, || self.pause())
    }

    /// Starts the service.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't start the service.
    pub fn start(&self) -> Result<(), ControlServiceError> {
        let handle = self.handle.ok_or(ControlServiceError::InvalidHandle(
            0,
            "[start] invalid service handle".to_string(),
        ))?;
        unsafe {
            if StartServiceW(handle, 0, std::ptr::null()) == FALSE {
                return Err(ControlServiceError::from((
                    get_last_error(),
                    "[start] StartServiceW failed".to_string(),
                )));
            }
        }

        Ok(())
    }

    /// Stops the service.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't stop the service.
    pub fn stop(&self) -> Result<(), ControlServiceError> {
        self.control(SERVICE_CONTROL_STOP)
    }

    /// Pauses the service.
    ///
    /// # Errors
    ///
    /// This function will return an error if it can't pause the service.
    pub fn pause(&self) -> Result<(), ControlServiceError> {
        self.control(SERVICE_CONTROL_PAUSE)
    }
    #[doc(hidden)]
    fn control(&self, control: u32) -> Result<(), ControlServiceError> {
        let handle = self.handle.ok_or(ControlServiceError::InvalidHandle(
            0,
            "[control] invalid service handle".to_string(),
        ))?;

        unsafe {
            let mut service_status = std::mem::zeroed::<SERVICE_STATUS>();

            if ControlService(handle, control, &mut service_status) == FALSE {
                return Err(ControlServiceError::from((
                    get_last_error(),
                    "[control] ControlService failed".to_string(),
                )));
            }
        }

        Ok(())
    }
    #[doc(hidden)]
    fn control_blocking<F>(
        &self,
        service_state: ServiceState,
        control_fn: F,
    ) -> Result<(), ControlServiceError>
    where
        F: Fn() -> Result<(), ControlServiceError>,
    {
        control_fn()?;

        loop {
            if let Ok(status) = self.state() {
                if status == service_state {
                    break;
                }
            } else {
                return Err(ControlServiceError::Unknown(
                    get_last_error(),
                    "[control_blocking] failed to get service state".to_string(),
                ));
            }

            std::thread::sleep(std::time::Duration::from_millis(100))
        }

        Ok(())
    }
    #[doc(hidden)]
    fn get_config(&self) -> Result<QUERY_SERVICE_CONFIGW, QueryServiceError> {
        let handle = self.handle.ok_or(QueryServiceError::InvalidHandle(
            0,
            "[get_config] invalid service handle".to_string(),
        ))?;
        unsafe {
            let mut bytes_needed: u32 = 0;

            if QueryServiceConfigW(handle, std::ptr::null_mut(), 0, &mut bytes_needed) == FALSE
                && get_last_error() != ERROR_INSUFFICIENT_BUFFER
            {
                return Err(QueryServiceError::from((
                    get_last_error(),
                    "[get_config] QueryServiceConfig failed".to_string(),
                )));
            }

            let config_buffer = vec![0u8; bytes_needed as usize];
            let config = config_buffer.as_ptr() as *mut QUERY_SERVICE_CONFIGW;

            if QueryServiceConfigW(
                handle,
                config,
                config_buffer.len() as u32,
                &mut bytes_needed,
            ) == FALSE
            {
                return Err(QueryServiceError::from((
                    get_last_error(),
                    "[get_config] QueryServiceConfig failed".to_string(),
                )));
            }
            Ok(*config)
        }
    }
    #[doc(hidden)]
    fn get_status(&self) -> Result<SERVICE_STATUS, QueryServiceError> {
        let handle = self.handle.ok_or(QueryServiceError::InvalidHandle(
            0,
            "[get_status] invalid service handle".to_string(),
        ))?;
        unsafe {
            let mut status = std::mem::zeroed::<SERVICE_STATUS>();
            if QueryServiceStatus(handle, &mut status) == FALSE {
                return Err(QueryServiceError::from((
                    get_last_error(),
                    "[get_status] QueryServiceConfig failed".to_string(),
                )));
            }
            Ok(status)
        }
    }
}
