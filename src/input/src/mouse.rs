//! Back-end agnostic mouse buttons.

use crate::{Event, Input, Motion};

/// Represent a mouse button.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
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

/// The position of the mouse cursor.
pub trait MouseCursorEvent: Sized {
    /// Creates a mouse cursor event.
    ///
    /// Preserves time stamp from original input event, if any.
    fn from_pos(pos: [f64; 2], old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a mouse cursor event.
    fn mouse_cursor<U, F>(&self, f: F) -> Option<U>
    where
        F: FnMut([f64; 2]) -> U;
    /// Returns mouse cursor arguments.
    fn mouse_cursor_args(&self) -> Option<[f64; 2]> {
        self.mouse_cursor(|pos| pos)
    }
}

impl MouseCursorEvent for Event {
    fn from_pos(pos: [f64; 2], old_event: &Self) -> Option<Self> {
        let timestamp = if let Event::Input(_, x) = old_event {
            *x
        } else {
            None
        };
        Some(Event::Input(
            Input::Move(Motion::MouseCursor(pos)),
            timestamp,
        ))
    }

    fn mouse_cursor<U, F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut([f64; 2]) -> U,
    {
        match *self {
            Event::Input(Input::Move(Motion::MouseCursor(pos)), _) => Some(f(pos)),
            _ => None,
        }
    }
}

/// The relative movement of mouse cursor.
///
/// These events might be emitted even the cursor is captured by the window.
pub trait MouseRelativeEvent: Sized {
    /// Creates a mouse relative event.
    ///
    /// Preserves time stamp from original input event, if any.
    fn from_pos(x: [f64; 2], old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a mouse relative event.
    fn mouse_relative<U, F>(&self, f: F) -> Option<U>
    where
        F: FnMut([f64; 2]) -> U;
    /// Returns mouse relative arguments.
    fn mouse_relative_args(&self) -> Option<[f64; 2]> {
        self.mouse_relative(|pos| pos)
    }
}

impl MouseRelativeEvent for Event {
    fn from_pos(pos: [f64; 2], old_event: &Self) -> Option<Self> {
        let timestamp = if let Event::Input(_, x) = old_event {
            *x
        } else {
            None
        };
        Some(Event::Input(
            Input::Move(Motion::MouseRelative(pos)),
            timestamp,
        ))
    }

    fn mouse_relative<U, F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut([f64; 2]) -> U,
    {
        match *self {
            Event::Input(Input::Move(Motion::MouseRelative(pos)), _) => Some(f(pos)),
            _ => None,
        }
    }
}

/// The scroll of the mouse wheel.
pub trait MouseScrollEvent: Sized {
    /// Creates a mouse scroll event.
    ///
    /// Preserves time stamp from original input event, if any.
    fn from_pos(pos: [f64; 2], old_event: &Self) -> Option<Self>;
    /// Calls a closure if this is a mouse scroll event.
    fn mouse_scroll<U, F>(&self, f: F) -> Option<U>
    where
        F: FnMut([f64; 2]) -> U;
    /// Returns mouse scroll arguments.
    fn mouse_scroll_args(&self) -> Option<[f64; 2]> {
        self.mouse_scroll(|pos| pos)
    }
}

impl MouseScrollEvent for Event {
    fn from_pos(pos: [f64; 2], old_event: &Self) -> Option<Self> {
        let timestamp = if let Event::Input(_, x) = old_event {
            *x
        } else {
            None
        };
        Some(Event::Input(
            Input::Move(Motion::MouseScroll(pos)),
            timestamp,
        ))
    }

    fn mouse_scroll<U, F>(&self, mut f: F) -> Option<U>
    where
        F: FnMut([f64; 2]) -> U,
    {
        match *self {
            Event::Input(Input::Move(Motion::MouseScroll(pos)), _) => Some(f(pos)),
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

        let e: Event = Motion::MouseCursor([0.0, 0.0]).into();
        let a: Option<Event> = MouseCursorEvent::from_pos([1.0, 0.0], &e);
        let b: Option<Event> = a
            .clone()
            .unwrap()
            .mouse_cursor(|pos| MouseCursorEvent::from_pos(pos, a.as_ref().unwrap()))
            .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_relative() {
        use super::super::Motion;

        let e: Event = Motion::MouseRelative([0.0, 0.0]).into();
        let a: Option<Event> = MouseRelativeEvent::from_pos([1.0, 0.0], &e);
        let b: Option<Event> = a
            .clone()
            .unwrap()
            .mouse_relative(|pos| MouseRelativeEvent::from_pos(pos, a.as_ref().unwrap()))
            .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_input_mouse_scroll() {
        use super::super::Motion;

        let e: Event = Motion::MouseScroll([0.0, 0.0]).into();
        let a: Option<Event> = MouseScrollEvent::from_pos([1.0, 0.0], &e);
        let b: Option<Event> = a
            .clone()
            .unwrap()
            .mouse_scroll(|pos| MouseScrollEvent::from_pos(pos, a.as_ref().unwrap()))
            .unwrap();
        assert_eq!(a, b);
    }
}
