use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// Implemented by event structures that support mouse cursor event.
pub trait MouseCursorEvent {
    /// Creates a mouse cursor event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse cursor event.
    fn mouse_cursor<U>(&self, f: |f64, f64| -> U) -> Option<U>;
}

impl<T: GenericEvent> MouseCursorEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseCursorEvent>>();
        Ptr::with_ref::<(f64, f64), Option<T>>(&(x, y), |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn mouse_cursor<U>(&self, f: |f64, f64| -> U) -> Option<U> {
        let id = TypeId::of::<Box<MouseCursorEvent>>();
        self.with_event(id, |ptr| {
            let &(x, y) = ptr.expect::<(f64, f64)>();
            f(x, y)
        })
    }
}

/// Implemented by event structures that support mouse relative event.
pub trait MouseRelativeEvent {
    /// Creates a mouse relative event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse relative event.
    fn mouse_relative<U>(&self, f: |f64, f64| -> U) -> Option<U>;
}

impl<T: GenericEvent> MouseRelativeEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseRelativeEvent>>();
        Ptr::with_ref::<(f64, f64), Option<T>>(&(x, y), |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn mouse_relative<U>(&self, f: |f64, f64| -> U) -> Option<U> {
        let id = TypeId::of::<Box<MouseRelativeEvent>>();
        self.with_event(id, |ptr| {
            let &(x, y) = ptr.expect::<(f64, f64)>();
            f(x, y)
        })
    }
}

/// Implemented by event structures that support mouse scroll event.
pub trait MouseScrollEvent {
    /// Creates a mouse scroll event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls a closure if this is a mouse scroll event.
    fn mouse_scroll<U>(&self, f: |f64, f64| -> U) -> Option<U>;
}

impl<T: GenericEvent> MouseScrollEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseScrollEvent>>();
        Ptr::with_ref::<(f64, f64), Option<T>>(&(x, y), |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn mouse_scroll<U>(&self, f: |f64, f64| -> U) -> Option<U> {
        let id = TypeId::of::<Box<MouseScrollEvent>>();
        self.with_event(id, |ptr| {
            let &(x, y) = ptr.expect::<(f64, f64)>();
            f(x, y)
        })
    }
}
