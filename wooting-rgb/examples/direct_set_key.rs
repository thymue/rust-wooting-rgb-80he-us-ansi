use std::{thread::sleep, time::Duration};

use wooting_rgb::{Key, RgbKeyboard};

pub const ALL_KEYS: &[Key] = &[
    Key::Escape,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::PrintScreen,
    Key::Pause,
    Key::Mode,
    Key::Tilde,
    Key::One,
    Key::Two,
    Key::Three,
    Key::Four,
    Key::Five,
    Key::Six,
    Key::Seven,
    Key::Eight,
    Key::Nine,
    Key::Zero,
    Key::Dash,
    Key::Equals,
    Key::Backspace,
    Key::Insert,
    Key::PageUp,
    Key::Tab,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::T,
    Key::Y,
    Key::U,
    Key::I,
    Key::O,
    Key::P,
    Key::LeftBracket,
    Key::RightBracket,
    Key::Backslash,
    Key::Delete,
    Key::PageDown,
    Key::CapsLock,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::G,
    Key::H,
    Key::J,
    Key::K,
    Key::L,
    Key::SemiColon,
    Key::Apostrophe,
    Key::Return,
    Key::LeftShift,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
    Key::B,
    Key::N,
    Key::M,
    Key::Comma,
    Key::Period,
    Key::ForwardSlash,
    Key::RightShift,
    Key::UpArrow,
    Key::LeftControl,
    Key::LeftMod,
    Key::LeftAlt,
    Key::SpaceLed1,
    Key::SpaceLed2,
    Key::Space,
    Key::SpaceLed4,
    Key::SpaceLed5,
    Key::RightAlt,
    Key::Fn,
    Key::RightControl,
    Key::LeftArrow,
    Key::DownArrow,
    Key::RightArrow,
];

fn main() {
    println!(
        "Keyboard connected? {}",
        wooting_rgb::is_wooting_keyboard_connected()
    );

    let mut keyboard = RgbKeyboard;

    for key in ALL_KEYS {
        println!("Setting {} to white!", key);
        let _ = keyboard.direct_set_key(*key, 255, 255, 255);
        sleep(Duration::from_millis(500));
    }

    println!("Finished!");
}
