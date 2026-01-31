
use enum_iterator::Sequence;


type EadkKeyboardState = u64;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, Sequence, Debug)]
#[repr(u8)]
pub enum Key {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Home = 6,
        OnOff = 8,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
    Minus = 46,
    Zero = 48,
    Dot = 49,
    Ee = 50,
    Ans = 51,
    Exe = 52,
}

unsafe extern "C" {
    fn eadk_keyboard_scan() -> EadkKeyboardState;
}

#[derive(Clone, Copy)]
pub struct KeyboardState(EadkKeyboardState);

impl Default for KeyboardState {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardState {
    pub fn scan() -> Self {
        Self::from_raw(unsafe { eadk_keyboard_scan() })
    }

    pub fn new() -> Self {
        KeyboardState(0)
    }

    pub fn from_raw(state: EadkKeyboardState) -> Self {
        Self(state)
    }

    pub fn key_down(&self, key: Key) -> bool {
        (self.0 >> (key as u8)) & 1 != 0
    }

    pub fn get_just_pressed(&self, old: KeyboardState) -> Self {
        KeyboardState(self.0 & (!old.0))
    }

    pub fn get_just_realeased(&self, old: KeyboardState) -> Self {
        KeyboardState((!self.0) & old.0)
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Event {
        Left = 0,
        Up = 1,
        Down = 2,
        Right = 3,
        Ok = 4,
        Back = 5,
        Shift = 12,
        Alpha = 13,
        Xnt = 14,
        Var = 15,
        Toolbox = 16,
        Backspace = 17,
        Exp = 18,
        Ln = 19,
        Log = 20,
        Imaginary = 21,
        Comma = 22,
        Power = 23,
        Sine = 24,
        Cosine = 25,
        Tangent = 26,
        Pi = 27,
        Sqrt = 28,
        Square = 29,
        Seven = 30,
        Eight = 31,
        Nine = 32,
        LeftParenthesis = 33,
        RightParenthesis = 34,
        Four = 36,
        Five = 37,
        Six = 38,
        Multiplication = 39,
        Division = 40,
        One = 42,
        Two = 43,
        Three = 44,
        Plus = 45,
        Minus = 46,
        Zero = 48,
        Dot = 49,
        Ee = 50,
        Ans = 51,
        Exe = 52,
        ShiftLeft = 54,
        ShiftUp = 55,
        ShiftDown = 56,
        ShiftRight = 57,
        AlphaLock = 67,
        Cut = 68,
        Copy = 69,
        Paste = 70,
        Clear = 71,
        LeftBracket = 72,
        RightBracket = 73,
        LeftBrace = 74,
        RightBrace = 75,
        Underscore = 76,
        Sto = 77,
        Arcsine = 78,
        Arccosine = 79,
        Arctangent = 80,
        Equal = 81,
        Lower = 82,
        Greater = 83,
        Colon = 122,
        Semicolon = 123,
        DoubleQuotes = 124,
        Percent = 125,
        LowerA = 126,
        LowerB = 127,
        LowerC = 128,
        LowerD = 129,
        LowerE = 130,
        LowerF = 131,
        LowerG = 132,
        LowerH = 133,
        LowerI = 134,
        LowerJ = 135,
        LowerK = 136,
        LowerL = 137,
        LowerM = 138,
        LowerN = 139,
        LowerO = 140,
        LowerP = 141,
        LowerQ = 142,
        LowerR = 144,
        LowerS = 145,
        LowerT = 146,
        LowerU = 147,
        LowerV = 148,
        LowerW = 150,
        LowerX = 151,
        LowerY = 152,
        LowerZ = 153,
        Space = 154,
        Question = 156,
        Exclamation = 157,
        UpperA = 180,
        UpperB = 181,
        UpperC = 182,
        UpperD = 183,
        UpperE = 184,
        UpperF = 185,
        UpperG = 186,
        UpperH = 187,
        UpperI = 188,
        UpperJ = 189,
        UpperK = 190,
        UpperL = 191,
        UpperM = 192,
        UpperN = 193,
        UpperO = 194,
        UpperP = 195,
        UpperQ = 196,
        UpperR = 198,
        UpperS = 199,
        UpperT = 200,
        UpperU = 201,
        UpperV = 202,
        UpperW = 204,
        UpperX = 205,
        UpperY = 206,
        UpperZ = 207,
}

impl Event {
    pub fn is_digit(&self) -> bool {
        matches!(
            self,
            Event::Zero
                | Event::One
                | Event::Two
                | Event::Three
                | Event::Four
                | Event::Five
                | Event::Six
                | Event::Seven
                | Event::Eight
                | Event::Nine
        )
    }

    pub fn to_digit(&self) -> Option<u8> {
        match self {
            Event::Zero => Some(0),
            Event::One => Some(1),
            Event::Two => Some(2),
            Event::Three => Some(3),
            Event::Four => Some(4),
            Event::Five => Some(5),
            Event::Six => Some(6),
            Event::Seven => Some(7),
            Event::Eight => Some(8),
            Event::Nine => Some(9),
            _ => None,
        }
    }
}

unsafe extern "C" {
    fn eadk_event_get(timeout: &i32) -> Event;
}

pub fn event_get(timeout: i32) -> Event {
    unsafe { eadk_event_get(&timeout) }
}