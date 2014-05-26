
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
pub struct PressEvent<'a> {
    /// The key which was pressed.
    pub key: Field<'a, &'a KeyType>,
}

impl<'a> Clone for PressEvent<'a> {
    fn clone(&self) -> PressEvent<'a> {
        PressEvent {
            key: Value(*self.key.get()),
        }
    }
}

impl<'a> Triggered for PressEvent<'a> {
    fn get_observer(&self) -> Box<Observer> {
        box PressEventObserver::new(*self.key.get()) as Box<Observer>
    }
}

impl<'a> AddRelease<'a, ReleasePressEvent<'a>> for PressEvent<'a> {
    #[inline(always)]
    fn release(&'a self) -> ReleasePressEvent<'a> {
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

