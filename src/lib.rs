#![crate_name = "input"]
#![deny(missing_doc)]
#![feature(globs)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

pub use input_event::{
    InputEvent,
    KeyPress,
    KeyRelease,
    MousePress,
    MouseRelease,
    MouseMove,
    MouseRelativeMove,
    MouseScroll,
};
pub use args::{
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseMoveArgs,
    MouseRelativeMoveArgs,
    MouseScrollArgs,
};

pub mod keyboard;
pub mod mouse;

mod args;
mod input_event;

