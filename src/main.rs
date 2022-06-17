use std::process;

mod device;
mod frame;
mod parse_radiotap;

fn main() {

// Get list all net devices.
    let devices = device::AllDevices::new();

// Get device supporting monitor, promiscouos or normal mode.
    let wifi_device = devices.get_wifi_device();

    match wifi_device.mode {
        device::DeviceMode::Monitor => {
            println!("Device: {}, Mode: monitor,", wifi_device.name);
            wifi_device.frames_all_channels();
        },
        _ => {
            println!("Not found wifi devices. Scan of channels not posible.
NOTE 1. For promiscuous or normal mode require enable wifi device and connect to wlan.
NOTE 2. Sometimes superuser rights are needed, try using sudo.");
            process::exit(1);
        }
    }
}