use std::{time::Duration, thread, process, collections::BTreeMap, ffi::CString, io};

use pcap::{Linktype, Capture, Active};

use crate::{device, parse_radiotap};

pub struct WifiDevice {
    pub name: String,
    pub mode: device::DeviceMode,
    pub linktype: Linktype
}

pub struct NetSignals {
    pub channel: String,
    pub ssid_signal: BTreeMap<String, i32>,
}

pub struct AirNoise {
    radio_air: Vec<NetSignals>,
}

extern "C" {
    fn mac_select_channel(ptr_name: *const i8, channel: isize) -> isize;
}


impl WifiDevice {
    pub fn scan_channels_monitor(&self) -> AirNoise {
        let mut status_select: isize;
        let mut radio_air: Vec<NetSignals> = vec![];

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
                    println!("Scanning chanel {}", &i);
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
                    println!("Problem no find in list supported channels for WiFi device");
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

            let mut capture_device = device::set_monitor_mode(&self.name).unwrap();
            capture_device.set_datalink(self.linktype).unwrap();
            let net_signal = get_frames(capture_device, self.linktype);

            radio_air.push(net_signal)
        }

        AirNoise {
            radio_air,
        }
    }

    pub fn scan_channels_promiscouos(&self) -> AirNoise {
        let mut radio_air: Vec<NetSignals> = vec![];
        let mut capture_device = device::set_promiscouos_mode(&self.name).unwrap();
        capture_device.set_datalink(self.linktype).unwrap();
        let net_signal = get_frames(capture_device, self.linktype);
        radio_air.push(net_signal);
        AirNoise {
            radio_air,
        }
    }
    pub fn scan_channels_normal(&self) -> AirNoise {
        let mut radio_air: Vec<NetSignals> = vec![];
        let mut capture_device = device::set_normal_mode(&self.name).unwrap();
        capture_device.set_datalink(self.linktype).unwrap();
        let net_signal = get_frames(capture_device, self.linktype);
        radio_air.push(net_signal);
        AirNoise {
            radio_air,
        }
    }
}

fn get_frames(device: Capture<Active>, linktype: Linktype) -> NetSignals {
    match linktype {
        Linktype(127) => parse_radiotap::frames_data(device),
        _ => {

            NetSignals {
                channel: "".to_string(),
                ssid_signal: BTreeMap::new(),
            }
        }
    }
}

impl AirNoise {
    pub fn show(self, wifi_device: &WifiDevice) {
        let mut list_ssid: Vec<&String> = Vec::new();
        for air in &self.radio_air {
            let ssid_set: Vec<_> = air.ssid_signal.keys().clone().collect();
            for ssid in ssid_set {
                if !list_ssid.contains(&ssid) {list_ssid.push(ssid)}
            }
        
        }
        let home_ssid = choice_home_ssid(list_ssid);

        let mode = match wifi_device.mode {
            device::DeviceMode::Monitor => "monitor".to_owned(),
            device::DeviceMode::Promiscouos => "promiscous".to_owned(),
            device::DeviceMode::Normal => "normal".to_owned(),
            
        };
        let linktype = wifi_device.linktype.get_name().unwrap().to_owned();
        println!("Device: {}, Mode: {}, Linktype: {}", wifi_device.name, mode, linktype);
        for mut air in self.radio_air { 
            air.ssid_signal.remove_entry(&home_ssid);
            let number_ap = air.ssid_signal.len();
            let mut signals: Vec<_> = air.ssid_signal.values().cloned().collect();
            signals.sort();
            let signal_max = signals.last().unwrap_or(&0);
            println!("Chanel: {}, Namber acces point: {}, Max signal, dB: {};", air.channel, number_ap, signal_max);
        }
    }
}

fn choice_home_ssid(list_ssid: Vec<&String>) -> String {
    let mut i = 0;
    for ssid in &list_ssid {
        i += 1;
        println!("{}. {}", &i, ssid);
    }
    println!("Choose your home ssid:");

    let buf = loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .unwrap_or_else(|err| {
                println!("Failed read yuor choice: {}", err);
                process::exit(1)
            });
        let buf = match buf.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect choice");
                continue;
            },
        };
        if buf > list_ssid.len() {
            println!("Incorrect choice: {}", &buf);
            continue;
        } else {
            break buf
        };
    };
    list_ssid[buf - 1].to_owned()
}