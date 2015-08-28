use std::any::Any;

use { Button, GenericEvent, RELEASE };

/// The release of a button
pub trait ReleaseEvent: Sized {
    /// Creates a release event.
    fn from_button(button: Button, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a release event.
    fn release<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U;
    /// Returns release arguments.
    fn release_args(&self) -> Option<Button> {
        self.release(|button| button)
    }
}

impl<T: GenericEvent> ReleaseEvent for T {
    fn from_button(button: Button, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(RELEASE, &button as &Any, old_event)
    }

    fn release<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if self.event_id() != RELEASE {
            return None;
        }
        self.with_args(|any| {
            if let Some(&button) = any.downcast_ref::<Button>() {
                Some(f(button))
            } else {
                panic!("Expected Button")
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_release() {
        use super::super::{ Button, Key, Input };

        let e = Input::Release(Button::Keyboard(Key::S));
        let button = Button::Keyboard(Key::A);
        let x: Option<Input> = ReleaseEvent::from_button(button, &e);
        let y: Option<Input> = x.clone().unwrap().release(|button|
            ReleaseEvent::from_button(button, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn test_event_release() {
        use Event;
        use super::super::{ Button, Key, Input };

        let e = Event::Input(Input::Release(Button::Keyboard(Key::S)));
        let button = Button::Keyboard(Key::A);
        let x: Option<Event> = ReleaseEvent::from_button(button, &e);
        let y: Option<Event> = x.clone().unwrap().release(|button|
            ReleaseEvent::from_button(button, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
