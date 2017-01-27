use Input;

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

impl CursorEvent for Input {
    fn from_cursor(cursor: bool, _old_event: &Self) -> Option<Self> {
        Some(Input::Cursor(cursor))
    }

    fn cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        match *self {
            Input::Cursor(val) => Some(f(val)),
            _ => None
        }
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
}
