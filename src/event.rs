
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
    #[inline(always)]
    pub fn new() -> Event<'a> {
        Event
    }
/*
    #[inline(always)]
    pub fn after<'b>(before: &'a Triggered, after: &'a Triggered) -> AfterEvent<'a, 'b> {
        AfterEvent {
            before: Value(before),
            after: Value(after),
        }
    }
    */
}

impl<'a> Clone for Event<'a> {
    fn clone(&self) -> Event<'static> {
        Event
    }
}

impl<'a, 'b> AddPress<'a, PressEvent<'a, 'b>> for Event<'a> {
    #[inline(always)]
    fn press(&'a self, key: &'a KeyType) -> PressEvent<'a, 'a> {
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

