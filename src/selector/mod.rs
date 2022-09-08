#[cfg(target_os = "macos")]
    mod macos_selector;
#[cfg(target_os = "macos")]
    pub use macos_selector::MacOsSelector as SelectorChannel;

#[cfg(target_os = "linux")]
    mod  linux_selector;
#[cfg(target_os = "linux")]
    pub use linux_selector::LinuxSelector as SelectorChannel;