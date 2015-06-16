//! A library for event threading

#![crate_type = "lib"]
#![crate_name = "event"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

extern crate input;
extern crate event_loop;
extern crate window;

use std::cell::RefCell;
use std::rc::Rc;
use window::Window;

pub use event_loop::*;
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

/// Used to identify events arguments provided by traits.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct EventId(pub &'static str);

const FOCUS: EventId = EventId("piston/focus");
const RESIZE: EventId = EventId("piston/resize");
const TEXT: EventId = EventId("piston/text");
const MOUSE_SCROLL: EventId = EventId("piston/mouse_scroll");
const MOUSE_RELATIVE: EventId = EventId("piston/mouse_relative");
const MOUSE_CURSOR: EventId = EventId("piston/mouse_cursor");
const RELEASE: EventId = EventId("piston/release");
const PRESS: EventId = EventId("piston/press");
const IDLE: EventId = EventId("piston/idle");
const AFTER_RENDER: EventId = EventId("piston/after_render");
const RENDER: EventId = EventId("piston/render");
const UPDATE: EventId = EventId("piston/update");

/// A trait for create event iterator from window.
pub trait Events<W> where W: Window {
    /// Creates event iterator from window.
    fn events(self) -> WindowEvents<W, Event<W::Event>>;
}

impl<W> Events<W> for Rc<RefCell<W>> where W: Window {
    fn events(self) -> WindowEvents<W, Event<W::Event>> {
        WindowEvents::new(self)
    }
}

impl<W> Events<W> for W where W: Window {
    fn events(self) -> WindowEvents<W, Event<W::Event>> {
        Rc::new(RefCell::new(self)).events()
    }
}

impl<'a, W> Events<W> for &'a Rc<RefCell<W>> where W: Window {
    fn events(self) -> WindowEvents<W, Event<W::Event>> {
        self.clone().events()
    }
}
