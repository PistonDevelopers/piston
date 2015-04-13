use std::borrow::ToOwned;
use std::any::Any;

use { GenericEvent, TEXT };

/// When receiving text from user, such as typing a character
pub trait TextEvent {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input_text() {
        use input::Input;

        let e = Input::Text("".to_string());
        let x: Option<Input> = TextEvent::from_text("hello", &e);
        let y: Option<Input> = x.clone().unwrap().text(|text|
            TextEvent::from_text(text, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_text(bencher: &mut Bencher) {
        use input::Input;

        let e = Input::Text("".to_string());
        bencher.iter(|| {
            let _: Option<Input> = TextEvent::from_text("hello", &e);
        });
    }

    #[test]
    fn test_event_text() {
        use Event;
        use input::Input;

        let e = Event::Input(Input::Text("".to_string()));
        let x: Option<Event> = TextEvent::from_text("hello", &e);
        let y: Option<Event> = x.clone().unwrap().text(|text|
            TextEvent::from_text(text, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_text(bencher: &mut Bencher) {
        use Event;
        use input::Input;

        let e = Event::Input(Input::Text("".to_string()));
        bencher.iter(|| {
            let _: Option<Event> = TextEvent::from_text("hello", &e);
        });
    }
}
