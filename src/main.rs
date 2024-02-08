mod keyboard;

use std::io::Error;

fn main() -> Result<(), Error> {
    use evdev::{Device, Key};
    // let mut device = Device::open("/dev/input/event8")?;
    let mut device = Device::open("/dev/input/event10")?;
// check if the device has an ENTER key
    loop {
        for ev in device.fetch_events().unwrap() {
            println!("{ev:?}");
        }
    }
    return Ok(())
}