use pcap::{Capture, Active};
use std::array::TryFromSliceError;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use crate::frame::NetSignals;


struct RadioData {
    length: usize,
    channel: String,
    signal: i32,
}

pub fn frames_data_ppi(mut device: Capture<Active>) -> NetSignals {
    let mut ssid_signal: BTreeMap<String, i32> = BTreeMap::new();
    let now = Instant::now();
    let timeout = Duration::from_secs(3);

    loop {
        if let Ok(packet) = device.next() {

// Checking field with common (pre-n and .11n) radio information
            if u16::from_le_bytes(packet.data[8..10].try_into().unwrap()) != 2 {
                println!("Broken frame!");
                continue;
            }

            let ppi_data = match get_ppi_data(packet.data) {
                Ok(data) => data,
                Err(_) => {
                    println!("Broken PPI header!");
                    continue;
                },
            };

// Checking beacon frame
            let frame_control = u8::from_le_bytes(packet.data[ppi_data.length..(ppi_data.length + 1)].try_into().unwrap());
            if frame_control != 128 {
                continue;
            }

            let len_frame = match usize::try_from(packet.header.len) {
                Ok(len) => len,
                Err(_) => {
                    println!("Broken length frame!");
                    continue;
                },
            };

            let name_net = match get_name_net(packet.data, len_frame, ppi_data.length) {
                Ok(name) => name,
                Err(_) => {
                    println!("Broken frame!");
                    continue;
                },
            };

                ssid_signal.entry(name_net)
                .and_modify(|sig| {*sig = (*sig + ppi_data.signal) / 2})
                .or_insert(ppi_data.signal);

            if now.elapsed() >= timeout {
                break NetSignals {
                    channel: ppi_data.channel,
                    ssid_signal,
                };
            }
        }
    }
}

fn get_ppi_data (data: &[u8]) -> Result<RadioData, TryFromSliceError> {

    let len_ppi = u16::from_le_bytes(data[2..4].try_into()?);    // usize::from_le_bytes() not working;
    let len_ppi = usize::from(len_ppi);

    let channel_freq = u16::from_le_bytes(data[24..26].try_into()?);
    let channel_num = match channel_freq {
        2412 => "1".to_string(),
        2417 => "2".to_string(),
        2422 => "3".to_string(),
        2427 => "4".to_string(),
        2432 => "5".to_string(),
        2437 => "6".to_string(),
        2442 => "7".to_string(),
        2447 => "8".to_string(),
        2452 => "9".to_string(),
        2457 => "10".to_string(),
        2462 => "11".to_string(),
        2467 => "12".to_string(),
        2472 => "13".to_string(),
        2484 => "14".to_string(),
        freq => freq.to_string(),
    };

    let signal = i8::from_be_bytes(data[30..31].try_into()?) as i32;

    Ok(RadioData {
        length: len_ppi,
        channel: channel_num,
        signal,
    })
}

fn get_name_net(data: &[u8], len_frame: usize, len_ppi: usize) -> Result<String, TryFromSliceError> {
    let frame_control = u8::from_le_bytes(data[len_ppi..(len_ppi + 1)].try_into()?);

// Checking 802.11ah standart
    if (frame_control & (1 << 0)) != 0 {
        Ok("HaLow".to_string())

    } else {
// Cheking length frame
        if len_frame < (len_ppi + 36 + 2) {
//            println!("len: {}", len_frame);
            Ok("Unknown".to_string())

        } else {
// Checking SSID:
            let pos_id = len_ppi + 36;
            let len_id = usize::from(data[pos_id + 1]);
            let ssid = if data[pos_id] == 0 && len_id <= 32 {
                String::from_utf8_lossy(&data[(pos_id + 2)..(pos_id + 2 + len_id)]).into_owned()
                } else {
                format!("{:X?}", &data[(len_ppi + 16)..(len_ppi + 22)]).to_string()
            };
            Ok(ssid)
        }
    }
}



