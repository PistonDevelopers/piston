use { Event, Input };

/// Idle arguments, such as expected idle time in seconds.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IdleArgs {
    /// Expected idle time in seconds.
    pub dt: f64
}

/// When background tasks should be performed
pub trait IdleEvent: Sized {
    /// Creates an idle event.
    fn from_idle_args(args: &IdleArgs, old_event: &Self) -> Option<Self>;
    /// Creates an update event with delta time.
    fn from_dt(dt: f64, old_event: &Self) -> Option<Self> {
        IdleEvent::from_idle_args(&IdleArgs { dt: dt }, old_event)
    }
    /// Calls closure if this is an idle event.
    fn idle<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&IdleArgs) -> U;
    /// Returns idle arguments.
    fn idle_args(&self) -> Option<IdleArgs> {
        self.idle(|args| args.clone())
    }
}

/* TODO: Enable when specialization gets stable.
impl<T> IdleEvent for T where T: GenericEvent {
    fn from_idle_args(args: &IdleArgs, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(IDLE, args as &Any, old_event)
    }

    fn idle<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&IdleArgs) -> U
    {
        if self.event_id() != IDLE {
            return None;
        }
        self.with_args(|any| {
            if let Some(args) = any.downcast_ref::<IdleArgs>() {
                Some(f(args))
            } else {
                panic!("Expected IdleArgs")
            }
        })
    }
}
*/

impl IdleEvent for Input {
    fn from_idle_args(_args: &IdleArgs, _old_event: &Self) -> Option<Self> {
        None
    }

    fn idle<U, F>(&self, mut _f: F) -> Option<U>
        where F: FnMut(&IdleArgs) -> U
    {
        None
    }
}

impl<I> IdleEvent for Event<I> {
    fn from_idle_args(args: &IdleArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Idle(*args))
    }

    fn idle<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&IdleArgs) -> U
    {
        match *self {
            Event::Idle(ref args) => Some(f(args)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_idle() {
        use Event;
        use IdleArgs;

        let e = Event::Idle(IdleArgs { dt: 1.0 });
        let x: Option<Event> = IdleEvent::from_idle_args(
            &IdleArgs { dt: 1.0 }, &e);
        let y: Option<Event> = x.clone().unwrap().idle(|args|
            IdleEvent::from_idle_args(args, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
