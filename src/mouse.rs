use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use GenericEvent;

/// Implemented by event structures that support mouse cursor event.
pub trait MouseCursorEvent {
    /// Creates a mouse cursor event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse cursor event.
    fn mouse_cursor(&self, f: |f64, f64|);
}

impl<T: GenericEvent> MouseCursorEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseCursorEvent>>();
        GenericEvent::from_event(id, &(x, y) as &Any)
    }

    #[inline(always)]
    fn mouse_cursor(&self, f: |f64, f64|) {
        let id = TypeId::of::<Box<MouseCursorEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<(f64, f64)>() {
                Some(&(x, y)) => f(x, y),
                None => fail!("Expected `(f64, f64)`")
            }
        });
    }
}

/// Implemented by event structures that support mouse relative event.
pub trait MouseRelativeEvent {
    /// Creates a mouse relative event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse relative event.
    fn mouse_relative(&self, f: |f64, f64|);
}

impl<T: GenericEvent> MouseRelativeEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseRelativeEvent>>();
        GenericEvent::from_event(id, &(x, y) as &Any)
    }

    #[inline(always)]
    fn mouse_relative(&self, f: |f64, f64|) {
        let id = TypeId::of::<Box<MouseRelativeEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<(f64, f64)>() {
                Some(&(x, y)) => f(x, y),
                None => fail!("Expected `(f64, f64)`")
            }
        });
    }
}

/// Implemented by event structures that support mouse scroll event.
pub trait MouseScrollEvent {
    /// Creates a mouse scroll event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls a closure if this is a mouse scroll event.
    fn mouse_scroll(&self, f: |f64, f64|);
}

impl<T: GenericEvent> MouseScrollEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseScrollEvent>>();
        GenericEvent::from_event(id, &(x, y) as &Any)
    }

    #[inline(always)]
    fn mouse_scroll(&self, f: |f64, f64|) {
        let id = TypeId::of::<Box<MouseScrollEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<(f64, f64)>() {
                Some(&(x, y)) => f(x, y),
                None => fail!("Expected `(f64, f64)`")
            }
        });
    }
}
