
//! Back-end agnostic mouse buttons.

use {Event, Input, Motion};

/// Represent a mouse button.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq,
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

impl From<u32> for MouseButton {
    fn from(n: u32) -> MouseButton {
        match n {
            0 => MouseButton::Unknown,
            1 => MouseButton::Left,
            2 => MouseButton::Right,
            3 => MouseButton::Middle,
            4 => MouseButton::X1,
            5 => MouseButton::X2,
            6 => MouseButton::Button6,
            7 => MouseButton::Button7,
            8 => MouseButton::Button8,
            _ => MouseButton::Unknown,
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            MouseButton::Unknown => 0,
            MouseButton::Left => 1,
            MouseButton::Right => 2,
            MouseButton::Middle => 3,
            MouseButton::X1 => 4,
            MouseButton::X2 => 5,
            MouseButton::Button6 => 6,
            MouseButton::Button7 => 7,
            MouseButton::Button8 => 8,
        }
    }
}

#[cfg(test)]
mod mouse_button_tests {
    use super::*;

    #[test]
    fn test_mouse_button_primitives() {
        for i in 0u32..9 {
            let button: MouseButton = i.into();
            let j: u32 = button.into();
            assert_eq!(i, j);
        }
    }
}

/// The position of the mouse cursor
pub trait MouseCursorEvent: Sized {
    /// Creates a mouse cursor event.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a mouse cursor event.
    fn mouse_cursor<U, F>(&self, f: F) -> Option<U> where F: FnMut(f64, f64) -> U;
    /// Returns mouse cursor arguments.
    fn mouse_cursor_args(&self) -> Option<[f64; 2]> {
        self.mouse_cursor(|x, y| [x, y])
    }
}

impl MouseCursorEvent for Event {
    fn from_xy(x: f64, y: f64, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Move(Motion::MouseCursor(x, y))))
    }

    fn mouse_cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        match *self {
            Event::Input(Input::Move(Motion::MouseCursor(x, y))) => Some(f(x, y)),
            _ => None,
        }
    }
}

/// The relative movement of mouse cursor
pub trait MouseRelativeEvent: Sized {
    /// Creates a mouse relative event.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a mouse relative event.
    fn mouse_relative<U, F>(&self, f: F) -> Option<U> where F: FnMut(f64, f64) -> U;
    /// Returns mouse relative arguments.
    fn mouse_relative_args(&self) -> Option<[f64; 2]> {
        self.mouse_relative(|x, y| [x, y])
    }
}

impl MouseRelativeEvent for Event {
    fn from_xy(x: f64, y: f64, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Move(Motion::MouseRelative(x, y))))
    }

    fn mouse_relative<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        match *self {
            Event::Input(Input::Move(Motion::MouseRelative(x, y))) => Some(f(x, y)),
            _ => None,
        }
    }
}

/// The scroll of the mouse wheel
pub trait MouseScrollEvent: Sized {
    /// Creates a mouse scroll event.
    fn from_xy(x: f64, y: f64, old_event: &Self) -> Option<Self>;
    /// Calls a closure if this is a mouse scroll event.
    fn mouse_scroll<U, F>(&self, f: F) -> Option<U> where F: FnMut(f64, f64) -> U;
    /// Returns mouse scroll arguments.
    fn mouse_scroll_args(&self) -> Option<[f64; 2]> {
        self.mouse_scroll(|x, y| [x, y])
    }
}

impl MouseScrollEvent for Event {
    fn from_xy(x: f64, y: f64, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Move(Motion::MouseScroll(x, y))))
    }

    fn mouse_scroll<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        match *self {
            Event::Input(Input::Move(Motion::MouseScroll(x, y))) => Some(f(x, y)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod mouse_event_tests {
    use super::*;

    #[test]
    fn test_input_mouse_cursor() {
        use super::super::Motion;

        let e: Event = Motion::MouseCursor(0.0, 0.0).into();
        let a: Option<Event> = MouseCursorEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone()
            .unwrap()
            .mouse_cursor(|x, y| MouseCursorEvent::from_xy(x, y, a.as_ref().unwrap()))
            .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_relative() {
        use super::super::Motion;

        let e: Event = Motion::MouseRelative(0.0, 0.0).into();
        let a: Option<Event> = MouseRelativeEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone()
            .unwrap()
            .mouse_relative(|x, y| MouseRelativeEvent::from_xy(x, y, a.as_ref().unwrap()))
            .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_scroll() {
        use super::super::Motion;

        let e: Event = Motion::MouseScroll(0.0, 0.0).into();
        let a: Option<Event> = MouseScrollEvent::from_xy(1.0, 0.0, &e);
        let b: Option<Event> = a.clone()
            .unwrap()
            .mouse_scroll(|x, y| MouseScrollEvent::from_xy(x, y, a.as_ref().unwrap()))
            .unwrap();
        assert_eq!(a, b);
    }
}
