
use std::cell::RefCell;
use std::rc::Rc;
use {
    AfterEvent,
    Triggered,
};

pub trait AddAfter<'a> {
    fn after(&'a self, event: &'a Triggered) -> AfterEvent;
}

impl<'a, E: Triggered> AddAfter<'a> for E {
    // Because we can't make `self` as `Value`, so use Rc
    fn after(&'a self, event: &'a Triggered) -> AfterEvent {
        AfterEvent {
            before: Rc::new(RefCell::new(event.get_observer())),
            after: Rc::new(RefCell::new(self.get_observer())),
        }
    }
}

