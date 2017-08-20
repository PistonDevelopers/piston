use {Button, Event, Input};

/// The press of a button
pub trait PressEvent: Sized {
    /// Creates a press event.
    fn from_button(button: Button, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press<U, F>(&self, f: F) -> Option<U> where F: FnMut(Button) -> U;
    /// Returns press arguments.
    fn press_args(&self) -> Option<Button> {
        self.press(|button| button)
    }
}

impl PressEvent for Event {
    fn from_button(button: Button, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Press(button)))
    }

    fn press<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        match *self {
            Event::Input(Input::Press(button)) => Some(f(button)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_press() {
        use super::super::{Button, Key, Input};

        let e: Event = Input::Press(Key::S.into()).into();
        let button = Button::Keyboard(Key::A);
        let x: Option<Event> = PressEvent::from_button(button, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .press(|button| PressEvent::from_button(button, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
