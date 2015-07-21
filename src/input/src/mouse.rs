
//! Back-end agnostic mouse buttons.

use num::{ FromPrimitive, ToPrimitive };
use std::any::Any;

use { GenericEvent, MOUSE_SCROLL, MOUSE_RELATIVE, MOUSE_CURSOR };

/// Represent a mouse button.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq,
    Eq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    /// Unknown mouse button.
    Unknown,
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Extra mouse button number 1.
    X1,
    /// Extra mouse button number 2.
    X2,
    /// Mouse button number 6.
    Button6,
    /// Mouse button number 7.
    Button7,
    /// Mouse button number 8.
    Button8,
}

impl FromPrimitive for MouseButton {
    fn from_u64(n: u64) -> Option<MouseButton> {
        match n {
            0 => Some(MouseButton::Unknown),
            1 => Some(MouseButton::Left),
            2 => Some(MouseButton::Right),
            3 => Some(MouseButton::Middle),
            4 => Some(MouseButton::X1),
            5 => Some(MouseButton::X2),
            6 => Some(MouseButton::Button6),
            7 => Some(MouseButton::Button7),
            8 => Some(MouseButton::Button8),
            _ => Some(MouseButton::Unknown),
        }
    }

    #[inline(always)]
    fn from_i64(n: i64) -> Option<MouseButton> {
        FromPrimitive::from_u64(n as u64)
    }

    #[inline(always)]
    fn from_isize(n: isize) -> Option<MouseButton> {
        FromPrimitive::from_u64(n as u64)
    }
}

impl ToPrimitive for MouseButton {
    fn to_u64(&self) -> Option<u64> {
        match self {
            &MouseButton::Unknown => Some(0),
            &MouseButton::Left => Some(1),
            &MouseButton::Right => Some(2),
            &MouseButton::Middle => Some(3),
            &MouseButton::X1 => Some(4),
            &MouseButton::X2 => Some(5),
            &MouseButton::Button6 => Some(6),
            &MouseButton::Button7 => Some(7),
            &MouseButton::Button8 => Some(8),
        }
    }

    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        self.to_u64().map(|x| x as i64)
    }

    #[inline(always)]
    fn to_isize(&self) -> Option<isize> {
        self.to_u64().map(|x| x as isize)
    }
}

#[cfg(test)]
mod mouse_button_tests {
    use super::*;

    #[test]
    fn test_mouse_button_primitives() {
        use num::{ FromPrimitive, ToPrimitive };

        for i in 0u64..9 {
            let button: MouseButton = FromPrimitive::from_u64(i).unwrap();
            let j = ToPrimitive::to_u64(&button).unwrap();
            assert_eq!(i, j);
        }
    }
}

/// The position of the mouse cursor
pub trait MouseCursorEvent {
    /// Creates a mouse cursor event.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a mouse cursor event.
    fn mouse_cursor<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse cursor arguments.
    fn mouse_cursor_args(&self) -> Option<[f64; 2]> {
        self.mouse_cursor(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseCursorEvent for T {
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(MOUSE_CURSOR, &(x, y) as &Any, old_event)
    }

    fn mouse_cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != MOUSE_CURSOR {
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
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a mouse relative event.
    fn mouse_relative<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse relative arguments.
    fn mouse_relative_args(&self) -> Option<[f64; 2]> {
        self.mouse_relative(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseRelativeEvent for T {
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(MOUSE_RELATIVE, &(x, y) as &Any, old_event)
    }

    fn mouse_relative<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != MOUSE_RELATIVE {
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
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    /// Calls a closure if this is a mouse scroll event.
    fn mouse_scroll<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse scroll arguments.
    fn mouse_scroll_args(&self) -> Option<[f64; 2]> {
        self.mouse_scroll(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseScrollEvent for T {
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(MOUSE_SCROLL, &(x, y) as &Any, old_event)
    }

    fn mouse_scroll<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if self.event_id() != MOUSE_SCROLL {
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
mod mouse_event_tests {
    use super::*;

    #[test]
    fn test_input_mouse_cursor() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::MouseCursor(0.0, 0.0));
        let a: Option<Input> = MouseCursorEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Input> = a.clone().unwrap().mouse_cursor(|x, y|
            MouseCursorEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_mouse_cursor() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::MouseCursor(0.0, 0.0)));
        let a: Option<Event> = MouseCursorEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone().unwrap().mouse_cursor(|x, y|
            MouseCursorEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_relative() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::MouseRelative(0.0, 0.0));
        let a: Option<Input> = MouseRelativeEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Input> = a.clone().unwrap().mouse_relative(|x, y|
            MouseRelativeEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_mouse_relative() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::MouseRelative(0.0, 0.0)));
        let a: Option<Event> = MouseRelativeEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone().unwrap().mouse_relative(|x, y|
            MouseRelativeEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_scroll() {
        use super::super::{ Input, Motion };

        let e = Input::Move(Motion::MouseScroll(0.0, 0.0));
        let a: Option<Input> = MouseScrollEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Input> = a.clone().unwrap().mouse_scroll(|x, y|
            MouseScrollEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_event_mouse_scroll() {
        use Event;
        use super::super::{ Input, Motion };

        let e = Event::Input(Input::Move(Motion::MouseScroll(0.0, 0.0)));
        let a: Option<Event> = MouseScrollEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone().unwrap().mouse_scroll(|x, y|
            MouseScrollEvent::from_xy(x, y, a.as_ref().unwrap())).unwrap();
        assert_eq!(a, b);
    }
}
