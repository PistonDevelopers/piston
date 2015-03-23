//! A library for event threading

#![crate_type = "lib"]
#![crate_name = "event"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]
#![feature(core)]
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;
extern crate input;
extern crate event_loop;
extern crate window;

use window::Window;

pub use event_loop as events;
pub use events::{
    Events,
    MaxFps,
    Ups,
    UpdateArgs,
    RenderArgs,
    AfterRenderArgs,
    IdleArgs,
};
pub use generic_event::GenericEvent;
pub use update::UpdateEvent;
pub use render::RenderEvent;
pub use after_render::AfterRenderEvent;
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
mod after_render;
mod idle;
mod event;
mod press;
mod release;
mod mouse;
mod text;
mod resize;
mod focus;

/// Creates event iterator from window.
pub fn events<W>(window: W) -> event_loop::Events<W, Event<<W as Window>::Event>>
    where
        W: Window,
{
    event_loop::Events::new(window)
}
