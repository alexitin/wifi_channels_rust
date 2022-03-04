use pcap::Device;
use wifinsa::Captured;
use std::process;

fn main() {
// Get list all net devices.
    let devices = match Device::list() {
        Ok(dev) => dev,
        Err(err) => panic!("Problem get list devices: {}", err),
    };

    // Get wlan device and check them for supporting monitor, promiscuous or immediate mode,
// and enable if it possible.
    let capture_device = Captured::get_device(devices);
    println!("Device: {:?}, Mode: {}, Linktype {:?}",
        capture_device.device,
        capture_device.mode, 
        capture_device.linktype
    );
    if capture_device.device.is_none() {
        println!("Not found wifi devices capable of operating in monitor or promiscuous mode.
            Scan of channels not posible.");
        process::exit(1);
    }
}