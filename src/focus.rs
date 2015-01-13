use input::Input;

use Event;

/// When window gets or looses focus
pub trait FocusEvent {
    /// Creates a focus event.
    fn from_focused(focused: bool) -> Option<Self>;
    /// Calls closure if this is a focus event.
    fn focus<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(bool) -> U;
    /// Returns focus arguments.
    fn focus_args(&self) -> Option<bool> {
        self.focus(|val| val)
    }
}

impl FocusEvent for Input {
    fn from_focused(focused: bool) -> Option<Self> {
        Some(Input::Focus(focused))
    }

    fn focus<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        if let &Input::Focus(focused) = self {
            Some(f(focused))
        } else {
            None
        }
    }
}

impl<I> FocusEvent for Event<I>
    where I: FocusEvent
{
    fn from_focused(focused: bool) -> Option<Self> {
        if let Some(input) = FocusEvent::from_focused(focused) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn focus<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        if let &Event::Input(ref input) = self {
            input.focus(f)
        } else {
            None
        }
    }
}
