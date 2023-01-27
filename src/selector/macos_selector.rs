use super::error::{ResultSelector, ErrorSelector};

pub struct MacOsSelector;

extern "C" {
    fn mac_select_channel(ptr_name: *const i8, channel: isize) -> isize;
    fn mac_get_current_channel(ptr_name: *const i8) -> isize;
}

impl MacOsSelector {

    pub fn set_channel(ptr_name: *const i8, channel: isize) -> ResultSelector {
        let status_select: isize;
        unsafe {
            status_select = mac_select_channel(ptr_name, channel);
        }
        match status_select {
            0 => Ok(status_select),
            1 => Err(ErrorSelector::NotEnableWifi),
            2 => Err(ErrorSelector::NotSetChannel),
            3 => Err(ErrorSelector::NotSupportChannel),
            4 => Err(ErrorSelector::NotSupportDevice),
            _ => Err(ErrorSelector::NotGetInterface),
        }
    }

    pub fn get_channel(ptr_name: *const i8) -> ResultSelector {
        let status_channel: isize;
        unsafe {
            status_channel = mac_get_current_channel(ptr_name);
        }
        match status_channel {
            -1 => Err(ErrorSelector::NotGetInterface),
            0 => Err(ErrorSelector::NotEnableWifi),
            _ => Ok(status_channel)
        }
    }
}