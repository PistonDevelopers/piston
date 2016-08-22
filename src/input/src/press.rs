use { Button, Event, Input };

/// The press of a button
pub trait PressEvent: Sized {
    /// Creates a press event.
    fn from_button(button: Button, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U;
    /// Returns press arguments.
    fn press_args(&self) -> Option<Button> {
        self.press(|button| button)
    }
}

/* TODO: Enable when specialization gets stable.
impl<T: GenericEvent> PressEvent for T {
    fn from_button(button: Button, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(PRESS, &button as &Any, old_event)
    }

    fn press<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if self.event_id() != PRESS {
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
*/

impl PressEvent for Input {
    fn from_button(button: Button, _old_event: &Self) -> Option<Self> {
        Some(Input::Press(button))
    }

    fn press<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        match *self {
            Input::Press(button) => Some(f(button)),
            _ => None
        }
    }
}

impl<I: PressEvent> PressEvent for Event<I> {
    fn from_button(button: Button, old_event: &Self) -> Option<Self> {
        if let &Event::Input(ref old_input) = old_event {
            <I as PressEvent>::from_button(button, old_input)
                .map(|x| Event::Input(x))
        } else {
            None
        }
    }

    fn press<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        match *self {
            Event::Input(ref x) => x.press(f),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_press() {
        use super::super::{ Button, Key, Input };

        let e = Input::Press(Button::Keyboard(Key::S));
        let button = Button::Keyboard(Key::A);
        let x: Option<Input> = PressEvent::from_button(button, &e);
        let y: Option<Input> = x.clone().unwrap().press(|button|
            PressEvent::from_button(button, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }


    #[test]
    fn test_event_press() {
        use Event;
        use super::super::{ Button, Key, Input };

        let e = Event::Input(Input::Press(Button::Keyboard(Key::S)));
        let button = Button::Keyboard(Key::A);
        let x: Option<Event> = PressEvent::from_button(button, &e);
        let y: Option<Event> = x.clone().unwrap().press(|button|
            PressEvent::from_button(button, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
