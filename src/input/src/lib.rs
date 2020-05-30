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
use std::path::PathBuf;
use std::cmp::Ordering;

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
pub use resize::{ResizeArgs, ResizeEvent};
pub use render::{RenderArgs, RenderEvent};
pub use text::TextEvent;
pub use touch::{Touch, TouchArgs, TouchEvent};
pub use update::{UpdateArgs, UpdateEvent};

use event_id::EventId;

pub mod event_id;
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

/// The type of time stamp.
///
/// Measured in milliseconds since initialization of window.
pub type TimeStamp = u32;

/// Models different kinds of buttons.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
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
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Debug)]
pub enum Motion {
    /// Position in window coordinates.
    MouseCursor([f64; 2]),
    /// Position in relative coordinates.
    MouseRelative([f64; 2]),
    /// Position in scroll ticks.
    MouseScroll([f64; 2]),
    /// Controller axis move event.
    ControllerAxis(ControllerAxisArgs),
    /// Touch event.
    Touch(TouchArgs),
}

/// Stores controller hat state.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum HatState {
    /// Centered (no direction).
    Centered,
    /// Up direction.
    Up,
    /// Right direction.
    Right,
    /// Down direction.
    Down,
    /// Left direction.
    Left,
    /// Right-up direction.
    RightUp,
    /// Right-down direction.
    RightDown,
    /// Left-up direction.
    LeftUp,
    /// Left-down direction.
    LeftDown,
}

/// Models dragging and dropping files.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Hash)]
pub enum FileDrag {
    /// A file is being hovered over the window.
    Hover(PathBuf),
    /// A file has been dropped into the window.
    Drop(PathBuf),
    /// A file was hovered, but has exited the window.
    Cancel,
}

/// Models input events.
#[derive(Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum Input {
    /// Changed button state.
    Button(ButtonArgs),
    /// Moved mouse cursor.
    Move(Motion),
    /// Text (usually from keyboard).
    Text(String),
    /// Window got resized.
    Resize(ResizeArgs),
    /// Window gained or lost focus.
    Focus(bool),
    /// Window gained or lost cursor.
    Cursor(bool),
    /// A file is being dragged or dropped over the window.
    FileDrag(FileDrag),
    /// Window closed.
    Close(CloseArgs),
}

/// Models loop events.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Deserialize, Serialize)]
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
    ///
    /// Time stamp is ignored when comparing input events for equality and order.
    Input(Input, Option<TimeStamp>),
    /// Events that commonly used by event loops.
    Loop(Loop),
    /// Custom event.
    ///
    /// When comparing two custom events for equality,
    /// they always return `false`.
    ///
    /// When comparing partial order of two custom events,
    /// the event ids are checked and if they are equal it returns `None`.
    ///
    /// Time stamp is ignored both when comparing custom events for equality and order.
    Custom(EventId, Arc<dyn Any + Send + Sync>, Option<TimeStamp>),
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::Input(ref input, _) => write!(f, "{:?}", input),
            Event::Loop(ref l) => write!(f, "{:?}", l),
            Event::Custom(ref id, _, _) => write!(f, "Custom({:?}, _)", id),
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        use Event::*;

        match (self, other) {
            (&Input(ref a, _), &Input(ref b, _)) => a == b,
            (&Loop(ref a), &Loop(ref b)) => a == b,
            (_, _) => false,
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        use Event::*;

        match (self, other) {
            (&Input(ref a, _), &Input(ref b, _)) => a.partial_cmp(b),
            (&Loop(ref a), &Loop(ref b)) => a.partial_cmp(b),
            (&Custom(ref a_id, _, _), &Custom(ref b_id, _, _)) => {
                let res = a_id.partial_cmp(b_id);
                if res == Some(Ordering::Equal) {None}
                else {res}
            }
            (&Input(_, _), _) => Some(Ordering::Less),
            (_, &Input(_, _)) => Some(Ordering::Greater),
            (&Loop(_), &Custom(_, _, _)) => Some(Ordering::Less),
            (&Custom(_, _, _), &Loop(_)) => Some(Ordering::Greater),
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
        Event::Input(args.into(), None)
    }
}

impl<T> From<(T, Option<TimeStamp>)> for Event
    where Input: From<T>
{
    fn from(args: (T, Option<TimeStamp>)) -> Self {
        Event::Input(args.0.into(), args.1)
    }
}

impl From<Loop> for Event {
    fn from(l: Loop) -> Self {
        Event::Loop(l)
    }
}

impl Into<Option<Input>> for Event {
    fn into(self) -> Option<Input> {
        if let Event::Input(input, _) = self {
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
        chk::<Loop>();
        chk::<Event>();
    }
}
