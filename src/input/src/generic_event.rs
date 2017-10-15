//! Trait for generic events

use std::any::Any;

use {AfterRenderEvent, ButtonEvent, CloseEvent, ControllerAxisEvent, CursorEvent, FocusEvent,
     IdleEvent, MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent, PressEvent, ReleaseEvent,
     RenderEvent, ResizeEvent, TextEvent, TouchEvent, UpdateEvent};
use {Event, EventId, Input, Loop, Motion};
use {AFTER_RENDER, BUTTON, CONTROLLER_AXIS, CURSOR, FOCUS, CLOSE, IDLE, MOUSE_CURSOR, MOUSE_RELATIVE,
     MOUSE_SCROLL, RENDER, RESIZE, TEXT, TOUCH, UPDATE};

/// Implemented by all events
pub trait GenericEvent: Sized +
    AfterRenderEvent + CloseEvent + ControllerAxisEvent + CursorEvent + FocusEvent + IdleEvent +
    MouseCursorEvent + MouseRelativeEvent + MouseScrollEvent + ButtonEvent + PressEvent +
    ReleaseEvent + RenderEvent + ResizeEvent + TextEvent + TouchEvent + UpdateEvent +
    From<Input> + From<Loop> + Into<Option<Input>> + Into<Option<Loop>>
{
/// The id of this event.
    fn event_id(&self) -> EventId;
/// Calls closure with arguments
    fn with_args<'a, F, U>(&'a self, f: F) -> U
        where F: FnMut(&Any) -> U
;
}

impl GenericEvent for Event {
    fn event_id(&self) -> EventId {
        match *self {
            Event::Input(Input::Cursor(_)) => CURSOR,
            Event::Input(Input::Focus(_)) => FOCUS,
            Event::Input(Input::Close(_)) => CLOSE,
            Event::Input(Input::Move(Motion::MouseCursor(_, _))) => MOUSE_CURSOR,
            Event::Input(Input::Move(Motion::MouseRelative(_, _))) => MOUSE_RELATIVE,
            Event::Input(Input::Move(Motion::MouseScroll(_, _))) => MOUSE_SCROLL,
            Event::Input(Input::Move(Motion::ControllerAxis(_))) => CONTROLLER_AXIS,
            Event::Input(Input::Move(Motion::Touch(_))) => TOUCH,
            Event::Input(Input::Button(_)) => BUTTON,
            Event::Input(Input::Resize(_, _)) => RESIZE,
            Event::Input(Input::Text(_)) => TEXT,
            Event::Loop(Loop::Update(_)) => UPDATE,
            Event::Loop(Loop::Render(_)) => RENDER,
            Event::Loop(Loop::AfterRender(_)) => AFTER_RENDER,
            Event::Loop(Loop::Idle(_)) => IDLE,
            Event::Custom(event_id, _) => event_id,
        }
    }

    fn with_args<'a, F, U>(&'a self, mut f: F) -> U
        where F: FnMut(&Any) -> U
    {
        match *self {
            Event::Input(Input::Cursor(cursor)) => f(&cursor as &Any),
            Event::Input(Input::Focus(focused)) => f(&focused as &Any),
            Event::Input(Input::Close(ref args)) => f(args as &Any),
            Event::Input(Input::Move(Motion::ControllerAxis(args))) => f(&args as &Any),
            Event::Input(Input::Move(Motion::MouseCursor(x, y))) => f(&(x, y) as &Any),
            Event::Input(Input::Move(Motion::MouseRelative(x, y))) => f(&(x, y) as &Any),
            Event::Input(Input::Move(Motion::MouseScroll(x, y))) => f(&(x, y) as &Any),
            Event::Input(Input::Move(Motion::Touch(args))) => f(&args as &Any),
            Event::Input(Input::Button(ref args)) => f(args as &Any),
            Event::Input(Input::Resize(w, h)) => f(&(w, h) as &Any),
            Event::Input(Input::Text(ref text)) => f(text as &Any),
            Event::Loop(Loop::Update(ref args)) => f(args as &Any),
            Event::Loop(Loop::Render(ref args)) => f(args as &Any),
            Event::Loop(Loop::AfterRender(ref args)) => f(args as &Any),
            Event::Loop(Loop::Idle(ref args)) => f(args as &Any),
            Event::Custom(_, ref args) => f(args),
        }
    }
}
