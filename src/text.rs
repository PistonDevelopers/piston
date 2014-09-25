use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use GenericEvent;

/// Implemented by event structures that support text event.
pub trait TextEvent {
    /// Creates a text event.
    fn from_text(text: &str) -> Option<Self>;
    /// Calls closure if this is a text event.
    fn text(&self, f: |&str|);
}

impl<T: GenericEvent> TextEvent for T {
    #[inline(always)]
    fn from_text(text: &str) -> Option<T> {
        let id = TypeId::of::<Box<TextEvent>>();
        GenericEvent::from_event(id, &text as &Any)
    }

    #[inline(always)]
    fn text(&self, f: |&str|) {
        let id = TypeId::of::<Box<TextEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<&str>() {
                Some(&text) => f(text),
                None => fail!("Expected `&str`")
            }
        });
    }
}
