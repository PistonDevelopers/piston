
use {
    AddRelease,
    Borrowed,
    Call,
    EventCenter,
    EventType,
    Field,
    KeyType,
    Observer,
    ReleasePressEvent,
};

pub struct PressEvent<'a> {
    pub key: Field<'a, &'a KeyType>,
}

impl<'a> AddRelease<'a, ReleasePressEvent<'a>> for PressEvent<'a> {
    #[inline(always)]
    fn release(&'a self) -> ReleasePressEvent<'a> {
        ReleasePressEvent {
            key: Borrowed(self.key.get())
        }
    }
}

impl<'a> Call for PressEvent<'a> {
    fn call(&self, ec: &mut EventCenter, command: ||: 'static) -> uint {
        ec.add_observer(box PressEventObserver::new(*self.key.get(), command))
    }
}

struct PressEventObserver<'a> {
    command: ||: 'static,
    key: &'a KeyType,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> PressEventObserver<'a> {
    pub fn new(key: &'a KeyType, command: ||: 'static) -> PressEventObserver<'a> {
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

