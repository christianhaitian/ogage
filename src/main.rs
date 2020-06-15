extern crate evdev_rs as evdev;

use evdev::*;
use evdev::enums::*;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::{thread, time};

static HOTKEY:      EventCode = EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY2);
static BRIGHT_UP:   EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_UP);
static BRIGHT_DOWN: EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_DOWN);
static VOL_UP:      EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_RIGHT);
static VOL_DOWN:    EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_LEFT);
static PERF_MAX:    EventCode = EventCode::EV_KEY(EV_KEY::BTN_TR);
static PERF_NORM:   EventCode = EventCode::EV_KEY(EV_KEY::BTN_TL);

fn handle_js(dev: &Device, ev: &InputEvent) {
//    println!("Event: time {}.{}, type {} , code {} , value {}",
//             ev.time.tv_sec,
//             ev.time.tv_usec,
//             ev.event_type,
//             ev.event_code,
//             ev.value);

    if ev.event_type != EventType::EV_KEY || ev.value != 1 {
        return;
    }

    if dev.event_value(&HOTKEY) != Some(1) { return; }

    if ev.event_code == BRIGHT_UP {
        Command::new("light").args(&["-T","1.1"]).output().expect("Failed to execute light");
        Command::new("light").arg("-O").output().expect("Failed to execute light");
    }
    else if ev.event_code == BRIGHT_DOWN {
        Command::new("light").args(&["-T","0.9"]).output().expect("Failed to execute light");
        Command::new("light").arg("-O").output().expect("Failed to execute light");
    }
    else if ev.event_code == VOL_UP {
        Command::new("amixer").args(&["-q", "sset", "Playback", "1%+"]).output().expect("Failed to execute amixer");
    }
    else if ev.event_code == VOL_DOWN {
        Command::new("amixer").args(&["-q", "sset", "Playback", "1%-"]).output().expect("Failed to execute amixer");
    }
    else if ev.event_code == PERF_MAX {
        Command::new("performance").arg("on").output().expect("Failed to execute performance");
    }
    else if ev.event_code == PERF_NORM {
        Command::new("performance").arg("off").output().expect("Failed to execute performance");
    }
}

fn main() {
    Command::new("light").arg("-I").output().expect("Failed to execute light");
    let mut dev_pwr = Device::new().unwrap();
    dev_pwr.set_fd(File::open(Path::new("/dev/input/event0")).unwrap()).unwrap();

    let mut dev_hp = Device::new().unwrap();
    dev_hp.set_fd(File::open(Path::new("/dev/input/event1")).unwrap()).unwrap();

    let mut dev_js = Device::new().unwrap();
    dev_js.set_fd(File::open(Path::new("/dev/input/event2")).unwrap()).unwrap();

    let repeat_rate = time::Duration::from_millis(100);

    loop {
        while dev_js.has_event_pending() {
            let a = dev_js.next_event(evdev_rs::ReadFlag::NORMAL);
            match a {
                Ok(k) => {
                    handle_js(&dev_js, &k.1);
                },
                _ => ()
            }
        }

        if dev_hp.has_event_pending() {
            let a = dev_hp.next_event(evdev_rs::ReadFlag::NORMAL);
            match a {
                Ok(k) => {
                    let ev = &k.1;
                    if ev.event_code == EventCode::EV_SW(EV_SW::SW_HEADPHONE_INSERT) {
                        let mut dest = "HP";
                        if ev.value == 1 { dest = "SPK" }
                        Command::new("amixer").args(&["-q", "sset", "'Playback Path'", dest]).output().expect("Failed to execute amixer");
                    }
                },
                _ => ()
            }
        }

        if dev_pwr.has_event_pending() {
            let a = dev_pwr.next_event(evdev_rs::ReadFlag::NORMAL);
            match a {
                Ok(k) => {
                    let ev = &k.1;
                    if ev.event_code == EventCode::EV_KEY(EV_KEY::KEY_POWER) && ev.value == 1 {
                        if dev_js.event_value(&HOTKEY) == Some(1) {
                            Command::new("sudo").args(&["systemctl", "poweroff"]).output().expect("Failed to execute systemctl");
                        }
                        else {
                            Command::new("sudo").args(&["systemctl", "suspend"]).output().expect("Failed to execute systemctl");
                        }
                    }
                },
                _ => ()
            }
        }

        thread::sleep(repeat_rate);
    }
}
