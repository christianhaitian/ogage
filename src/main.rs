extern crate evdev_rs as evdev;
extern crate mio;

use evdev::*;
use evdev::enums::*;
use std::io;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::os::unix::io::AsRawFd;
use mio::{Poll,Events,Token,Interest};
use mio::unix::SourceFd;

static HOTKEY:      EventCode = EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY5);
static BRIGHT_UP:   EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_UP);
static BRIGHT_DOWN: EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_DOWN);
static VOL_UP:      EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_RIGHT);
static VOL_DOWN:    EventCode = EventCode::EV_KEY(EV_KEY::BTN_DPAD_LEFT);
//static PERF_MAX:    EventCode = EventCode::EV_KEY(EV_KEY::BTN_TR);
//static PERF_NORM:   EventCode = EventCode::EV_KEY(EV_KEY::BTN_TL);
//static DARK_ON:     EventCode = EventCode::EV_KEY(EV_KEY::BTN_TR2);
//static DARK_OFF:    EventCode = EventCode::EV_KEY(EV_KEY::BTN_TL2);
static VOLUME_UP:   EventCode = EventCode::EV_KEY(EV_KEY::KEY_VOLUMEUP);
static VOLUME_DOWN: EventCode = EventCode::EV_KEY(EV_KEY::KEY_VOLUMEDOWN);
static MUTE:        EventCode = EventCode::EV_KEY(EV_KEY::KEY_PLAYPAUSE);

/*fn blink1() {
    Command::new("brightnessctl").arg("-O").output().expect("Failed to execute brightnessctl");

    Command::new("brightnessctl").args(&["-T","1.5"]).output().expect("Failed to execute brightnessctl");
    Command::new("sleep").arg("0.1").output().expect("Failed to execute brightnessctl");

    Command::new("brightnessctl").arg("-I").output().expect("Failed to execute brightnessctl");
}

fn blink2() {
    Command::new("brightnessctl").arg("-O").output().expect("Failed to execute brightnessctl");

    Command::new("brightnessctl").args(&["-T","1.5"]).output().expect("Failed to execute brightnessctl");
    Command::new("sleep").arg("0.1").output().expect("Failed to execute brightnessctl");

    Command::new("brightnessctl").arg("-I").output().expect("Failed to execute brightnessctl");
    Command::new("sleep").arg("0.1").output().expect("Failed to execute brightnessctl");

    Command::new("brightnessctl").args(&["-T","1.5"]).output().expect("Failed to execute brightnessctl");
    Command::new("sleep").arg("0.1").output().expect("Failed to execute brightnessctl");

    Command::new("brightnessctl").arg("-I").output().expect("Failed to execute brightnessctl");
}*/

fn process_event(_dev: &Device, ev: &InputEvent, hotkey: bool) {
//    println!("Event: time {}.{} type {} code {} value {} hotkey {}",
//             ev.time.tv_sec,
//             ev.time.tv_usec,
//             ev.event_type,
//             ev.event_code,
//             ev.value,
//             hotkey);

    if hotkey && ev.value == 1 {
        if ev.event_code == BRIGHT_UP {
            Command::new("brightnessctl").args(&["s","+2%"]).output().expect("Failed to execute brightnessctl");
            //Command::new("brightnessctl").arg("-O").output().expect("Failed to execute brightnessctl");
        }
        else if ev.event_code == BRIGHT_DOWN {
            Command::new("brightnessctl").args(&["-n","s","2%-"]).output().expect("Failed to execute brightnessctl");
            //Command::new("brightnessctl").arg("-O").output().expect("Failed to execute brightnessctl");
        }
        else if ev.event_code == VOL_UP {
            Command::new("amixer").args(&["-q", "sset", "Playback", "1%+"]).output().expect("Failed to execute amixer");
        }
        else if ev.event_code == VOL_DOWN {
            Command::new("amixer").args(&["-q", "sset", "Playback", "1%-"]).output().expect("Failed to execute amixer");
        }
        /*else if ev.event_code == PERF_MAX {
            Command::new("sudo").args(&["perfmax", "On"]).output().expect("Failed to execute performance");
            //blink1();
        }
        else if ev.event_code == PERF_NORM {
            Command::new("sudo").arg("perfnorm").output().expect("Failed to execute performance");
            //blink1();
        }*/
        else if ev.event_code == EventCode::EV_KEY(EV_KEY::KEY_POWER) && ev.value > 0 {
            //blink2();
            Command::new("finish.sh").spawn().ok().expect("Failed to execute shutdown process");
        }
        /*else if ev.event_code == DARK_ON {
            //Command::new("sudo").args(&["rfkill", "block", "all"]).output().expect("Failed to execute rfkill");
            //blink1();
        //}
        //else if ev.event_code == DARK_OFF {
            //Command::new("sudo").args(&["rfkill", "unblock", "all"]).output().expect("Failed to execute rfkill");
            //blink1();
        }*/
    }
    else if ev.event_code == EventCode::EV_SW(EV_SW::SW_HEADPHONE_INSERT) {
        let dest = match ev.value { 1 => "SPK", _ => "HP" };
        Command::new("amixer").args(&["-q", "sset", "'Playback Path'", dest]).output().expect("Failed to execute amixer");
        //blink1();
    }
    else if ev.event_code == EventCode::EV_KEY(EV_KEY::KEY_POWER) && ev.value == 1 {
        //blink2();
        Command::new("pause.sh").spawn().ok().expect("Failed to execute suspend process");
    }
    else if ev.event_code == VOLUME_UP  && ev.value > 0 {
        Command::new("amixer").args(&["-q", "sset", "Playback", "1%+"]).output().expect("Failed to execute amixer");
    }
    else if ev.event_code == VOLUME_DOWN  && ev.value > 0 {
        Command::new("amixer").args(&["-q", "sset", "Playback", "1%-"]).output().expect("Failed to execute amixer");
    }
    else if ev.event_code == MUTE && ev.value > 0 {
        Command::new("mute_toggle.sh").output().expect("Failed to execute amixer");
    }
}

fn process_event2(_dev: &Device, ev: &InputEvent, selectkey: bool) {
    /*println!("Event: time {}.{} type {} code {} value {} selectkey {}",
             ev.time.tv_sec,
             ev.time.tv_usec,
             ev.event_type,
             ev.event_code,
             ev.value,
             selectkey);*/

    if selectkey{
        if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY4) && ev.value == 1 {
            Command::new("speak_bat_life.sh").spawn().ok().expect("Failed to execute battery reading out loud");
        }
    }
}

fn main() -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1);
    let mut devs: Vec<Device> = Vec::new();
    let mut hotkey = false;
    let mut selectkey = false;

    let mut i = 0;
for s in ["/dev/input/event10", "/dev/input/event9", "/dev/input/event8", "/dev/input/event7", "/dev/input/event6", "/dev/input/event5", "/dev/input/event4", "/dev/input/event3", "/dev/input/event2", "/dev/input/event1", "/dev/input/event0"].iter() {
        if !Path::new(s).exists() {
            println!("Path {} doesn't exist", s);
            continue;
        }
        let fd = File::open(Path::new(s)).unwrap();
        let mut dev = Device::new().unwrap();
        poll.registry().register(&mut SourceFd(&fd.as_raw_fd()), Token(i), Interest::READABLE)?;
        dev.set_fd(fd)?;
        devs.push(dev);
        println!("Added {}", s);
        i += 1;
    }

    //Command::new("brightnessctl").arg("-I").output().expect("Failed to execute brightnessctl");

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            let dev = &mut devs[event.token().0];
            while dev.has_event_pending() {
                let e = dev.next_event(evdev_rs::ReadFlag::NORMAL);
                match e {
                    Ok(k) => {
                        let ev = &k.1;
                        if ev.event_code == HOTKEY {
                            hotkey = ev.value == 1;
                            //let grab = if hotkey { GrabMode::Grab } else { GrabMode::Ungrab };
                            //dev.grab(grab)?;
                        }
                        process_event(&dev, &ev, hotkey);
                        if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY1) {
                            selectkey = ev.value == 1 || ev.value == 2;
                        }
                        process_event2(&dev, &ev, selectkey)
                    },
                    _ => ()
                }
            }
        }
    }
}
