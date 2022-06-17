use std::{time::Duration, thread, process, collections::BTreeMap, ffi::CString};

use pcap::{Linktype, Capture, Active};

use crate::{device::DeviceMode, parse_radiotap};

pub struct WifiDevice {
    pub name: String,
    pub mode: DeviceMode,
}

pub struct NetSignals {
    pub channel: String,
    pub linktype: String,
    pub ssid_signal: BTreeMap<String, i32>,
}

extern "C" {
    fn mac_select_channel(ptr_name: *const i8, channel: isize) -> isize;
}


impl WifiDevice {
    pub fn frames_all_channels(self) {
        let mut status_select: isize;

        let b_name = self.name.as_bytes().to_vec();
        let c_name = CString::new(b_name).unwrap();

        let ptr_name = c_name.as_ptr();
        for i in 1..12  {
            let channel = i;
            unsafe {
                status_select = mac_select_channel(ptr_name, channel);
            }

            match status_select {
                0 => {
                    let time_select = Duration::new(3, 0);
                    thread::sleep(time_select);
                },
                1 => {
                    println!("Problem enable WiFi device");
                    process::exit(1);
                },
                2 => {
                    println!("Problem set channel WiFi device");
                    process::exit(1);
                },
                3 => {
                    println!("Problem no find in list selected channel for WiFi device");
                    process::exit(1);
                },
                4 => {
                    println!("Problem no get list supported WiFi device");
                    process::exit(1);
                },
                _ => {
                    println!("Problem no get interface of WiFi device");
                    process::exit(1);
                },
            }

            println!("From swift - {}", status_select);



            let net_signal = WifiDevice::get_frames(&self.name);
            println!("Linktype: {}.\nChannel: {}", net_signal.linktype, net_signal.channel);
            println!("{:?}", net_signal.ssid_signal)
        }
    }
    pub fn get_frames(name: &str) -> NetSignals {
        let mut device = crate::device::set_monitor_mode(name).unwrap();
        device = get_linktype(device);
        match device.get_datalink() {
            Linktype(127) => parse_radiotap::frames_data(device),
            _ => {
                println!("Todo next, linktype: {:?}", device.get_datalink());
                NetSignals {
                    channel: "".to_string(),
                    linktype: "".to_string(),
                    ssid_signal: BTreeMap::new(),
                }
            }
        }
    }
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