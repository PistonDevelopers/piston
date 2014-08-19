use {
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseMoveArgs,
    MouseRelativeMoveArgs,
    MouseScrollArgs,
};

/// Models input events.
#[deriving(Clone)]
pub enum InputEvent {
    /// Pressed a keyboard key.
    KeyPress(KeyPressArgs),
    /// Released a keyboard key.
    KeyRelease(KeyReleaseArgs),
    /// Pressed a mouse button.
    MousePress(MousePressArgs),
    /// Released a mouse button.
    MouseRelease(MouseReleaseArgs),
    /// Moved mouse cursor.
    MouseMove(MouseMoveArgs),
    /// Moved mouse relative, not bounded by cursor.
    MouseRelativeMove(MouseRelativeMoveArgs),
    /// Scrolled mouse.
    MouseScroll(MouseScrollArgs),
}

