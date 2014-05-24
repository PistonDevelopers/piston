
use {
    EventType,
    Field,
    KeyType,
    Observer,
    Triggered,
};

pub struct ReleasePressEvent<'a> {
    pub key: Field<'a, &'a KeyType>,
}

impl<'a> Triggered<'a> for ReleasePressEvent<'a> {
    fn get_observer(&'a self) -> Box<Observer> {
        (box ReleasePressEventObserver::new(*self.key.get())) as Box<Observer>
    }
}

struct ReleasePressEventObserver<'a> {
    key: &'a KeyType,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> ReleasePressEventObserver<'a> {
    pub fn new(key: &'a KeyType) -> ReleasePressEventObserver<'a>{
        ReleasePressEventObserver {
            key: key,
            can_trigger: false,
            is_pressed: false,
        }
    }
}

impl<'a> Observer for ReleasePressEventObserver<'a> {
    fn can_trigger(&self) -> bool {
        self.can_trigger
    }

    fn after_trigger(&mut self) {
        self.can_trigger = false
    }

    fn on_event(&mut self, e: &EventType) {
        if e.is_press_key(self.key) {
            self.is_pressed = true;
        } else if e.is_release_key(self.key) {
            if self.is_pressed {
                self.can_trigger = true;
            }
            self.is_pressed = false;
        }
    }
}

