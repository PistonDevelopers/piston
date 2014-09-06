#![crate_name = "input"]
#![deny(missing_doc)]
#![feature(globs)]
#![feature(struct_variant)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

pub mod keyboard;
pub mod mouse;

/// Models different kinds of buttons.
#[deriving(Clone, PartialEq, Eq, Show)]
pub enum Button {
    /// A keyboard button.
    Keyboard(keyboard::Key),
    /// A mouse button.
    Mouse(mouse::Button),
}

/// Models different kinds of motion.
#[deriving(Clone, Show)]
pub enum Motion {
    /// x and y in window coordinates.
    MouseCursor(f64, f64),
    /// x and y in relative coordinates.
    MouseRelative(f64, f64),
    /// x and y in scroll ticks.
    MouseScroll(f64, f64),
}

/// Models input events.
#[deriving(Clone, Show)]
pub enum InputEvent {
    /// Pressed a button.
    Press(Button),
    /// Released a button.
    Release(Button),
    /// Moved mouse cursor.
    Move(Motion),
    /// Text (usually from keyboard).
    Text(String),
    /// Window got resized.
    Resize(u32, u32),
    /// Window gained or lost focus.
    Focus(bool),
}

