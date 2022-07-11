use std::{time::Duration, thread, process, collections::BTreeMap, ffi::CString};

use pcap::{Linktype, Capture, Active};

use crate::{device, parse_radiotap, parse_avs, parse_ppi, parse_80211, show};

pub struct WifiDevice {
    pub name: String,
    pub mode: device::DeviceMode,
    pub linktype: Linktype
}

pub struct NetSignals {
    pub channel: isize,
    pub ssid_rssi: BTreeMap<String, i32>,
}

extern "C" {
    fn mac_select_channel(ptr_name: *const i8, channel: isize) -> isize;
}


impl WifiDevice {
    pub fn scan_channels_monitor(&self) -> show::AirNoise {
        let time_select = Duration::new(1, 0);
        thread::sleep(time_select);

        let mut status_select: isize;
        let mut radio_air: Vec<NetSignals> = vec![];

        let b_name = self.name.as_bytes().to_vec();
        let c_name = CString::new(b_name).unwrap();

        let ptr_name = c_name.as_ptr();
        for channel in 1..12  {
//            let channel = i;
            unsafe {
                status_select = mac_select_channel(ptr_name, channel);
            }

            match status_select {
                0 => {
                    let time_select = Duration::new(3, 0);
                    thread::sleep(time_select);
                    println!("Scanning chanel {}", &channel);
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
            let ssid_rssi = get_frames(capture_device, self.linktype);
            let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
            radio_air.push(channel_ssid_rssi);
        }

        show::AirNoise {
            radio_air,
        }
    }

    pub fn scan_channels_promiscouos(&self) -> show::AirNoise {
        let time_select = Duration::new(1, 0);
        thread::sleep(time_select);
        let channel = 1;
        let mut radio_air: Vec<NetSignals> = vec![];
        let mut capture_device = device::set_promiscouos_mode(&self.name).unwrap();
        capture_device.set_datalink(self.linktype).unwrap();
        let ssid_rssi = get_frames(capture_device, self.linktype);
        let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
        radio_air.push(channel_ssid_rssi);
        show::AirNoise {
            radio_air,
        }
    }
    pub fn scan_channels_normal(&self) -> show::AirNoise {
        let time_select = Duration::new(1, 0);
        thread::sleep(time_select);
        let channel = 1;
        let mut radio_air: Vec<NetSignals> = vec![];
        let mut capture_device = device::set_normal_mode(&self.name).unwrap();
        capture_device.set_datalink(self.linktype).unwrap();
        let ssid_rssi = get_frames(capture_device, self.linktype);
        let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
        radio_air.push(channel_ssid_rssi);
        show::AirNoise {
            radio_air,
        }
    }
}

fn get_frames(device: Capture<Active>, linktype: Linktype) -> BTreeMap<String, i32> {

    match linktype {
        Linktype(127) => parse_radiotap::frames_data_radiotap(device),
        Linktype(163) => parse_avs::frames_data_avs(device),
        Linktype(192) => parse_ppi::frames_data_ppi(device),
        Linktype(105) => parse_80211::frames_data_80211(device),
        _ => {
            let ssid_rssi: BTreeMap<String, i32> = BTreeMap::new();
            ssid_rssi
        },
    }
}