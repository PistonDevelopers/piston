
use {
    AddPress,
    PressEvent,
    Value,
    KeyType,
};

/// An immutable event context. All Request starting here.
pub struct Event;

impl Event {
    /// Returns a new event context.
    pub fn new() -> Event {
        Event
    }
}

impl<'a> AddPress<'a, PressEvent<'a>> for Event {
    #[inline(always)]
    fn press(&'a self, key: &'a KeyType) -> PressEvent<'a> {
        PressEvent {
            key: Value(key),
        }
    }
}

