use std::intrinsics::TypeId;
use input::Button;

use GenericEvent;
use ptr::Ptr;

/// The release of a button
pub trait ReleaseEvent {
    /// Creates a release event.
    fn from_button(button: Button) -> Option<Self>;
    /// Calls closure if this is a release event.
    fn release<U>(&self, f: |Button| -> U) -> Option<U>;
}

impl<T: GenericEvent> ReleaseEvent for T {
    #[inline(always)]
    fn from_button(button: Button) -> Option<T> {
        let id = TypeId::of::<Box<ReleaseEvent>>();
        Ptr::with_ref::<Button, Option<T>>(&button, |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn release<U>(&self, f: |Button| -> U) -> Option<U> {
        let id = TypeId::of::<Box<ReleaseEvent>>();
        self.with_event(id, |ptr| {
            f(*ptr.expect::<Button>())
        })
    }
}
