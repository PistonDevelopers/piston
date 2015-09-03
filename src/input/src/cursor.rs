use std::any::Any;

use { GenericEvent, CURSOR };

/// When window gets or loses cursor
pub trait CursorEvent: Sized {
    /// Creates a cursor event.
    fn from_cursor(cursor: bool, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a cursor event.
    fn cursor<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(bool) -> U;
    /// Returns cursor arguments.
    fn cursor_args(&self) -> Option<bool> {
        self.cursor(|val| val)
    }
}

impl<T: GenericEvent> CursorEvent for T {
    fn from_cursor(cursor: bool, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(CURSOR, &cursor as &Any, old_event)
    }

    fn cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        if self.event_id() != CURSOR {
            return None;
        }
        self.with_args(|any| {
            if let Some(&cursor) = any.downcast_ref::<bool>() {
                Some(f(cursor))
            } else {
                panic!("Expected bool")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_cursor() {
        use super::super::Input;

        let e = Input::Cursor(false);
        let x: Option<Input> = CursorEvent::from_cursor(true, &e);
        let y: Option<Input> = x.clone().unwrap().cursor(|cursor|
            CursorEvent::from_cursor(cursor, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn test_event_cursor() {
        use Event;
        use super::super::Input;

        let e = Event::Input(Input::Cursor(false));
        let x: Option<Event> = CursorEvent::from_cursor(true, &e);
        let y: Option<Event> = x.clone().unwrap().cursor(|cursor|
            CursorEvent::from_cursor(cursor, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
