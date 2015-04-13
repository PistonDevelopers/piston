use std::any::Any;

use { GenericEvent, FOCUS };

/// When window gets or looses focus
pub trait FocusEvent {
    /// Creates a focus event.
    fn from_focused(focused: bool, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a focus event.
    fn focus<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(bool) -> U;
    /// Returns focus arguments.
    fn focus_args(&self) -> Option<bool> {
        self.focus(|val| val)
    }
}

impl<T: GenericEvent> FocusEvent for T {
    fn from_focused(focused: bool, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(FOCUS, &focused as &Any, old_event)
    }

    fn focus<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(bool) -> U
    {
        if self.event_id() != FOCUS {
            return None;
        }
        self.with_args(|any| {
            if let Some(&focused) = any.downcast_ref::<bool>() {
                Some(f(focused))
            } else {
                panic!("Expected bool")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input_focus() {
        use input::Input;

        let e = Input::Focus(false);
        let x: Option<Input> = FocusEvent::from_focused(true, &e);
        let y: Option<Input> = x.clone().unwrap().focus(|focused|
            FocusEvent::from_focused(focused, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_focus(bencher: &mut Bencher) {
        use input::Input;

        let e = Input::Focus(false);
        bencher.iter(|| {
            let _: Option<Input> = FocusEvent::from_focused(true, &e);
        });
    }

    #[test]
    fn test_event_focus() {
        use Event;
        use input::Input;

        let e = Event::Input(Input::Focus(false));
        let x: Option<Event> = FocusEvent::from_focused(true, &e);
        let y: Option<Event> = x.clone().unwrap().focus(|focused|
            FocusEvent::from_focused(focused, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_focus(bencher: &mut Bencher) {
        use Event;
        use input::Input;

        let e = Event::Input(Input::Focus(false));
        bencher.iter(|| {
            let _: Option<Event> = FocusEvent::from_focused(true, &e);
        });
    }
}
