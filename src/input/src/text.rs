use std::borrow::ToOwned;

use Input;

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
}
