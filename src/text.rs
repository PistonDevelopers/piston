use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// Implemented by event structures that support text event.
pub trait TextEvent {
    /// Creates a text event.
    fn from_text(text: &str) -> Option<Self>;
    /// Calls closure if this is a text event.
    fn text<U>(&self, f: |&str| -> U) -> Option<U>;
}

impl<T: GenericEvent> TextEvent for T {
    #[inline(always)]
    fn from_text(text: &str) -> Option<T> {
        let id = TypeId::of::<Box<TextEvent>>();
        Ptr::with_str::<Option<T>>(text, |ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn text<U>(&self, f: |&str| -> U) -> Option<U> {
        let id = TypeId::of::<Box<TextEvent>>();
        self.with_event(id, |ptr| {
            f(ptr.expect_str())
        })
    }
}
