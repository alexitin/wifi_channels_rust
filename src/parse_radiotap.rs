use pcap::{Capture, Active};
use std::array::TryFromSliceError;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use crate::frame::NetSignals;


struct RadiotapData {
    length: usize,
    channel: String,
    signal: Option<i8>,
}

pub fn frames_data(mut device: Capture<Active>) -> NetSignals {
    let linktype = device.get_datalink().get_name().expect("No such linktype");
    device.filter("type mgt subtype beacon", false).expect("need oter linktype for BPF");

    let mut ssid_signal: BTreeMap<String, i32> = BTreeMap::new();
    let now = Instant::now();
    let timeout = Duration::from_secs(3);

    loop {
        if let Ok(packet) = device.next() {
            let radiotap_data = match get_radiotap_data(packet.data) {
                Ok(r_data) => r_data,
                Err(_) => {
                    println!("Broken radiotap header!");
                    continue;
                },
            };

            let signal = match radiotap_data.signal {
                Some(sig) => i32::from(sig),
                None => {
                    println!("Signal not present!");
                    continue;
                },
            };

            let len_frame = match usize::try_from(packet.header.len) {
                Ok(len) => len,
                Err(_) => {
                    println!("Broken length frame!");
                    continue;
                },
            };

            let name_net = match get_name_net(packet.data, len_frame, radiotap_data.length) {
                Ok(name) => name,
                Err(_) => {
                    println!("Broken frame!");
                    continue;
                },
            };

            ssid_signal.entry(name_net)
                .and_modify(|sig| {*sig = (*sig + signal) / 2})
                .or_insert(signal);

            if now.elapsed() >= timeout {
                break NetSignals {
                    channel: radiotap_data.channel,
                    linktype,
                    ssid_signal,
                };
            }
        }
    }
}

fn get_radiotap_data (data: &[u8]) -> Result<RadiotapData, TryFromSliceError> {
    let len_radiotap = u16::from_le_bytes(data[2..4].try_into()?);    // usize::from_le_bytes() not working;
    let len_radiotap = usize::from(len_radiotap);

    let it_present =u32::from_le_bytes(data[4..8].try_into()?);

    let mut pos_item = 7;

// Checking ext fields
    if (it_present & (1 << 31)) != 0 {
        pos_item += 32
    }

// Checking TSTF
    if (it_present & (1 << 0)) != 0 {
        pos_item += 8
    }

//Checking flags
    if (it_present & (1 << 1)) != 0 {
        pos_item += 1
    }

// Checking rate
    if (it_present & (1 << 2)) != 0 {
        pos_item += 1
    }

// Checking channel
    let channel_freq;
    if (it_present & (1 << 3)) != 0 {
        channel_freq = Some(u16::from_le_bytes(data[(pos_item + 1)..(pos_item + 3)].try_into()?));
        pos_item += 4;
    } else {
        channel_freq = None;
    };
    let channel_num = match channel_freq {
        Some(2412) => "1".to_string(),
        Some(2417) => "2".to_string(),
        Some(2422) => "3".to_string(),
        Some(2427) => "4".to_string(),
        Some(2432) => "5".to_string(),
        Some(2437) => "6".to_string(),
        Some(2442) => "7".to_string(),
        Some(2447) => "8".to_string(),
        Some(2452) => "9".to_string(),
        Some(2457) => "10".to_string(),
        Some(2462) => "11".to_string(),
        Some(2467) => "12".to_string(),
        Some(2472) => "13".to_string(),
        Some(2484) => "14".to_string(),
        Some(freq) => freq.to_string(),
        None => "Not present".to_owned(),
    };

// Checking FHSS
    if (it_present & (1 << 4)) != 0 {
        pos_item += 2
    }

// Checking antenna signal dBm
    let signal = if (it_present & (1 << 5)) != 0 {
        Some(i8::from_le_bytes(data[(pos_item + 1)..(pos_item + 2)].try_into()?))
    } else {
        None
    };

    Ok(RadiotapData {
        length: len_radiotap,
        channel: channel_num,
        signal,
    })
}

fn get_name_net(data: &[u8], len_frame: usize, len_radiotap: usize) -> Result<String, TryFromSliceError> {
    let frame_control = u16::from_le_bytes(data[len_radiotap..(len_radiotap + 2)].try_into()?);

// Checking 802.11ah standart
    if (frame_control & (1 << 0)) != 0 {
        Ok("HaLow".to_string())

    } else {
// Cheking length frame
        if len_frame < (len_radiotap + 36 + 2) {
            println!("len: {}", len_frame);
            Ok("Unknown".to_string())

        } else {
// Checking SSID:
            let pos_id = len_radiotap + 36;
            let len_id = usize::from(data[pos_id + 1]);
            let ssid = if data[pos_id] == 0 && len_id <= 32 {
                String::from_utf8_lossy(&data[(pos_id + 2)..(pos_id + 2 + len_id)]).into_owned()
                } else {
                format!("{:X?}", &data[(len_radiotap + 16)..(len_radiotap + 22)]).to_string()
            };
            Ok(ssid)
        }
    }
}
