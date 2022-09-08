use std::process;

pub struct LinuxSelector;

extern "C" {
    fn lin_select_channel(ptr_name: *const i8, channel_freq: isize) -> isize;
    fn lin_get_channel(ptr_name: *const i8) -> isize;
}

impl LinuxSelector {
    pub fn set_channel(ptr_name: *const i8, channel: isize) {
        let channel_freq: isize = match channel {
            1 => 2412,
            2 => 2417,
            3 => 2422,
            4 => 2427,
            5 => 2432,
            6 => 2437,
            7 => 2442,
            8 => 2447,
            9 => 2452,
            10 => 2457,
            11 => 2462,
            _ => 2412,
        };
        let status_select: isize;
        unsafe {
            status_select = lin_select_channel(ptr_name, channel_freq);
        }
        match status_select {
            0 => {
                println!("Scanning channel {}", &channel);
            },
            1 => {
                println!("Problem failed creation socket");
                process::exit(1);
            },
            2 => {
                println!("Problem DOWN device");
                process::exit(1);
            },
            3 => {
                println!("Problem set monitor mode");
                process::exit(1);
            },
            4 => {
                println!("Problem UP device");
                process::exit(1);
            },
            5 => {
                println!("Problem get current freq channel");
                process::exit(1);
            },
            6 => {
                println!("Problem driver doesn't report freq");
                process::exit(1);
            },
            7 => {
                println!("Problem pass new freq channel down to driver");
                process::exit(1);
            },
            8 => {
                println!("Problem pass new freq channel up to driver");
                process::exit(1);
            },
            9 => {
                println!("Problem select freq channel driver");
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
            status_channel = lin_get_channel(ptr_name);
        }
        match status_channel {
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
            1 => {
                println!("Problem failed creation socket");
                process::exit(1);
            },
            5 => {
                println!("Problem get current freq channel");
                process::exit(1);
            },
            6 => {
                println!("Problem driver doesn't report freq");
                process::exit(1);
            },
            _ => {
                println!("Problem unknown freq");
                process::exit(1);
            },
        }
    }
}