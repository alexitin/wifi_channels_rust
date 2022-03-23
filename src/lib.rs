use pcap::{Capture, Device, Linktype};
use std::io;
use std::process;

pub struct Captured {
    pub device: Option<Device>,
    pub mode: String,
    pub linktype: Vec<Linktype>,
}

impl Captured {

    pub fn get_monitor_device(devices: Vec<Device>) -> Captured {
        let device = devices.into_iter()
            .filter(|dev| Capture::from_device(dev.clone())
                .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
                .rfmon(true)
                .open()
                .is_ok())
            .next();
//        let device: Option<Device> = None;
        if device.is_some() {
            let name = device.clone().unwrap().name;
            Captured {
                device: device,
                mode: "monitor".to_string(),
                linktype: Capture::from_device(name.as_str())
                    .unwrap()
                    .rfmon(true)
                    .open()
                    .unwrap()
                    .list_datalinks()
                    .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
            }
        } else {
            Captured {
                device: None,
                mode: "".to_string(),
                linktype: vec![]
            }
        }
    }

    pub fn get_promiscuous_device(devices: Vec<Device>) -> Captured {
        let devices = devices.iter()
            .cloned()
            .filter(|dev| (*dev).addresses.len() > 1)
            .filter(|dev| 
                ((*dev).addresses[0].broadcast_addr.is_some() ||
                (*dev).addresses[1].broadcast_addr.is_some()))
            .collect::<Vec<_>>();

        let device = if devices.len() > 1 {
                let mut i = 0;
                for dev in &devices {
                    i += 1;
                    println!("{}. Device: {} - IPV6: {}, IPV4: {}",
                        i,
                        dev.name,
                        dev.addresses[0].addr,
                        dev.addresses[1].addr
                    );
                }
                println!("Choose wlan device, or press any key to quit:");
                let buf = loop {
                    let mut buf = String::new();
                    io::stdin()
                        .read_line(&mut buf)
                        .expect("Failed read yuor choice:");
                    let buf = buf.trim().parse::<usize>().unwrap_or(99);
                    if buf == 99 {
                        println!("Not found wifi devices");
                        process::exit(1);
                    } else if buf > devices.len() {
                        println!("buf: {}", &buf);
                        continue;
                    } else {
                        break buf
                    };
                };
                Some(devices[buf - 1].clone())
            } else  if devices.len() == 1 {
                Some(devices[0].clone())
            } else {
                None
            };

//        let device: Option<Device> = None;
        if device.is_some() {
            let name = device.clone().unwrap().name;
            if Capture::from_device(device.clone().unwrap())
                .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
                .promisc(true)
                .open()
                .is_ok() {
                    Captured {
                        device: device,
                        mode: "promiscuous".to_string(),
                        linktype: Capture::from_device(name.as_str())
                            .unwrap()
                            .promisc(true)
                            .open()
                            .unwrap()
                            .list_datalinks()
                            .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
                    }
                } else {
                    Captured {
                        device: device,
                        mode: "normal".to_string(),
                        linktype: Capture::from_device(name.as_str())
                            .unwrap()
                            .promisc(true)
                            .open()
                            .unwrap()
                            .list_datalinks()
                            .unwrap_or_else(|err| panic!("Problem capture device: {}", err))
                    }
                }
            } else {
            Captured {
                device: None,
                mode: "".to_string(),
                linktype: vec![]
            }
        }
    }

    pub fn get_frame(capture_device: Captured) {
        let mut cap = Capture::from_device(capture_device.device.clone().unwrap())
            .unwrap()
            .rfmon(true)
            .snaplen(128)
            .buffer_size(256)
            .timeout(10000)
            .open()
            .unwrap();

        if capture_device.linktype.contains(&Linktype(192)) {
            cap.set_datalink(Linktype::PPI).unwrap()
        } else if capture_device.linktype.contains(&Linktype(163)) {
            cap.set_datalink(Linktype::IEEE802_11_AVS).unwrap()
        } else if capture_device.linktype.contains(&Linktype(127)) {
            cap.set_datalink(Linktype::IEEE802_11_RADIOTAP).unwrap()
        } else if capture_device.linktype.contains(&Linktype(119)) {
            cap.set_datalink(Linktype::IEEE802_11_PRISM).unwrap()
        } else if capture_device.linktype.contains(&Linktype(105)) {
            cap.set_datalink(Linktype::IEEE802_11).unwrap()
        } else if capture_device.linktype.contains(&Linktype(1)) {
            cap.set_datalink(Linktype::ETHERNET).unwrap()
        } else {
            println!("Not posible capture wifi packets");
            process::exit(1);
        };

//        if let Ok(packet) =  cap.next() {
            println!("Received packets: {:?}", cap.next());
            println!("Device: {:?},\nMode: {},\nLinktype {:?}",
                &capture_device.device,
                &capture_device.mode, 
                &capture_device.linktype);
//        }

    }

}
