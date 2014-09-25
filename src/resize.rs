use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use GenericEvent;

/// Implemented by event structures that support resize event.
pub trait ResizeEvent {
    /// Creates a resize event.
    fn from_width_height(w: u32, h: u32) -> Option<Self>;
    /// Calls closure if this is a resize event.
    fn resize(&self, f: |u32, u32|);
}

impl<T: GenericEvent> ResizeEvent for T {
    #[inline(always)]
    fn from_width_height(w: u32, h: u32) -> Option<T> {
        let id = TypeId::of::<Box<ResizeEvent>>();
        GenericEvent::from_event(id, &(w, h) as &Any)
    }

    #[inline(always)]
    fn resize(&self, f: |u32, u32|) {
        let id = TypeId::of::<Box<ResizeEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<(u32, u32)>() {
                Some(&(w, h)) => f(w, h),
                None => fail!("Expected `(u32, u32)`")
            }
        });
    }
}
