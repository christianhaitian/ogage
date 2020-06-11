extern crate evdev_rs as evdev;

use evdev::*;
use evdev::enums::*;
use std::io;
use std::fs::File;
use std::path::Path;
use std::process::Command;

//fn print_abs_bits(dev: &Device, axis: &EV_ABS) {
//
//    let code = EventCode::EV_ABS(axis.clone());
//
//	if !dev.has(&code) { return; }
//
//	let abs = dev.abs_info(&code).unwrap();
//
//	println!("	Value	{}", abs.value);
//	println!("	Min	{}", abs.minimum);
//	println!("	Max	{}", abs.maximum);
//	if abs.fuzz != 0 {
//		println!("	Fuzz	{}", abs.fuzz);
//    }
//	if abs.flat != 0 {
//		println!("	Flat	{}", abs.flat);
//    }
//	if abs.resolution != 0 {
//		println!("	Resolution	{}", abs.resolution);
//    }
//}

//fn print_code_bits(dev: &Device, ev_code: &EventCode, max: &EventCode) {
//    for code in ev_code.iter() {
//        if code == *max {
//            break;
//        }
//        if !dev.has(&code) {
//            continue;
//        }
//
//		println!("    Event code: {}", code);
//        match code {
//            EventCode::EV_ABS(k) => print_abs_bits(dev, &k),
//            _ => ()
//        }
//    }
//}

//fn print_bits(dev: &Device) {
//    println!("Supported events:");
//
//    for ev_type in  EventType::EV_SYN.iter() {
//		if dev.has(&ev_type) {
//			println!("  Event type: {} ", ev_type);
//        }
//
//        match ev_type {
//            EventType::EV_KEY => print_code_bits(dev, &EventCode::EV_KEY(EV_KEY::KEY_RESERVED),
//                                                 &EventCode::EV_KEY(EV_KEY::KEY_MAX)),
//            EventType::EV_REL => print_code_bits(dev, &EventCode::EV_REL(EV_REL::REL_X),
//                                                 &EventCode::EV_REL(EV_REL::REL_MAX)),
//            EventType::EV_ABS => print_code_bits(dev, &EventCode::EV_ABS(EV_ABS::ABS_X),
//                                                 &EventCode::EV_ABS(EV_ABS::ABS_MAX)),
//            EventType::EV_LED => print_code_bits(dev, &EventCode::EV_LED(EV_LED::LED_NUML),
//                                                 &EventCode::EV_LED(EV_LED::LED_MAX)),
//            _ => (),
//		}
//	}
//}

//fn print_props(dev: &Device) {
//	println!("Properties:");
//
//	for input_prop in InputProp::INPUT_PROP_POINTER.iter() {
//		if dev.has(&input_prop) {
//			println!("  Property type: {}", input_prop);
//        }
//    }
//}

fn handle_event(dev: &Device, ev: &InputEvent) {
//    if ev.event_type != EventType::EV_KEY {
//        return;
//    }

    let f3 = dev.event_value(&EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY3));
    println!("Event: time {}.{}, type {} , code {} , value {} f3 {}",
             ev.time.tv_sec,
             ev.time.tv_usec,
             ev.event_type,
             ev.event_code,
             ev.value,
             f3.unwrap());

    if ev.event_code == EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY5) {
        println!("F5 was pressed");
    }
}

fn main() {
    let f = File::open(Path::new("/dev/input/event8")).unwrap();

    let mut dev = Device::new().unwrap();
    dev.set_fd(f).unwrap();

    let mut a: io::Result<(ReadStatus, InputEvent)>;
    loop {
        a = dev.next_event(evdev::ReadFlag::NORMAL | evdev::ReadFlag::BLOCKING);
        if a.is_ok() {
            let result = a.ok().unwrap();
            if result.0 == ReadStatus::Success {
                handle_event(&dev, &result.1);
            }
        } else {
            println!("Error");
        }
    }
}
