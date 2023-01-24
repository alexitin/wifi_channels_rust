use pcap::{Capture, Active};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub fn frames_data_80211(mut device: Capture<Active>) -> HashMap<String, i32> {

    let mut ssid_signal: HashMap<String, i32> = HashMap::new();
    let now = Instant::now();
    let timeout = Duration::from_secs(3);

    'calc: loop {
        while now.elapsed() > timeout {
            break 'calc;
        }

        if let Ok(packet) = device.next_packet() {

            let len_radio_header: usize = 0;
            let signal_net = 0;

// Checking beacon frame
            if packet.data[len_radio_header] >> 2 != 32 {
                continue;
            }

            let len_frame = match usize::try_from(packet.header.len) {
                Ok(len) => len,
                Err(_) => continue,
            };

            let name_net = get_name_net(packet.data, len_frame, len_radio_header);

            ssid_signal.entry(name_net).or_insert(signal_net);
        }
    }
    ssid_signal
}

pub fn get_name_net(data: &[u8], len_frame: usize, len_radio_header: usize) -> String {
//    let len_frame_control = 2;
//    let len_duration = 2;
//    let len_destination_adress = 6;
//    let len_source_adress = 6;
//    let len_bssid = 6;
//    let len_seq_ctl = 2;
//    let len_timestamp = 8;
//    let len_beacon_inter = 2;
//    let len_capability_info = 2;
//Position ssid field 36

    let pos_ssid = len_radio_header + 36;

    // Checking 802.11ah standart
    if (data[len_radio_header] & (1 << 0)) != 0 {
        return "HaLow".to_string()
    }

// Cheking length frame
    if len_frame < (pos_ssid + 2) {
        return "Unknown".to_string()
    }

    let len_ssid = if data[pos_ssid] == 0 {
        usize::from(data[pos_ssid + 1])
    } else {
        0
    };
// Checking hidden SSID:
    if len_ssid == 0 || len_ssid >= 32 {
        let ssid = format!("{:X?}", &data[(len_radio_header + 16)..(len_radio_header + 22)]).to_string();
        return ssid
    } else {
        let ssid = String::from_utf8_lossy(&data[(pos_ssid + 2)..(pos_ssid + 2 + len_ssid)]).into_owned();
        return ssid
    }
}
