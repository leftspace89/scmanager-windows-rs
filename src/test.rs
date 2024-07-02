#[cfg(test)]
#[cfg(feature = "tests")]
mod tests {
    use crate::{
        service::{ServiceStartType, ServiceState, ServiceType},
        service_manager::{ServiceConfig, ServiceManager},
    };

    use anyhow::Result;

    #[test]
    fn test_all() -> Result<()> {
        println!("Test Begin!");
        let service_manager = ServiceManager::new()?;

        let service_name = "test";
        let service_path = r"C:\Windows\system32\test.sys";

        let service_handle = service_manager.create_or_get(ServiceConfig {
            service_name: service_name.to_string(),
            display_name: service_name.to_string(),
            binary_path: "invalid path test".to_string(),
            start_type: ServiceStartType::DemandStart,
            service_type: ServiceType::KernelDriver,
            ..Default::default()
        })?;

        assert_eq!(
            service_handle.get_start_type()?,
            ServiceStartType::DemandStart
        );

        service_handle.update_config(ServiceConfig {
            display_name: service_name.to_string(),
            binary_path: service_path.to_string(),
            service_type: ServiceType::KernelDriver,
            ..Default::default()
        })?;
        service_handle.start_blocking()?;

        assert_eq!(service_handle.state()?, ServiceState::Running);

        service_handle.stop_blocking()?;

        assert_eq!(service_handle.state()?, ServiceState::Stopped);

        std::thread::sleep(std::time::Duration::from_secs(2));

        service_handle.delete()?;

        println!("Test End!");

        Ok(())
    }
}
