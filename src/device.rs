use pcap::{Device, Capture, Active, Linktype};

#[derive(Debug, Clone, Copy)]
pub enum DeviceMode {
    Monitor,
    Promiscouos,
    Normal,
}

pub struct AllDevices {
    pub devices: Vec<Device>
}
#[derive(Debug)]
pub struct WifiDevices {
    pub devices: Vec<String>,
    pub mode: Option<DeviceMode>,
}


pub struct WifiDevice {
    pub name: String,
    pub mode: DeviceMode,
    pub linktype: Linktype,
}

impl AllDevices {

    pub fn new() -> Result<AllDevices, pcap::Error> {
        Ok(AllDevices {
            devices: Device::list()?
        })
    }

    pub fn get_wifi_devices(self) -> WifiDevices {

// Check all devices for monitor mode
        let devices: Vec<String> = self.devices.iter()
            .filter(|dev| {
                let capture_dev = set_monitor_mode(&dev.name);
                capture_dev.is_ok() && 
                (get_linktype(&mut capture_dev.unwrap()) != None)
            })
            .map(|dev| dev.name.to_owned())
            .collect();
        if !devices.is_empty() {
            WifiDevices {
                devices,
                mode: Some(DeviceMode::Monitor)
            }

        } else {
// Check device for promiscous mode
            let devices: Vec<String> = self.devices.iter()
            .filter(|dev| {
                let capture_dev = set_promiscouos_mode(&dev.name);
                capture_dev.is_ok() && 
                (get_linktype(&mut capture_dev.unwrap()) != None)
            })
            .map(|dev| dev.name.to_owned())
            .collect();

            if !devices.is_empty() {
                WifiDevices {
                    devices,
                    mode: Some(DeviceMode::Promiscouos)
                }
            } else {
// Check device for normal mode
                let devices: Vec<String> = self.devices.iter()
                    .filter(|dev| {
                        let capture_dev = set_normal_mode(&dev.name);
                        capture_dev.is_ok() && 
                        (get_linktype(&mut capture_dev.unwrap()) != None)
                    })
                    .map(|dev| dev.name.to_owned())
                    .collect();

                if !devices.is_empty(){
                    WifiDevices {
                        devices,
                        mode: Some(DeviceMode::Normal)
                    }
                } else {
                    WifiDevices {
                        devices,
                        mode: None
                    }
                }
            }
        }
    }
}

impl WifiDevice {
    pub fn get_wifi_device<'a>(name: String, mode: DeviceMode) -> WifiDevice {
//        let mode = wifi_devices.mode.unwrap();
        let mut device_capture = match mode {
            DeviceMode::Monitor => set_monitor_mode(&name),
            DeviceMode::Promiscouos => set_promiscouos_mode(&name),
            DeviceMode::Normal => set_normal_mode(&name),
        }.expect("Checked for error before");
        let linktype = get_linktype(&mut device_capture).expect("Checked for error before");
        WifiDevice { name, mode, linktype, }
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

pub fn get_linktype(device: &mut Capture<Active>) -> Option<Linktype> {
    if device.set_datalink(Linktype::IEEE802_11_RADIOTAP).is_ok() {
        Some(Linktype(127))
    } else if device.set_datalink(Linktype::IEEE802_11_AVS).is_ok() {
        Some(Linktype(163))
    } else if device.set_datalink(Linktype::PPI).is_ok() {
        Some(Linktype(192))
    } else if device.set_datalink(Linktype::IEEE802_11).is_ok() {
        Some(Linktype(105))
    } else {
        None
    }
}
