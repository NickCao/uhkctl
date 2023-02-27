use hidapi::{DeviceInfo, HidApi};

pub mod consts;
pub mod device;
pub mod models;

pub fn devices(api: &HidApi) -> Vec<&DeviceInfo> {
    let devices = api
        .device_list()
        .filter(|dev| dev.vendor_id() == models::UHK_VENDOR_ID && dev.interface_number() == 0)
        .collect();
    log::debug!("Found UHK devices: {:?}", devices);
    devices
}
