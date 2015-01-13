use std::borrow::ToOwned;

use input::Input;

use Event;

/// When receiving text from user, such as typing a character
pub trait TextEvent {
    /// Creates a text event.
    fn from_text(text: &str) -> Option<Self>;
    /// Calls closure if this is a text event.
    fn text<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&str) -> U;
    /// Returns text arguments.
    fn text_args(&self) -> Option<String> {
        self.text(|text| text.to_owned())
    }
}

impl TextEvent for Input {
    fn from_text(text: &str) -> Option<Self> {
        Some(Input::Text(text.to_owned()))
    }

    fn text<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        if let &Input::Text(ref text) = self {
            Some(f(&text[]))
        } else {
            None
        }
    }
}

impl<I> TextEvent for Event<I>
    where I: TextEvent
{
    fn from_text(text: &str) -> Option<Self> {
        if let Some(input) = TextEvent::from_text(text) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn text<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        if let &Event::Input(ref input) = self {
            input.text(f)
        } else {
            None
        }
    }
}
