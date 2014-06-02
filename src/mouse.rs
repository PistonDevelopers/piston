
//! Back-end agnostic mouse buttons.

/// Represent a mouse button.
#[deriving(Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Show)]
pub enum Button {
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

