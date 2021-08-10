//! Trait for generic events

use std::any::Any;

use crate::{
    AfterRenderEvent, ButtonEvent, CloseEvent, ControllerAxisEvent, CursorEvent, Event, EventId,
    FocusEvent, IdleEvent, Input, Loop, Motion, MouseCursorEvent, MouseRelativeEvent,
    MouseScrollEvent, PressEvent, ReleaseEvent, RenderEvent, ResizeEvent, TextEvent, TimeStamp,
    TouchEvent, UpdateEvent,
};

/// Implemented by all events.
///
/// Use this trait when you need to handle events, e.g. `fn event(&mut self, e: &impl GenericEvent)`.
/// Events are usually handles by controllers (in the Model-View-Controller programming pattern).
/// There is no requirement that you need to implement some trait for controllers,
/// just that the standard convention for handling events is through a `event` method.
/// For more information about Model-View-Controller, see [Wikipedia article](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller).
///
/// This trait makes it possible to auto impl new events for all types that implements `GenericEvent`.
/// This way, you can define your own event types without breaking compatibility with Piston.
pub trait GenericEvent:
    Sized
    + AfterRenderEvent
    + CloseEvent
    + ControllerAxisEvent
    + CursorEvent
    + FocusEvent
    + IdleEvent
    + MouseCursorEvent
    + MouseRelativeEvent
    + MouseScrollEvent
    + ButtonEvent
    + PressEvent
    + ReleaseEvent
    + RenderEvent
    + ResizeEvent
    + TextEvent
    + TouchEvent
    + UpdateEvent
    + From<Input>
    + From<Loop>
    + Into<Option<Input>>
    + Into<Option<Loop>>
{
    /// The id of this event.
    fn event_id(&self) -> EventId;
    /// Calls closure with arguments
    fn with_args<F, U>(&'_ self, f: F) -> U
    where
        F: FnMut(&dyn Any) -> U;
    /// Gets the time stamp of this event.
    ///
    /// Measured in milliseconds since initialization of window.
    fn time_stamp(&self) -> Option<TimeStamp>;
}

impl GenericEvent for Event {
    fn event_id(&self) -> EventId {
        use crate::event_id::*;

        match *self {
            Event::Input(Input::Cursor(_), _) => CURSOR,
            Event::Input(Input::Focus(_), _) => FOCUS,
            Event::Input(Input::Close(_), _) => CLOSE,
            Event::Input(Input::Move(Motion::MouseCursor(_)), _) => MOUSE_CURSOR,
            Event::Input(Input::Move(Motion::MouseRelative(_)), _) => MOUSE_RELATIVE,
            Event::Input(Input::Move(Motion::MouseScroll(_)), _) => MOUSE_SCROLL,
            Event::Input(Input::Move(Motion::ControllerAxis(_)), _) => CONTROLLER_AXIS,
            Event::Input(Input::Move(Motion::Touch(_)), _) => TOUCH,
            Event::Input(Input::Button(_), _) => BUTTON,
            Event::Input(Input::Resize(_), _) => RESIZE,
            Event::Input(Input::Text(_), _) => TEXT,
            Event::Input(Input::FileDrag(_), _) => FILE_DRAG,
            Event::Loop(Loop::Update(_)) => UPDATE,
            Event::Loop(Loop::Render(_)) => RENDER,
            Event::Loop(Loop::AfterRender(_)) => AFTER_RENDER,
            Event::Loop(Loop::Idle(_)) => IDLE,
            Event::Custom(event_id, _, _) => event_id,
        }
    }

    fn with_args<F, U>(&'_ self, mut f: F) -> U
    where
        F: FnMut(&dyn Any) -> U,
    {
        match *self {
            Event::Input(Input::Cursor(cursor), _) => f(&cursor as &dyn Any),
            Event::Input(Input::Focus(focused), _) => f(&focused as &dyn Any),
            Event::Input(Input::Close(ref args), _) => f(args as &dyn Any),
            Event::Input(Input::Move(Motion::ControllerAxis(args)), _) => f(&args as &dyn Any),
            Event::Input(Input::Move(Motion::MouseCursor(pos)), _) => f(&pos as &dyn Any),
            Event::Input(Input::Move(Motion::MouseRelative(pos)), _) => f(&pos as &dyn Any),
            Event::Input(Input::Move(Motion::MouseScroll(pos)), _) => f(&pos as &dyn Any),
            Event::Input(Input::Move(Motion::Touch(args)), _) => f(&args as &dyn Any),
            Event::Input(Input::Button(ref args), _) => f(args as &dyn Any),
            Event::Input(Input::Resize(ref args), _) => f(args as &dyn Any),
            Event::Input(Input::Text(ref text), _) => f(text as &dyn Any),
            Event::Input(Input::FileDrag(ref file_drag), _) => f(file_drag as &dyn Any),
            Event::Loop(Loop::Update(ref args)) => f(args as &dyn Any),
            Event::Loop(Loop::Render(ref args)) => f(args as &dyn Any),
            Event::Loop(Loop::AfterRender(ref args)) => f(args as &dyn Any),
            Event::Loop(Loop::Idle(ref args)) => f(args as &dyn Any),
            Event::Custom(_, ref args, _) => f(args),
        }
    }

    fn time_stamp(&self) -> Option<TimeStamp> {
        match self {
            Event::Input(_, x) => *x,
            Event::Loop(_) => None,
            Event::Custom(_, _, x) => *x,
        }
    }
}
