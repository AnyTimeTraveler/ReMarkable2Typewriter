use core::mem;

use evdev;

const UNDO_BUTTON_POS: (u32, u32) = (50, 12100);

pub(crate) struct Keyboard {
    device: evdev::Device,
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    win_pressed: bool,
    caps_lock: bool,
}

pub(crate) struct KeyEvent {
    code: evdev::Key,
    action: KeyAction,
}

enum KeyAction {
    Press,
    Release,
    Hold,
}

impl Keyboard {
    pub(crate) fn with_device(device: evdev::Device) -> Keyboard {
        Keyboard {
            device,
            shift_pressed: false,
            ctrl_pressed: false,
            alt_pressed: false,
            win_pressed: false,
            caps_lock: false,
        }
    }


    pub(crate) fn get_events(&mut self) -> Vec<KeyEvent> {
        self.device.events().unwrap()
            .filter(|e| e._type == 1)
            .filter(|e| { (0..=2).contains(&e.value) })
            .map(|event| {
                KeyEvent {
                    code: unsafe { mem::transmute(event.code as u32) },
                    action: match event.value {
                        0 => KeyAction::Release,
                        1 => KeyAction::Press,
                        2 => KeyAction::Hold,
                        _ => unreachable!()
                    },
                }
            })
            .collect()
    }
}
