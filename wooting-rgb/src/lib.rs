//! # Wooting-RGB
//! Wooting RGB SDK Rust Bindings and Library
//!
//! ## Example
//! See the [`wooting-rgb/src/examples/`][examples] directory for more examples.
//!
//! [examples]: https://github.com/ShayBox/Wooting-RGB/tree/master/wooting-rgb/examples

use std::{
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use thiserror::Error;

/// Represents an error that can occur when querying the state of a Wooting keyboard.
#[derive(Clone, Copy, Debug, Error, Eq, Hash, PartialEq)]
pub enum WootingError {
    #[error("Wooting keyboard is not connected")]
    Disconnected,
    #[error("Requested analog value of too many keys")]
    InvalidBufferSize,
}

/// Types that implement this trait can be transformed into a matrix row and column.
pub trait IntoMatrixRowColumn {
    /// Return a tuple `(row, column)` that represents the matrix row and column for this type.
    fn get_matrix_row_and_column(&self) -> (u8, u8);
}

impl IntoMatrixRowColumn for (u8, u8) {
    fn get_matrix_row_and_column(&self) -> (u8, u8) {
        *self
    }
}

/// Types that implement this trait can be associated with a scan index.
pub trait FromScanIndex: Sized {
    /// Return the instance of this type for the given scan index.
    fn from_scan_index(index: u8) -> Option<Self>;
}

/// Represents a key on the keyboard.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Key {
    /// Escape key (`Esc`). Generates the escape character (ASCII 27).
    Escape,
    /// Function key (`F1`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F1,
    /// Function key (`F2`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F2,
    /// Function key (`F3`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F3,
    /// Function key (`F4`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F4,
    /// Function key (`F5`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F5,
    /// Function key (`F6`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F6,
    /// Function key (`F7`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F7,
    /// Function key (`F8`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F8,
    /// Function key (`F9`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F9,
    /// Function key (`F10`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F10,
    /// Function key (`F11`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F11,
    /// Function key (`F12`). Normally programmed to cause an operating system or application to
    /// perform certain actions.
    F12,
    /// Mode key. Toggles between digital and analog modes.
    Mode,
    /// Print screen key (`Prt Sc`). May share the same key as system request. Normally takes a
    /// screenshot.
    PrintScreen,
    /// Pause (or break) key. Has no well defined purpose.
    Pause,
    /// Tilde key (`~`).
    Tilde,
    /// Number one key (`1`).
    One,
    /// Number two key (`2`).
    Two,
    /// Number three key (`3`).
    Three,
    /// Number four key (`4`).
    Four,
    /// Number five key (`5`).
    Five,
    /// Number six key (`6`).
    Six,
    /// Number seven key (`7`).
    Seven,
    /// Number eight key (`8`).
    Eight,
    /// Number nine key (`9`).
    Nine,
    /// Number zero key (`0`).
    Zero,
    /// Dash or hyphen key (`-`).
    Dash,
    /// Equals key (`=`).
    Equals,
    /// Backspace key. Moves display cursor one position backwards, deleting the character at
    /// that position and shifting back the text after that position by one position.
    Backspace,
    /// Insert key (`Ins`). Switches between two text entry modes - over-type or insert. Over-type
    /// mode replaces the character present in the current location. Insert mode inserts a
    /// character at the current position, forcing all characters past it one position further.
    Insert,
    /// Page up key (`Pg Up`). Scrolls up in documents.
    PageUp,
    /// Tab key. Advances cursor to next tab stop.
    Tab,
    /// Letter `q` key.
    Q,
    /// Letter `w` key.
    W,
    /// Letter `e` key.
    E,
    /// Letter `r` key.
    R,
    /// Letter `t` key.
    T,
    /// Letter `y` key.
    Y,
    /// Letter `u` key.
    U,
    /// Letter `i` key.
    I,
    /// Letter `o` key.
    O,
    /// Letter `p` key.
    P,
    /// Left square bracket key (`[`).
    LeftBracket,
    /// Right square bracket key (`]`).
    RightBracket,
    /// Backslash key (`\`).
    Backslash,
    /// Delete key (`Del`). Deletes the character in the position after the cursor.
    Delete,
    /// Page down key (`Pg Dn`). Scrolls down in documents.
    PageDown,
    /// Capitalization lock key. Causes all letters in latin-based scripts to be generated in
    /// capitals.
    CapsLock,
    /// Letter `a` key.
    A,
    /// Letter `s` key.
    S,
    /// Letter `d` key.
    D,
    /// Letter `f` key.
    F,
    /// Letter `g` key.
    G,
    /// Letter `h` key.
    H,
    /// Letter `j` key.
    J,
    /// Letter `k` key.
    K,
    /// Letter `l` key.
    L,
    /// Semi-colon key (`;`).
    SemiColon,
    /// Apostrophe key (`'`).
    Apostrophe,
    /// Return (or enter) key.
    Return,
    /// Left shift modifier key. Used to type capital letters and other alternate "upper"
    /// characters.
    LeftShift,
    /// Letter `z` key.
    Z,
    /// Letter `x` key.
    X,
    /// Letter `c` key.
    C,
    /// Letter `v` key.
    V,
    /// Letter `b` key.
    B,
    /// Letter `n` key.
    N,
    /// Letter `m` key.
    M,
    /// Comma key (`,`).
    Comma,
    /// Period key (`.`).
    Period,
    /// Forward slash key (`/`).
    ForwardSlash,
    /// Right shift modifier key. Performs the same function as left shift.
    RightShift,
    /// Up arrow key. Moves the cursor in the upwards direction.
    UpArrow,
    /// Left control modifier key. Performs a special operation when pressed in conjunction with
    /// another key.
    LeftControl,
    /// Left mod (or Windows) modifier key. Normally invokes the operating system's start menu.
    LeftMod,
    /// Left alt modifier key. Used to change (alternate) the function of other pressed keys.
    LeftAlt,
    /// Space key LED 1 (starts from the left)
    SpaceLed1,
    /// Space key LED 2
    SpaceLed2,
    /// Space key LED 3 (actual space key) (` `).
    Space,
    /// Space key LED 4.
    SpaceLed4,
    /// Space key LED 5.
    SpaceLed5,
    /// Right alt modifier key. Performs the same function as left alt.
    RightAlt,
    /// Function key (`Fn`). Performs an alternative operation for some keys, normally defined
    /// by the keyboard and indicated by symbols on the key.
    Fn,
    /// Right control modifier key. Performs the same function as left control.
    RightControl,
    /// Left arrow key. Moves the cursor in the left direction.
    LeftArrow,
    /// Down arrow key. Moves the cursor in the down direction.
    DownArrow,
    /// Right arrow key. Moves the cursor in the right direction.
    RightArrow,
}

impl Display for Key {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Key::*;
        write!(
            fmt,
            "{}",
            match self {
                Escape => "Esc",
                F1 => "F1",
                F2 => "F2",
                F3 => "F3",
                F4 => "F4",
                F5 => "F5",
                F6 => "F6",
                F7 => "F7",
                F8 => "F8",
                F9 => "F9",
                F10 => "F10",
                F11 => "F11",
                F12 => "F12",
                PrintScreen => "Print Screen",
                Pause => "Pause",
                Mode => "Mode",
                Tilde => "~",
                One => "1",
                Two => "2",
                Three => "3",
                Four => "4",
                Five => "5",
                Six => "6",
                Seven => "7",
                Eight => "8",
                Nine => "9",
                Zero => "0",
                Dash => "-",
                Equals => "=",
                Backspace => "Backspace",
                Insert => "Insert",
                PageUp => "Page Up",
                Tab => "Tab",
                Q => "Q",
                W => "W",
                E => "E",
                R => "R",
                T => "T",
                Y => "Y",
                U => "U",
                I => "I",
                O => "O",
                P => "P",
                LeftBracket => "[",
                RightBracket => "]",
                Backslash => "\\",
                Delete => "Delete",
                PageDown => "Page Down",
                CapsLock => "Caps Lock",
                A => "A",
                S => "S",
                D => "D",
                F => "F",
                G => "G",
                H => "H",
                J => "J",
                K => "K",
                L => "L",
                SemiColon => ";",
                Apostrophe => "'",
                Return => "Return",
                LeftShift => "Left Shift",
                Z => "Z",
                X => "X",
                C => "C",
                V => "V",
                B => "B",
                N => "N",
                M => "M",
                Comma => "Comma",
                Period => "Period",
                ForwardSlash => "/",
                RightShift => "Right Shift",
                UpArrow => "Up Arrow",
                LeftControl => "Left Control",
                LeftMod => "Left Mod",
                LeftAlt => "Left Alt",
                Space => "Space",
                RightAlt => "Right Alt",
                Fn => "Fn",
                RightControl => "Right Control",
                LeftArrow => "Left Arrow",
                DownArrow => "Down Arrow",
                RightArrow => "Right Arrow",
                SpaceLed1 => "Space LED 1",
                SpaceLed2 => "Space LED 2",
                SpaceLed4 => "Space LED 4",
                SpaceLed5 => "Space LED 5",
            }
        )
    }
}

impl FromScanIndex for Key {
    /// Return the key that corresponds to the provided scan index, if any.
    fn from_scan_index(index: u8) -> Option<Self> {
        use Key::*;
        Some(match index {
            0 => Escape,
            1 => F1,
            2 => F2,
            3 => F3,
            4 => F4,
            5 => F5,
            6 => F6,
            7 => F7,
            8 => F8,
            9 => F9,
            10 => F10,
            11 => F11,
            12 => F12,
            13 => PrintScreen,
            14 => Pause,
            16 => Tilde,
            17 => One,
            18 => Two,
            19 => Three,
            20 => Four,
            21 => Five,
            22 => Six,
            23 => Seven,
            24 => Eight,
            25 => Nine,
            26 => Zero,
            27 => Dash,
            28 => Equals,
            29 => Backspace,
            30 => Insert,
            32 => Tab,
            33 => Q,
            34 => W,
            35 => E,
            36 => R,
            37 => T,
            38 => Y,
            39 => U,
            40 => I,
            41 => O,
            42 => P,
            43 => LeftBracket,
            44 => RightBracket,
            // 45 is also associated with `Key::Backslash`
            46 => Delete,
            48 => CapsLock,
            49 => A,
            50 => S,
            51 => D,
            52 => F,
            53 => G,
            54 => H,
            55 => I,
            56 => J,
            57 => K,
            58 => L,
            59 => SemiColon,
            60 => Return,
            61 => PageUp,
            62 => PageDown,
            63 => UpArrow,
            64 => LeftShift,
            65 => Z,
            66 => X,
            67 => C,
            68 => V,
            69 => B,
            70 => N,
            71 => M,
            72 => Comma,
            73 => Period,
            74 => ForwardSlash,
            75 => RightShift,
            76 => LeftArrow,
            77 => DownArrow,
            78 => RightArrow,
            79 => RightControl,
            80 => LeftControl,
            81 => LeftMod,
            82 => LeftAlt,
            83 => Space,
            84 => RightAlt,
            86 => Fn,
            110 => Mode,
            // Invalid index should only ever be one of: 88 | 89 | 111 ..= 255
            _ => return None,
        })
    }
}

impl IntoMatrixRowColumn for Key {
    /// Returns a tuple `(row, column)` that represents the matrix row and column of the key.
    fn get_matrix_row_and_column(&self) -> (u8, u8) {
        use Key::*;
        match self {
            Escape => (0, 0),
            F1 => (0, 1),
            F2 => (0, 2),
            F3 => (0, 3),
            F4 => (0, 4),
            F5 => (0, 5),
            F6 => (0, 6),
            F7 => (0, 7),
            F8 => (0, 8),
            F9 => (0, 10),
            F10 => (0, 11),
            F11 => (0, 12),
            F12 => (0, 13),
            Mode => (0, 14),
            PrintScreen => (0, 15),
            Pause => (0, 16),

            Tilde => (1, 0),
            One => (1, 1),
            Two => (1, 2),
            Three => (1, 3),
            Four => (1, 4),
            Five => (1, 5),
            Six => (1, 6),
            Seven => (1, 7),
            Eight => (1, 8),
            Nine => (1, 9),
            Zero => (1, 10),
            Dash => (1, 11),
            Equals => (1, 12),
            Backspace => (1, 14),
            Insert => (1, 15),
            PageUp => (1, 16),

            Tab => (2, 0),
            Q => (2, 1),
            W => (2, 2),
            E => (2, 3),
            R => (2, 4),
            T => (2, 5),
            Y => (2, 6),
            U => (2, 7),
            I => (2, 8),
            O => (2, 9),
            P => (2, 10),
            LeftBracket => (2, 11),
            RightBracket => (2, 12),
            Backslash => (2, 14),
            Delete => (2, 15),
            PageDown => (2, 16),

            CapsLock => (3, 0),
            A => (3, 1),
            S => (3, 2),
            D => (3, 3),
            F => (3, 4),
            G => (3, 5),
            H => (3, 6),
            J => (3, 7),
            K => (3, 8),
            L => (3, 9),
            SemiColon => (3, 10),
            Apostrophe => (3, 11),
            Return => (3, 14),

            LeftShift => (4, 0),
            Z => (4, 2),
            X => (4, 3),
            C => (4, 4),
            V => (4, 5),
            B => (4, 6),
            N => (4, 7),
            M => (4, 8),
            Comma => (4, 9),
            Period => (4, 10),
            ForwardSlash => (4, 11),
            RightShift => (4, 14),
            UpArrow => (4, 15),

            LeftControl => (5, 0),
            LeftMod => (5, 1),
            LeftAlt => (5, 2),
            SpaceLed1 => (5, 4),
            SpaceLed2 => (5, 5),
            Space => (5, 6),
            SpaceLed4 => (5, 7),
            SpaceLed5 => (5, 8),
            RightAlt => (5, 10),
            Fn => (5, 11),
            RightControl => (5, 12),
            LeftArrow => (5, 14),
            DownArrow => (5, 15),
            RightArrow => (5, 16),
        }
    }
}

/// How many columns are there?
const COLUMNS: usize = 21;
/// How many rows are there?
const ROWS: usize = 6;
/// How many components are there in a color?
const COMPONENTS: usize = 3;
lazy_static! {
    static ref CALLBACK: Mutex<Option<Box<dyn Fn() + Send>>> = Default::default();
}

/// Is there a Wooting keyboard connected?
///
/// ```rust,no_run
/// // Assert that a Wooting keyboard is connected..
/// assert!(wooting_rgb::is_wooting_keyboard_connected());
/// ```
pub fn is_wooting_keyboard_connected() -> bool {
    unsafe { wooting_rgb_sys::wooting_rgb_kbd_connected() }
}

/// This is a trampoline function that is provided to the C function to be invoked which will
/// in turn invoke the user provided callback. The user provided callback would normally be
/// stored in userdata but due to the lack of any, we use a static instead.
extern "C" fn set_disconnected_callback_handler() {
    if let Some(ref mut callback) = *CALLBACK.lock().unwrap() {
        callback();
    } else {
        panic!("Callback static has not been set");
    }
}

/// Set a callback to be invoked when a keyboard is disconnected. Currently only happens on a
/// failed read.
///
/// See [`rgb_disconnected_callback`][example] example for usage.
///
/// [example]: https://github.com/shaybox/wooting-rgb/blob/master/wooting-rgb/examples/rgb_disconnected_callback.rs
pub fn set_disconnected_callback<F: 'static + Fn() + Send>(callback: F) {
    *CALLBACK.lock().unwrap() = Some(Box::new(callback));
    unsafe {
        wooting_rgb_sys::wooting_rgb_set_disconnected_cb(Some(set_disconnected_callback_handler));
    }
}

/// Represents the connected keyboard to perform RGB operations. This struct is empty and
/// only exists to enforce that `reset` is called on drop.
#[derive(Clone, Debug, Default)]
pub struct RgbKeyboard;

impl RgbKeyboard {
    /// Set the color of a single key. This will not influence the keyboard color array. Use
    /// this function for simple amplifications, like a notification. Use the array functions
    /// if you want to change the entire keyboard. Returns `true` if the color is set.
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Set the A key to white...
    /// keyboard.direct_set_key(Key::A, 255, 255, 255);
    /// ```
    pub fn direct_set_key<K: IntoMatrixRowColumn>(
        &mut self,
        key: K,
        red: u8,
        green: u8,
        blue: u8,
    ) -> bool {
        let (row, column) = key.get_matrix_row_and_column();
        unsafe { wooting_rgb_sys::wooting_rgb_direct_set_key(row, column, red, green, blue) }
    }

    /// Directly reset the color of a single key on the keyboard. This will not influence the
    /// keyboard color array. Use this function for simple amplifications, like a notification.
    /// Use the array functions if you want to change the entire keyboard. Returns `true` if
    /// the color is reset.
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Set the A key to white...
    /// keyboard.direct_set_key(Key::A, 255, 255, 255);
    /// // ..and then reset it back!
    /// keyboard.direct_reset_key(Key::A);
    /// ```
    pub fn direct_reset_key<K: IntoMatrixRowColumn>(&mut self, key: K) -> bool {
        let (row, column) = key.get_matrix_row_and_column();
        unsafe { wooting_rgb_sys::wooting_rgb_direct_reset_key(row, column) }
    }

    /// Apply any updates made by the `array_set_single` and `array_set_full` functions.
    /// Returns `true` if the colors are updated.
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Modify keyboard array so A will be set to white..
    /// keyboard.array_set_single(Key::A, 255, 255, 255);
    /// // ..and apply the change.
    /// keyboard.array_update();
    /// ```
    pub fn array_update(&mut self) -> bool {
        unsafe { wooting_rgb_sys::wooting_rgb_array_update_keyboard() }
    }

    /// Set an auto-update trigger after every change with the `array_set_single` and
    /// `array_set_full` functions. By default, no auto-update trigger is set.
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Make keyboard array changes apply automatically..
    /// keyboard.array_auto_update(true);
    /// // ..and then modify the array so QWERTY are set to white...
    /// // ..with no need for a call to `array_update`!
    /// keyboard.array_set_full(&[
    ///     (Key::Q, (255, 255, 255)),
    ///     (Key::W, (255, 255, 255)),
    ///     (Key::E, (255, 255, 255)),
    ///     (Key::R, (255, 255, 255)),
    ///     (Key::T, (255, 255, 255)),
    ///     (Key::Y, (255, 255, 255)),
    /// ]);
    /// ```
    pub fn array_auto_update(&mut self, auto_update: bool) {
        unsafe { wooting_rgb_sys::wooting_rgb_array_auto_update(auto_update) }
    }

    /// Set a single color in the color array. This will not directly update the keyboard
    /// unless the auto update flag is set (see `array_auto_update`), so it can be called
    /// frequently (i.e. in a loop that updates the entire keyboard). Returns `true` if the
    /// colors have changed.
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Modify the keyboard array so QWERTY will be set to white..
    /// keyboard.array_set_single(Key::Q, 255, 255, 255);
    /// keyboard.array_set_single(Key::W, 255, 255, 255);
    /// keyboard.array_set_single(Key::E, 255, 255, 255);
    /// keyboard.array_set_single(Key::R, 255, 255, 255);
    /// keyboard.array_set_single(Key::T, 255, 255, 255);
    /// keyboard.array_set_single(Key::Y, 255, 255, 255);
    /// // ..and apply the change.
    /// keyboard.array_update();
    /// ```
    pub fn array_set_single<K: IntoMatrixRowColumn>(
        &mut self,
        key: K,
        red: u8,
        green: u8,
        blue: u8,
    ) -> bool {
        let (row, column) = key.get_matrix_row_and_column();
        unsafe { wooting_rgb_sys::wooting_rgb_array_set_single(row, column, red, green, blue) }
    }

    /// Set a single color in the color array manually using a (row, column) touple instead
    /// of a Key. This will not directly update the keyboard unless the auto update flag is
    /// set (see `array_auto_update`), so it can be called frequently (i.e. in a loop that
    /// updates the entire keyboard). Returns `true` if the colors have changed.
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Modify the keyboard array so QWERTY will be set to white..
    /// keyboard.array_set_single_matrix((2, 1), 255, 255, 255);
    /// keyboard.array_set_single_matrix((2, 2), 255, 255, 255);
    /// keyboard.array_set_single_matrix((2, 3), 255, 255, 255);
    /// keyboard.array_set_single_matrix((2, 4), 255, 255, 255);
    /// keyboard.array_set_single_matrix((2, 5), 255, 255, 255);
    /// keyboard.array_set_single_matrix((2, 6), 255, 255, 255);
    /// // ..and apply the change.
    /// keyboard.array_update();
    /// ```
    pub fn array_set_single_matrix(
        &mut self,
        row_and_column: (u8, u8),
        red: u8,
        green: u8,
        blue: u8,
    ) -> bool {
        let (row, column) = row_and_column;
        unsafe { wooting_rgb_sys::wooting_rgb_array_set_single(row, column, red, green, blue) }
    }

    /// Set a complete color array. This will not directly update the keyboard unless the auto
    /// update flag is set (see `array_auto_update`). Returns `true` if the colors have
    /// changed.
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Modify the keyboard array so QWERTY will be set to white..
    /// keyboard.array_set_full(&[
    ///     (Key::Q, (255, 255, 255)),
    ///     (Key::W, (255, 255, 255)),
    ///     (Key::E, (255, 255, 255)),
    ///     (Key::R, (255, 255, 255)),
    ///     (Key::T, (255, 255, 255)),
    ///     (Key::Y, (255, 255, 255)),
    /// ]);
    /// // ..and apply the change.
    /// keyboard.array_update();
    /// ```
    pub fn array_set_full<K: IntoMatrixRowColumn>(&mut self, array: &[(K, (u8, u8, u8))]) -> bool {
        let mut flattened: [u8; COMPONENTS * COLUMNS * ROWS] = [0; COMPONENTS * COLUMNS * ROWS];
        for (key, (red, green, blue)) in array {
            let (row, column) = key.get_matrix_row_and_column();
            let index: usize =
                (row as usize) * (COLUMNS * COMPONENTS) + (column as usize) * COMPONENTS;
            flattened[index] = *red;
            flattened[index + 1] = *green;
            flattened[index + 2] = *blue;
        }
        unsafe { wooting_rgb_sys::wooting_rgb_array_set_full(flattened.as_ptr()) }
    }

    /// Restore all colors to those that were originally on the keyboard. Must be called when
    /// application is closed (this will be invoked when this struct is dropped).
    ///
    /// ```rust,no_run
    /// use wooting_rgb::{Key, RgbKeyboard};
    ///
    /// let mut keyboard = RgbKeyboard::default();
    /// // Set ABC to white..
    /// keyboard.direct_set_key(Key::A, 255, 255, 255);
    /// keyboard.direct_set_key(Key::B, 255, 255, 255);
    /// keyboard.direct_set_key(Key::C, 255, 255, 255);
    /// // ..and then reset the entire keyboard back to how it was previously.
    /// keyboard.reset_all();
    /// ```
    pub fn reset_all(&mut self) -> bool {
        unsafe { wooting_rgb_sys::wooting_rgb_reset() }
    }
}

impl Drop for RgbKeyboard {
    fn drop(&mut self) {
        // By restricting all rgb functions to get performed on a struct then we can ensure
        // that there is something to be dropped and therefore force a reset.
        let _ = self.reset_all();
        // Also, make sure that the auto update has been reset.
        self.array_auto_update(false);
    }
}
