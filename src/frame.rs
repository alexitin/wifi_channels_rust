use std::{time::Duration, thread, collections::HashMap, ffi::CString, sync::mpsc};

use cursive::Cursive;

use pcap::{Linktype, Capture, Active};

use crate::{device, parse_radiotap, parse_avs, parse_ppi, parse_80211, show};
use crate::{selector::SelectorChannel, selector::error::ErrorSelector};

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
    pub fn scan_channels(wifi_device: device::WifiDevice, tx2: mpsc::Sender<isize>, cb_sink2: cursive::CbSink) -> AirNoise {
        thread::sleep(Duration::from_secs(1));

        let mut net_signals: Vec<NetSignals> = Vec::with_capacity(12);

        let b_name = wifi_device.name.as_bytes().to_vec();
        let c_name = CString::new(b_name).unwrap();
        let ptr_name = c_name.as_ptr();

        match wifi_device.mode {
            device::DeviceMode::Monitor => {
                (1..14).for_each(|channel| {
                    let status_select = SelectorChannel::set_channel(ptr_name, channel);
                    if status_select.is_ok() {
                        if let Err(err) = tx2.send(channel) {
                            let text = format!("Problem sending info: {err}");
                            cb_sink2.send(Box::new(move |s| show::exit_cursive(s, &text))).unwrap();
                            thread::park();
                        }
                        cb_sink2.send(Box::new(Cursive::noop)).unwrap();
                        let mut capture_device = device::set_monitor_mode(&wifi_device.name).unwrap();
                        capture_device.set_datalink(wifi_device.linktype).expect("cheking early");
                        let ssid_rssi = get_frames(capture_device, wifi_device.linktype);
                        let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
                        net_signals.push(channel_ssid_rssi);
                    } else {
                        let status_err = status_select.err().unwrap();
                        if status_err != ErrorSelector::NotSupportChannel {
                            let text = format!("Problem of slector channel: {status_err}");
                            cb_sink2.send(Box::new(move |s| show::exit_cursive(s, &text))).unwrap();
                            thread::park();
                        }
                    }
                });

                #[cfg(target_os = "linux")]
                SelectorChannel::set_managed_mode(ptr_name);
            },
            device::DeviceMode::Promiscouos => {
                let status_channel = SelectorChannel::get_channel(ptr_name);
                if status_channel.is_err() {
                    let status_err = status_channel.err().unwrap();
                    let text = format!("Problem of slector channel: {status_err}");
                    cb_sink2.send(Box::new(move |s| show::exit_cursive(s, &text))).unwrap();
                    thread::park();
                };
                let channel = status_channel.unwrap();
                if let Err(err) = tx2.send(channel) {
                    let text = format!("Problem sending info: {err}");
                    cb_sink2.send(Box::new(move |s| show::exit_cursive(s, &text))).unwrap();
                    thread::park();
                }
                cb_sink2.send(Box::new(Cursive::noop)).unwrap();
                let mut capture_device = device::set_promiscouos_mode(&wifi_device.name).unwrap();
                capture_device.set_datalink(wifi_device.linktype).expect("cheking early");
                let ssid_rssi = get_frames(capture_device, wifi_device.linktype);
                let channel_ssid_rssi = NetSignals {channel, ssid_rssi};
                net_signals.push(channel_ssid_rssi);
            },
            device::DeviceMode::Normal => {
                let status_channel = SelectorChannel::get_channel(ptr_name);
                if status_channel.is_err() {
                    let status_err = status_channel.err().unwrap();
                    let text = format!("Problem of slector channel: {status_err}");
                    cb_sink2.send(Box::new(move |s| show::exit_cursive(s, &text))).unwrap();
                    thread::park();
                };
                let channel = status_channel.unwrap();
                if let Err(err) = tx2.send(channel) {
                    let text = format!("Problem sending info: {err}");
                    cb_sink2.send(Box::new(move |s| show::exit_cursive(s, &text))).unwrap();
                    thread::park();
                }
                cb_sink2.send(Box::new(Cursive::noop)).unwrap();
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
