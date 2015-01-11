use std::intrinsics::TypeId;

use GenericEvent;
use ptr::Ptr;

/// When window gets or looses focus
pub trait FocusEvent {
    /// Creates a focus event.
    fn from_focused(focused: bool) -> Option<Self>;
    /// Calls closure if this is a focus event.
    fn focus<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(bool) -> U;
    /// Returns focus arguments.
    fn focus_args(&self) -> Option<bool> {
        self.focus(|val| val)
    }
}

impl<T: GenericEvent> FocusEvent for T {
    #[inline(always)]
    fn from_focused(focused: bool) -> Option<T> {
        let id = TypeId::of::<Box<FocusEvent>>();
        Ptr::with_ref::<bool, Option<T>, _>(&focused, |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn focus<U, F>(&self, mut f: F) -> Option<U>
        where
            F: FnMut(bool) -> U
    {
        let id = TypeId::of::<Box<FocusEvent>>();
        self.with_event(id, |&mut: ptr| {
            f(*ptr.expect::<bool>())
        })
    }
}
