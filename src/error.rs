use thiserror::Error;
use windows_sys::Win32::Foundation::{
    ERROR_ACCESS_DENIED, ERROR_CIRCULAR_DEPENDENCY, ERROR_DATABASE_DOES_NOT_EXIST,
    ERROR_DUPLICATE_SERVICE_NAME, ERROR_FILE_NOT_FOUND, ERROR_INVALID_HANDLE, ERROR_INVALID_NAME,
    ERROR_INVALID_PARAMETER, ERROR_INVALID_SERVICE_ACCOUNT, ERROR_PATH_NOT_FOUND,
    ERROR_SERVICE_ALREADY_RUNNING, ERROR_SERVICE_DATABASE_LOCKED, ERROR_SERVICE_DEPENDENCY_DELETED,
    ERROR_SERVICE_DEPENDENCY_FAIL, ERROR_SERVICE_DISABLED, ERROR_SERVICE_DOES_NOT_EXIST,
    ERROR_SERVICE_EXISTS, ERROR_SERVICE_LOGON_FAILED, ERROR_SERVICE_MARKED_FOR_DELETE,
    ERROR_SERVICE_NOT_ACTIVE, ERROR_SERVICE_NO_THREAD, ERROR_SERVICE_REQUEST_TIMEOUT,
};

#[derive(Error, Debug)]
pub enum OpenServiceError {
    #[error("Access denied: {0}, {1}")]
    AccessDenied(u32, String),
    #[error("Invalid handle: {0}, {1}")]
    InvalidHandle(u32, String),
    #[error("Invalid name: {0}, {1}")]
    InvalidName(u32, String),
    #[error("Service does not exist: {0}, {1}")]
    ServiceDoesNotExist(u32, String),
    #[error("Unknown error: {0}, {1}")]
    Unknown(u32, String),
}

impl From<(u32, String)> for OpenServiceError {
    fn from(value: (u32, String)) -> Self {
        let (err, display) = value;
        match err {
            ERROR_ACCESS_DENIED => Self::AccessDenied(err, display),
            ERROR_INVALID_HANDLE => Self::InvalidHandle(err, display),
            ERROR_INVALID_NAME => Self::InvalidName(err, display),
            ERROR_SERVICE_DOES_NOT_EXIST => Self::ServiceDoesNotExist(err, display),
            _ => Self::Unknown(err, display),
        }
    }
}

#[derive(Error, Debug)]
pub enum CreateServiceError {
    #[error("Access denied: {0}, {1}")]
    AccessDenied(u32, String),
    #[error("Circular dependency error: {0}, {1}")]
    CircularDependency(u32, String),
    #[error("Invalid handle: {0}, {1}")]
    InvalidHandle(u32, String),
    #[error("Invalid name: {0}, {1}")]
    InvalidName(u32, String),
    #[error("Invalid parameter: {0}, {1}")]
    InvalidParameter(u32, String),
    #[error("Invalid service account: {0}, {1}")]
    InvalidServiceAccount(u32, String),
    #[error("Service already exists: {0}, {1}")]
    ServiceExists(u32, String),
    #[error("Service marked for deletion: {0}, {1}")]
    ServiceMarkedForDelete(u32, String),
    #[error("Unknown error: {0}, {1}")]
    Unknown(u32, String),
}

impl From<(u32, String)> for CreateServiceError {
    fn from(value: (u32, String)) -> Self {
        let (err, display) = value;
        match err {
            ERROR_ACCESS_DENIED => Self::AccessDenied(err, display),
            ERROR_CIRCULAR_DEPENDENCY => Self::CircularDependency(err, display),
            ERROR_DUPLICATE_SERVICE_NAME => Self::ServiceExists(err, display),
            ERROR_INVALID_HANDLE => Self::InvalidHandle(err, display),
            ERROR_INVALID_NAME => Self::InvalidName(err, display),
            ERROR_INVALID_PARAMETER => Self::InvalidParameter(err, display),
            ERROR_INVALID_SERVICE_ACCOUNT => Self::InvalidServiceAccount(err, display),
            ERROR_SERVICE_EXISTS => Self::ServiceExists(err, display),
            ERROR_SERVICE_MARKED_FOR_DELETE => Self::ServiceMarkedForDelete(err, display),
            _ => Self::Unknown(err, display),
        }
    }
}

#[derive(Error, Debug)]
pub enum ServiceManagerError {
    #[error("Access denied: {0}, {1}")]
    AccessDenied(u32, String),
    #[error("Circular dependency error: {0}, {1}")]
    DatabaseDoesNotExist(u32, String),
    #[error("Invalid Service Manager handle: {0}, {1}")]
    InvalidHandle(u32, String),
    #[error("Unknown error: {0}, {1}")]
    Unknown(u32, String),
}

impl From<(u32, String)> for ServiceManagerError {
    fn from(value: (u32, String)) -> Self {
        let (err, display) = value;
        match err {
            ERROR_ACCESS_DENIED => Self::AccessDenied(err, display),
            ERROR_DATABASE_DOES_NOT_EXIST => Self::DatabaseDoesNotExist(err, display),
            ERROR_INVALID_HANDLE => Self::DatabaseDoesNotExist(err, display),
            _ => Self::Unknown(err, display),
        }
    }
}
#[derive(Error, Debug)]
pub enum ControlServiceError {
    #[error("Access denied: {0}, {1}")]
    AccessDenied(u32, String),
    #[error("Invalid handle: {0}, {1}")]
    InvalidHandle(u32, String),
    #[error("Path not found: {0}, {1}")]
    PathNotFound(u32, String),
    #[error("Service already running: {0}, {1}")]
    ServiceAlreadyRunning(u32, String),
    #[error("Service not active: {0}, {1}")]
    ServiceNotActive(u32, String),
    #[error("Service database locked: {0}, {1}")]
    ServiceDatabaseLocked(u32, String),
    #[error("Service dependency deleted: {0}, {1}")]
    ServiceDependencyDeleted(u32, String),
    #[error("Service dependency failed: {0}, {1}")]
    ServiceDependencyFail(u32, String),
    #[error("Service disabled: {0}, {1}")]
    ServiceDisabled(u32, String),
    #[error("Service logon failed: {0}, {1}")]
    ServiceLogonFailed(u32, String),
    #[error("Service marked for deletion: {0}, {1}")]
    ServiceMarkedForDelete(u32, String),
    #[error("Service no thread: {0}, {1}")]
    ServiceNoThread(u32, String),
    #[error("Service request timeout: {0}, {1}")]
    ServiceRequestTimeout(u32, String),
    #[error("Unknown error: {0}, {1}")]
    Unknown(u32, String),
}

impl From<(u32, String)> for ControlServiceError {
    fn from(value: (u32, String)) -> Self {
        let (err, display) = value;
        match err {
            ERROR_ACCESS_DENIED => Self::AccessDenied(err, display),
            ERROR_INVALID_HANDLE => Self::InvalidHandle(err, display),
            ERROR_PATH_NOT_FOUND | ERROR_FILE_NOT_FOUND => Self::PathNotFound(err, display),
            ERROR_SERVICE_ALREADY_RUNNING => Self::ServiceAlreadyRunning(err, display),
            ERROR_SERVICE_NOT_ACTIVE => Self::ServiceNotActive(err, display),
            ERROR_SERVICE_DATABASE_LOCKED => Self::ServiceDatabaseLocked(err, display),
            ERROR_SERVICE_DEPENDENCY_DELETED => Self::ServiceDependencyDeleted(err, display),
            ERROR_SERVICE_DEPENDENCY_FAIL => Self::ServiceDependencyFail(err, display),
            ERROR_SERVICE_DISABLED => Self::ServiceDisabled(err, display),
            ERROR_SERVICE_LOGON_FAILED => Self::ServiceLogonFailed(err, display),
            ERROR_SERVICE_MARKED_FOR_DELETE => Self::ServiceMarkedForDelete(err, display),
            ERROR_SERVICE_NO_THREAD => Self::ServiceNoThread(err, display),
            ERROR_SERVICE_REQUEST_TIMEOUT => Self::ServiceRequestTimeout(err, display),
            _ => Self::Unknown(err, display),
        }
    }
}

#[derive(Error, Debug)]
pub enum DeleteServiceError {
    #[error("Access denied: {0}, {1}")]
    AccessDenied(u32, String),
    #[error("The specified service has already been marked for deletion: {0}, {1}")]
    ErrorServiceMarkedForDelete(u32, String),
    #[error("Invalid handle: {0}, {1}")]
    InvalidHandle(u32, String),
    #[error("Unknown error: {0}, {1}")]
    Unknown(u32, String),
}

impl From<(u32, String)> for DeleteServiceError {
    fn from(value: (u32, String)) -> Self {
        let (err, display) = value;
        match err {
            ERROR_ACCESS_DENIED => Self::AccessDenied(err, display),
            ERROR_SERVICE_MARKED_FOR_DELETE => Self::ErrorServiceMarkedForDelete(err, display),
            ERROR_INVALID_HANDLE => Self::InvalidHandle(err, display),
            _ => Self::Unknown(err, display),
        }
    }
}


#[derive(Error, Debug)]
pub enum UpdateServiceError {
    #[error("Access denied: {0}, {1}")]
    AccessDenied(u32, String),
    #[error("Circular dependency error: {0}, {1}")]
    CircularDependency(u32, String),
    #[error("Duplicate service name: {0}, {1}")]
    DuplicateServiceName(u32, String),
    #[error("Invalid handle: {0}, {1}")]
    InvalidHandle(u32, String),
    #[error("Invalid parameter: {0}, {1}")]
    InvalidParameter(u32, String),
    #[error("Invalid service account: {0}, {1}")]
    InvalidServiceAccount(u32, String),
    #[error("Service marked for deletion: {0}, {1}")]
    ServiceMarkedForDelete(u32, String),
    #[error("Unknown error: {0}, {1}")]
    Unknown(u32, String),
}

impl From<(u32, String)> for UpdateServiceError {
    fn from(value: (u32, String)) -> Self {
        let (err, display) = value;
        match err {
            ERROR_ACCESS_DENIED => Self::AccessDenied(err, display),
            ERROR_CIRCULAR_DEPENDENCY => Self::CircularDependency(err, display),
            ERROR_DUPLICATE_SERVICE_NAME => Self::DuplicateServiceName(err, display),
            ERROR_INVALID_HANDLE => Self::InvalidHandle(err, display),
            ERROR_INVALID_PARAMETER => Self::InvalidParameter(err, display),
            ERROR_INVALID_SERVICE_ACCOUNT => Self::InvalidServiceAccount(err, display),
            ERROR_SERVICE_MARKED_FOR_DELETE => Self::ServiceMarkedForDelete(err, display),
            _ => Self::Unknown(err, display),
        }
    }
}


#[derive(Error, Debug)]
pub enum QueryServiceError {
    #[error("Access denied: {0}, {1}")]
    AccessDenied(u32, String),
    #[error("Invalid service handle: {0}, {1}")]
    InvalidHandle(u32, String),
    #[error("Unknown error: {0}, {1}")]
    Unknown(u32, String),
}

impl From<(u32, String)> for QueryServiceError {
    fn from(value: (u32, String)) -> Self {
        let (err, display) = value;
        match err {
            ERROR_ACCESS_DENIED => Self::AccessDenied(err, display),
            ERROR_INVALID_HANDLE => Self::InvalidHandle(err, display),
            _ => Self::Unknown(err, display),
        }
    }
}
