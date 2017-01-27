use { Button, Input };

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

impl ReleaseEvent for Input {
    fn from_button(button: Button, _old_event: &Self) -> Option<Self> {
        Some(Input::Release(button))
    }

    fn release<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        match *self {
            Input::Release(button) => Some(f(button)),
            _ => None
        }
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
}
