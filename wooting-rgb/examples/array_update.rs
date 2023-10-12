use std::{thread::sleep, time::Duration};

use wooting_rgb::{Key, RgbKeyboard};

fn main() {
    println!(
        "Keyboard connected? {}",
        wooting_rgb::is_wooting_keyboard_connected()
    );

    let mut keyboard = RgbKeyboard;

    keyboard.array_set_single(Key::Q, 255, 255, 255);
    keyboard.array_set_single(Key::W, 255, 255, 255);
    keyboard.array_set_single(Key::E, 255, 255, 255);
    keyboard.array_set_single(Key::R, 255, 255, 255);
    keyboard.array_set_single(Key::T, 255, 255, 255);
    keyboard.array_set_single(Key::Y, 255, 255, 255);
    sleep(Duration::from_millis(1000));

    println!("Updating... {}", keyboard.array_update());
    sleep(Duration::from_millis(1000));

    println!("Finished!");
}
