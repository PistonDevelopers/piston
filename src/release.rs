use input::{ Button, Input };

use Event;

/// The release of a button
pub trait ReleaseEvent {
    /// Creates a release event.
    fn from_button(button: Button) -> Option<Self>;
    /// Calls closure if this is a release event.
    fn release<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U;
    /// Returns release arguments.
    fn release_args(&self) -> Option<Button> {
        self.release(|button| button)
    }
}

impl ReleaseEvent for Input {
    fn from_button(button: Button) -> Option<Self> {
        Some(Input::Release(button))
    }

    fn release<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if let &Input::Release(button) = self {
            Some(f(button))
        } else {
            None
        }
    }
}

impl<I> ReleaseEvent for Event<I>
    where I: ReleaseEvent
{
    fn from_button(button: Button) -> Option<Self> {
        if let Some(input) = ReleaseEvent::from_button(button) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn release<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if let &Event::Input(ref input) = self {
            input.release(f)
        } else {
            None
        }
    }
}
