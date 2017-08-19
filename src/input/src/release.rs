use {Button, Event, Input};

/// The release of a button
pub trait ReleaseEvent: Sized {
    /// Creates a release event.
    fn from_button(button: Button, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a release event.
    fn release<U, F>(&self, f: F) -> Option<U> where F: FnMut(Button) -> U;
    /// Returns release arguments.
    fn release_args(&self) -> Option<Button> {
        self.release(|button| button)
    }
}

impl ReleaseEvent for Event {
    fn from_button(button: Button, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Release(button)))
    }

    fn release<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        match *self {
            Event::Input(Input::Release(button)) => Some(f(button)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_release() {
        use super::super::{Button, Key, Input};

        let e: Event = Input::Release(Key::S.into()).into();
        let button = Button::Keyboard(Key::A);
        let x: Option<Event> = ReleaseEvent::from_button(button, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .release(|button| ReleaseEvent::from_button(button, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
