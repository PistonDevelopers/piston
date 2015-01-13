use input::Input;

use {
    Event,
    FocusEvent,
    MouseCursorEvent,
    MouseRelativeEvent,
    MouseScrollEvent,
    PressEvent,
    ReleaseEvent,
    RenderEvent,
    ResizeEvent,
    TextEvent,
    UpdateEvent,
};

/// A collection of most common events.
pub trait GenericEvent:
    FocusEvent
  + MouseCursorEvent
  + MouseRelativeEvent
  + MouseScrollEvent
  + PressEvent
  + ReleaseEvent
  + ResizeEvent
  + TextEvent
  + RenderEvent
  + UpdateEvent
{}

impl GenericEvent for Input {}

impl GenericEvent for Event {}
