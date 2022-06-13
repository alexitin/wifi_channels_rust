use pcap::{Device, Linktype};
use std::{collections::BTreeMap, time::Duration, thread, rc::Rc, process};

mod check_device;
mod parse_radiotap;

extern "C" {
    fn mac_select_channel(ptr_name: *const u8, channel: isize) -> isize;
}

pub enum DeviceMode {
    Monitor,
    Promiscouos,
    Normal,
}

pub struct AllDevices {
    devices: Vec<Device>
}
pub struct WifiDevice {
    pub name: String,
    pub mode: DeviceMode,
}

pub struct NetSignals {
    pub channel: String,
    pub linktype: String,
    pub ssid_signal: BTreeMap<String, i32>,
}

impl AllDevices {

    pub fn new () -> Result<AllDevices, pcap::Error> {
        let devices =  Device::list()?; {
            Ok(AllDevices {devices})
        }
    }

    pub fn get_wifi_device(self) -> WifiDevice {

// Check all devices for monitor mode compatibility and use the first match
        if let Some(position) = self.devices.iter()
            .position(|dev| check_device::set_monitor_mode(&dev.name).is_ok()) {
            
            let name = self.devices[position].name.to_owned();
            
            WifiDevice {
                name,
                mode: DeviceMode::Monitor,
            }
        } else {

// Choice devices connected to the local network
            let devices = check_device::choice_device(self.devices);
            let name = devices.name.to_owned();

// Check device for promiscouos mode
            let device = check_device::set_promiscouos_mode(&devices.name).ok();
//            let device = None;
            if device.is_some() {
                WifiDevice {
//                    device,
                    name,
                    mode: DeviceMode::Promiscouos,
                }
            } else {
// Device are normal mode
                WifiDevice {
                    name,
                    mode: DeviceMode::Normal,
                }
            }
        }
    }
}

impl WifiDevice {
    pub fn frames_all_channels(self) {
        let mut status_select: isize;

        let name = Rc::new(self.name);

        let ptr_name = Rc::clone(&name).as_ptr();
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



            let net_signal = WifiDevice::get_frames(&name.to_string());
            println!("Linktype: {}.\nChannel: {}", net_signal.linktype, net_signal.channel);
            println!("{:?}", net_signal.ssid_signal)
        }
    }
    pub fn get_frames(name: &str) -> NetSignals {
        let mut device = check_device::set_monitor_mode(name).unwrap();
        device = check_device::get_linktype(device);
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