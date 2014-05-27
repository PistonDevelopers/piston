
use {
    AddRelease,
    Borrowed,
    EventType,
    Field,
    KeyType,
    Observer,
    ReleasePressEvent,
    Triggered,
    Value,
};

/// A event context that can be triggered after certain key was pressed.
pub struct PressEvent<'a, 'b> {
    /// The key which was pressed.
    pub key: Field<'a, &'b KeyType>,
}

impl<'a, 'b> Clone for PressEvent<'a, 'b> {
    fn clone(&self) -> PressEvent<'static, 'b> {
        PressEvent {
            key: Value(*self.key.get()),
        }
    }
}

impl<'a, 'b> Triggered for PressEvent<'a, 'b> {
    fn get_observer(&self) -> Box<Observer> {
        box PressEventObserver::new(*self.key.get()) as Box<Observer>
    }
}

impl<'a, 'b> AddRelease<'a, ReleasePressEvent<'a, 'b>> for PressEvent<'a, 'b> {
    #[inline(always)]
    fn release(&'a self) -> ReleasePressEvent<'a, 'b> {
        ReleasePressEvent {
            key: Borrowed(self.key.get())
        }
    }
}

struct PressEventObserver<'a> {
    key: &'a KeyType,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> PressEventObserver<'a> {
    pub fn new(key: &'a KeyType) -> PressEventObserver<'a> {
        PressEventObserver {
            key: key,
            can_trigger: false,
            is_pressed: false,
        }
    }
}

impl<'a> Observer for PressEventObserver<'a> {
    fn reset(&mut self) {
        self.is_pressed = false;
        self.can_trigger = false;
    }

    fn can_trigger(&self) -> bool {
        self.can_trigger
    }

    fn after_trigger(&mut self) {
        self.is_pressed = false;
        self.can_trigger = false;
    }

    fn on_event(&mut self, e: &EventType) {
        if e.is_press_key(self.key) {
            if !self.is_pressed {
                self.is_pressed = true;
                self.can_trigger = true;
            }
        } else if e.is_release_key(self.key) {
            self.is_pressed = false;
        }
    }
}

