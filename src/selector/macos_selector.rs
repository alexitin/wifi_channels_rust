use std::process;

pub struct MacOsSelector;

extern "C" {
    fn mac_select_channel(ptr_name: *const i8, channel: isize) -> isize;
    fn mac_get_current_channel(ptr_name: *const i8) -> isize;
}

impl MacOsSelector {

    pub fn set_channel(ptr_name: *const i8, channel: isize) {
        let status_select: isize;
        unsafe {
            status_select = mac_select_channel(ptr_name, channel);
        }
        match status_select {
            0 => {
                println!("Scanning channel {}", &channel);
            },
            1 => {
                println!("Problem enable WiFi device");
                process::exit(1);
            },
            2 => {
                println!("Problem set channel WiFi device");
                process::exit(1);
            },
            3 => {
                println!("Problem no find in list supported channels for WiFi device");
                process::exit(1);
            },
            4 => {
                println!("Problem no get list supported WiFi device");
                process::exit(1);
            },
            _ => {
                println!("Problem no get interface of WiFi device");
                process::exit(1);
            },
        }
    }

    pub fn get_channel(ptr_name: *const i8) -> isize {
        let status_channel: isize;
        unsafe {
            status_channel = mac_get_current_channel(ptr_name);
        }
        if status_channel == -1 {
            println!("Problem no get interface of WiFi device");
            process::exit(1);
        } else if status_channel == 0 {
            println!("Problem enable WiFi device");
            process::exit(1);
        } else {
            status_channel
        }
    }
}