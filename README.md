# Why I Made This Crate
* I am trying to learn Rust as much as I can, so here I am.
* I couldn't find any crate that does a similar thing.
* It has some missing cases and is not fully implemented yet.
* It does what I need, and I think it will help others with basic usage.
* Yes, the documentation isn't great. Sorry about that.
# Example
* The function names are self-explanatory, but here are some examples for convenience (for those like me).

```rust

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
```
