use std::{time::Duration, thread, process, collections::BTreeMap, ffi::CString};

use pcap::{Linktype, Capture, Active};

use crate::{device, parse_radiotap, show::AirNoise, parse_avs};

pub struct WifiDevice {
    pub name: String,
    pub mode: device::DeviceMode,
    pub linktype: Linktype
}

pub struct NetSignals {
    pub channel: String,
    pub ssid_signal: BTreeMap<String, i32>,
}

extern "C" {
    fn mac_select_channel(ptr_name: *const i8, channel: isize) -> isize;
}


impl WifiDevice {
    pub fn scan_channels_monitor(&self) -> AirNoise {
        let time_select = Duration::new(1, 0);
        thread::sleep(time_select);

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

            radio_air.push(net_signal);
        }

        AirNoise {
            radio_air,
        }
    }

    pub fn scan_channels_promiscouos(&self) -> AirNoise {
        let time_select = Duration::new(1, 0);
        thread::sleep(time_select);
        
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
        let time_select = Duration::new(1, 0);
        thread::sleep(time_select);

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

fn get_frames(mut device: Capture<Active>, linktype: Linktype) -> NetSignals {
    device.filter("type mgt subtype beacon", false).expect("need oter linktype for BPF");
    match linktype {
        Linktype(127) => parse_radiotap::frames_data_radiotap(device),
        Linktype(163) => parse_avs::frames_data_avs(device),
        _ => {

            NetSignals {
                channel: "".to_string(),
                ssid_signal: BTreeMap::new(),
            }
        }
    }
}