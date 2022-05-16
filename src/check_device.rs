use pcap::{Capture, Device, Linktype, Active};
use std::{io, process};

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
        .snaplen(256)
        .buffer_size(256)
        .timeout(10000)
        .open()
}

pub fn choice_device(devices: Vec<Device>) -> Device {
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

pub fn get_linktype(mut device: Capture<Active>) -> Capture<Active> {
    if device.set_datalink(Linktype::IEEE802_11_RADIOTAP).is_ok() {
        device
    } else if device.set_datalink(Linktype::IEEE802_11_AVS).is_ok() {
        device
    } else if device.set_datalink(Linktype::IEEE802_11_PRISM).is_ok() {
        device
    } else if device.set_datalink(Linktype::PPI).is_ok() {
        device
    } else if device.set_datalink(Linktype::IEEE802_11).is_ok() {
        device
    } else if device.set_datalink(Linktype::ETHERNET).is_ok() {
        device
    } else {
        println!("Not one of the DLTs not supported by this device. Not posible capture wifi packets");
        process::exit(1);
    }
}