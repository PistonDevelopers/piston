use keyboard;
use mouse;

/// Key press arguments.
#[deriving(Clone)]
pub struct KeyPressArgs {
    /// Keyboard key.
    pub key: keyboard::Key,
}

/// Key release arguments.
#[deriving(Clone)]
pub struct KeyReleaseArgs {
    /// Keyboard key.
    pub key: keyboard::Key,
}

/// Mouse press arguments.
#[deriving(Clone)]
pub struct MousePressArgs {
    /// Mouse button.
    pub button: mouse::Button,
}

/// Mouse release arguments.
#[deriving(Clone)]
pub struct MouseReleaseArgs {
    /// Mouse button.
    pub button: mouse::Button,
}

/// Mouse move arguments.
#[deriving(Clone)]
pub struct MouseMoveArgs {
    /// x in window coordinates.
    pub x: f64,
    /// y in window coordinates.
    pub y: f64,
    /// x in drawing coordinates.
    pub draw_x: f64,
    /// y in drawing coordinates.
    pub draw_y: f64,
}

/// Mouse relative move arguments.
#[deriving(Clone)]
pub struct MouseRelativeMoveArgs {
    /// Delta x in window coordinates.
    pub dx: f64,
    /// Delta y in window coordinates.
    pub dy: f64,
    /// Delta x in drawing coordinates.
    pub draw_dx: f64,
    /// Delta y in drawing coordinates.
    pub draw_dy: f64,
}

/// Mouse scroll arguments.
#[deriving(Clone)]
pub struct MouseScrollArgs {
    /// x.
    pub x: f64,
    /// y.
    pub y: f64,
}

