
//! Event data.

use keyboard;

/// Contains the information associated with an event.
pub enum Event {
    /// No event occured.
    NoEvent,
    /// A key was released.
    KeyReleased(keyboard::Key),
    /// A key was pressed.
    KeyPressed(keyboard::Key),
}


