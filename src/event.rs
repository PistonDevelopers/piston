
use {
    AddKeyboard,
    KeyboardEvent,
};

/// An immutable event context. All Request starting here.
pub struct Event<'a>;

impl<'a> Event<'a> {
    /// Returns a new event context.
    pub fn new() -> Event {
        Event
    }
}

impl<'a> AddKeyboard<'a, KeyboardEvent<'a>> for Event<'a> {
    #[inline(always)]
    fn keyboard(&self) -> KeyboardEvent<'a> {
        KeyboardEvent
    }
}

