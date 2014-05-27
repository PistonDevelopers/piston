
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
pub struct Event<'a>;

impl<'a> Event<'a> {
    /// Returns a new event context.
    pub fn new() -> Event<'a> {
        Event
    }
}

impl<'a> Clone for Event<'a> {
    fn clone(&self) -> Event<'static> {
        Event
    }
}

impl<'a, 'b> AddPress<'a, PressEvent<'a, 'b>> for Event<'a> {
    #[inline(always)]
    fn press(&'a self, key: &'a KeyType) -> PressEvent<'a, 'b> {
        PressEvent {
            key: Value(key),
        }
    }
}

impl<'a> AddInterval<IntervalEvent<'a>> for Event<'a> {
    #[inline(always)]
    fn interval(&self, seconds: f64) -> IntervalEvent<'a> {
        IntervalEvent {
            interval: Value(seconds),
        }
    }
}

impl<'a, 'b> AddAll<'a, AllEvent<'a, 'b>> for Event<'a> {
    #[inline(always)]
    fn all(&'a self, events: &'b [&'b Triggered]) -> AllEvent<'a, 'b> {
        AllEvent {
            events: Value(events),
        }
    }
}

impl<'a, 'b> AddAny<'a, AnyEvent<'a, 'b>> for Event<'a> {
    #[inline(always)]
    fn any(&'a self, events: &'b [&'b Triggered]) -> AnyEvent<'a, 'b> {
        AnyEvent {
            events: Value(events),
        }
    }
}

