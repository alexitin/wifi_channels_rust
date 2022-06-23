use std::{io, process};

use pcap::{Device, Capture, Active, Linktype};

use crate::frame::WifiDevice;

pub enum DeviceMode {
    Monitor,
    Promiscouos,
    Normal,
}

pub struct AllDevices {
    devices: Vec<Device>
}

impl AllDevices {

    pub fn new () -> AllDevices {
        let devices = Device::list().unwrap_or_else(|err| {
            println!("Problem get list all net devices: {}", err);
            process::exit(1)
            });
        AllDevices {
            devices,
        }
    }

    pub fn get_wifi_device(self) -> WifiDevice {

// Check all devices for monitor mode compatibility and use the first match
        if let Some(position) = self.devices.iter()
            .position(|dev| set_monitor_mode(&dev.name).is_ok()) {
            let device = set_monitor_mode(&self.devices[position].name).unwrap();
            let linktype = get_linktype(device);
            let name = self.devices[position].name.to_owned();

            WifiDevice {
                name,
                mode: DeviceMode::Monitor,
                linktype,
            }
        } else {

// Choice devices connected to the local network
            let devices = choice_device(self.devices);
            let name = devices.name.to_owned();

// Check device for promiscouos mode
            let device = set_promiscouos_mode(&devices.name).ok();

            if device.is_some() {
                let linktype = get_linktype(device.unwrap());

                WifiDevice {
                    name,
                    mode: DeviceMode::Promiscouos,
                    linktype,
                }
            } else {
// heck device for normal mode
                let device = set_normal_mode(&devices.name).ok();

                if device.is_some() {
                    let linktype = get_linktype(device.unwrap());

                    WifiDevice {
                        name,
                        mode: DeviceMode::Normal,
                        linktype,
                    }
                } else {
                    println!("Not found wifi devices. Scan of channels not posible.
NOTE 1. For promiscuous or normal mode require enable wifi device and connect to wlan.
NOTE 2. Sometimes superuser rights are needed, try using sudo.");
                    process::exit(1);
                }
            }
        }
    }
}

pub fn set_monitor_mode (dev: &str) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev)
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .rfmon(true)
        .snaplen(256)
        .buffer_size(256)
        .timeout(10000)
        .open()
}

pub fn set_promiscouos_mode (dev: &str) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev)
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .promisc(true)
        .snaplen(256)
        .buffer_size(256)
        .timeout(10000)
        .open()
}

pub fn set_normal_mode (dev: &str) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev)
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .promisc(true)
        .snaplen(256)
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

pub fn get_linktype(mut device: Capture<Active>) -> Linktype {
    if device.set_datalink(Linktype::IEEE802_11_RADIOTAP).is_ok() {
        Linktype(127)
    } else if device.set_datalink(Linktype::IEEE802_11_AVS).is_ok() {
        Linktype(163)
    } else if device.set_datalink(Linktype::IEEE802_11_PRISM).is_ok() {
        Linktype(119)
    } else if device.set_datalink(Linktype::PPI).is_ok() {
        Linktype(192)
    } else if device.set_datalink(Linktype::IEEE802_11).is_ok() {
        Linktype(105)
    } else if device.set_datalink(Linktype::ETHERNET).is_ok() {
        Linktype(1)
    } else {
        println!("Not one of the DLTs not supported by this device. Not posible capture wifi packets");
        process::exit(1);
    }
}