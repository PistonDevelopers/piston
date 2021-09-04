use crate::{Event, Input};

/// When window gets or loses cursor.
pub trait CursorEvent: Sized {
    /// Creates a cursor event.
    ///
    /// Preserves time stamp from original input event, if any.
    fn from_cursor(cursor: bool, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a cursor event.
    fn cursor<U, F>(&self, f: F) -> Option<U>
    where
        F: FnMut(bool) -> U;
    /// Returns cursor arguments.
    fn cursor_args(&self) -> Option<bool> {
        self.cursor(|val| val)
    }
}

impl CursorEvent for Event {
    fn from_cursor(cursor: bool, old_event: &Self) -> Option<Self> {
        let timestamp = if let Event::Input(_, x) = old_event {
            *x
        } else {
            None
        };
        Some(Event::Input(Input::Cursor(cursor), timestamp))
    }

    fn cursor<U, F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut(bool) -> U,
    {
        match *self {
            Event::Input(Input::Cursor(val), _) => Some(f(val)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_cursor() {
        use super::super::Input;

        let e: Event = Input::Cursor(false).into();
        let x: Option<Event> = CursorEvent::from_cursor(true, &e);
        let y: Option<Event> = x
            .clone()
            .unwrap()
            .cursor(|cursor| CursorEvent::from_cursor(cursor, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
