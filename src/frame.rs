use std::{time::Duration, thread, collections::HashMap, ffi::CString, sync::mpsc};

use cursive::Cursive;
use pcap::{Linktype, Capture, Active};

use crate::{device, parse_radiotap, parse_avs, parse_ppi, parse_80211, selector::SelectorChannel};

#[derive(Debug, Clone)]
pub struct AirNoise {
    pub net_signals: Vec<NetSignals>,
}

#[derive(Debug, Clone)]
pub struct NetSignals {
    pub channel: isize,
    pub ssid_rssi: HashMap<String, i32>,
}

impl AirNoise {
    pub fn scan_channels(wifi_device: device::WifiDevice, tx2: mpsc::Sender<isize>, cb_sink: cursive::CbSink) -> AirNoise {
        thread::sleep(Duration::from_secs(1));

        let mut net_signals: Vec<NetSignals> = Vec::with_capacity(12);

        let b_name = wifi_device.name.as_bytes().to_vec();
        let c_name = CString::new(b_name).unwrap();
        let ptr_name = c_name.as_ptr();

        match wifi_device.mode {
            device::DeviceMode::Monitor => {
                (1..13).for_each(|channel| {
                    let _status_select = SelectorChannel::set_channel(ptr_name, channel);
                    tx2.send(channel).unwrap();
                    cb_sink.send(Box::new(Cursive::noop)).unwrap();
                    let mut capture_device = device::set_monitor_mode(&wifi_device.name).unwrap();
                    capture_device.set_datalink(wifi_device.linktype).expect("cheking early");
                    let ssid_rssi = get_frames(capture_device, wifi_device.linktype);
                    let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
                    net_signals.push(channel_ssid_rssi);
                });
                #[cfg(target_os = "linux")]
                SelectorChannel::set_managed_mode(ptr_name);
            },
            device::DeviceMode::Promiscouos => {
                let channel = SelectorChannel::get_channel(ptr_name);
                tx2.send(channel).unwrap();
                cb_sink.send(Box::new(Cursive::noop)).unwrap();
                let mut capture_device = device::set_promiscouos_mode(&wifi_device.name).unwrap();
                capture_device.set_datalink(wifi_device.linktype).expect("cheking early");
                let ssid_rssi = get_frames(capture_device, wifi_device.linktype);
                let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
                net_signals.push(channel_ssid_rssi);
            },
            device::DeviceMode::Normal => {
                let channel = SelectorChannel::get_channel(ptr_name);
                tx2.send(channel).unwrap();
                cb_sink.send(Box::new(Cursive::noop)).unwrap();
                let mut capture_device = device::set_normal_mode(&wifi_device.name).unwrap();
                capture_device.set_datalink(wifi_device.linktype).expect("cheking early");
                let ssid_rssi = get_frames(capture_device, wifi_device.linktype);
                let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
                net_signals.push(channel_ssid_rssi);
            },
        };
        AirNoise {
            net_signals,
        }
    }
}

pub fn get_frames(device: Capture<Active>, linktype: Linktype) -> HashMap<String, i32> {
    let ssid_rssi = match linktype {
        Linktype(127) => parse_radiotap::frames_data_radiotap(device),
        Linktype(163) => parse_avs::frames_data_avs(device),
        Linktype(192) => parse_ppi::frames_data_ppi(device),
        Linktype(105) => parse_80211::frames_data_80211(device),
        _ => HashMap::new(),
    };
    ssid_rssi
}
