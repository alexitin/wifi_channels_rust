use pcap::{Capture, Active};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::parse_80211;

pub fn frames_data_radiotap(mut device: Capture<Active>) -> HashMap<String, i32> {

    let mut ssid_rssi: HashMap<String, i32> = HashMap::new();
    let now = Instant::now();
    let timeout = Duration::from_secs(3);

    'calc: loop {
        while now.elapsed() > timeout {
            break 'calc;
        }

        if let Ok(packet) = device.next_packet() {

            let len_radiotap = match packet.data[2..4].try_into() {
                Ok(len) => u16::from_le_bytes(len),
                Err(_) => continue,
            };
            let len_radiotap = usize::from(len_radiotap);

// Checking beacon frame
            if packet.data[len_radiotap] >> 2 != 32 {
                continue;
            }
// Get bitmask radiotap header
            let it_present = match packet.data[4..8].try_into() {
                Ok(bitmask) => u32::from_le_bytes(bitmask),
                Err(_) => continue,
            };

// Length radiotap header
            let mut pos_item = 8;

// Checking ext fields
            if (it_present & (1 << 31)) != 0 {
                pos_item += 32
            }

// Checking TSTF field
            if (it_present & (1 << 0)) != 0 {
                pos_item += 8
            }

//Checking flags field
            if (it_present & (1 << 1)) != 0 {
                pos_item += 1
            }

// Checking rate field
            if (it_present & (1 << 2)) != 0 {
                pos_item += 1
            }

// Checking channel field
            if (it_present & (1 << 3)) != 0 {
                pos_item += 4;
            }

// Checking FHSS field
            if (it_present & (1 << 4)) != 0 {
                pos_item += 2
            }
            
// Checking antenna signal dBm
            let signal_net = if (it_present & (1 << 5)) != 0 {
                i8::from_le_bytes([packet.data[pos_item]])
            } else {
                0
            };

            let len_frame = match usize::try_from(packet.header.len) {
                Ok(len) => len,
                Err(_) => continue,
            };

            let name_net = parse_80211::get_name_net(packet.data, len_frame, len_radiotap);

            ssid_rssi.entry(name_net)
                .and_modify(|signal| {*signal = (*signal + (signal_net as i32)) / 2})
                .or_insert(signal_net as i32);
        }
    }
    ssid_rssi
}
