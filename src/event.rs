
use {
    AddAll,
    AddAny,
    AddInterval,
    AddPress,
    AllEvent,
    AnyEvent,
    IntervalEvent,
    KeyType,
    PressEvent,
    Triggered,
    Value,
};

/// An immutable event context. All Request starting here.
pub struct Event;

impl Event {
    /// Returns a new event context.
    pub fn new() -> Event {
        Event
    }
}

impl Clone for Event {
    fn clone(&self) -> Event {
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

impl<'a> AddInterval<IntervalEvent<'a>> for Event {
    #[inline(always)]
    fn interval(&self, seconds: f64) -> IntervalEvent<'a> {
        IntervalEvent {
            interval: Value(seconds),
        }
    }
}

impl<'a> AddAll<'a, AllEvent<'a>> for Event {
    #[inline(always)]
    fn all(&'a self, events: &'a [&'a Triggered]) -> AllEvent<'a> {
        AllEvent {
            events: Value(events),
        }
    }
}

impl<'a> AddAny<'a, AnyEvent<'a>> for Event {
    #[inline(always)]
    fn any(&'a self, events: &'a [&'a Triggered]) -> AnyEvent<'a> {
        AnyEvent {
            events: Value(events),
        }
    }
}

