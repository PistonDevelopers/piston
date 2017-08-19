use std::borrow::ToOwned;

use {Event, Input};

/// When receiving text from user, such as typing a character
pub trait TextEvent: Sized {
    /// Creates a text event.
    fn from_text(text: &str, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a text event.
    fn text<U, F>(&self, f: F) -> Option<U> where F: FnMut(&str) -> U;
    /// Returns text arguments.
    fn text_args(&self) -> Option<String> {
        self.text(|text| text.to_owned())
    }
}

impl TextEvent for Event {
    fn from_text(text: &str, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Text(text.into())))
    }

    fn text<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        match *self {
            Event::Input(Input::Text(ref s)) => Some(f(s)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_text() {
        use super::super::Input;

        let e: Event = Input::Text("".to_string()).into();
        let x: Option<Event> = TextEvent::from_text("hello", &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .text(|text| TextEvent::from_text(text, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
