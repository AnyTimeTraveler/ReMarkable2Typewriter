use core::mem;
use std::collections::VecDeque;

use evdev;
use evdev::Key;
use std::thread::sleep;
use std::time::Duration;

const UNDO_BUTTON_POS: (u32, u32) = (50, 12100);

pub(crate) struct Keyboard {
    device: evdev::Device,
    shift_pressed: bool,
    ctrl_pressed: bool,
    alt_pressed: bool,
    win_pressed: bool,
    caps_lock_active: bool,
    event_queue: VecDeque<KeyEvent>,
}

pub(crate) struct KeyEvent {
    code: evdev::Key,
    action: KeyAction,
}

#[derive(PartialEq)]
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
            caps_lock_active: false,
            event_queue: VecDeque::with_capacity(100),
        }
    }


    fn get_events(&mut self) {
        let x: Vec<KeyEvent> = self.device.events_no_sync().unwrap()
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
            }).collect();

        for event in x {
            self.event_queue.push_back(event);
        }
    }
}

impl Iterator for Keyboard {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            while self.event_queue.is_empty() {
                sleep(Duration::from_millis(100));
                self.get_events();
            }

            loop {
                let event = self.event_queue.pop_front();
                if event.is_none() { break; }
                let event = event.unwrap();
                let pressed = event.action != KeyAction::Release;
                match event.code {
                    Key::KEY_RIGHTSHIFT | Key::KEY_LEFTSHIFT => self.shift_pressed = pressed,
                    Key::KEY_RIGHTALT | Key::KEY_LEFTALT => self.alt_pressed = pressed,
                    Key::KEY_RIGHTCTRL | Key::KEY_LEFTCTRL => self.ctrl_pressed = pressed,
                    Key::KEY_RIGHTMETA | Key::KEY_LEFTMETA => self.win_pressed = pressed,
                    Key::KEY_CAPSLOCK => if event.action == KeyAction::Release { self.caps_lock_active = !self.caps_lock_active },
                    Key::KEY_NUMLOCK => {}
                    Key::KEY_SCROLLLOCK => {}
                    code => {
                        if pressed {
                            let capital = self.shift_pressed ^ self.caps_lock_active;
                            return Some(
                                match code {
                                    Key::KEY_A => if capital { 'A' } else { 'a' },
                                    Key::KEY_B => if capital { 'B' } else { 'b' },
                                    Key::KEY_C => if capital { 'C' } else { 'c' },
                                    Key::KEY_D => if capital { 'D' } else { 'd' },
                                    Key::KEY_E => if capital { 'E' } else { 'e' },
                                    Key::KEY_F => if capital { 'F' } else { 'f' },
                                    Key::KEY_G => if capital { 'G' } else { 'g' },
                                    Key::KEY_H => if capital { 'H' } else { 'h' },
                                    Key::KEY_I => if capital { 'I' } else { 'i' },
                                    Key::KEY_J => if capital { 'J' } else { 'j' },
                                    Key::KEY_K => if capital { 'K' } else { 'k' },
                                    Key::KEY_L => if capital { 'L' } else { 'l' },
                                    Key::KEY_M => if capital { 'M' } else { 'm' },
                                    Key::KEY_N => if capital { 'N' } else { 'n' },
                                    Key::KEY_O => if capital { 'O' } else { 'o' },
                                    Key::KEY_P => if capital { 'P' } else { 'p' },
                                    Key::KEY_Q => if capital { 'Q' } else { 'q' },
                                    Key::KEY_R => if capital { 'R' } else { 'r' },
                                    Key::KEY_S => if capital { 'S' } else { 's' },
                                    Key::KEY_T => if capital { 'T' } else { 't' },
                                    Key::KEY_U => if capital { 'U' } else { 'u' },
                                    Key::KEY_V => if capital { 'V' } else { 'v' },
                                    Key::KEY_W => if capital { 'W' } else { 'w' },
                                    Key::KEY_X => if capital { 'X' } else { 'x' },
                                    Key::KEY_Y => if capital { 'Y' } else { 'y' },
                                    Key::KEY_Z => if capital { 'Z' } else { 'z' },
                                    Key::KEY_1 => if capital { '!' } else { '1' },
                                    Key::KEY_2 => if capital { '@' } else { '2' },
                                    Key::KEY_3 => if capital { '#' } else { '3' },
                                    Key::KEY_4 => if capital { '$' } else { '4' },
                                    Key::KEY_5 => if capital { '%' } else { '5' },
                                    Key::KEY_6 => if capital { '^' } else { '6' },
                                    Key::KEY_7 => if capital { '&' } else { '7' },
                                    Key::KEY_8 => if capital { '*' } else { '8' },
                                    Key::KEY_9 => if capital { '(' } else { '9' },
                                    Key::KEY_0 => if capital { ')' } else { '0' },
                                    Key::KEY_SPACE => if capital { ' ' } else { ' ' },
                                    Key::KEY_DOT => if capital { '>' } else { '.' },
                                    Key::KEY_COMMA => if capital { '<' } else { ',' },
                                    Key::KEY_SLASH => if capital { '?' } else { '/' },
                                    Key::KEY_SEMICOLON => if capital { ':' } else { ';' },
                                    Key::KEY_APOSTROPHE => if capital { '"' } else { '\'' },
                                    Key::KEY_LEFTBRACE => if capital { '{' } else { '[' },
                                    Key::KEY_RIGHTBRACE => if capital { '}' } else { ']' },
                                    Key::KEY_BACKSLASH => if capital { '|' } else { '\\' },
                                    Key::KEY_MINUS => if capital { '_' } else { '-' },
                                    Key::KEY_EQUAL => if capital { '+' } else { '=' },
                                    Key::KEY_GRAVE =>  if capital {'~'} else {'`'},
                                    Key::KEY_BACKSPACE =>  '\0',
                                    id => {
                                        println!("Unknown Key: {:?}", id);
                                        continue;
                                    }
                                }
                            );
                        }
                    }
                }
            }
        }
    }
}
