use std::any::Any;

use { Event, GenericEvent, Input, FOCUS };

/// When window gets or loses focus
pub trait FocusEvent: Sized {
    /// Creates a focus event.
    fn from_focused(focused: bool, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a focus event.
    fn focus<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(bool) -> U;
    /// Returns focus arguments.
    fn focus_args(&self) -> Option<bool> {
        self.focus(|val| val)
    }
}

/* TODO: Enable when specialization gets stable.
impl<T: GenericEvent> FocusEvent for T {
    fn from_focused(focused: bool, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(FOCUS, &focused as &Any, old_event)
    }

    fn focus<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        if self.event_id() != FOCUS {
            return None;
        }
        self.with_args(|any| {
            if let Some(&focused) = any.downcast_ref::<bool>() {
                Some(f(focused))
            } else {
                panic!("Expected bool")
            }
        })
    }
}
*/

impl FocusEvent for Input {
    fn from_focused(focused: bool, _old_event: &Self) -> Option<Self> {
        Some(Input::Focus(focused))
    }

    fn focus<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        match *self {
            Input::Focus(focused) => Some(f(focused)),
            _ => None
        }
    }
}

// TODO: Add impl for `Event<Input>` when specialization gets stable.
impl<I: GenericEvent> FocusEvent for Event<I> {
    fn from_focused(focused: bool, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(FOCUS, &focused as &Any, old_event)
    }

    fn focus<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        if self.event_id() != FOCUS {
            return None;
        }
        self.with_args(|any| {
            if let Some(&focused) = any.downcast_ref::<bool>() {
                Some(f(focused))
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
    fn test_input_focus() {
        use super::super::Input;

        let e = Input::Focus(false);
        let x: Option<Input> = FocusEvent::from_focused(true, &e);
        let y: Option<Input> = x.clone().unwrap().focus(|focused|
            FocusEvent::from_focused(focused, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn test_event_focus() {
        use Event;
        use super::super::Input;

        let e = Event::Input(Input::Focus(false));
        let x: Option<Event> = FocusEvent::from_focused(true, &e);
        let y: Option<Event> = x.clone().unwrap().focus(|focused|
            FocusEvent::from_focused(focused, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
