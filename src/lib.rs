use pcap::{Capture, Device, Linktype};

pub struct Captured {
    pub device: Option<Device>,
    pub mode: String,
    pub linktype: Vec<Linktype>,
}

impl Captured {
    pub fn get_device (devices: Vec<Device>) -> Captured {
        let linktype: Vec<Linktype> = vec![];

// Check devices for support monitor mode:
        let device = devices.clone().into_iter()
            .filter(|dev| Capture::from_device(dev.clone())
                .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
                .rfmon(true)
                .open()
                .is_ok())
            .next();

        if device.is_some() {
            Captured { 
                device: device,
                mode: "monitor".to_string(),
                linktype,
            }
        } else {

//Chec devices for support promiscuous mode:
            let device = devices.clone().into_iter()
                .filter(|dev| Capture::from_device(dev.clone())
                    .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
                    .promisc(true)
                    .open()
                    .is_ok())
                .next();

                if device.is_some() {
                Captured {
                device: device,
                mode: "promiscuous".to_string(),
                linktype,
                }
            } else {

//"Not found devices suppoted monitor or promiscuous mode.
                Captured {
                    device: device,
                    mode: "".to_string(),
                    linktype,
                }
            }
        }
    }
}
