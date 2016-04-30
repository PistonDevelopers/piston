#![crate_name = "input"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

#[macro_use]
extern crate bitflags;
extern crate rustc_serialize;
extern crate viewport;

pub use mouse::MouseButton;
pub use keyboard::Key;
pub use controller::{ ControllerAxisArgs, ControllerButton };

pub mod controller;
pub mod keyboard;
pub mod mouse;

pub use after_render::{ AfterRenderArgs, AfterRenderEvent };
pub use controller::{ ControllerAxisEvent };
pub use cursor::CursorEvent;
pub use event::Event;
pub use focus::FocusEvent;
pub use generic_event::GenericEvent;
pub use idle::{ IdleArgs, IdleEvent };
pub use mouse::{ MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent };
pub use press::PressEvent;
pub use release::ReleaseEvent;
pub use resize::ResizeEvent;
pub use render::{ RenderArgs, RenderEvent };
pub use text::TextEvent;
pub use touch::{ Touch, TouchArgs, TouchEvent };
pub use update::{ UpdateArgs, UpdateEvent };

pub mod generic_event;

mod after_render;
mod cursor;
mod event;
mod focus;
mod idle;
mod press;
mod release;
mod render;
mod resize;
mod text;
mod touch;
mod update;

/// Used to identify events arguments provided by traits.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct EventId(pub &'static str);

const AFTER_RENDER: EventId = EventId("piston/after_render");
const CONTROLLER_AXIS: EventId = EventId("piston/controller_axis");
const CURSOR: EventId = EventId("piston/cursor");
const FOCUS: EventId = EventId("piston/focus");
const IDLE: EventId = EventId("piston/idle");
const MOUSE_SCROLL: EventId = EventId("piston/mouse_scroll");
const MOUSE_RELATIVE: EventId = EventId("piston/mouse_relative");
const MOUSE_CURSOR: EventId = EventId("piston/mouse_cursor");
const PRESS: EventId = EventId("piston/press");
const RELEASE: EventId = EventId("piston/release");
const RENDER: EventId = EventId("piston/render");
const RESIZE: EventId = EventId("piston/resize");
const TEXT: EventId = EventId("piston/text");
const TOUCH: EventId = EventId("piston/touch");
const UPDATE: EventId = EventId("piston/update");

/// Models different kinds of buttons.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Hash, Debug)]
pub enum Button {
    /// A keyboard button.
    Keyboard(Key),
    /// A mouse button.
    Mouse(MouseButton),
    /// A controller button.
    Controller(ControllerButton),
}

/// Models different kinds of motion.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum Motion {
    /// x and y in window coordinates.
    MouseCursor(f64, f64),
    /// x and y in relative coordinates.
    MouseRelative(f64, f64),
    /// x and y in scroll ticks.
    MouseScroll(f64, f64),
    /// controller axis move event.
    ControllerAxis(ControllerAxisArgs),
    /// touch event.
    Touch(TouchArgs),
}

/// Models input events.
#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum Input {
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
    /// Window gained or lost cursor.
    Cursor(bool),
}

impl From<Key> for Button {
    fn from(key: Key) -> Self {
        Button::Keyboard(key)
    }
}

impl From<MouseButton> for Button {
    fn from(btn: MouseButton) -> Self {
        Button::Mouse(btn)
    }
}

impl From<ControllerButton> for Button {
    fn from(btn: ControllerButton) -> Self {
        Button::Controller(btn)
    }
}

impl From<ControllerAxisArgs> for Motion {
    fn from(args: ControllerAxisArgs) -> Self {
        Motion::ControllerAxis(args)
    }
}

impl From<Motion> for Input {
    fn from(motion: Motion) -> Self {
        Input::Move(motion)
    }
}
