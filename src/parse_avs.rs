use pcap::{Capture, Active};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::parse_80211;

pub fn frames_data_avs(mut device: Capture<Active>) -> HashMap<String, i32> {

    let mut ssid_rssi: HashMap<String, i32> = HashMap::new();
    let now = Instant::now();
    let timeout = Duration::from_secs(3);

    'calc: loop {
        while now.elapsed() > timeout {
            break 'calc;
        }

        if let Ok(packet) = device.next() {

            let len_avs = match packet.data[4..8].try_into() {
                Ok(len) => u32::from_be_bytes(len),
                Err(_) => continue,
            };

            let len_avs = match usize::try_from(len_avs) {
                Ok(len) => len,
                Err(_) => continue,
            };

// Checking beacon frame
            if packet.data[len_avs] >> 2 != 32 {
                continue;
            }

// Checking type rssi 0 - none, 1 - normalized 0-1000, 2 - dBm, 3 - raw;
            let signal_type = match packet.data[44..48].try_into() {
                Ok(len) => u32::from_be_bytes(len),
                Err(_) => continue,
            };
        
            let signal_net = if signal_type !=0 {
                match packet.data[48..52].try_into() {
                    Ok(signal) => i32::from_be_bytes(signal),
                    Err(_) => continue,
                }
            } else {
                0
            };

            let len_frame = match usize::try_from(packet.header.len) {
                Ok(len) => len,
                Err(_) => continue,
            };

            let name_net = parse_80211::get_name_net(packet.data, len_frame, len_avs);

            ssid_rssi.entry(name_net)
                .and_modify(|signal| {*signal = (*signal + signal_net) / 2})
                .or_insert(signal_net);
        }
    }
    ssid_rssi
}
