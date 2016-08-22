use std::borrow::ToOwned;
use std::any::Any;

use { Event, GenericEvent, Input, TEXT };

/// When receiving text from user, such as typing a character
pub trait TextEvent: Sized {
    /// Creates a text event.
    fn from_text(text: &str, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a text event.
    fn text<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&str) -> U;
    /// Returns text arguments.
    fn text_args(&self) -> Option<String> {
        self.text(|text| text.to_owned())
    }
}

/* TODO: Enable when specialization gets stable.
impl<T: GenericEvent> TextEvent for T {
    fn from_text(text: &str, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(TEXT, &text.to_owned() as &Any, old_event)
    }

    fn text<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        if self.event_id() != TEXT {
            return None;
        }
        self.with_args(|any| {
            if let Some(text) = any.downcast_ref::<String>() {
                Some(f(&text))
            } else {
                panic!("Expected &str")
            }
        })
    }
}
*/

impl TextEvent for Input {
    fn from_text(text: &str, _old_event: &Self) -> Option<Self> {
        Some(Input::Text(text.into()))
    }

    fn text<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        match *self {
            Input::Text(ref s) => Some(f(s)),
            _ => None
        }
    }
}

// TODO: Add impl for `Event<Input>` when specialization gets stable.
impl<I: GenericEvent> TextEvent for Event<I> {
    fn from_text(text: &str, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(TEXT, &text.to_owned() as &Any, old_event)
    }

    fn text<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        if self.event_id() != TEXT {
            return None;
        }
        self.with_args(|any| {
            if let Some(text) = any.downcast_ref::<String>() {
                Some(f(&text))
            } else {
                panic!("Expected &str")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_text() {
        use super::super::Input;

        let e = Input::Text("".to_string());
        let x: Option<Input> = TextEvent::from_text("hello", &e);
        let y: Option<Input> = x.clone().unwrap().text(|text|
            TextEvent::from_text(text, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }


    #[test]
    fn test_event_text() {
        use Event;
        use super::super::Input;

        let e = Event::Input(Input::Text("".to_string()));
        let x: Option<Event> = TextEvent::from_text("hello", &e);
        let y: Option<Event> = x.clone().unwrap().text(|text|
            TextEvent::from_text(text, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
