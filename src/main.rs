#[macro_use]
extern crate lazy_static;

use evdev::Device;
use keyboard::{KeyEvent,Keyboard};

mod hershey_font;
mod devices;
mod keyboard;
mod wacom;


fn main() {
    println!("Wacom:    {:?}", devices::find_wacom());
    println!("Keyboard: {:?}", devices::find_keyboard());

    let path = String::from("/dev/input/by-path/pci-0000:02:00.0-usb-0:2.2:1.0-event-kbd");
    // let path = find_keyboard();
    let mut device = Device::open(&path).unwrap();
    let mut keyboard = Keyboard::with_device(device);

    loop {
        let events = keyboard.get_events();

        for event in events {
            handle_event(event);
        }
    }
}

fn handle_event(event: KeyEvent) {

}
