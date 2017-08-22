use {Event, Loop};

/// Update arguments, such as delta time in seconds
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct UpdateArgs {
    /// Delta time in seconds.
    pub dt: f64,
}

/// When the application state should be updated
pub trait UpdateEvent: Sized {
    /// Creates an update event.
    fn from_update_args(args: &UpdateArgs, old_event: &Self) -> Option<Self>;
    /// Creates an update event with delta time.
    fn from_dt(dt: f64, old_event: &Self) -> Option<Self> {
        UpdateEvent::from_update_args(&UpdateArgs { dt: dt }, old_event)
    }
    /// Calls closure if this is an update event.
    fn update<U, F>(&self, f: F) -> Option<U> where F: FnMut(&UpdateArgs) -> U;
    /// Returns update arguments.
    fn update_args(&self) -> Option<UpdateArgs> {
        self.update(|args| args.clone())
    }
}

impl UpdateEvent for Event {
    fn from_update_args(args: &UpdateArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Loop(Loop::Update(*args)))
    }

    fn update<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U
    {
        match *self {
            Event::Loop(Loop::Update(ref args)) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_update() {
        use Event;
        use UpdateArgs;

        let e: Event = UpdateArgs { dt: 0.0 }.into();
        let x: Option<Event> = UpdateEvent::from_update_args(&UpdateArgs { dt: 1.0 }, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .update(|args| UpdateEvent::from_update_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
