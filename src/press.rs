use input::{ Button, Input };

use Event;

/// The press of a button
pub trait PressEvent {
    /// Creates a press event.
    fn from_button(button: Button) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U;
    /// Returns press arguments.
    fn press_args(&self) -> Option<Button> {
        self.press(|button| button)
    }
}

impl PressEvent for Input {
    fn from_button(button: Button) -> Option<Self> {
        Some(Input::Press(button))
    }

    fn press<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if let &Input::Press(button) = self {
            Some(f(button))
        } else {
            None
        }
    }
}

impl<I> PressEvent for Event<I>
    where I: PressEvent
{
    fn from_button(button: Button) -> Option<Self> {
        if let Some(input) = PressEvent::from_button(button) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn press<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if let &Event::Input(ref input) = self {
            input.press(f)
        } else {
            None
        }
    }
}
