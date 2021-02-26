#[macro_use]
extern crate lazy_static;

use evdev::Device;

use keyboard::{Keyboard, KeyEvent};

mod hershey_font;
mod devices;
mod keyboard;
mod wacom;
mod drawer;


fn main() {
    println!("Wacom:    {:?}", devices::find_wacom());
    println!("Keyboard: {:?}", devices::find_keyboard());

    let path = String::from("/dev/input/by-path/pci-0000:02:00.0-usb-0:2.2:1.0-event-kbd");
    // let path = find_keyboard();
    let device = Device::open(&path).unwrap();
    let mut keyboard = Keyboard::with_device(device);

    loop {
        while let Some(event) = keyboard.next() {
            println!("Key: {:?}", event);
            // handle_event(event);
        }
    }
}

fn handle_event(event: KeyEvent) {

}
