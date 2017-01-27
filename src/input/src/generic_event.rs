//! Trait for generic events

use std::any::Any;

use {AfterRenderEvent, ControllerAxisEvent, CursorEvent, FocusEvent, IdleEvent,
     MouseCursorEvent, MouseRelativeEvent, MouseScrollEvent,
     PressEvent, ReleaseEvent, RenderEvent, ResizeEvent,
     TextEvent, TouchEvent, UpdateEvent};
use {EventId, Input, Motion};
use {AFTER_RENDER, CONTROLLER_AXIS, CURSOR, FOCUS, CLOSE, IDLE, MOUSE_CURSOR,
     MOUSE_RELATIVE, MOUSE_SCROLL, PRESS, RENDER, RELEASE, RESIZE,
     TEXT, TOUCH, UPDATE};

/// Implemented by all events
pub trait GenericEvent: Sized +
    AfterRenderEvent + ControllerAxisEvent + CursorEvent + FocusEvent + IdleEvent +
    MouseCursorEvent + MouseRelativeEvent + MouseScrollEvent +
    PressEvent + ReleaseEvent + RenderEvent + ResizeEvent +
    TextEvent + TouchEvent + UpdateEvent + From<Input> {
    /// The id of this event.
    fn event_id(&self) -> EventId;
    /// Calls closure with arguments
    fn with_args<'a, F, U>(&'a self, f: F) -> U
        where F: FnMut(&Any) -> U
    ;
}

impl GenericEvent for Input {
    fn event_id(&self) -> EventId {
        match self {
            &Input::Cursor(_) => CURSOR,
            &Input::Focus(_) => FOCUS,
            &Input::Close(_) => CLOSE,
            &Input::Move(Motion::MouseCursor(_, _)) => MOUSE_CURSOR,
            &Input::Move(Motion::MouseRelative(_, _)) => MOUSE_RELATIVE,
            &Input::Move(Motion::MouseScroll(_, _)) => MOUSE_SCROLL,
            &Input::Move(Motion::ControllerAxis(_)) => CONTROLLER_AXIS,
            &Input::Move(Motion::Touch(_)) => TOUCH,
            &Input::Press(_) => PRESS,
            &Input::Release(_) => RELEASE,
            &Input::Resize(_, _) => RESIZE,
            &Input::Text(_) => TEXT,
            &Input::Update(_) => UPDATE,
            &Input::Render(_) => RENDER,
            &Input::AfterRender(_) => AFTER_RENDER,
            &Input::Idle(_) => IDLE,
            &Input::Custom(event_id, _) => event_id,
        }
    }

    fn with_args<'a, F, U>(&'a self, mut f: F) -> U
        where F: FnMut(&Any) -> U
    {
        match self {
            &Input::Cursor(cursor) =>
                f(&cursor as &Any),
            &Input::Focus(focused) =>
                f(&focused as &Any),
            &Input::Close(ref args) =>
                f(args as &Any),
            &Input::Move(Motion::ControllerAxis(args)) =>
                f(&args as &Any),
            &Input::Move(Motion::MouseCursor(x, y)) =>
                f(&(x, y) as &Any),
            &Input::Move(Motion::MouseRelative(x, y)) =>
                f(&(x, y) as &Any),
            &Input::Move(Motion::MouseScroll(x, y)) =>
                f(&(x, y) as &Any),
            &Input::Move(Motion::Touch(args)) =>
                f(&args as &Any),
            &Input::Press(button) =>
                f(&button as &Any),
            &Input::Release(button) =>
                f(&button as &Any),
            &Input::Resize(w, h) =>
                f(&(w, h) as &Any),
            &Input::Text(ref text) =>
                f(text as &Any),
            &Input::Update(ref args) =>
                f(args as &Any),
            &Input::Render(ref args) =>
                f(args as &Any),
            &Input::AfterRender(ref args) =>
                f(args as &Any),
            &Input::Idle(ref args) =>
                f(args as &Any),
            &Input::Custom(_, ref args) =>
                f(args),
        }
    }
}
