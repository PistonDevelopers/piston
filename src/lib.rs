//! A Behavior Tree implementation for event logic and game AI
//!
//! Each action returns either `Success`, `Failure` or `Running`.
//! Actions are combined with behaviors such as `Wait` and `Select`.
//! The combined behavior is stored in a `Behavior` object.
//!
//! For each `Behavior` there is a `State`.
//! The state tracks the behavior over time.

#![crate_type = "lib"]
#![crate_name = "event"]
#![deny(missing_docs)]
#![feature(default_type_params)]

extern crate time;
extern crate input;
extern crate serialize;
extern crate current;

pub use window::{
    Window,
    WindowSettings,
    NoWindow
};
pub use events::{
    Events,
    MaxFps,
    Ups,
};
pub use generic_event::{ assert_event_trait, GenericEvent };
pub use update::{ UpdateArgs, UpdateEvent };
pub use render::{ RenderArgs, RenderEvent };
pub use event::{ Event, Render, Update, Input };
pub use press::PressEvent;
pub use release::ReleaseEvent;
pub use mouse::{ MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent };
pub use text::TextEvent;
pub use resize::ResizeEvent;
pub use focus::FocusEvent;

pub mod ptr;
pub mod drag_controller;
pub mod window;

mod events;
mod generic_event;
mod update;
mod render;
mod event;
mod press;
mod release;
mod mouse;
mod text;
mod resize;
mod focus;
