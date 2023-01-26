use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver};
use std::{thread, process, isize};
use std::ffi::CString;

use cursive::{Cursive, CursiveExt};
use cursive::views::{SelectView, Dialog, TextView, ProgressBar};
use cursive::view::{Nameable, Resizable};
use cursive::traits::*;

use crate::device::WifiDevice;
use crate::{device, frame};
use crate::selector::SelectorChannel;

struct ScanningView {
    msg: String,
    rx: mpsc::Receiver<isize>,
}

impl ScanningView {
    fn new(rx: mpsc::Receiver<isize>) -> Self {
        let msg = "Start scanning channels".to_string();
        ScanningView {msg, rx}
    }

    fn update(&mut self) {
        if let Ok(channel) = self.rx.try_recv() {
            let channel_msg = format!("Scanning channel {}", channel);
            self.msg = channel_msg;
        }
    }
}

impl View for ScanningView {
    fn layout(&mut self, _: cursive::Vec2) {
        self.update();
    }

    fn draw(&self, printer: &cursive::Printer) {
        printer.print((0,printer.size.y - 1), &self.msg);
    }
}

pub fn get_info(s: &mut Cursive, wifi_devices: &device::WifiDevices) {
    let wifi_devices = wifi_devices.to_owned();

    let select_device = SelectView::new()
        .with_all_str(wifi_devices.devices)
        .on_submit(move |s, name: &str| {
            let mode = wifi_devices.mode.unwrap();
            let name = name.to_string();
            let wifi_device = device::WifiDevice::get_wifi_device(name, mode);
            s.set_user_data(wifi_device.clone());
            let device_content = show_device(&wifi_device);
            s.call_on_name("device_info", |view: &mut TextView| {
                view.set_content(device_content);
            }).unwrap();
            s.pop_layer();
            
            let cb_sink1 = s.cb_sink().clone();
            let cb_sink2 = s.cb_sink().clone();

            let (tx1, rx1) = mpsc::channel();
            let (tx2, rx2) = mpsc::channel();

            thread::spawn(move || {
                let a_n = frame::AirNoise::scan_channels(wifi_device, tx2, cb_sink2);
                cb_sink1.send(Box::new(|s| select_id(s, rx1))).unwrap();
                tx1.send(a_n).unwrap();
            });

            let scanning_info = ScanningView::new(rx2);
            s.add_layer(Dialog::around(scanning_info).title("Scanning info").fixed_width(50));

        })
        .with_name("select_device");
    s.add_layer(Dialog::around(select_device)
        .title("Select wifi device")
    );
}

pub fn exit_cursive(siv: &mut Cursive, text: &str) -> ! {
    siv.add_layer(Dialog::text(format!("{}\nPress 'q' to quit", text))
        .title("Error")
        .button("Quit", |s| s.quit()));
    siv.run();
    process::exit(1)
}

fn show_device(wifi_device: &device::WifiDevice) -> String {
    let mode = match wifi_device.mode {
        device::DeviceMode::Monitor => "monitor",
        device::DeviceMode::Promiscouos => "promiscous",
        device::DeviceMode::Normal => "normal",
    };
    let linktype = wifi_device.linktype.get_name().expect("Checked for error before");
    let content = format!("Device: {},  Mode: {},   Link type: {}", wifi_device.name, mode, linktype);
    content
}

fn select_id (s: &mut Cursive, rx1: Receiver<frame::AirNoise>) {
    if let Ok(air_noise) = rx1.try_recv() {
        s.pop_layer();
        let noise_info = show_noise(air_noise);
        s.add_layer(noise_info);
    }
}

fn show_noise(air_noise: frame::AirNoise) -> Dialog {
    let mut list_ssid = air_noise.net_signals.iter()
        .map(|net_signals| net_signals.ssid_rssi
            .keys()
            .collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    let none = "<<None>>".to_string();
    list_ssid.sort();
    list_ssid.dedup();
    list_ssid.insert(0,&none);

    let select_home_net = SelectView::<String>::new()
        .with_all_str(list_ssid)
        .on_submit(move |s, home_ssid: &str| {
            let home_info= air_noise.net_signals.iter()
                .filter(|net| net.ssid_rssi.contains_key(home_ssid))
                .filter(|net| net.ssid_rssi.get(home_ssid) < Some(&0))
                .max_by_key(|net| net.ssid_rssi.get(home_ssid));

            let channels_info: Vec<_> = air_noise.net_signals.iter()
                .map(|net| {
                    let mut n = net.clone();
                    n.ssid_rssi.remove(home_ssid);
                    n
                })
                .collect();
            let mut channels_content: Vec<String> = Vec::with_capacity(12);
            channels_info.iter().for_each(|net_signal| {
                let mut signals: Vec<_> = net_signal.ssid_rssi.values().cloned().collect();
                signals.sort();
                let signal_max = signals.last().unwrap_or(&0);
                let number_ap = net_signal.ssid_rssi.len();
                let unit_measurement = if signal_max < &0 {
                    "dBm".to_owned()
                } else {
                    "     ".to_owned()
                };
                let channel_info = if number_ap != 0 {
                    format!("{:02}{:^20}{} {}", &net_signal.channel, number_ap, signal_max, unit_measurement)
                } else {
                    format!("{:02}{:>27}", &net_signal.channel, "empty")
                };
                channels_content.push(channel_info);
            });
            s.pop_layer();

            s.call_on_name("channels_info", |view: &mut SelectView::<String>| {
                view.add_all_str(channels_content);
                view.set_on_select(move |s, _ch_content| {
                let channel_id = s.find_name::<SelectView<String>>("channels_info").unwrap()
                    .selected_id().unwrap();
                let ssid_rssi = channels_info[channel_id].ssid_rssi.clone();
                show_list_ssid(s, ssid_rssi);
                })
            }).unwrap();

            if let Some(home_info) = home_info {
                let home_channel = home_info.channel;
                let wifi_device = s.take_user_data::<WifiDevice>().unwrap();
                let home_ssid = home_ssid.to_string();
                let cb = s.cb_sink().clone();

                s.call_on_name("home_info", |view: &mut TextView| {
                    let home_content = format!("Home network: {}, Maximamal RSSI detected on channel: {}",
                        &home_ssid, &home_channel);
                    view.set_content(home_content);
                }).unwrap();

                s.call_on_name("home_bar", |view: &mut ProgressBar| {
                    view.set_range(0, 100);
                    view.set_label(|val, (_min, _max)| {
                        format!("{:<80}{}{:>60}", "-100", val as isize - 100, "0 dBm")
                    });
                    view.start(move |counter| {
                        let b_name = wifi_device.name.as_bytes().to_vec();
                        let c_name = CString::new(b_name).unwrap();
                        let ptr_name = c_name.as_ptr();
                        let _status_select = SelectorChannel::set_channel(ptr_name, home_channel);
                        loop {
                            let mut capture_device = device::set_monitor_mode(&wifi_device.name).expect("cheking early");
                            capture_device.set_datalink(wifi_device.linktype).expect("cheking early");
                            let ssid_rssi = frame::get_frames(capture_device, wifi_device.linktype);
                            if let Some(home_rssi) = ssid_rssi.get(&home_ssid) {
                                let counter_content = home_rssi + 100;
                                if counter_content >= 0 {
                                counter.set(counter_content as usize);
                                }
                            }
                            cb.send(Box::new(Cursive::noop)).unwrap();
                        }
                    })
                }).unwrap();
            }
        })
        .autojump()
        .scrollable();

    Dialog::around(select_home_net)
            .title("Select home network or <<None>>")
}

fn show_list_ssid(s: &mut Cursive, net_signals: HashMap<String, i32>) {
    s.call_on_name("channel_SSID", |view: &mut TextView| {
        let mut channel_content = String::new();
        for (ssid, rssi) in net_signals.iter() {
            let unit_measurement = if rssi < &0 {
                "dBm".to_owned()
            } else {
                "   ".to_owned()
            };
            let ssid_rssi = format!{"{:<25}{:>10} {}\n", ssid, rssi, unit_measurement};
            channel_content.push_str(&ssid_rssi);
        }
        view.set_content(channel_content);
    }).unwrap();
}
