
use std::cell::RefCell;
use std::rc::Rc;
use {
    AfterEvent,
    Triggered,
};

/// Implemented by all contexts that can triggered after another event happened.
pub trait AddAfter<'a> {
    /// Triggered after `event` happened.
    fn after(&'a self, event: &'a Triggered) -> AfterEvent;
}

impl<'a, E: Triggered> AddAfter<'a> for E {
    // Because we can't make `self` as `Value`, so we use Rc.
    fn after(&'a self, event: &'a Triggered) -> AfterEvent {
        AfterEvent {
            before: Rc::new(RefCell::new(event.get_observer())),
            after: Rc::new(RefCell::new(self.get_observer())),
        }
    }
}

