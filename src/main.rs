use cursive::align;
use cursive::views::{SelectView, Dialog, TextView, LinearLayout};
use cursive::view::{Nameable, Resizable};

mod device;
mod selector;
mod frame;
mod parse_radiotap;
mod parse_avs;
mod parse_ppi;
mod parse_80211;
mod show;

fn main() {
    
//Cursive tui
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

// Get list all net devices.
    let all_devices = match device::AllDevices::new() {
        Ok(dev) => dev,
        Err(pcap_err) => {
            let text = format!("Pcap problem get list devices: {pcap_err}");
            show::exit_cursive(siv, &text)
        },
    };

// Get wifi devices supporting monitor, promiscouos or normal mode and check support radio DLT
    let wifi_devices = all_devices.get_wifi_devices();

    if wifi_devices.mode.is_none() || wifi_devices.devices.is_empty() {
        let text = ("Not found device for explore Wifi").to_string();
        show::exit_cursive(siv, &text);
    }

    let device_content = format!("Name: _______     Mode: _______     Link type: _______");
    let device_info = Dialog::around(TextView::new(device_content)
            .with_name("device_info"))
        .title("Device info");
    let channels_info = Dialog::around(SelectView::<String>::new()
            .align(align::Align::top_center())
            .with_name("channels_info"))
        .title("Channels    Namber AP   Maximal RSSI")
        .fixed_width(85);
    let channel_ssid = Dialog::around(TextView::new("")
            .align(align::Align::top_center())
            .with_name("channel_SSID"))
        .title("SSID            RSSI")
        .full_screen();
    let home = Dialog::around(TextView::new("Press q to quit")
            .align(align::Align::top_center())
            .with_name("home"))
        .title("Info");

    siv.add_layer(LinearLayout::vertical()
        .child(device_info)
        .child(LinearLayout::horizontal()
            .child(channels_info)
            .child(channel_ssid))
        .child(home)
    );

    show::get_info(&mut siv, wifi_devices);

    siv.run();
}
