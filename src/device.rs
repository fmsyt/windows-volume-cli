use windows::Devices::Enumeration::{DeviceClass, DeviceInformation};

pub async fn print_devices() {
    let devices = DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::AudioRender)
        .expect("Failed to get devices")
        .await
        .expect("Failed to get devices");

    for device in devices {
        let name = device.Name().unwrap().to_os_string().into_string().unwrap();
        println!("{}", name);
    }
}
