use {Event, Input};

/// When window gets or loses focus
pub trait FocusEvent: Sized {
    /// Creates a focus event.
    fn from_focused(focused: bool, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a focus event.
    fn focus<U, F>(&self, f: F) -> Option<U> where F: FnMut(bool) -> U;
    /// Returns focus arguments.
    fn focus_args(&self) -> Option<bool> {
        self.focus(|val| val)
    }
}

impl FocusEvent for Event {
    fn from_focused(focused: bool, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Focus(focused)))
    }

    fn focus<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        match *self {
            Event::Input(Input::Focus(focused)) => Some(f(focused)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_focus() {
        use super::super::Input;

        let e: Event = Input::Focus(false).into();
        let x: Option<Event> = FocusEvent::from_focused(true, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .focus(|focused| FocusEvent::from_focused(focused, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
