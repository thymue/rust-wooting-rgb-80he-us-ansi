use std::{thread::sleep, time::Duration};

use wooting_rgb::{Key, RgbKeyboard};

fn main() {
    println!(
        "Keyboard connected? {}",
        wooting_rgb::is_wooting_keyboard_connected()
    );

    let mut keyboard = RgbKeyboard;

    let array = vec![
        (Key::Escape, (255, 255, 255)),
        (Key::F1, (255, 255, 255)),
        (Key::F2, (255, 255, 255)),
        (Key::F3, (255, 255, 255)),
        (Key::F4, (255, 255, 255)),
        (Key::F5, (255, 255, 255)),
        (Key::F6, (255, 255, 255)),
        (Key::F7, (255, 255, 255)),
        (Key::F8, (255, 255, 255)),
        (Key::F9, (255, 255, 255)),
        (Key::F10, (255, 255, 255)),
        (Key::F11, (255, 255, 255)),
        (Key::F12, (255, 255, 255)),
        (Key::PrintScreen, (255, 255, 255)),
        (Key::Pause, (255, 255, 255)),
        (Key::Mode, (255, 255, 255)),
        (Key::Tilde, (255, 255, 255)),
        (Key::One, (255, 255, 255)),
        (Key::Two, (255, 255, 255)),
        (Key::Three, (255, 255, 255)),
        (Key::Four, (255, 255, 255)),
        (Key::Five, (255, 255, 255)),
        (Key::Six, (255, 255, 255)),
        (Key::Seven, (255, 255, 255)),
        (Key::Eight, (255, 255, 255)),
        (Key::Nine, (255, 255, 255)),
        (Key::Zero, (255, 255, 255)),
        (Key::Dash, (255, 255, 255)),
        (Key::Equals, (255, 255, 255)),
        (Key::Backspace, (255, 255, 255)),
        (Key::Insert, (255, 255, 255)),
        (Key::PageUp, (255, 255, 255)),
        (Key::Tab, (255, 255, 255)),
        (Key::Q, (255, 255, 255)),
        (Key::W, (255, 255, 255)),
        (Key::E, (255, 255, 255)),
        (Key::R, (255, 255, 255)),
        (Key::T, (255, 255, 255)),
        (Key::Y, (255, 255, 255)),
        (Key::U, (255, 255, 255)),
        (Key::I, (255, 255, 255)),
        (Key::O, (255, 255, 255)),
        (Key::P, (255, 255, 255)),
        (Key::LeftBracket, (255, 255, 255)),
        (Key::RightBracket, (255, 255, 255)),
        (Key::Backslash, (255, 255, 255)),
        (Key::Delete, (255, 255, 255)),
        (Key::PageDown, (255, 255, 255)),
        (Key::CapsLock, (255, 255, 255)),
        (Key::A, (255, 255, 255)),
        (Key::S, (255, 255, 255)),
        (Key::D, (255, 255, 255)),
        (Key::F, (255, 255, 255)),
        (Key::G, (255, 255, 255)),
        (Key::H, (255, 255, 255)),
        (Key::J, (255, 255, 255)),
        (Key::K, (255, 255, 255)),
        (Key::L, (255, 255, 255)),
        (Key::SemiColon, (255, 255, 255)),
        (Key::Apostrophe, (255, 255, 255)),
        (Key::Return, (255, 255, 255)),
        (Key::LeftShift, (255, 255, 255)),
        (Key::Z, (255, 255, 255)),
        (Key::X, (255, 255, 255)),
        (Key::C, (255, 255, 255)),
        (Key::V, (255, 255, 255)),
        (Key::B, (255, 255, 255)),
        (Key::N, (255, 255, 255)),
        (Key::M, (255, 255, 255)),
        (Key::Comma, (255, 255, 255)),
        (Key::Period, (255, 255, 255)),
        (Key::ForwardSlash, (255, 255, 255)),
        (Key::RightShift, (255, 255, 255)),
        (Key::UpArrow, (255, 255, 255)),
        (Key::LeftControl, (255, 255, 255)),
        (Key::LeftMod, (255, 255, 255)),
        (Key::LeftAlt, (255, 255, 255)),
        (Key::SpaceLed1, (255, 255, 255)),
        (Key::SpaceLed2, (255, 255, 255)),
        (Key::Space, (255, 255, 255)),
        (Key::SpaceLed4, (255, 255, 255)),
        (Key::SpaceLed5, (255, 255, 255)),
        (Key::RightAlt, (255, 255, 255)),
        (Key::Fn, (255, 255, 255)),
        (Key::RightControl, (255, 255, 255)),
        (Key::LeftArrow, (255, 255, 255)),
        (Key::DownArrow, (255, 255, 255)),
        (Key::RightArrow, (255, 255, 255)),
    ];

    keyboard.array_set_full(&array);
    sleep(Duration::from_millis(5000));

    println!("Updating... {}", keyboard.array_update());
    sleep(Duration::from_millis(5000));

    println!("Finished!");
}
