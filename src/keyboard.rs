//! Back-end agnostic keyboard keys.

use std::hash;
use std::hash::sip::SipState;
use std::num::FromPrimitive;
use std::num::ToPrimitive;
use std::default::Default;

use Input;
use Button;

// Defining every combination to allow assignment in static expressions.
bitflags!(
    #[deriving(Show)]
    #[allow(missing_docs)]
    #[deriving(Decodable, Encodable)]
    flags ModifierKey: u8 {
        const NO_MODIFIER           = 0b00000000,
        const CTRL                  = 0b00000001,
        const SHIFT                 = 0b00000010,
        const ALT                   = 0b00000100,
        const GUI                   = 0b00001000,
        const CTRL_SHIFT            = CTRL.bits
                                    | SHIFT.bits,
        const CTRL_ALT              = CTRL.bits
                                    | ALT.bits,
        const CTRL_GUI              = CTRL.bits
                                    | GUI.bits,
        const CTRL_SHIFT_ALT        = CTRL.bits
                                    | SHIFT.bits
                                    | ALT.bits,
        const CTRL_SHIFT_GUI        = CTRL.bits
                                    | SHIFT.bits
                                    | GUI.bits,
        const CTRL_SHIFT_ALT_GUI    = CTRL.bits
                                    | SHIFT.bits
                                    | ALT.bits
                                    | GUI.bits,
        const SHIFT_ALT             = SHIFT.bits
                                    | ALT.bits,
        const SHIFT_GUI             = SHIFT.bits
                                    | GUI.bits,
        const SHIFT_ALT_GUI         = SHIFT.bits
                                    | ALT.bits
                                    | GUI.bits,
        const ALT_GUI               = ALT.bits
                                    | GUI.bits
    }
)

impl ModifierKey {
    /// Change modifier key state depending on input.
    ///
    /// If the left or side button is released, it counts as a release.
    pub fn handle_input(&mut self, input: &Input) {
        match *input {
            Input::Press(Button::Keyboard(Key::LCtrl))
          | Input::Press(Button::Keyboard(Key::RCtrl)) => self.insert(CTRL),
            Input::Release(Button::Keyboard(Key::LCtrl))
          | Input::Release(Button::Keyboard(Key::RCtrl)) => self.remove(CTRL),
            Input::Press(Button::Keyboard(Key::LShift))
          | Input::Press(Button::Keyboard(Key::RShift)) => self.insert(SHIFT),
            Input::Release(Button::Keyboard(Key::LShift))
          | Input::Release(Button::Keyboard(Key::RShift)) => self.remove(SHIFT),
            Input::Press(Button::Keyboard(Key::LAlt))
          | Input::Press(Button::Keyboard(Key::RAlt)) => self.insert(ALT),
            Input::Release(Button::Keyboard(Key::LAlt))
          | Input::Release(Button::Keyboard(Key::RAlt)) => self.remove(ALT),
            Input::Press(Button::Keyboard(Key::LGui))
          | Input::Press(Button::Keyboard(Key::RGui)) => self.insert(GUI),
            Input::Release(Button::Keyboard(Key::LGui))
          | Input::Release(Button::Keyboard(Key::RGui)) => self.remove(GUI),
            Input::Focus(false) => *self = NO_MODIFIER,
            _ => {}
        }
    }
}

impl Default for ModifierKey {
    fn default() -> ModifierKey { NO_MODIFIER }
}

/// Represent a keyboard key.
#[allow(missing_docs)]
#[deriving(Copy, Clone, Decodable, Encodable, Show)]
pub enum Key {
    Unknown                 = 0,
    Backspace               = 8,
    Tab                     = 9,
    Return                  = 13,
    Escape                  = 27,
    Space                   = 32,
    Exclaim                 = 33,
    Quotedbl                = 34,
    Hash                    = 35,
    Dollar                  = 36,
    Percent                 = 37,
    Ampersand               = 38,
    Quote                   = 39,
    LeftParen               = 40,
    RightParen              = 41,
    Asterisk                = 42,
    Plus                    = 43,
    Comma                   = 44,
    Minus                   = 45,
    Period                  = 46,
    Slash                   = 47,
    D0                      = 48,
    D1                      = 49,
    D2                      = 50,
    D3                      = 51,
    D4                      = 52,
    D5                      = 53,
    D6                      = 54,
    D7                      = 55,
    D8                      = 56,
    D9                      = 57,
    Colon                   = 58,
    Semicolon               = 59,
    Less                    = 60,
    Equals                  = 61,
    Greater                 = 62,
    Question                = 63,
    At                      = 64,
    LeftBracket             = 91,
    Backslash               = 92,
    RightBracket            = 93,
    Caret                   = 94,
    Underscore              = 95,
    Backquote               = 96,
    A                       = 97,
    B                       = 98,
    C                       = 99,
    D                       = 100,
    E                       = 101,
    F                       = 102,
    G                       = 103,
    H                       = 104,
    I                       = 105,
    J                       = 106,
    K                       = 107,
    L                       = 108,
    M                       = 109,
    N                       = 110,
    O                       = 111,
    P                       = 112,
    Q                       = 113,
    R                       = 114,
    S                       = 115,
    T                       = 116,
    U                       = 117,
    V                       = 118,
    W                       = 119,
    X                       = 120,
    Y                       = 121,
    Z                       = 122,
    Delete                  = 127,
    CapsLock                = 1073741881,
    F1                      = 1073741882,
    F2                      = 1073741883,
    F3                      = 1073741884,
    F4                      = 1073741885,
    F5                      = 1073741886,
    F6                      = 1073741887,
    F7                      = 1073741888,
    F8                      = 1073741889,
    F9                      = 1073741890,
    F10                     = 1073741891,
    F11                     = 1073741892,
    F12                     = 1073741893,
    PrintScreen             = 1073741894,
    ScrollLock              = 1073741895,
    Pause                   = 1073741896,
    Insert                  = 1073741897,
    Home                    = 1073741898,
    PageUp                  = 1073741899,
    End                     = 1073741901,
    PageDown                = 1073741902,
    Right                   = 1073741903,
    Left                    = 1073741904,
    Down                    = 1073741905,
    Up                      = 1073741906,
    NumLockClear            = 1073741907,
    NumPadDivide            = 1073741908,
    NumPadMultiply          = 1073741909,
    NumPadMinus             = 1073741910,
    NumPadPlus              = 1073741911,
    NumPadEnter             = 1073741912,
    NumPad1                 = 1073741913,
    NumPad2                 = 1073741914,
    NumPad3                 = 1073741915,
    NumPad4                 = 1073741916,
    NumPad5                 = 1073741917,
    NumPad6                 = 1073741918,
    NumPad7                 = 1073741919,
    NumPad8                 = 1073741920,
    NumPad9                 = 1073741921,
    NumPad0                 = 1073741922,
    NumPadPeriod            = 1073741923,
    Application             = 1073741925,
    Power                   = 1073741926,
    NumPadEquals            = 1073741927,
    F13                     = 1073741928,
    F14                     = 1073741929,
    F15                     = 1073741930,
    F16                     = 1073741931,
    F17                     = 1073741932,
    F18                     = 1073741933,
    F19                     = 1073741934,
    F20                     = 1073741935,
    F21                     = 1073741936,
    F22                     = 1073741937,
    F23                     = 1073741938,
    F24                     = 1073741939,
    Execute                 = 1073741940,
    Help                    = 1073741941,
    Menu                    = 1073741942,
    Select                  = 1073741943,
    Stop                    = 1073741944,
    Again                   = 1073741945,
    Undo                    = 1073741946,
    Cut                     = 1073741947,
    Copy                    = 1073741948,
    Paste                   = 1073741949,
    Find                    = 1073741950,
    Mute                    = 1073741951,
    VolumeUp                = 1073741952,
    VolumeDown              = 1073741953,
    NumPadComma             = 1073741957,
    NumPadEqualsAS400       = 1073741958,
    AltErase                = 1073741977,
    Sysreq                  = 1073741978,
    Cancel                  = 1073741979,
    Clear                   = 1073741980,
    Prior                   = 1073741981,
    Return2                 = 1073741982,
    Separator               = 1073741983,
    Out                     = 1073741984,
    Oper                    = 1073741985,
    ClearAgain              = 1073741986,
    CrSel                   = 1073741987,
    ExSel                   = 1073741988,
    NumPad00                = 1073742000,
    NumPad000               = 1073742001,
    ThousandsSeparator      = 1073742002,
    DecimalSeparator        = 1073742003,
    CurrencyUnit            = 1073742004,
    CurrencySubUnit         = 1073742005,
    NumPadLeftParen         = 1073742006,
    NumPadRightParen        = 1073742007,
    NumPadLeftBrace         = 1073742008,
    NumPadRightBrace        = 1073742009,
    NumPadTab               = 1073742010,
    NumPadBackspace         = 1073742011,
    NumPadA                 = 1073742012,
    NumPadB                 = 1073742013,
    NumPadC                 = 1073742014,
    NumPadD                 = 1073742015,
    NumPadE                 = 1073742016,
    NumPadF                 = 1073742017,
    NumPadXor               = 1073742018,
    NumPadPower             = 1073742019,
    NumPadPercent           = 1073742020,
    NumPadLess              = 1073742021,
    NumPadGreater           = 1073742022,
    NumPadAmpersand         = 1073742023,
    NumPadDblAmpersand      = 1073742024,
    NumPadVerticalBar       = 1073742025,
    NumPadDblVerticalBar    = 1073742026,
    NumPadColon             = 1073742027,
    NumPadHash              = 1073742028,
    NumPadSpace             = 1073742029,
    NumPadAt                = 1073742030,
    NumPadExclam            = 1073742031,
    NumPadMemStore          = 1073742032,
    NumPadMemRecall         = 1073742033,
    NumPadMemClear          = 1073742034,
    NumPadMemAdd            = 1073742035,
    NumPadMemSubtract       = 1073742036,
    NumPadMemMultiply       = 1073742037,
    NumPadMemDivide         = 1073742038,
    NumPadPlusMinus         = 1073742039,
    NumPadClear             = 1073742040,
    NumPadClearEntry        = 1073742041,
    NumPadBinary            = 1073742042,
    NumPadOctal             = 1073742043,
    NumPadDecimal           = 1073742044,
    NumPadHexadecimal       = 1073742045,
    LCtrl                   = 1073742048,
    LShift                  = 1073742049,
    LAlt                    = 1073742050,
    LGui                    = 1073742051,
    RCtrl                   = 1073742052,
    RShift                  = 1073742053,
    RAlt                    = 1073742054,
    RGui                    = 1073742055,
    Mode                    = 1073742081,
    AudioNext               = 1073742082,
    AudioPrev               = 1073742083,
    AudioStop               = 1073742084,
    AudioPlay               = 1073742085,
    AudioMute               = 1073742086,
    MediaSelect             = 1073742087,
    Www                     = 1073742088,
    Mail                    = 1073742089,
    Calculator              = 1073742090,
    Computer                = 1073742091,
    AcSearch                = 1073742092,
    AcHome                  = 1073742093,
    AcBack                  = 1073742094,
    AcForward               = 1073742095,
    AcStop                  = 1073742096,
    AcRefresh               = 1073742097,
    AcBookmarks             = 1073742098,
    BrightnessDown          = 1073742099,
    BrightnessUp            = 1073742100,
    DisplaySwitch           = 1073742101,
    KbdIllumToggle          = 1073742102,
    KbdIllumDown            = 1073742103,
    KbdIllumUp              = 1073742104,
    Eject                   = 1073742105,
    Sleep                   = 1073742106,
}


impl PartialEq for Key {
    fn eq(&self, other: &Key) -> bool {
        return (*self as i32) == (*other as i32);
    }
}

impl Eq for Key {}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        let (s_id, o_id)  = (*self as i32, *other as i32);
        s_id.partial_cmp(&o_id)
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        let (s_id, o_id)  = (*self as i32, *other as i32);
        s_id.cmp(&o_id)
    }
}

impl Key {
    /// Returns an id of the key
    #[inline(always)]
    pub fn code(&self) -> i32 {
        *self as i32
    }
}

impl hash::Hash for Key {
    #[inline(always)]
    fn hash(&self, state: &mut SipState) {
        self.code().hash(state);
    }
}

impl ToPrimitive for Key {
    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        Some(self.code() as i64)
    }

    #[inline(always)]
    fn to_u64(&self) -> Option<u64> {
        Some(self.code() as u64)
    }

    #[inline(always)]
    fn to_int(&self) -> Option<int> {
        Some(self.code() as int)
    }
}

impl FromPrimitive for Key {
    fn from_u64(n: u64) -> Option<Key> {
        match n {
            0 => Some(Key::Unknown),
            8 => Some(Key::Backspace),
            9 => Some(Key::Tab),
            13 => Some(Key::Return),
            27 => Some(Key::Escape),
            32 => Some(Key::Space),
            33 => Some(Key::Exclaim),
            34 => Some(Key::Quotedbl),
            35 => Some(Key::Hash),
            36 => Some(Key::Dollar),
            37 => Some(Key::Percent),
            38 => Some(Key::Ampersand),
            39 => Some(Key::Quote),
            40 => Some(Key::LeftParen),
            41 => Some(Key::RightParen),
            42 => Some(Key::Asterisk),
            43 => Some(Key::Plus),
            44 => Some(Key::Comma),
            45 => Some(Key::Minus),
            46 => Some(Key::Period),
            47 => Some(Key::Slash),
            48 => Some(Key::D0),
            49 => Some(Key::D1),
            50 => Some(Key::D2),
            51 => Some(Key::D3),
            52 => Some(Key::D4),
            53 => Some(Key::D5),
            54 => Some(Key::D6),
            55 => Some(Key::D7),
            56 => Some(Key::D8),
            57 => Some(Key::D9),
            58 => Some(Key::Colon),
            59 => Some(Key::Semicolon),
            60 => Some(Key::Less),
            61 => Some(Key::Equals),
            62 => Some(Key::Greater),
            63 => Some(Key::Question),
            64 => Some(Key::At),
            91 => Some(Key::LeftBracket),
            92 => Some(Key::Backslash),
            93 => Some(Key::RightBracket),
            94 => Some(Key::Caret),
            95 => Some(Key::Underscore),
            96 => Some(Key::Backquote),
            97 => Some(Key::A),
            98 => Some(Key::B),
            99 => Some(Key::C),
            100 => Some(Key::D),
            101 => Some(Key::E),
            102 => Some(Key::F),
            103 => Some(Key::G),
            104 => Some(Key::H),
            105 => Some(Key::I),
            106 => Some(Key::J),
            107 => Some(Key::K),
            108 => Some(Key::L),
            109 => Some(Key::M),
            110 => Some(Key::N),
            111 => Some(Key::O),
            112 => Some(Key::P),
            113 => Some(Key::Q),
            114 => Some(Key::R),
            115 => Some(Key::S),
            116 => Some(Key::T),
            117 => Some(Key::U),
            118 => Some(Key::V),
            119 => Some(Key::W),
            120 => Some(Key::X),
            121 => Some(Key::Y),
            122 => Some(Key::Z),
            127 => Some(Key::Delete),
            1073741881 => Some(Key::CapsLock),
            1073741882 => Some(Key::F1),
            1073741883 => Some(Key::F2),
            1073741884 => Some(Key::F3),
            1073741885 => Some(Key::F4),
            1073741886 => Some(Key::F5),
            1073741887 => Some(Key::F6),
            1073741888 => Some(Key::F7),
            1073741889 => Some(Key::F8),
            1073741890 => Some(Key::F9),
            1073741891 => Some(Key::F10),
            1073741892 => Some(Key::F11),
            1073741893 => Some(Key::F12),
            1073741894 => Some(Key::PrintScreen),
            1073741895 => Some(Key::ScrollLock),
            1073741896 => Some(Key::Pause),
            1073741897 => Some(Key::Insert),
            1073741898 => Some(Key::Home),
            1073741899 => Some(Key::PageUp),
            1073741901 => Some(Key::End),
            1073741902 => Some(Key::PageDown),
            1073741903 => Some(Key::Right),
            1073741904 => Some(Key::Left),
            1073741905 => Some(Key::Down),
            1073741906 => Some(Key::Up),
            1073741907 => Some(Key::NumLockClear),
            1073741908 => Some(Key::NumPadDivide),
            1073741909 => Some(Key::NumPadMultiply),
            1073741910 => Some(Key::NumPadMinus),
            1073741911 => Some(Key::NumPadPlus),
            1073741912 => Some(Key::NumPadEnter),
            1073741913 => Some(Key::NumPad1),
            1073741914 => Some(Key::NumPad2),
            1073741915 => Some(Key::NumPad3),
            1073741916 => Some(Key::NumPad4),
            1073741917 => Some(Key::NumPad5),
            1073741918 => Some(Key::NumPad6),
            1073741919 => Some(Key::NumPad7),
            1073741920 => Some(Key::NumPad8),
            1073741921 => Some(Key::NumPad9),
            1073741922 => Some(Key::NumPad0),
            1073741923 => Some(Key::NumPadPeriod),
            1073741925 => Some(Key::Application),
            1073741926 => Some(Key::Power),
            1073741927 => Some(Key::NumPadEquals),
            1073741928 => Some(Key::F13),
            1073741929 => Some(Key::F14),
            1073741930 => Some(Key::F15),
            1073741931 => Some(Key::F16),
            1073741932 => Some(Key::F17),
            1073741933 => Some(Key::F18),
            1073741934 => Some(Key::F19),
            1073741935 => Some(Key::F20),
            1073741936 => Some(Key::F21),
            1073741937 => Some(Key::F22),
            1073741938 => Some(Key::F23),
            1073741939 => Some(Key::F24),
            1073741940 => Some(Key::Execute),
            1073741941 => Some(Key::Help),
            1073741942 => Some(Key::Menu),
            1073741943 => Some(Key::Select),
            1073741944 => Some(Key::Stop),
            1073741945 => Some(Key::Again),
            1073741946 => Some(Key::Undo),
            1073741947 => Some(Key::Cut),
            1073741948 => Some(Key::Copy),
            1073741949 => Some(Key::Paste),
            1073741950 => Some(Key::Find),
            1073741951 => Some(Key::Mute),
            1073741952 => Some(Key::VolumeUp),
            1073741953 => Some(Key::VolumeDown),
            1073741957 => Some(Key::NumPadComma),
            1073741958 => Some(Key::NumPadEqualsAS400),
            1073741977 => Some(Key::AltErase),
            1073741978 => Some(Key::Sysreq),
            1073741979 => Some(Key::Cancel),
            1073741980 => Some(Key::Clear),
            1073741981 => Some(Key::Prior),
            1073741982 => Some(Key::Return2),
            1073741983 => Some(Key::Separator),
            1073741984 => Some(Key::Out),
            1073741985 => Some(Key::Oper),
            1073741986 => Some(Key::ClearAgain),
            1073741987 => Some(Key::CrSel),
            1073741988 => Some(Key::ExSel),
            1073742000 => Some(Key::NumPad00),
            1073742001 => Some(Key::NumPad000),
            1073742002 => Some(Key::ThousandsSeparator),
            1073742003 => Some(Key::DecimalSeparator),
            1073742004 => Some(Key::CurrencyUnit),
            1073742005 => Some(Key::CurrencySubUnit),
            1073742006 => Some(Key::NumPadLeftParen),
            1073742007 => Some(Key::NumPadRightParen),
            1073742008 => Some(Key::NumPadLeftBrace),
            1073742009 => Some(Key::NumPadRightBrace),
            1073742010 => Some(Key::NumPadTab),
            1073742011 => Some(Key::NumPadBackspace),
            1073742012 => Some(Key::NumPadA),
            1073742013 => Some(Key::NumPadB),
            1073742014 => Some(Key::NumPadC),
            1073742015 => Some(Key::NumPadD),
            1073742016 => Some(Key::NumPadE),
            1073742017 => Some(Key::NumPadF),
            1073742018 => Some(Key::NumPadXor),
            1073742019 => Some(Key::NumPadPower),
            1073742020 => Some(Key::NumPadPercent),
            1073742021 => Some(Key::NumPadLess),
            1073742022 => Some(Key::NumPadGreater),
            1073742023 => Some(Key::NumPadAmpersand),
            1073742024 => Some(Key::NumPadDblAmpersand),
            1073742025 => Some(Key::NumPadVerticalBar),
            1073742026 => Some(Key::NumPadDblVerticalBar),
            1073742027 => Some(Key::NumPadColon),
            1073742028 => Some(Key::NumPadHash),
            1073742029 => Some(Key::NumPadSpace),
            1073742030 => Some(Key::NumPadAt),
            1073742031 => Some(Key::NumPadExclam),
            1073742032 => Some(Key::NumPadMemStore),
            1073742033 => Some(Key::NumPadMemRecall),
            1073742034 => Some(Key::NumPadMemClear),
            1073742035 => Some(Key::NumPadMemAdd),
            1073742036 => Some(Key::NumPadMemSubtract),
            1073742037 => Some(Key::NumPadMemMultiply),
            1073742038 => Some(Key::NumPadMemDivide),
            1073742039 => Some(Key::NumPadPlusMinus),
            1073742040 => Some(Key::NumPadClear),
            1073742041 => Some(Key::NumPadClearEntry),
            1073742042 => Some(Key::NumPadBinary),
            1073742043 => Some(Key::NumPadOctal),
            1073742044 => Some(Key::NumPadDecimal),
            1073742045 => Some(Key::NumPadHexadecimal),
            1073742048 => Some(Key::LCtrl),
            1073742049 => Some(Key::LShift),
            1073742050 => Some(Key::LAlt),
            1073742051 => Some(Key::LGui),
            1073742052 => Some(Key::RCtrl),
            1073742053 => Some(Key::RShift),
            1073742054 => Some(Key::RAlt),
            1073742055 => Some(Key::RGui),
            1073742081 => Some(Key::Mode),
            1073742082 => Some(Key::AudioNext),
            1073742083 => Some(Key::AudioPrev),
            1073742084 => Some(Key::AudioStop),
            1073742085 => Some(Key::AudioPlay),
            1073742086 => Some(Key::AudioMute),
            1073742087 => Some(Key::MediaSelect),
            1073742088 => Some(Key::Www),
            1073742089 => Some(Key::Mail),
            1073742090 => Some(Key::Calculator),
            1073742091 => Some(Key::Computer),
            1073742092 => Some(Key::AcSearch),
            1073742093 => Some(Key::AcHome),
            1073742094 => Some(Key::AcBack),
            1073742095 => Some(Key::AcForward),
            1073742096 => Some(Key::AcStop),
            1073742097 => Some(Key::AcRefresh),
            1073742098 => Some(Key::AcBookmarks),
            1073742099 => Some(Key::BrightnessDown),
            1073742100 => Some(Key::BrightnessUp),
            1073742101 => Some(Key::DisplaySwitch),
            1073742102 => Some(Key::KbdIllumToggle),
            1073742103 => Some(Key::KbdIllumDown),
            1073742104 => Some(Key::KbdIllumUp),
            1073742105 => Some(Key::Eject),
            1073742106 => Some(Key::Sleep),

            _ => Some(Key::Unknown)
        }
    }

    #[inline(always)]
    fn from_i64(n: i64) -> Option<Key> {
        FromPrimitive::from_u64(n as u64)
    }

    #[inline(always)]
    fn from_int(n: int) -> Option<Key> {
        FromPrimitive::from_u64(n as u64)
    }
}
