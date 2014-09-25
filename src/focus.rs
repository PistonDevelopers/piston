use std::intrinsics::TypeId;
use std::any::{ Any, AnyRefExt };

use GenericEvent;

/// Implemented by event structures that support focus event.
pub trait FocusEvent {
    /// Creates a focus event.
    fn from_focused(focused: bool) -> Option<Self>;
    /// Calls closure if this is a focus event.
    fn focus(&self, f: |bool|);
}

impl<T: GenericEvent> FocusEvent for T {
    #[inline(always)]
    fn from_focused(focused: bool) -> Option<T> {
        let id = TypeId::of::<Box<FocusEvent>>();
        GenericEvent::from_event(id, &focused as &Any)
    }

    #[inline(always)]
    fn focus(&self, f: |bool|) {
        let id = TypeId::of::<Box<FocusEvent>>();
        self.with_event(id, |any| {
            match any.downcast_ref::<bool>() {
                Some(&focused) => f(focused),
                None => fail!("Expected `bool`")
            }
        });
    }
}
