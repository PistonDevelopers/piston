use crate::{Event, Input};

/// Close arguments.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize, Hash)]
pub struct CloseArgs;

/// Window is closing.
pub trait CloseEvent: Sized {
    /// Creates a close event from arguments.
    ///
    /// Preserves time stamp from original input event, if any.
    fn from_close_args(args: &CloseArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a close event.
    fn close<U, F>(&self, f: F) -> Option<U>
    where
        F: FnMut(&CloseArgs) -> U;
    /// Returns close arguments.
    fn close_args(&self) -> Option<CloseArgs> {
        self.close(|args| *args)
    }
}

impl CloseEvent for Event {
    fn from_close_args(args: &CloseArgs, old_event: &Self) -> Option<Self> {
        let timestamp = if let Event::Input(_, x) = old_event {
            *x
        } else {
            None
        };
        Some(Event::Input(Input::Close(*args), timestamp))
    }

    fn close<U, F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut(&CloseArgs) -> U,
    {
        match *self {
            Event::Input(Input::Close(ref args), _) => Some(f(args)),
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
        let y: Option<Event> = x
            .clone()
            .unwrap()
            .close(|args| CloseEvent::from_close_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
