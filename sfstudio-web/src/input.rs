use cala::warn;

/// State of a key.
#[repr(u8)]
#[derive(PartialEq, Copy, Clone)]
pub enum KeyState {
    /// Not pressed
    Idle = 0b_000,
    /// Just got pressed
    Just = 0b_011,
    /// Held down.
    Held = 0b_010,
    /// Just got released
    Lift = 0b_001,
    /// A key was typed (like Just, but repeats when held down).
    Type = 0b_111,
}

impl KeyState {
    /// Returns true if the key is held down.
    #[inline(always)]
    pub fn held(self) -> bool {
        (self as u8) >> 1 != 0
    }

    /// Returns true if the key was just pressed.
    #[inline(always)]
    pub fn press(self) -> bool {
        self == KeyState::Just
    }
}

/// Borrowed from `window` crate.
///
/// Keys on the keyboard that may be mapped as controls.  Naming goes by
/// american QWERTY keyboard.
///
/// Escape & F11 are treated specially.  Escape is treated as the back key on a
/// phone, and F11 always toggles fullscreen.
#[allow(missing_docs)]
#[repr(usize)]
pub enum Key {
    Backtick = 0usize,
    Num1 = 1,
    Num2 = 2,
    Num3 = 3,
    Num4 = 4,
    Num5 = 5,
    Num6 = 6,
    Num7 = 7,
    Num8 = 8,
    Num9 = 9,
    Num0 = 10,
    Minus = 11,
    Equals = 12,
    Backspace = 13,
    Tab = 14,
    Q = 15,
    W = 16,
    E = 17,
    R = 18,
    T = 19,
    Y = 20,
    U = 21,
    I = 22,
    O = 23,
    P = 24,
    SquareBracketOpen = 25,
    SquareBracketClose = 26,
    Backslash = 27,
    CapsLock = 28,
    A = 29,
    S = 30,
    D = 31,
    F = 32,
    G = 33,
    H = 34,
    J = 35,
    K = 36,
    L = 37,
    Semicolon = 38,
    Quote = 39,
    Enter = 40,
    LeftShift = 41,
    Z = 42,
    X = 43,
    C = 44,
    V = 45,
    B = 46,
    N = 47,
    M = 48,
    Comma = 49,
    Period = 50,
    Slash = 51,
    RightShift = 52,
    LeftCtrl = 53,
    System = 54, // windows key, etc.
    LeftAlt = 55,
    Space = 56,
    RightAlt = 57,
    Menu = 58,
    RightCtrl = 59,
    Up = 60,
    Down = 61,
    Left = 62,
    Right = 63,
    Insert = 64,
    Delete = 65,
    PageUp = 66,
    PageDown = 67,
    Home = 68,
    End = 69,
    Numpad0 = 70,
    Numpad1 = 71,
    Numpad2 = 72,
    Numpad3 = 73,
    Numpad4 = 74,
    Numpad5 = 75,
    Numpad6 = 76,
    Numpad7 = 77,
    Numpad8 = 78,
    Numpad9 = 79,
    NumpadDot = 80,
    NumpadLock = 81,
    NumpadDivide = 82,
    NumpadMultiply = 83,
    NumpadSubtract = 84,
    NumpadAdd = 85,
    NumpadEnter = 86,
    PrintScreen = 87,
    PausePlay = 88,
    Stop = 89,
    Rewind = 90,
    FastForward = 91,
    BrightnessDown = 92,
    BrightnessUp = 93,
    TrackpadOn = 94,
    TrackpadOff = 95,
    AirplaneMode = 96,
    Mute = 97,
    VolumeDown = 98,
    VolumeUp = 99,
    Back = 100,
    F1 = 101,
    F2 = 102,
    F3 = 103,
    F4 = 104,
    F5 = 105,
    F6 = 106,
    F7 = 107,
    F8 = 108,
    F9 = 109,
    F10 = 110,
    F11 = 111, // Toggle Fullscreen
    F12 = 112, // Controls
    MicrophoneToggle = 113,
    CameraToggle = 114,
    Break = 115,
    ScrollLock = 116,
    ScrollUp = 117,
    ScrollDown = 118,
    ScrollLeft = 119,
    ScrollRight = 120,
    LeftClick = 121,
    MiddleClick = 122,
    RightClick = 123,
    SideClick = 124,
    ForwardClick = 125,
    BackwardClick = 126,
    ExtraClick = 127,
    Max = 128,
}

/// Input state.
pub struct InputState {
    /// Left Arrow Key
    pub keys: [KeyState; Key::Max as usize],
    /// If any Just or Lift has happened.
    pub has_input: bool,
    /// A unicode character.
    pub text: char,
}

impl InputState {
    /// Create a new keys / buttons state.
    pub fn new() -> InputState {
        InputState {
            keys: [KeyState::Idle; Key::Max as usize],
            has_input: false,
            text: '\0',
        }
    }

    /// Turn Just & Lift into Held & Idle
    pub fn reset(&mut self) {
        for key in self.keys.iter_mut() {
            *key = match *key {
                KeyState::Just => KeyState::Held,
                KeyState::Lift => KeyState::Idle,
                a => a,
            };
        }
        self.has_input = false;
    }

    fn code_into_key(&mut self, key: String, code: String, state: KeyState, ic: bool) {
        if key.len() == 1 && !ic {
            self.text = key.chars().nth(0).unwrap();
        }

        // A unicode character has been typed.
        self.keys[match code.as_str() {
            "Numpad0" => Key::Numpad0,
            "Numpad1" => Key::Numpad1,
            "Numpad2" => Key::Numpad2,
            "Numpad3" => Key::Numpad3,
            "Numpad4" => Key::Numpad4,
            "Numpad5" => Key::Numpad5,
            "Numpad6" => Key::Numpad6,
            "Numpad7" => Key::Numpad7,
            "Numpad8" => Key::Numpad8,
            "Numpad9" => Key::Numpad9,
            "AltLeft" => Key::LeftAlt,
            "AltRight" => Key::RightAlt,
            "ControlLeft" => Key::LeftCtrl,
            "ControlRight" => Key::RightCtrl,
            "ShiftLeft" => Key::LeftShift,
            "ShiftRight" => Key::RightShift,
            "Space" => Key::Space,
            "Tab" => Key::Tab,
            "Backspace" => Key::Backspace,
            "Escape" => Key::Back,
            "Enter" => Key::Enter,
            "NumpadEnter" => Key::NumpadEnter,
            "ArrowUp" => Key::Up,
            "ArrowDown" => Key::Down,
            "ArrowLeft" => Key::Left,
            "ArrowRight" => Key::Right,
            "KeyA" => Key::A,
            "KeyB" => Key::B,
            "KeyC" => Key::C,
            "KeyD" => Key::D,
            "KeyE" => Key::E,
            "KeyF" => Key::F,
            "KeyG" => Key::G,
            "KeyH" => Key::H,
            "KeyI" => Key::I,
            "KeyJ" => Key::J,
            "KeyK" => Key::K,
            "KeyL" => Key::L,
            "KeyM" => Key::M,
            "KeyN" => Key::N,
            "KeyO" => Key::O,
            "KeyP" => Key::P,
            "KeyQ" => Key::Q,
            "KeyR" => Key::R,
            "KeyS" => Key::S,
            "KeyT" => Key::T,
            "KeyU" => Key::U,
            "KeyV" => Key::V,
            "KeyW" => Key::W,
            "KeyX" => Key::X,
            "KeyY" => Key::Y,
            "KeyZ" => Key::Z,
            "Digit0" => Key::Num0,
            "Digit1" => Key::Num1,
            "Digit2" => Key::Num2,
            "Digit3" => Key::Num3,
            "Digit4" => Key::Num4,
            "Digit5" => Key::Num5,
            "Digit6" => Key::Num6,
            "Digit7" => Key::Num7,
            "Digit8" => Key::Num8,
            "Digit9" => Key::Num9,
            a => {
                warn!("Unknown {}", a);
                return;
            },
        } as usize] = state;
    }

    /// Update `InputState` from IO
    pub fn update(&mut self, key: String, code: String, ic: bool, held: bool) {
        let state = if held { KeyState::Just } else { KeyState::Lift };
        self.has_input = true;
        self.code_into_key(key, code, state, ic);
    }

    /// Returns true if key was just pressed.
    pub fn press(&self, key: Key) -> bool {
        self.keys[key as usize].press()
    }

    /// Returns true if key is held down.
    pub fn held(&self, key: Key) -> bool {
        self.keys[key as usize].held()
    }
}
