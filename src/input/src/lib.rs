#![crate_name = "input"]
#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A flexible structure for user interactions
//! to be used in window frameworks and widgets libraries.

#[macro_use]
extern crate bitflags;
extern crate rustc_serialize;
extern crate viewport;

use std::any::Any;
use std::sync::Arc;

use rustc_serialize::{Decoder, Decodable, Encoder, Encodable};

pub use mouse::MouseButton;
pub use keyboard::Key;
pub use controller::{ ControllerAxisArgs, ControllerButton };

pub mod controller;
pub mod keyboard;
pub mod mouse;

pub use after_render::{ AfterRenderArgs, AfterRenderEvent };
pub use controller::{ ControllerAxisEvent };
pub use cursor::CursorEvent;
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
const CLOSE: EventId = EventId("piston/close");
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
#[derive(Clone, Debug)]
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
    /// Window closed.
    Close,
    /// Render graphics.
    Render(RenderArgs),
    /// After rendering and swapping buffers.
    AfterRender(AfterRenderArgs),
    /// Update the state of the application.
    Update(UpdateArgs),
    /// Do background tasks that can be done incrementally.
    Idle(IdleArgs),
    /// Custom event.
    ///
    /// When comparing two custom events for equality,
    /// they always return `false`.
    Custom(EventId, Arc<Any>),
}

impl PartialEq for Input {
    fn eq(&self, other: &Input) -> bool {
        use Input::*;

        match (self, other) {
            (&Press(ref a), &Press(ref b)) => a == b,
            (&Release(ref a), &Release(ref b)) => a == b,
            (&Move(ref a), &Move(ref b)) => a == b,
            (&Text(ref a), &Text(ref b)) => a == b,
            (&Resize(aw, ah), &Resize(bw, bh)) => aw == bw && ah == bh,
            (&Focus(a), &Focus(b)) => a == b,
            (&Cursor(a), &Cursor(b)) => a == b,
            (&Close, &Close) => true,
            (&Render(ref a), &Render(ref b)) => a == b,
            (&AfterRender(ref a), &AfterRender(ref b)) => a == b,
            (&Update(ref a), &Update(ref b)) => a == b,
            (&Idle(ref a), &Idle(ref b)) => a == b,
            (_, _) => false,
        }
    }
}

impl Decodable for Input {
    fn decode<D: Decoder>(d: &mut D) -> Result<Input, D::Error> {
        d.read_enum("Input", |d| {
            d.read_enum_variant(&[
                "Press", "Release", "Move", "Text",
                "Resize", "Focus", "Cursor", "Close",
                "Render", "AfterRender", "Update", "Idle"
            ], |d, ind| {
                Ok(match ind {
                    0 => Input::Press(try!(d.read_enum_variant_arg(0, |d| Button::decode(d)))),
                    1 => Input::Release(try!(d.read_enum_variant_arg(0, |d| Button::decode(d)))),
                    2 => Input::Move(try!(d.read_enum_variant_arg(0, |d| Motion::decode(d)))),
                    3 => Input::Text(try!(d.read_enum_variant_arg(0, |d| String::decode(d)))),
                    4 => Input::Resize(try!(d.read_enum_variant_arg(0, |d| u32::decode(d))),
                        try!(d.read_enum_variant_arg(1, |d| u32::decode(d)))),
                    5 => Input::Focus(try!(d.read_enum_variant_arg(0, |d| bool::decode(d)))),
                    6 => Input::Cursor(try!(d.read_enum_variant_arg(0, |d| bool::decode(d)))),
                    7 => Input::Close,
                    8 => Input::Render(try!(
                        d.read_enum_variant_arg(0, |d| RenderArgs::decode(d)))),
                    9 => Input::AfterRender(try!(
                        d.read_enum_variant_arg(0, |d| AfterRenderArgs::decode(d)))),
                    10 => Input::Update(try!(
                        d.read_enum_variant_arg(0, |d| UpdateArgs::decode(d)))),
                    11 => Input::Idle(try!(d.read_enum_variant_arg(0, |d| IdleArgs::decode(d)))),
                    _ => unimplemented!(),
                })
            })
        })
    }
}

impl Encodable for Input {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_enum("Input", |s| {
            match *self {
                Input::Press(ref button) =>
                    s.emit_enum_variant("Press", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| button.encode(s))),
                Input::Release(ref button) =>
                    s.emit_enum_variant("Release", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| button.encode(s))),
                Input::Move(ref motion) =>
                    s.emit_enum_variant("Move", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| motion.encode(s))),
                Input::Text(ref text) =>
                    s.emit_enum_variant("Text", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| text.encode(s))),
                Input::Resize(w, h) =>
                    s.emit_enum_variant("Resize", 0, 2, |s| {
                        try!(s.emit_enum_variant_arg(0, |s| w.encode(s)));
                        try!(s.emit_enum_variant_arg(1, |s| h.encode(s)));
                        Ok(())
                    }),
                Input::Focus(focus) =>
                    s.emit_enum_variant("Focus", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| focus.encode(s))),
                Input::Cursor(cursor) =>
                    s.emit_enum_variant("Cursor", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| cursor.encode(s))),
                Input::Close =>
                    s.emit_enum_variant("Close", 0, 0, |_s| Ok(())),
                Input::Render(ref render_args) =>
                    s.emit_enum_variant("Render", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| render_args.encode(s))),
                Input::AfterRender(ref after_render_args) =>
                    s.emit_enum_variant("AfterRender", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| after_render_args.encode(s))),
                Input::Update(ref update_args) =>
                    s.emit_enum_variant("Update", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| update_args.encode(s))),
                Input::Idle(ref idle_args) =>
                    s.emit_enum_variant("Idle", 0, 1, |s|
                        s.emit_enum_variant_arg(0, |s| idle_args.encode(s))),
                Input::Custom(_, _) => Ok(()),
            }
        })
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

impl From<RenderArgs> for Input {
    fn from(args: RenderArgs) -> Self {
        Input::Render(args)
    }
}

impl From<AfterRenderArgs> for Input {
    fn from(args: AfterRenderArgs) -> Self {
        Input::AfterRender(args)
    }
}

impl From<UpdateArgs> for Input {
    fn from(args: UpdateArgs) -> Self {
        Input::Update(args)
    }
}

impl From<IdleArgs> for Input {
    fn from(args: IdleArgs) -> Self {
        Input::Idle(args)
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json;
    use super::*;

    #[test]
    fn test_encode_decode() {
        let test = |input| {
            let encoded = json::encode(&input).unwrap().to_string();
            let decoded: Input = json::decode(&encoded).unwrap();
            assert_eq!(decoded, input);
        };

        test(Input::Press(Button::Keyboard(Key::A)));
        test(Input::Release(Button::Keyboard(Key::A)));
        test(Input::Move(Motion::MouseCursor(0.0, 0.0)));
        test(Input::Text("hello".into()));
        test(Input::Resize(0, 0));
        test(Input::Focus(true));
        test(Input::Cursor(true));
        test(Input::Close);
        test(Input::Render(RenderArgs {
            width: 0,
            height: 0,
            draw_width: 0,
            draw_height: 0,
            ext_dt: 0.0,
        }));
        test(Input::AfterRender(AfterRenderArgs));
        test(Input::Update(UpdateArgs {dt: 0.0}));
        test(Input::Idle(IdleArgs {dt: 0.0}));
    }
}
