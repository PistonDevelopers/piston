
use graphics::Value;
use piston::keyboard;
use {
    AddPress,
    AddPressing,
    KeyboardPressEvent,
    KeyboardPressingEvent,
};

/// A event context which used to observe keyboard event.
pub struct KeyboardEvent<'a>;

impl<'a> AddPress<'a, keyboard::Key, KeyboardPressEvent<'a>> for KeyboardEvent<'a> {
    #[inline(always)]
    fn press(&self, key: keyboard::Key) -> KeyboardPressEvent<'a> {
        KeyboardPressEvent {
            key: Value(key),
        }
    }
}

impl<'a> AddPressing<'a, keyboard::Key, KeyboardPressingEvent<'a>> for KeyboardEvent<'a> {
    #[inline(always)]
    fn pressing(&self, key: keyboard::Key) -> KeyboardPressingEvent<'a> {
        KeyboardPressingEvent {
            key: Value(key),
        }
    }
}

