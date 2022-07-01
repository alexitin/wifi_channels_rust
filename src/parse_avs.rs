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

pub fn frames_data_avs(mut device: Capture<Active>) -> NetSignals {
    device.filter("type mgt subtype beacon", false).expect("need oter linktype for BPF");

    let mut ssid_signal: BTreeMap<String, i32> = BTreeMap::new();
    let now = Instant::now();
    let timeout = Duration::from_secs(3);

    loop {
        if let Ok(packet) = device.next() {

            let avs_data = match get_avs_data(packet.data) {
                Ok(data) => data,
                Err(_) => {
                    println!("Broken AVS header!");
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

            let name_net = match get_name_net(packet.data, len_frame, avs_data.length) {
                Ok(name) => name,
                Err(_) => {
                    println!("Broken frame!");
                    continue;
                },
            };

            ssid_signal.entry(name_net)
                .and_modify(|sig| {*sig = (*sig + avs_data.signal) / 2})
                .or_insert(avs_data.signal);

            if now.elapsed() >= timeout {
                break NetSignals {
                    channel: avs_data.channel,
                    ssid_signal,
                };
            }
        }
    }
}

fn get_avs_data (data: &[u8]) -> Result<RadioData, TryFromSliceError> {

    let len_avs = u32::from_be_bytes(data[4..8].try_into()?) as usize;

    let channel_num = u32::from_be_bytes(data[28..32].try_into()?).to_string();

    let signal = i32::from_be_bytes(data[48..52].try_into()?);

    Ok(RadioData {
        length: len_avs,
        channel: channel_num,
        signal,
    })
}

fn get_name_net(data: &[u8], len_frame: usize, len_avs: usize) -> Result<String, TryFromSliceError> {
    let frame_control = u16::from_le_bytes(data[len_avs..(len_avs + 2)].try_into()?);

// Checking 802.11ah standart
    if (frame_control & (1 << 0)) != 0 {
        Ok("HaLow".to_string())

    } else {
// Cheking length frame
        if len_frame < (len_avs + 36 + 2) {
//            println!("len: {}", len_frame);
            Ok("Unknown".to_string())

        } else {
// Checking SSID:
            let pos_id = len_avs + 36;
            let len_id = usize::from(data[pos_id + 1]);
            let ssid = if data[pos_id] == 0 && len_id <= 32 {
                String::from_utf8_lossy(&data[(pos_id + 2)..(pos_id + 2 + len_id)]).into_owned()
                } else {
                format!("{:X?}", &data[(len_avs + 16)..(len_avs + 22)]).to_string()
            };
            Ok(ssid)
        }
    }
}
