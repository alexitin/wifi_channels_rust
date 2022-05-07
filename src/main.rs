use std::process;
use wifinsa::{AllDevices, WifiDevice};

fn main() {

// Get list all net devices.
    let devices = AllDevices::new().unwrap_or_else(|err| {
        println!("Problem get list all net devices: {}", err);
        process::exit(1);
    });

// Get device supporting monitor, promiscouos or normal mode.
    let wifi_device = devices.get_wifi_device();

    match wifi_device.device {
        Some(device) => {
            println!("Device: {}, Mode: {},", wifi_device.name, wifi_device.mode);
            WifiDevice::get_frame(device);
        },
        None => {
            println!("Not found wifi devices. Scan of channels not posible.
NOTE 1. For promiscuous or normal mode require enable wifi device and connect to wlan.
NOTE 2. Sometimes superuser rights are needed, try using sudo.");
            process::exit(1);
        }
    }
}