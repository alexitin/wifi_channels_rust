//use std::process;

//use crate::frame::WifiDevice;

mod device;
mod frame;
mod parse_radiotap;

fn main() {

// Get list all net devices.
    let devices = device::AllDevices::new();

// Get device supporting monitor, promiscouos or normal mode.
    let wifi_device = devices.get_wifi_device();

    let air_noise = match wifi_device.mode {
        device::DeviceMode::Monitor => wifi_device.scan_channels_monitor(),
        device::DeviceMode::Promiscouos => wifi_device.scan_channels_promiscouos(),
        device::DeviceMode::Normal => wifi_device.scan_channels_normal(),
    };
    air_noise.show(&wifi_device);
}