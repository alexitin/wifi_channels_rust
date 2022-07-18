mod device;
mod selector;
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
    let air_noise = wifi_device.scan_channels();

// Show results
    air_noise.show(&wifi_device);
}