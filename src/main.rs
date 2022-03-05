use pcap::Device;
use std::process;
use wifinsa::Captured;

fn main() {
// Get list all net devices.
    let devices = match Device::list() {
        Ok(dev) => dev,
        Err(err) => panic!("Problem get list devices: {}", err),
    };

// Get device supporting monitor mode.
    let mut capture_device = Captured::get_monitor_device(devices.clone());

//If not found wifi devices capable of operating in monitor mode,
//get device supporting promiscuous mode.
    if capture_device.device.is_none() {
    capture_device = Captured::get_promiscuous_device(devices);
    }
    println!("Device: {:?},\nMode: {},\nLinktype {:?}",
        capture_device.device,
        capture_device.mode, 
        capture_device.linktype
    );

//If not found devices suppoted monitor or promiscuous mode.
    if capture_device.device.is_none() {
        println!("Not found wifi devices capable of operating in monitor or promiscuous mode.
                Scan of channels not posible.");
        process::exit(1);
    }
}