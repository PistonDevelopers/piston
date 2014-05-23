
use {
    Call,
    KeyType,
    Field,
    Observer,
    EventCenter,
    EventType,
};

pub struct PressEvent<'a> {
    pub key: Field<'a, &'a KeyType>,
}

impl<'a> Call<'a> for PressEvent<'a> {
    fn call<'a>(&self, ec: &mut EventCenter, command: ||: 'a) -> uint {
        ec.add_observer(box PressEventObserver::new(*self.key.get(), command))
    }
}

struct PressEventObserver<'a> {
    command: ||: 'a,
    key: &'a KeyType,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> PressEventObserver<'a> {
    pub fn new(key: &'a KeyType, command: ||: 'a) -> PressEventObserver<'a> {
        PressEventObserver {
            command: command,
            key: key,
            can_trigger: false,
            is_pressed: false,
        }
    }
}

impl<'a> Observer for PressEventObserver<'a> {
    fn can_trigger(&self) -> bool {
        self.can_trigger
    }

    fn trigger(&mut self) {
        (self.command)();
        self.can_trigger = false
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

