
//! Back-end agnostic keyboard keys.

/// Represent a keyboard key.
#[deriving(Eq)]
pub enum Key {
    /// Key that not supported currently.
    Unknown,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Return
    Enter,
    /// Space bar
    Space,
}

