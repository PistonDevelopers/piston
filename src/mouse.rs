use input::{ Input, Motion };

use Event;

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

impl MouseCursorEvent for Input {
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        Some(Input::Move(Motion::MouseCursor(x, y)))
    }

    fn mouse_cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if let &Input::Move(Motion::MouseCursor(x, y)) = self {
            Some(f(x, y))
        } else {
            None
        }
    }
}

impl<I> MouseCursorEvent for Event<I>
    where I: MouseCursorEvent
{
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        if let Some(input) = MouseCursorEvent::from_xy(x, y) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn mouse_cursor<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if let &Event::Input(ref input) = self {
            input.mouse_cursor(f)
        } else {
            None
        }
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

impl MouseRelativeEvent for Input {
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        Some(Input::Move(Motion::MouseRelative(x, y)))
    }

    fn mouse_relative<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if let &Input::Move(Motion::MouseRelative(x, y)) = self {
            Some(f(x, y))
        } else {
            None
        }
    }
}

impl<I> MouseRelativeEvent for Event<I>
    where I: MouseRelativeEvent
{
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        if let Some(input) = MouseRelativeEvent::from_xy(x, y) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn mouse_relative<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if let &Event::Input(ref input) = self {
            input.mouse_relative(f)
        } else {
            None
        }
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

impl MouseScrollEvent for Input {
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        Some(Input::Move(Motion::MouseScroll(x, y)))
    }

    fn mouse_scroll<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if let &Input::Move(Motion::MouseScroll(x, y)) = self {
            Some(f(x, y))
        } else {
            None
        }
    }
}

impl<I> MouseScrollEvent for Event<I>
    where I: MouseScrollEvent
{
    fn from_xy(x: f64, y: f64) -> Option<Self> {
        if let Some(input) = MouseScrollEvent::from_xy(x, y) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn mouse_scroll<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        if let &Event::Input(ref input) = self {
            input.mouse_scroll(f)
        } else {
            None
        }
    }
}
