use std::process;
use wifinsa::AllDevices;

fn main() {

// Get list all net devices.
    let devices = AllDevices::new().unwrap_or_else(|err| {
        println!("Problem get list all net devices: {}", err);
        process::exit(1);
    });

// Get device supporting monitor mode.
    let wifi_device = devices.get_wifi_device();

//If not found devices suppoted monitor or promiscuous mode.
    if wifi_device.device.is_none() {
        println!("Not found wifi devices. Scan of channels not posible.
NOTE 1. For promiscuous or normal mode require enable wifi device and connect to wlan.
NOTE 2. Sometimes superuser rights are needed, try using sudo.");
        process::exit(1);
    } else {
//        WifiDevice::get_frame(capture_device);
        println!("Mode: {},",
//        wifi_device.device,
        wifi_device.mode); 
//        capture_device.linktype);
    }
}