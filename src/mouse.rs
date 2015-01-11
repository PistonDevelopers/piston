use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// The position of the mouse cursor
pub trait MouseCursorEvent {
    /// Creates a mouse cursor event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse cursor event.
    fn mouse_cursor<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse cursor arguments.
    fn mouse_cursor_args(&self) -> Option<[f64; 2]> {
        self.mouse_cursor(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseCursorEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseCursorEvent>>();
        Ptr::with_ref::<(f64, f64), Option<T>, _>(&(x, y), |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn mouse_cursor<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        let id = TypeId::of::<Box<MouseCursorEvent>>();
        self.with_event(id, |&mut: ptr| {
            let &(x, y) = ptr.expect::<(f64, f64)>();
            f(x, y)
        })
    }
}

/// The relative movement of mouse cursor
pub trait MouseRelativeEvent {
    /// Creates a mouse relative event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a mouse relative event.
    fn mouse_relative<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse relative arguments.
    fn mouse_relative_args(&self) -> Option<[f64; 2]> {
        self.mouse_relative(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseRelativeEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseRelativeEvent>>();
        Ptr::with_ref::<(f64, f64), Option<T>, _>(&(x, y), |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn mouse_relative<U, F>(&self, mut f: F) -> Option<U>
        where
            F: FnMut(f64, f64) -> U
    {
        let id = TypeId::of::<Box<MouseRelativeEvent>>();
        self.with_event(id, |&mut: ptr| {
            let &(x, y) = ptr.expect::<(f64, f64)>();
            f(x, y)
        })
    }
}

/// The scroll of the mouse wheel
pub trait MouseScrollEvent {
    /// Creates a mouse scroll event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls a closure if this is a mouse scroll event.
    fn mouse_scroll<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U;
    /// Returns mouse scroll arguments.
    fn mouse_scroll_args(&self) -> Option<[f64; 2]> {
        self.mouse_scroll(|x, y| [x, y])
    }
}

impl<T: GenericEvent> MouseScrollEvent for T {
    #[inline(always)]
    fn from_xy(x: f64, y: f64) -> Option<T> {
        let id = TypeId::of::<Box<MouseScrollEvent>>();
        Ptr::with_ref::<(f64, f64), Option<T>, _>(&(x, y), |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn mouse_scroll<U, F>(&self, mut f: F) -> Option<U>
        where
            F: FnMut(f64, f64) -> U
    {
        let id = TypeId::of::<Box<MouseScrollEvent>>();
        self.with_event(id, |&mut: ptr| {
            let &(x, y) = ptr.expect::<(f64, f64)>();
            f(x, y)
        })
    }
}
