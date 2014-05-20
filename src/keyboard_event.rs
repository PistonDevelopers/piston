
use graphics::Value;
use piston::keyboard;
use {
    AddPress,
    AddPressing,
    AddRelease,
    PressKeyboardEvent,
    PressingKeyboardEvent,
    ReleaseKeyboardEvent,
};

/// A event context which used to observe keyboard event.
pub struct KeyboardEvent<'a>;

impl<'a> AddPress<'a, keyboard::Key, PressKeyboardEvent<'a>> for KeyboardEvent<'a> {
    #[inline(always)]
    fn press(&self, key: keyboard::Key) -> PressKeyboardEvent<'a> {
        PressKeyboardEvent {
            key: Value(key),
        }
    }
}

impl<'a> AddPressing<'a, keyboard::Key, PressingKeyboardEvent<'a>> for KeyboardEvent<'a> {
    #[inline(always)]
    fn pressing(&self, key: keyboard::Key) -> PressingKeyboardEvent<'a> {
        PressingKeyboardEvent {
            key: Value(key),
        }
    }
}

impl<'a> AddRelease<'a, keyboard::Key, ReleaseKeyboardEvent<'a>> for KeyboardEvent<'a> {
    #[inline(always)]
    fn release(&self, key: keyboard::Key) -> ReleaseKeyboardEvent<'a> {
        ReleaseKeyboardEvent {
            key: Value(key),
        }
    }
}

