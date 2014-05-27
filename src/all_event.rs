
use {
    EventType,
    Field,
    Observer,
    Triggered,
    Value,
};

/// A event context that can be triggered if ALL event in `events` happened.
pub struct AllEvent<'a, 'b> {
    /// A sequence of events.
    pub events: Field<'a, &'b [&'b Triggered]>,
}

impl<'a, 'b> Clone for AllEvent<'a, 'b> {
    fn clone(&self) -> AllEvent<'static, 'b> {
        AllEvent {
            events: Value(*self.events.get()),
        }
    }
}

impl<'a, 'b> Triggered for AllEvent<'a, 'b> {
    fn get_observer(&self) -> Box<Observer> {
        box AllEventObserver::new(*self.events.get()) as Box<Observer>
    }
}

struct AllEventObserver<'a> {
    observers: Vec<Box<Observer>>,
}

impl<'a> AllEventObserver<'a> {
    pub fn new(events: &'a [&'a Triggered]) -> AllEventObserver<'a> {
        let mut observers = Vec::<Box<Observer>>::new();
        for event in events.iter() {
            observers.push(event.get_observer());
        }
        AllEventObserver {
            observers: observers,
        }
    }
}

impl<'a> Observer for AllEventObserver<'a> {
    fn reset(&mut self) {
        for observer in self.observers.mut_iter() {
            observer.reset();
        }
    }

    fn can_trigger(&self) -> bool {
        for observer in self.observers.iter() {
            if !observer.can_trigger() {
                return false;
            }
        }
        true
    }

    fn after_trigger(&mut self) {
        for observer in self.observers.mut_iter() {
            observer.after_trigger();
        }
    }

    fn update(&mut self, dt: f64) {
        for observer in self.observers.mut_iter() {
            if !observer.can_trigger() {
                observer.update(dt);
            }
        }
    }

    fn on_event(&mut self, e: &EventType) {
        for observer in self.observers.mut_iter() {
            if !observer.can_trigger() {
                observer.on_event(e);
            }
        }
    }
}

