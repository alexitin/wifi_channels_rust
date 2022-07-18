#[cfg(target_os = "macos")]
    mod macos_selector;
    pub use macos_selector::MacOsSelector as SelectorChannel; 