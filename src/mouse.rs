
//! Back-end agnostic mouse buttons.

/// Represent a mouse button.
#[deriving(Eq)]
pub enum Button {
    /// Button that not supported currently.
    Unknown,
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
}

