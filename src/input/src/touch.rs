use std::any::Any;

use { GenericEvent, TOUCH };

/// Touch arguments, .
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug, Hash)]
pub struct TouchArgs {

}

impl TouchArgs {
    /// Create a new TouchArgs object.
    pub fn new() -> TouchArgs {
        TouchArgs {}
    }
}

pub trait TouchEvent: Sized {
    fn from_touch_args(args: &TouchArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a touch event.
    fn touch<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&TouchArgs) -> U;
    /// Returns touch arguments.
    fn touch_args(&self) -> Option<TouchArgs> {
        self.touch(|args| args.clone())
    }
}

impl<T> TouchEvent for T where T: GenericEvent {
    fn from_touch_args(args: &TouchArgs, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(TOUCH, args as &Any, old_event)
    }

    fn touch<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&TouchArgs) -> U
    {
        if self.event_id() != TOUCH {
            return None;
        }
        self.with_args(|any| {
            if let Some(args) = any.downcast_ref::<TouchArgs>() {
                Some(f(args))
            } else {
                panic!("Expected TouchArgs")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_touch() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::Touch(TouchArgs::new()));
        let a: Option<Input> = TouchEvent::from_touch_args(
            &TouchArgs::new(), &e);
        let b: Option<Input> = a.clone().unwrap().touch(|_t|
            TouchEvent::from_touch_args(
                &TouchArgs::new(),
                a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_touch() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::Touch(TouchArgs::new())));
        let a: Option<Event> = TouchEvent::from_touch_args(&TouchArgs::new(), &e);
        let b: Option<Event> = a.clone().unwrap().touch(|_t|
            TouchEvent::from_touch_args(
                &TouchArgs::new(),
                a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }
}
