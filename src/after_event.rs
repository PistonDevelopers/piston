
use {
    EventType,
    Field,
    Observer,
    Triggered,
    Value,
};

pub struct AfterEvent<'a, 'b> {
    pub before: Field<'a, &'b Triggered>,
    pub after: Field<'a, &'b Triggered>,
}

impl<'a, 'b> Clone for AfterEvent<'a, 'b> {
    fn clone(&self) -> AfterEvent<'static, 'b> {
        AfterEvent {
            before: Value(*self.before.get()),
            after: Value(*self.after.get()),
        }
    }
}

impl<'a, 'b> Triggered for AfterEvent<'a, 'b> {
    fn get_observer(&self) -> Box<Observer> {
        box AfterEventObserver::new(*self.before.get(), *self.after.get()) as Box<Observer>
    }
}

struct AfterEventObserver<'a> {
    before: Box<Observer>,
    after: Box<Observer>,
}

impl<'a> AfterEventObserver<'a> {
    pub fn new(before: &Triggered, after: &Triggered) -> AfterEventObserver<'a> {
        AfterEventObserver {
            before: before.get_observer(),
            after: after.get_observer(),
        }
    }
}

impl<'a> Observer for AfterEventObserver<'a> {
    fn reset(&mut self) {
        self.before.reset();
        self.after.reset();
    }

    fn can_trigger(&self) -> bool {
        self.after.can_trigger()
    }

    fn after_trigger(&mut self) {
        self.after.after_trigger();
        self.before.after_trigger();
    }

    fn update(&mut self, dt: f64) {
        if self.before.can_trigger() {
            self.after.update(dt);
        } else {
            self.before.update(dt);
        }
    }

    fn on_event(&mut self, e: &EventType) {
        if self.before.can_trigger() {
            self.after.on_event(e);
        } else {
            self.before.on_event(e);
        }
    }
}

