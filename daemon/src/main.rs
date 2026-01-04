use std::thread;
use std::time::Duration;

fn main() {
    println!("lock-keys daemon started");

    loop {
        // later: read evdev / libinput here
        thread::sleep(Duration::from_secs(1));
    }
}
