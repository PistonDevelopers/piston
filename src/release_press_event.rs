
use {
    Call,
    EventCenter,
    EventType,
    Field,
    KeyType,
    Observer,
};

pub struct ReleasePressEvent<'a> {
    pub key: Field<'a, &'a KeyType>,
}

impl<'a> Call<'a> for ReleasePressEvent<'a> {
    fn call(&'a self, ec: &mut EventCenter, command: ||: 'static) -> uint {
        ec.add_observer(box ReleasePressEventObserver::new(*self.key.get(), command))
    }
}

struct ReleasePressEventObserver<'a> {
    command: ||: 'static,
    key: &'a KeyType,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> ReleasePressEventObserver<'a> {
    pub fn new(key: &'a KeyType, command: ||: 'static) -> ReleasePressEventObserver<'a>{
        ReleasePressEventObserver {
            command: command,
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

    fn trigger(&mut self) {
        (self.command)();
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

