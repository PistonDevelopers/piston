//! A library for event threading

#![crate_type = "lib"]
#![crate_name = "event"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![allow(unstable)]

#[cfg(test)]
extern crate test;
extern crate input;
extern crate serialize;
extern crate event_loop;
extern crate "window" as window_lib;

pub use window_lib as window;
pub use event_loop as events;
pub use window::{
    WindowSettings,
    NoWindow
};
pub use events::{
    Events,
    MaxFps,
    Ups,
    UpdateArgs,
    RenderArgs,
    IdleArgs,
};
pub use generic_event::GenericEvent;
pub use update::UpdateEvent;
pub use render::RenderEvent;
pub use idle::IdleEvent;
pub use event::Event;
pub use press::PressEvent;
pub use release::ReleaseEvent;
pub use mouse::{ MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent };
pub use text::TextEvent;
pub use resize::ResizeEvent;
pub use focus::FocusEvent;

pub mod generic_event;
mod update;
mod render;
mod idle;
mod event;
mod press;
mod release;
mod mouse;
mod text;
mod resize;
mod focus;

/// Creates event iterator from window.
pub fn events<W>(window: W) -> event_loop::Events<W, input::Input, Event> {
    event_loop::Events::new(window)
}
