use {Event, Loop};

/// Idle arguments, such as expected idle time in seconds.
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct IdleArgs {
    /// Expected idle time in seconds.
    pub dt: f64,
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
    fn idle<U, F>(&self, f: F) -> Option<U> where F: FnMut(&IdleArgs) -> U;
    /// Returns idle arguments.
    fn idle_args(&self) -> Option<IdleArgs> {
        self.idle(|args| args.clone())
    }
}

impl IdleEvent for Event {
    fn from_idle_args(args: &IdleArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Loop(Loop::Idle(*args)))
    }

    fn idle<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&IdleArgs) -> U
    {
        match *self {
            Event::Loop(Loop::Idle(ref args)) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_idle() {
        use IdleArgs;

        let e: Event = IdleArgs { dt: 1.0 }.into();
        let x: Option<Event> = IdleEvent::from_idle_args(&IdleArgs { dt: 1.0 }, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .idle(|args| IdleEvent::from_idle_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
