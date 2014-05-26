
use {
    EventType,
    Field,
    Observer,
    Triggered,
    Value,
};

/// A event context that can be triggered if ANY event in `events` happened.
pub struct AnyEvent<'a, 'b> {
    /// A sequence of events.
    pub events: Field<'a, &'b [&'b Triggered]>,
}

impl<'a, 'b> Clone for AnyEvent<'a, 'b> {
    fn clone(&self) -> AnyEvent<'static, 'b> {
        AnyEvent {
            events: Value(*self.events.get()),
        }
    }
}

impl<'a, 'b> Triggered for AnyEvent<'a, 'b> {
    fn get_observer(&self) -> Box<Observer> {
        box AnyEventObserver::new(*self.events.get()) as Box<Observer>
    }
}

struct AnyEventObserver<'a> {
    observers: Vec<Box<Observer>>,
}

impl<'a> AnyEventObserver<'a> {
    pub fn new(events: &'a [&'a Triggered]) -> AnyEventObserver<'a> {
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

