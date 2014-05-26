
use {
    EventType,
    Field,
    Observer,
    Triggered,
    Value,
};

/// A event context that can be triggered if ANY event in `events` happened.
pub struct AnyEvent<'a> {
    /// A sequence of events.
    pub events: Field<'a, &'a [&'a Triggered<'a>]>,
}

impl<'a> Clone for AnyEvent<'a> {
    fn clone(&self) -> AnyEvent<'a> {
        AnyEvent {
            events: Value(*self.events.get()),
        }
    }
}

impl<'a> Triggered<'a> for AnyEvent<'a> {
    fn get_observer(&self) -> Box<Observer> {
        box AnyEventObserver::new(*self.events.get()) as Box<Observer>
    }
}

struct AnyEventObserver<'a> {
    observers: Vec<Box<Observer>>,
}

impl<'a> AnyEventObserver<'a> {
    pub fn new(events: &'a [&'a Triggered<'a>]) -> AnyEventObserver<'a> {
        let mut observers = Vec::<Box<Observer>>::new();
        for event in events.iter() {
            observers.push(event.get_observer());
        }
        AnyEventObserver {
            observers: observers,
        }
    }
}

impl<'a> Observer for AnyEventObserver<'a> {
    fn reset(&mut self) {
        for observer in self.observers.mut_iter() {
            observer.reset();
        }
    }

    fn can_trigger(&self) -> bool {
        for observer in self.observers.iter() {
            if observer.can_trigger() {
                return true;
            }
        }
        false
    }

    fn after_trigger(&mut self) {
        for observer in self.observers.mut_iter() {
            if observer.can_trigger() {
                observer.after_trigger();
                break;
            }
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

