use std::fmt;

#[cfg(target_os = "macos")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSelector {
    NotEnableWifi,
    NotSetChannel,
    NotSupportChannel,
    NotSupportDevice,
    NotGetInterface,
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorSelector {
    NotCreatSocet,
    NotPowerOffDevice,
    NotSetMonitorMode,
    NotPowerUpDevice,
    NotGetFreqChannel,
    DriverNotReportFreqChannel,
    NotPassNewFreqDown,
    NotPassNewFreqUp,
    DriverNotSelectFreqChannel,
    NotGetInterface,
    UnknownFreq,
    NotSetManagedMode,
}

pub type ResultSelector = Result<isize, ErrorSelector>;

impl fmt::Display for ErrorSelector {
    #[cfg(target_os = "macos")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorSelector::NotEnableWifi => write!(f, "not set power UP device"),
            ErrorSelector::NotSetChannel => write!(f, "not set channel"),
            ErrorSelector::NotSupportChannel => write!(f, "not support selected channel"),
            ErrorSelector::NotSupportDevice => write!(f, "not get list channels supported device"),
            ErrorSelector::NotGetInterface => write!(f, "not get wifi interfece"),
        }
    }

    #[cfg(target_os = "linux")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorSelector::NotCreatSocet => write!(f, "failed creation socket"),
            ErrorSelector::NotPowerOffDevice => write!(f, "not power off device"),
            ErrorSelector::NotSetMonitorMode => write!(f, "not set monitor mode"),
            ErrorSelector::NotPowerUpDevice => write!(f, "not power up device"),
            ErrorSelector::NotGetFreqChannel => write!(f, "not get current freq channel"),
            ErrorSelector::DriverNotReportFreqChannel => write!(f, "driver doesn't report freq"),
            ErrorSelector::NotPassNewFreqDown => write!(f, "not pass new freq channel down to driver"),
            ErrorSelector::NotPassNewFreqUp => write!(f, "fnot pass new freq channel up to driver"),
            ErrorSelector::DriverNotSelectFreqChannel => write!(f, "driver not select freq channel"),
            ErrorSelector::NotGetInterface => write!(f, "not get interface of WiFi device"),
            ErrorSelector::UnknownFreq => write!(f, "unknown freq"),
            ErrorSelector::NotSetManagedMode => write!(f, "not set managed mode"),
        }
    }
}