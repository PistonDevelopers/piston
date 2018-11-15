#![crate_name = "input"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate viewport;

use std::fmt;
use std::any::Any;
use std::sync::Arc;

pub use mouse::MouseButton;
pub use keyboard::Key;
pub use controller::{ControllerAxisArgs, ControllerButton, ControllerHat};

pub mod controller;
pub mod keyboard;
pub mod mouse;

pub use after_render::{AfterRenderArgs, AfterRenderEvent};
pub use close::{CloseArgs, CloseEvent};
pub use controller::ControllerAxisEvent;
pub use cursor::CursorEvent;
pub use focus::FocusEvent;
pub use generic_event::GenericEvent;
pub use idle::{IdleArgs, IdleEvent};
pub use mouse::{MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent};
pub use button::{ButtonState, ButtonArgs, ButtonEvent, PressEvent, ReleaseEvent};
pub use resize::ResizeEvent;
pub use render::{RenderArgs, RenderEvent};
pub use text::TextEvent;
pub use touch::{Touch, TouchArgs, TouchEvent};
pub use update::{UpdateArgs, UpdateEvent};

pub mod generic_event;

mod after_render;
mod button;
mod close;
mod cursor;
mod focus;
mod idle;
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
const CLOSE: EventId = EventId("piston/close");
const IDLE: EventId = EventId("piston/idle");
const MOUSE_SCROLL: EventId = EventId("piston/mouse_scroll");
const MOUSE_RELATIVE: EventId = EventId("piston/mouse_relative");
const MOUSE_CURSOR: EventId = EventId("piston/mouse_cursor");
const BUTTON: EventId = EventId("piston/button");
const RENDER: EventId = EventId("piston/render");
const RESIZE: EventId = EventId("piston/resize");
const TEXT: EventId = EventId("piston/text");
const TOUCH: EventId = EventId("piston/touch");
const UPDATE: EventId = EventId("piston/update");

/// Models different kinds of buttons.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub enum Button {
    /// A keyboard button.
    Keyboard(Key),
    /// A mouse button.
    Mouse(MouseButton),
    /// A controller button.
    Controller(ControllerButton),
    /// A controller hat (d-Pad)
    Hat(ControllerHat),
}

/// Models different kinds of motion.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
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

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Debug, Hash)]
#[allow(missing_docs)]
pub enum HatState {
  Centered,
  Up,
  Right,
  Down,
  Left,
  RightUp,
  RightDown,
  LeftUp,
  LeftDown,
}

/// Models input events.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Input {
    /// Changed button state.
    Button(ButtonArgs),
    /// Moved mouse cursor.
    Move(Motion),
    /// Text (usually from keyboard).
    Text(String),
    /// Window got resized.
    Resize(f64, f64),
    /// Window gained or lost focus.
    Focus(bool),
    /// Window gained or lost cursor.
    Cursor(bool),
    /// Window closed.
    Close(CloseArgs),
}

/// Models loop events.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Loop {
    /// Render graphics.
    Render(RenderArgs),
    /// After rendering and swapping buffers.
    AfterRender(AfterRenderArgs),
    /// Update the state of the application.
    Update(UpdateArgs),
    /// Do background tasks that can be done incrementally.
    Idle(IdleArgs),
}

/// Models all events.
#[derive(Clone)]
pub enum Event {
    /// Input events.
    Input(Input),
    /// Events that commonly used by event loops.
    Loop(Loop),
    /// Custom event.
    ///
    /// When comparing two custom events for equality,
    /// they always return `false`.
    Custom(EventId, Arc<Any + Send + Sync>),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::Input(ref input) => write!(f, "{:?}", input),
            Event::Loop(ref l) => write!(f, "{:?}", l),
            Event::Custom(ref id, _) => write!(f, "Custom({:?}, _)", id),
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        use Event::*;

        match (self, other) {
            (&Input(ref a), &Input(ref b)) => a == b,
            (&Loop(ref a), &Loop(ref b)) => a == b,
            (_, _) => false,
        }
    }
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

impl From<ButtonArgs> for Input {
    fn from(args: ButtonArgs) -> Self {
        Input::Button(args)
    }
}

impl From<ControllerAxisArgs> for Motion {
    fn from(args: ControllerAxisArgs) -> Self {
        Motion::ControllerAxis(args)
    }
}

impl From<ControllerAxisArgs> for Input {
    fn from(args: ControllerAxisArgs) -> Self {
        Input::Move(Motion::ControllerAxis(args))
    }
}

impl From<TouchArgs> for Motion {
    fn from(args: TouchArgs) -> Self {
        Motion::Touch(args)
    }
}

impl From<TouchArgs> for Input {
    fn from(args: TouchArgs) -> Self {
        Input::Move(Motion::Touch(args))
    }
}

impl From<Motion> for Input {
    fn from(motion: Motion) -> Self {
        Input::Move(motion)
    }
}

impl From<RenderArgs> for Loop {
    fn from(args: RenderArgs) -> Self {
        Loop::Render(args)
    }
}

impl From<RenderArgs> for Event {
    fn from(args: RenderArgs) -> Self {
        Event::Loop(Loop::Render(args))
    }
}

impl From<AfterRenderArgs> for Loop {
    fn from(args: AfterRenderArgs) -> Self {
        Loop::AfterRender(args)
    }
}

impl From<AfterRenderArgs> for Event {
    fn from(args: AfterRenderArgs) -> Self {
        Event::Loop(Loop::AfterRender(args))
    }
}

impl From<UpdateArgs> for Loop {
    fn from(args: UpdateArgs) -> Self {
        Loop::Update(args)
    }
}

impl From<UpdateArgs> for Event {
    fn from(args: UpdateArgs) -> Self {
        Event::Loop(Loop::Update(args))
    }
}

impl From<IdleArgs> for Loop {
    fn from(args: IdleArgs) -> Self {
        Loop::Idle(args)
    }
}

impl From<IdleArgs> for Event {
    fn from(args: IdleArgs) -> Self {
        Event::Loop(Loop::Idle(args))
    }
}

impl From<CloseArgs> for Input {
    fn from(args: CloseArgs) -> Self {
        Input::Close(args)
    }
}

impl<T> From<T> for Event
    where Input: From<T>
{
    fn from(args: T) -> Self {
        Event::Input(args.into())
    }
}

impl From<Loop> for Event {
    fn from(l: Loop) -> Self {
        Event::Loop(l)
    }
}

impl Into<Option<Input>> for Event {
    fn into(self) -> Option<Input> {
        if let Event::Input(input) = self {
            Some(input)
        } else {
            None
        }
    }
}

impl Into<Option<Loop>> for Event {
    fn into(self) -> Option<Loop> {
        if let Event::Loop(l) = self {
            Some(l)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_sync_send() {
        fn chk<T: Sync + Send>() {}

        chk::<Input>();
    }
}
