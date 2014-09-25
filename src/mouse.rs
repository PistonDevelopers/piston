use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use GenericEvent;

/// Implemented by event structures that support mouse cursor event.
pub trait MouseCursorEvent {
    /// Creates a release event.
    fn from_xy(x: f64, y: f64) -> Option<Self>;
    /// Calls closure if this is a release event.
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
