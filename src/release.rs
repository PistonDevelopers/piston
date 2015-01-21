use std::any::{ Any, TypeId };
use std::hash::{ hash, SipHasher };

use input::Button;
use GenericEvent;

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

impl<T: GenericEvent> ReleaseEvent for T {
    fn from_button(button: Button) -> Option<Self> {
        let id = hash::<_, SipHasher>(&TypeId::of::<Box<ReleaseEvent>>());
        GenericEvent::from_args(id, &button as &Any)
    }

    fn release<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        let id = hash::<_, SipHasher>(&TypeId::of::<Box<ReleaseEvent>>());
        if self.event_id() != id {
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


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input_release() {
        use input::{ Button, Key, Input };

        let a = Button::Keyboard(Key::A);
        let x: Option<Input> = ReleaseEvent::from_button(a);
        let y: Option<Input> = x.clone().unwrap().release(|button|
            ReleaseEvent::from_button(button)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_release(bencher: &mut Bencher) {
        use input::{ Button, Input, Key };

        let a = Button::Keyboard(Key::A);
        bencher.iter(|| {
            let _: Option<Input> = ReleaseEvent::from_button(a);
        });
    }

    #[test]
    fn test_event_release() {
        use Event;
        use input::{ Button, Key };

        let a = Button::Keyboard(Key::A);
        let x: Option<Event> = ReleaseEvent::from_button(a);
        let y: Option<Event> = x.clone().unwrap().release(|button|
            ReleaseEvent::from_button(button)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_release(bencher: &mut Bencher) {
        use Event;
        use input::{ Button, Key };

        let a = Button::Keyboard(Key::A);
        bencher.iter(|| {
            let _: Option<Event> = ReleaseEvent::from_button(a);
        });
    }
}
