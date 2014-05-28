
use std::cell::RefCell;
use std::rc::Rc;
use {
    EventType,
    Observer,
    Triggered,
};

/// A event context that can be triggered after `before` happened.
pub struct AfterEvent {
    /// Previous event
    pub before: Rc<RefCell<Box<Observer>>>,
    /// Next event
    pub after: Rc<RefCell<Box<Observer>>>,
}

impl Clone for AfterEvent {
    fn clone(&self) -> AfterEvent {
        AfterEvent {
            before: self.before.clone(),
            after: self.after.clone(),
        }
    }
}

impl Triggered for AfterEvent {
    fn get_observer(&self) -> Box<Observer> {
        box AfterEventObserver::new(self.before.clone(), self.after.clone()) as Box<Observer>
    }
}

struct AfterEventObserver {
    before: Rc<RefCell<Box<Observer>>>,
    after: Rc<RefCell<Box<Observer>>>,
}

impl AfterEventObserver {
    pub fn new(before: Rc<RefCell<Box<Observer>>>, after: Rc<RefCell<Box<Observer>>>) -> AfterEventObserver {
        AfterEventObserver {
            before: before,
            after: after,
        }
    }
}

impl Observer for AfterEventObserver {
    fn reset(&mut self) {
        self.before.borrow_mut().reset();
        self.after.borrow_mut().reset();
    }

    fn can_trigger(&self) -> bool {
        self.after.borrow().can_trigger()
    }

    fn after_trigger(&mut self) {
        self.after.borrow_mut().after_trigger();
        self.before.borrow_mut().after_trigger();
    }

    fn update(&mut self, dt: f64) {
        let mut can_trigger;
        {
            can_trigger = self.before.borrow().can_trigger();
        }
        if can_trigger {
            self.after.borrow_mut().update(dt);
        } else {
            self.before.borrow_mut().update(dt);
        }
    }

    fn on_event(&mut self, e: &EventType) {
        let mut can_trigger;
        {
            can_trigger = self.before.borrow().can_trigger();
        }
        if can_trigger {
            self.after.borrow_mut().on_event(e);
        } else {
            self.before.borrow_mut().on_event(e);
        }
    }
}

