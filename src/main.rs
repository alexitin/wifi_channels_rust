use cursive::{align, Cursive};
use cursive::views::{SelectView, Dialog, TextView, LinearLayout, ProgressBar};
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
    
    scan(&mut siv);

    siv.add_global_callback('r', move |s| {
        s.pop_layer();
        scan(s);
    });

    siv.run();
}

fn scan(s: &mut Cursive) {
    let all_devices = match device::AllDevices::new() {
        Ok(dev) => dev,
        Err(pcap_err) => {
            let text = format!("Pcap problem get list devices: {pcap_err}");
            show::exit_cursive(s, &text)
        },
    };

    let wifi_devices = all_devices.get_wifi_devices();

    if wifi_devices.mode.is_none() || wifi_devices.devices.is_empty() {
        let text = ("Not found device for explore Wifi").to_string();
        show::exit_cursive(s, &text);
    }

    let device_content = format!("Name: _______     Mode: _______     Link type: _______");
    let device_info = Dialog::around(TextView::new(device_content)
            .with_name("device_info"))
        .title("Device info");
    let channels_info = Dialog::around(SelectView::<String>::new()
            .align(align::Align::top_center())
            .with_name("channels_info"))
        .title("Channels    Namber AP   Maximal RSSI")
        .full_screen();
    let channel_ssid = Dialog::around(TextView::new("")
            .align(align::Align::top_center())
            .with_name("channel_SSID"))
        .title("SSID            RSSI")
        .full_screen();
    let home_info = Dialog::around(TextView::new("Home network: not selected")
            .with_name("home_info"))
        .title("Home info");
    let home_bar = Dialog::around(ProgressBar::new()
            .with_name("home_bar"));
    let proc_info = Dialog::around(TextView::new("Press q to quit      Press r to rescan")
            .align(align::Align::center())
            .with_name("proc_info"))
        .title("Proc info");

    s.add_layer(LinearLayout::vertical()
        .child(device_info)
        .child(LinearLayout::horizontal()
            .child(channels_info)
            .child(channel_ssid))
        .child(home_info)
        .child(home_bar)
        .child(proc_info)
    );

    show::get_info(s, &wifi_devices);
}
