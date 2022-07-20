use std::{io, process};

use pcap::{Device, Capture, Active};

use crate::frame;

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

    pub fn get_wifi_device(self) -> frame::WifiDevice {

// Check all devices for monitor mode compatibility and use the first match
        if let Some(position) = self.devices.iter()
            .position(|dev| set_monitor_mode(&dev.name).is_ok()) {
            let name = self.devices[position].name.to_owned();

            frame::WifiDevice {
                name,
                mode: DeviceMode::Monitor,
            }
        } else {

// Choice devices connected to the local network
            let devices = choice_device(self.devices);
            let name = devices.name.to_owned();

// Check device for promiscouos mode
            let device = set_promiscouos_mode(&devices.name).ok();
            if device.is_some() {

                frame::WifiDevice {
                    name,
                    mode: DeviceMode::Promiscouos,
                }
            } else {
// Check device for normal mode
                let device = set_normal_mode(&devices.name).ok();
                if device.is_some() {

                    frame::WifiDevice {
                        name,
                        mode: DeviceMode::Normal,
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
        .timeout(900)
        .open()
}

pub fn set_promiscouos_mode (dev: &str) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev)
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .promisc(true)
        .snaplen(256)
        .buffer_size(256)
        .timeout(900)
        .open()
}

pub fn set_normal_mode (dev: &str) -> Result<Capture<Active>, pcap::Error> {
    Capture::from_device(dev)
        .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
        .promisc(true)
        .snaplen(256)
        .buffer_size(256)
        .timeout(900)
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
