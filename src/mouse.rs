use std::intrinsics::TypeId;
use std::any::Any;

use GenericEvent;

/// The position of the mouse cursor
pub trait MouseCursorEvent {
    /// Creates a mouse cursor event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse cursor event.
    fn mouse_cursor<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse cursor arguments.
    fn mouse_cursor_args(&self) -> Option<[f64; 2]> {
        self.mouse_cursor(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseCursorEvent for T {
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        GenericEvent::from_args(
            TypeId::of::<Box<MouseCursorEvent>>().hash(),
            &(x, y) as &Any
        )
    }

    fn mouse_cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != TypeId::of::<Box<MouseCursorEvent>>().hash() {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                Some(f(x, y))
            } else {
                panic!("Expected (f64, f64)")
            }
        })
    }
}

/// The relative movement of mouse cursor
pub trait MouseRelativeEvent {
    /// Creates a mouse relative event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse relative event.
    fn mouse_relative<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse relative arguments.
    fn mouse_relative_args(&self) -> Option<[f64; 2]> {
        self.mouse_relative(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseRelativeEvent for T {
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        GenericEvent::from_args(
            TypeId::of::<Box<MouseRelativeEvent>>().hash(),
            &(x, y) as &Any
        )
    }

    fn mouse_relative<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != TypeId::of::<Box<MouseRelativeEvent>>().hash() {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                Some(f(x, y))
            } else {
                panic!("Expected (f64, f64)")
            }
        })
    }
}

/// The scroll of the mouse wheel
pub trait MouseScrollEvent {
    /// Creates a mouse scroll event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls a closure if this is a mouse scroll event.
    fn mouse_scroll<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse scroll arguments.
    fn mouse_scroll_args(&self) -> Option<[f64; 2]> {
        self.mouse_scroll(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseScrollEvent for T {
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        GenericEvent::from_args(
            TypeId::of::<Box<MouseScrollEvent>>().hash(),
            &(x, y) as &Any
        )
    }

    fn mouse_scroll<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != TypeId::of::<Box<MouseScrollEvent>>().hash() {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(x, y)) = any.downcast_ref::<(f64, f64)>() {
                Some(f(x, y))
            } else {
                panic!("Expected (f64, f64)")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input_mouse_cursor() {
        use input::Input;

        let x: Option<Input> = MouseCursorEvent::from_xy(1.0, 0.0);
        let y: Option<Input> = x.clone().unwrap().mouse_cursor(|x, y|
            MouseCursorEvent::from_xy(x, y)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_mouse_cursor(bencher: &mut Bencher) {
        use input::Input;

        bencher.iter(|| {
            let _: Option<Input> = MouseCursorEvent::from_xy(1.0, 0.0);
        });
    }

    #[test]
    fn test_event_mouse_cursor() {
        use Event;

        let x: Option<Event> = MouseCursorEvent::from_xy(1.0, 0.0);
        let y: Option<Event> = x.clone().unwrap().mouse_cursor(|x, y|
            MouseCursorEvent::from_xy(x, y)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_mouse_cursor(bencher: &mut Bencher) {
        use Event;

        bencher.iter(|| {
            let _: Option<Event> = MouseCursorEvent::from_xy(1.0, 0.0);
        });
    }

    #[test]
    fn test_input_mouse_relative() {
        use input::Input;

        let x: Option<Input> = MouseRelativeEvent::from_xy(1.0, 0.0);
        let y: Option<Input> = x.clone().unwrap().mouse_relative(|x, y|
            MouseRelativeEvent::from_xy(x, y)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_mouse_relative(bencher: &mut Bencher) {
        use input::Input;

        bencher.iter(|| {
            let _: Option<Input> = MouseRelativeEvent::from_xy(1.0, 0.0);
        });
    }

    #[test]
    fn test_event_mouse_relative() {
        use Event;

        let x: Option<Event> = MouseRelativeEvent::from_xy(1.0, 0.0);
        let y: Option<Event> = x.clone().unwrap().mouse_relative(|x, y|
            MouseRelativeEvent::from_xy(x, y)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_mouse_relative(bencher: &mut Bencher) {
        use Event;

        bencher.iter(|| {
            let _: Option<Event> = MouseRelativeEvent::from_xy(1.0, 0.0);
        });
    }

    #[test]
    fn test_input_mouse_scroll() {
        use input::Input;

        let x: Option<Input> = MouseScrollEvent::from_xy(1.0, 0.0);
        let y: Option<Input> = x.clone().unwrap().mouse_scroll(|x, y|
            MouseScrollEvent::from_xy(x, y)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_mouse_scroll(bencher: &mut Bencher) {
        use input::Input;

        bencher.iter(|| {
            let _: Option<Input> = MouseScrollEvent::from_xy(1.0, 0.0);
        });
    }

    #[test]
    fn test_event_mouse_scroll() {
        use Event;

        let x: Option<Event> = MouseScrollEvent::from_xy(1.0, 0.0);
        let y: Option<Event> = x.clone().unwrap().mouse_scroll(|x, y|
            MouseScrollEvent::from_xy(x, y)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_mouse_scroll(bencher: &mut Bencher) {
        use Event;

        bencher.iter(|| {
            let _: Option<Event> = MouseScrollEvent::from_xy(1.0, 0.0);
        });
    }
}
