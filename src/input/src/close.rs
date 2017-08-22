use {Event, Input};

/// Close arguments.
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct CloseArgs;

/// Window is closing.
pub trait CloseEvent: Sized {
    /// Creates a close event from arguments.
    fn from_close_args(args: &CloseArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a close event.
    fn close<U, F>(&self, f: F) -> Option<U> where F: FnMut(&CloseArgs) -> U;
    /// Returns close arguments.
    fn close_args(&self) -> Option<CloseArgs> {
        self.close(|args| args.clone())
    }
}

impl CloseEvent for Event {
    fn from_close_args(args: &CloseArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Close(*args)))
    }

    fn close<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&CloseArgs) -> U
    {
        match *self {
            Event::Input(Input::Close(ref args)) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_close() {
        use CloseArgs;
        use Event;

        let e: Event = CloseArgs.into();
        let x: Option<Event> = CloseEvent::from_close_args(&CloseArgs, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .close(|args| CloseEvent::from_close_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
