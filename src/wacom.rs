use std::time::{SystemTime, Duration};

use evdev;
use input_linux::sys::*;
use std::thread::sleep;

struct Wacom(Vec<evdev::raw::input_event>);

impl Wacom {
    fn press_ui_button(&mut self, x: i32, y: i32) {
        self.add_event(EV_KEY, BTN_TOOL_PEN, 1);
        self.add_event(EV_KEY, BTN_TOUCH, 0);
        self.add_event(EV_ABS, ABS_PRESSURE, 0);
        self.add_event(EV_ABS, ABS_DISTANCE, 80);
        self.add_event(EV_SYN, SYN_REPORT, 0);
        self.send_events();
        self.add_event(EV_ABS, ABS_X, y);
        self.add_event(EV_ABS, ABS_Y, x);
        self.add_event(EV_ABS, ABS_PRESSURE, 2048);
        self.add_event(EV_ABS, ABS_DISTANCE, 0);
        self.add_event(EV_ABS, ABS_TILT_X, 0);
        self.add_event(EV_ABS, ABS_TILT_Y, 0);
        self.add_event(EV_SYN, SYN_REPORT, 0);
        self.send_events();
        self.add_event(EV_KEY, BTN_TOUCH, 1);
        self.add_event(EV_SYN, SYN_REPORT, 0);
        self.send_events();
        sleep(Duration::from_millis(10));  // <---- If I remove this, strokes are missing.

        // Pen up
        // self.add_event(EV_ABS, ABS_X, y);
        // self.add_event(EV_ABS, ABS_Y, x);
        self.add_event(EV_KEY, BTN_TOUCH, 0);
        self.add_event(EV_ABS, ABS_DISTANCE, 80);
        self.send_events();
        self.add_event(EV_KEY, BTN_TOOL_PEN, 0);
        self.add_event(EV_SYN, SYN_REPORT, 0);
        self.send_events();
    }

    fn add_event(&mut self, type_: i32, code: i32, value: i32) {
        let time = SystemTime::now().elapsed().unwrap();
        self.0.push(evdev::raw::input_event {
            time: timeval {
                tv_sec: time.as_secs() as i64,
                tv_usec: time.subsec_micros() as i64,
            },
            _type: type_ as u16,
            code: code as u16,
            value,
        });
    }

    fn send_events(&mut self) {
        self.0.clear();
    }
}
