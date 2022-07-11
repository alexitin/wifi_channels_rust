mod device;
mod frame;
mod parse_radiotap;
mod parse_avs;
mod parse_ppi;
mod parse_80211;
mod show;

fn main() {

// Get list all net devices.
    let devices = device::AllDevices::new();

// Get device supporting monitor, promiscouos or normal mode.
    let wifi_device = devices.get_wifi_device();

// Scan channels
    let air_noise = match wifi_device.mode {
        device::DeviceMode::Monitor => wifi_device.scan_channels_monitor(),
        device::DeviceMode::Promiscouos => wifi_device.scan_channels_promiscouos(),
        device::DeviceMode::Normal => wifi_device.scan_channels_normal(),
    };

// Show results
    air_noise.show(&wifi_device);
}