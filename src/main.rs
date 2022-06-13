use std::process;
use wifinsa::{AllDevices, WifiDevice, DeviceMode};

fn main() {

// Get list all net devices.
    let devices = AllDevices::new().unwrap_or_else(|err| {
        println!("Problem get list all net devices: {}", err);
        process::exit(1);
    });

// Get device supporting monitor, promiscouos or normal mode.
    let wifi_device = devices.get_wifi_device();

    match wifi_device.mode {
        DeviceMode::Monitor => {
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