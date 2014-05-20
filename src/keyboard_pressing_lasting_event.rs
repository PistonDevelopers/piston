
use graphics::Field;
use piston::keyboard;
use piston::event::{
    Event,
    KeyPressed,
    KeyReleased,
};
use {
    BackEnd,
    Map,
    Observer,
};

/// A event context which used to observe whether a certain keyboard key is being pressed and lasting certain time.
pub struct KeyboardPressingLastingEvent<'a> {
    /// The key to be observed.
    pub key: Field<'a, keyboard::Key>,
    /// The time to lasting in seconds.
    pub lasting: Field<'a, f64>,
}

impl<'a> Map<'a> for KeyboardPressingLastingEvent<'a> {
    #[inline(always)]
    fn map<'a, B: BackEnd>(&self, back_end: &mut B, command: ||: 'a) -> uint {
        back_end.add_observer(box KeyboardPressingLastingEventObserver::new(command, *self.key.get(), *self.lasting.get()))
    }
}

struct KeyboardPressingLastingEventObserver<'a> {
    command: ||: 'a,
    key: keyboard::Key,
    is_pressing: bool,
    cur_time: f64,
    lasting_time: f64,
}

impl<'a> KeyboardPressingLastingEventObserver<'a> {
    pub fn new<'a>(command: ||: 'a, key: keyboard::Key, lasting: f64) -> KeyboardPressingLastingEventObserver<'a> {
        KeyboardPressingLastingEventObserver {
            command: command,
            key: key,
            is_pressing: false,
            cur_time: 0.0,
            lasting_time: lasting,
        }
    }
}

impl<'a> Observer for KeyboardPressingLastingEventObserver<'a> {
    fn can_trigger(&self) -> bool {
        self.is_pressing && self.cur_time > self.lasting_time
    }

    fn trigger(&mut self) {
        (self.command)();
    }

    fn update(&mut self, dt: f64) {
        if self.is_pressing {
            self.cur_time += dt;
        }
    }

    fn on_event(&mut self, e: Event) {
        match e {
            KeyPressed(key) if key == self.key => {
                self.is_pressing = true;
            },
            KeyReleased(key) if key == self.key => {
                self.is_pressing = false;
                self.cur_time = 0.0;
            }
            _ => {}
        }
    }
}
