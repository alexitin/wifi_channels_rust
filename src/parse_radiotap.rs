use pcap::{Capture, Active};

pub fn frame(mut device: Capture<Active>) {
    let linktype = device.get_datalink().get_name().expect("No such linktype");
    device.filter("type mgt subtype beacon", false).expect("need oter linktype for BPF");
    
    while let Ok(packet) = device.next() {

// Radiotap decoding

        let len_radiotap = u16::from_le_bytes(packet.data[2..4].try_into().unwrap());
        let len_radiotap = usize::from(len_radiotap);
        let it_present = u32::from_le_bytes(packet.data[4..8].try_into().expect("slice with incorrect length"));

        let mut pos_item = 7;

        if (it_present & (1 << 31)) != 0 {
            pos_item += 32
        }

        if (it_present & (1 << 0)) != 0 {
            pos_item += 8
        }

        if (it_present & (1 << 1)) != 0 {
            pos_item += 1
        }

        if (it_present & (1 << 2)) != 0 {
            pos_item += 1
        }

        let channel = if (it_present & (1 << 3)) != 0 {
            u16::from_le_bytes(packet.data[(pos_item + 1)..(pos_item + 3)]
                .try_into()
                .unwrap())
        } else {
            0
        };

        if (it_present & (1 << 3)) != 0 {
            pos_item += 4
        }

        if (it_present & (1 << 4)) != 0 {
            pos_item += 2
        }

        let signal = if (it_present & (1 << 5)) != 0 {
            i8::from_le_bytes(packet.data[(pos_item + 1) .. (pos_item + 2)]
                .try_into()
                .unwrap())
        } else {
            0
        };

        if (it_present & (1 << 5)) != 0 {
            pos_item += 1
        }

        let noise = if (it_present & (1 << 6)) != 0 {
            i8::from_le_bytes(packet.data[(pos_item + 1) .. (pos_item + 2)]
                .try_into()
                .unwrap())
        } else {
            0
        };

// MAC 802.11 frame decoding

        let frame_control = u16::from_le_bytes(packet.data[len_radiotap..(len_radiotap + 2)]
            .try_into()
            .expect("slice with incorrect length FC"));
        
        let mut pos_id = if packet.header.len >= (len_radiotap + 36).try_into().unwrap() {
            len_radiotap + 36
        } else {
            println!("HHHHHHAAAAAAYYYYYY!!!!!!
            Channel: {}.\nSignal: {}.\nNoise: {}.", &channel, &signal, &noise);
            len_radiotap
        };
        let len_id = usize::from(packet.data[pos_id + 1]);

// Checking SSID:
        let ssid = if (frame_control & (1 << 0)) != 0 {
            "HaLow".to_owned()
        } else if packet.data[pos_id] == 0 && len_id <= 32 {
            String::from_utf8_lossy(&packet.data[(pos_id + 2)..(pos_id + 2 + len_id)]).into_owned()
        } else {
            "hidden".to_owned()
        };

        if packet.data[pos_id] == 0 && len_id <= 32 {
            pos_id += 2 + len_id
        }

// Checking rate:
        let len_id = usize::from(packet.data[pos_id + 1]);

        if packet.data[pos_id] == 1 && len_id <= 8 {
            pos_id += 2 + len_id
        } else {
            pos_id += 10
        }

// Checking FH:
        let len_id = usize::from(packet.data[pos_id + 1]);

        if packet.data[pos_id] == 2 && len_id <= 5 {
            pos_id += 2 + len_id
        } else {
            pos_id += 7
        }

// Checking DS:
        let len_id = usize::from(packet.data[pos_id + 1]);

        let channel_cur = if (packet.data[pos_id] == 3) && (packet.data[pos_id + 1] == 1) {
            u8::from(packet.data[pos_id + 2])
        } else {
            match channel {
                2412 => 1,
                2417 => 2,
                2422 => 3,
                2427 => 4,
                2432 => 5,
                2437 => 6,
                2442 => 7,
                2447 => 8,
                2452 => 9,
                2457 => 10,
                2462 => 11,
                2467 => 12,
                2472 => 13,
                2484 => 14,
                _ => 15,
            }
        };

//            let channel_freq = u32::from_be_bytes(packet.data[]);
//        if len_ssid == 0 {
//        println!("nFC: {:#018b}.\nPosition: {}.\nLength_ssid: {}.\nPacket {:?}",
//                &frame_control, &pos_item, &len_ssid, &packet.data[len_radiotap..(len_radiotap + 64)]);
        println!("Linktype: {:?}.\nLength: {}.\nChannel: {}.\nSignal: {}.\nNoise: {}.\nSSID: {}.
Curient channel: {}.\nPacket: {:?}",
            &linktype, &len_radiotap, &channel, &signal, &noise,
            &ssid, &channel_cur, &packet.data[(len_radiotap + 36)..128]);
//            }
    }
}
