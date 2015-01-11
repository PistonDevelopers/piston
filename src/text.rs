use std::intrinsics::TypeId;
use std::borrow::ToOwned;

use GenericEvent;
use ptr::Ptr;

/// When receiving text from user, such as typing a character
pub trait TextEvent {
    /// Creates a text event.
    fn from_text(text: &str) -> Option<Self>;
    /// Calls closure if this is a text event.
    fn text<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&str) -> U;
    /// Returns text arguments.
    fn text_args(&self) -> Option<String> {
        self.text(|text| text.to_owned())
    }
}

impl<T: GenericEvent> TextEvent for T {
    #[inline(always)]
    fn from_text(text: &str) -> Option<T> {
        let id = TypeId::of::<Box<TextEvent>>();
        Ptr::with_str::<Option<T>, _>(text, |: ptr| {
            GenericEvent::from_event(id, ptr)
        })
    }

    #[inline(always)]
    fn text<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&str) -> U
    {
        let id = TypeId::of::<Box<TextEvent>>();
        self.with_event(id, |&mut: ptr| {
            f(ptr.expect_str())
        })
    }
}
