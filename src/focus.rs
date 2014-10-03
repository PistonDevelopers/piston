use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// Implemented by event structures that support focus event.
pub trait FocusEvent {
    /// Creates a focus event.
    fn from_focused(focused: bool) -> Option<Self>;
    /// Calls closure if this is a focus event.
    fn focus<U>(&self, f: |bool| -> U) -> Option<U>;
}

impl<T: GenericEvent> FocusEvent for T {
    #[inline(always)]
    fn from_focused(focused: bool) -> Option<T> {
        let id = TypeId::of::<Box<FocusEvent>>();
        Ptr::with_ref::<bool, Option<T>>(&focused, |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn focus<U>(&self, f: |bool| -> U) -> Option<U> {
        let id = TypeId::of::<Box<FocusEvent>>();
        self.with_event(id, |ptr| {
            f(*ptr.expect::<bool>())
        })
    }
}
