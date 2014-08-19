#![crate_name = "input"]
#![deny(missing_doc)]
#![feature(globs)]
#![feature(struct_variant)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

pub mod keyboard;
pub mod mouse;

/// Models input events.
#[deriving(Clone)]
pub enum InputEvent {
    /// Pressed a keyboard key.
    KeyPress {
        /// Keyboard key.
        pub key: keyboard::Key,
    },
    /// Released a keyboard key.
    KeyRelease {
        /// Keyboard key.
        pub key: keyboard::Key,
    },
    /// Pressed a mouse button.
    MousePress {
        /// Mouse button.
        pub button: mouse::Button,
    },
    /// Released a mouse button.
    MouseRelease {
        /// Mouse button.
        pub button: mouse::Button,
    },
    /// Moved mouse cursor.
    MouseMove {
        /// x in window coordinates.
        pub x: f64,
        /// y in window coordinates.
        pub y: f64,
        /// x in drawing coordinates.
        pub draw_x: f64,
        /// y in drawing coordinates.
        pub draw_y: f64,
    },
    /// Moved mouse relative, not bounded by cursor.
    MouseRelativeMove {
        /// Delta x in window coordinates.
        pub dx: f64,
        /// Delta y in window coordinates.
        pub dy: f64,
        /// Delta x in drawing coordinates.
        pub draw_dx: f64,
        /// Delta y in drawing coordinates.
        pub draw_dy: f64,
    },
    /// Scrolled mouse.
    MouseScroll {
        /// Delta x in undefined coordinates.
        pub dx: f64,
        /// Delta y in undefined coordinates.
        pub dy: f64,
    },
}

