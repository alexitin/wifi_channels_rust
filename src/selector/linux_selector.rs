use super::error::{ResultSelector, ErrorSelector};

pub struct LinuxSelector;

extern "C" {
    fn lin_select_channel(ptr_name: *const i8, channel_freq: isize) -> isize;
    fn lin_get_channel(ptr_name: *const i8) -> isize;
    fn lin_set_managed(ptr_name: *const i8) -> isize;
}

impl LinuxSelector {
    
    pub fn set_channel(ptr_name: *const i8, channel: isize) -> ResultSelector {
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
            12 => 2467,
            13 => 2472,
            _ => 2412,
        };
        let status_select: isize;
        unsafe {
            status_select = lin_select_channel(ptr_name, channel_freq);
        }
        match status_select {
            0 => Ok(status_select),
            1 => Err(ErrorSelector::NotCreatSocet),
            2 => Err(ErrorSelector::NotPowerOffDevice),
            3 => Err(ErrorSelector::NotSetMonitorMode),
            4 => Err(ErrorSelector::NotPowerUpDevice),
            5 => Err(ErrorSelector::NotGetFreqChannel),
            6 => Err(ErrorSelector::DriverNotReportFreqChannel),
            7 => Err(ErrorSelector::NotPassNewFreqDown),
            8 => Err(ErrorSelector::NotPassNewFreqUp),
            9 => Err(ErrorSelector::DriverNotSelectFreqChannel),
            _ => Err(ErrorSelector::NotGetInterface),
        }
    }

    pub fn get_channel(ptr_name: *const i8) -> ResultSelector {
        let status_channel: isize;
        unsafe {
            status_channel = lin_get_channel(ptr_name);
        }
        match status_channel {
            2412 => Ok(1),
            2417 => Ok(2),
            2422 => Ok(3),
            2427 => Ok(4),
            2432 => Ok(5),
            2437 => Ok(6),
            2442 => Ok(7),
            2447 => Ok(8),
            2452 => Ok(9),
            2457 => Ok(10),
            2462 => Ok(11),
            2467 => Ok(12),
            2472 => Ok(13),
            1 => Err(ErrorSelector::NotCreatSocet),
            5 => Err(ErrorSelector::NotGetFreqChannel),
            6 => Err(ErrorSelector::DriverNotReportFreqChannel),
            _ => Err(ErrorSelector::UnknownFreq),
        }
    }

    pub fn set_managed_mode(ptr_name: *const i8) -> ResultSelector {
        let status_managed_mode: isize;
        unsafe {
            status_managed_mode = lin_set_managed(ptr_name);
        }
        match status_managed_mode {
            0 => Ok(0),
            1 => Err(ErrorSelector::NotCreatSocet),
            2 => Err(ErrorSelector::NotPowerOffDevice),
            4 => Err(ErrorSelector::NotPowerUpDevice),
            10 => Err(ErrorSelector::NotSetManagedMode),
            _ => Err(ErrorSelector::NotGetInterface),
        }
    }

}