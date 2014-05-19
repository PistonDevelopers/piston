
use graphics::{
    Borrowed,
    Field,
    Value,
};
use piston::keyboard;
use piston::event::{
    Event,
    KeyPressed,
    KeyReleased,
};

use {
    AddLasting,
    KeyboardPressingLastingEvent,

    BackEnd,
    Map,
    Observer,
};

/// A event context which used to observe whether a certain keyboard key is being pressed.
pub struct KeyboardPressingEvent<'a> {
    /// The key to be observed.
    pub key: Field<'a, keyboard::Key>,
}

impl<'a> Map<'a> for KeyboardPressingEvent<'a> {
    #[inline(always)]
    fn map<'a, B: BackEnd>(&self, back_end: &mut B, command: ||: 'a) -> uint {
        back_end.add_observer(box KeyboardPressingEventObserver::new(command, *self.key.get()))
    }
}

impl<'a> AddLasting<'a, KeyboardPressingLastingEvent<'a>> for KeyboardPressingEvent<
'a> {
    #[inline(always)]
    fn lasting(&'a self, time: f64) -> KeyboardPressingLastingEvent<'a> {
        KeyboardPressingLastingEvent {
            key: Borrowed(self.key.get()),
            lasting: Value(time),
        }
    }
}

struct KeyboardPressingEventObserver<'a> {
    command: ||: 'a,
    key: keyboard::Key,
    is_pressing: bool,
}

impl<'a> KeyboardPressingEventObserver<'a> {
    pub fn new<'a>(command: ||: 'a, key: keyboard::Key) -> KeyboardPressingEventObserver<'a> {
        KeyboardPressingEventObserver {
            command: command,
            key: key,
            is_pressing: false,
        }
    }
}

impl<'a> Observer for KeyboardPressingEventObserver<'a> {
    fn can_trigger(&self) -> bool {
        self.is_pressing
    }

    fn trigger(&mut self) {
        (self.command)();
    }

    fn on_event(&mut self, e: Event) {
        match e {
            KeyPressed(key) if key == self.key => {
                self.is_pressing = true;
            },
            KeyReleased(key) if key == self.key => {
                self.is_pressing = false;
            }
            _ => {}
        }
    }
}
