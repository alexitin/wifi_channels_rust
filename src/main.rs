use pcap::{Capture, Device, Address};

fn main() {
// Get list all net devices.
    let devices = Device::list();
    let mut devices = match devices {
        Ok(dev) => dev,
        Err(err) => panic!("Error get list devices: {:?}", err),
    };
    println!("List all net devices:");
    for device in &devices {
        println!("{:?}", device);
    }

//Drop devices without access to ethernet,
//a sign of access to the network is the some value of the broadcast address.
//Ip addresses collect in Vec<Address>, where first element is values for IPV6 and second element is values for IPV4.
    devices.retain(|device| -> bool {
        (device.addresses.len() > 0 && device.addresses[0].broadcast_addr.is_some())
        ||
        (device.addresses.len() > 1 && device.addresses[1].broadcast_addr.is_some())
    });
    println!("Devices with access to ethernet:");
    for device in &devices {
        println!("{:?}", device);
    }

//If number devices connected to ethernet more than 1, user must be choice from them.
if devices.len() >1 {
    let i = 1;
    for device in &devices {
        println!("{}. Device: {}", i, device.name )
        
    }
}

}

