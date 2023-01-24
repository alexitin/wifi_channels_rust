use pcap::{Capture, Active};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::parse_80211;

pub fn frames_data_ppi(mut device: Capture<Active>) -> HashMap<String, i32> {

    let mut ssid_rssi: HashMap<String, i32> = HashMap::new();
    let now = Instant::now();
    let timeout = Duration::from_secs(3);

    'calc: loop {
        while now.elapsed() > timeout {
            break 'calc;
        }

        if let Ok(packet) = device.next_packet() {

            let len_ppi = match packet.data[2..4].try_into() {
                Ok(len) => u16::from_le_bytes(len),
                Err(_) => continue,
            };
            let len_ppi = usize::from(len_ppi);

// Checking beacon frame
            if packet.data[len_ppi] >> 2 != 32 {
                continue;
            }

// Checking field with common (pre-n and .11n) radio information
            if u16::from_le_bytes(packet.data[8..10].try_into().unwrap()) != 2 {
                continue;
            }

            let signal_net = i8::from_le_bytes([packet.data[30]]);

            let len_frame = match usize::try_from(packet.header.len) {
                Ok(len) => len,
                Err(_) => continue,
            };

            let name_net = parse_80211::get_name_net(packet.data, len_frame, len_ppi);

            ssid_rssi.entry(name_net)
                .and_modify(|signal| {*signal = (*signal + (signal_net as i32)) / 2})
                .or_insert(signal_net as i32);
        }
    }
    ssid_rssi
}
