use evdev::{Device, KeyCode};
use log::{debug, error, info};
use std::fs;

fn main() -> std::io::Result<()> {
    let mut dev: Option<Device> = None;

    for entry in fs::read_dir("/dev/input")? {
        let entry = entry?;
        let path = entry.path();

        let name = path.file_name().unwrap().to_string_lossy();

        if !name.starts_with("event") {
            continue;
        }

        let device = match Device::open(&path) {
            Ok(d) => d,
            Err(_) => continue,
        };

        let is_keyboard = device.supported_keys().map_or(false, |keys| {
            keys.contains(KeyCode::KEY_CAPSLOCK) || keys.contains(KeyCode::KEY_NUMLOCK)
        });

        if is_keyboard {
            println!("Keyboard found at: {}", &path.to_string_lossy());
            dev = Some(device);
            break; // Found a keyboard, stop searching
        }
    }

    let mut dev = dev.expect("No keyboard device found");

    loop {
        let events = dev.fetch_events()?;
        for ev in events {
            if ev.event_type() == evdev::EventType::KEY {
                let key = KeyCode(ev.code());
                if key == evdev::KeyCode::KEY_NUMLOCK || key == evdev::KeyCode::KEY_CAPSLOCK {
                    println!("{:?}", ev);
                }
            }
        }
    }
}
