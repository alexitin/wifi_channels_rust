use pcap::Device;

fn main() {
    let devices = Device::list().unwrap();
    for dev in devices {
        println!("{:?}", dev);
    }
}
