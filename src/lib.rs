use pcap::{Capture, Device, Linktype, Active};
use std::io;
use std::process;

pub struct AllDevices {
    devices: Vec<Device>
}
pub struct WifiDevice {
    pub device: Option<Capture<Active>>,
    pub mode: String,
}

impl AllDevices {

    pub fn new () -> Result<AllDevices, pcap::Error> {
        let devices =  Device::list()?; {
            Ok(AllDevices {devices})
        }
    }

    pub fn get_wifi_device(self) -> WifiDevice {

// Check all devices for monitor mode compatibility and use the first match
        let device = self.devices.iter()
            .find_map(|dev| set_monitor_mode(dev).ok());
//        let device = None;

        if device.is_some() {
            WifiDevice {
                device,
                mode: "monitor".to_string()
            }
        } else {

// Selecting devices connected to the local network
            let devices = choice_device(self.devices);

// Check device for promiscouos mode and set it
            let device = set_promiscouos_mode(&devices).ok();

            if device.is_some() {
                WifiDevice {
                    device,
                    mode:"promiscouos".to_string()
                }
            } else {
// Check device for normal mode and set it
                let device = set_normal_mode(&devices).ok();
                WifiDevice {
                    device,
                    mode: "normal".to_string()
                }
            }
        }
    }
}

impl WifiDevice {
    pub fn get_frame(device: Capture<Active>) {
        device = set_linktype(device);
    }
}

fn set_monitor_mode (dev: &Device) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev.clone())
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .rfmon(true)
        .snaplen(128)
        .buffer_size(256)
        .timeout(10000)
        .open()
}

fn set_promiscouos_mode (dev: &Device) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev.clone())
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .promisc(true)
        .snaplen(128)
        .buffer_size(256)
        .timeout(10000)
        .open()
}

fn set_normal_mode (dev: &Device) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev.clone())
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .snaplen(128)
        .buffer_size(256)
        .timeout(10000)
        .open()
}

fn choice_device(devices: Vec<Device>) -> Device {
    let mut i = 0;
    for dev in &devices {
        i += 1;
        match dev.addresses.len() {
            0 => println!("{}. Device: {} - not connect", &i, &dev.name),
            1 => println!("{}. Device: {} - IP: {}", i, &dev.name, &dev.addresses[0].addr),
            _ => println!("{}. Device: {} - IP: {}", i, &dev.name, &dev.addresses[1].addr),
        }
    }
    println!("Choose wlan device, or press any key to quit:");
    let buf = loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .unwrap_or_else(|err| {
                println!("Failed read yuor choice: {}", err);
                process::exit(1)
            });
        let buf = buf.trim().parse::<usize>().unwrap_or_else(|_| {
            println!("Bye!");
            process::exit(1)
        });
        if buf > devices.len() {
            println!("Incorrect choice: {}", &buf);
            continue;
        } else {
            break buf
        };
    };
    devices[buf - 1].clone()
}

fn set_linktype(mut device: Capture<Active>) -> Result<String, pcap::Error> { 
    if device.set_datalink(Linktype::PPI).is_ok() {
        Linktype::PPI.get_name()
    } else if device.set_datalink(Linktype::IEEE802_11_AVS).is_ok() {
        Linktype::IEEE802_11_AVS.get_name()
    } else if device.set_datalink(Linktype::IEEE802_11_RADIOTAP).is_ok() {
        Linktype::IEEE802_11_RADIOTAP.get_name()
    } else if device.set_datalink(Linktype::IEEE802_11_PRISM).is_ok() {
        Linktype::IEEE802_11_PRISM.get_name()
    } else if device.set_datalink(Linktype::IEEE802_11).is_ok() {
        Linktype::IEEE802_11.get_name()
    } else if device.set_datalink(Linktype::ETHERNET).is_ok() {
        Linktype::ETHERNET.get_name()
    } else {
        println!("Not posible capture wifi packets");
        process::exit(1);
    }
}