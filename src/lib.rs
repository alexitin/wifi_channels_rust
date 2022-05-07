use pcap::{Capture, Device, Linktype, Active};

mod check_device;
mod parse_radiotap;

pub struct AllDevices {
    devices: Vec<Device>
}
pub struct WifiDevice {
    pub device: Option<Capture<Active>>,
    pub name: String,
    pub mode: String,
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
                device: check_device::set_monitor_mode(&name).ok(),
                name,
                mode: "monitor".to_string()
            }
        } else {

// Selecting devices connected to the local network
            let devices = check_device::choice_device(self.devices);
            let name = devices.name.to_owned();
// Check device for promiscouos mode and set it
            let device = check_device::set_promiscouos_mode(&devices.name).ok();
//            let device = None;
            if device.is_some() {
                WifiDevice {
                    device,
                    name,
                    mode:"promiscouos".to_string()
                }
            } else {
// Check device for normal mode and set it
                let device = check_device::set_normal_mode(&devices.name).ok();
                WifiDevice {
                    device,
                    name,
                    mode: "normal".to_string()
                }
            }
        }
    }
}

impl WifiDevice {
    pub fn get_frame(device: Capture<Active>) {
        let device = check_device::get_linktype(device);
        match device.get_datalink() {
            Linktype(127) => parse_radiotap::frame(device),
            _ => println!("Todo next, linktype: {:?}", device.get_datalink()),
        };
    }
}