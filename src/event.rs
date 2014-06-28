
//! Event data.

use keyboard;
use mouse;

/// Contains the information associated with an event.
#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub enum Event {
    /// No event occured.
    NoEvent,
    /// A key was released.
    KeyReleased(keyboard::Key),
    /// A key was pressed.
    KeyPressed(keyboard::Key),
    /// A mouse button was pressed.
    MouseButtonPressed(mouse::Button),
    /// A mouse button was released.
    MouseButtonReleased(mouse::Button),
    /// The mouse moved.
    ///
    /// (x, y, Some((dx, dy)))
    /// The second part is relative motion which is not bounded.
    /// Relative motion might not be supported by all window back-ends.
    MouseMoved(f64, f64, Option<(f64, f64)>),
    /// The mouse wheel.
    ///
    /// (x, y)
    MouseScrolled(f64, f64),
}

