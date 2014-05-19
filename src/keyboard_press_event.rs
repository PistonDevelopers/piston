
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

/// A event context which used to observe whether a certain keyboard key was pressed.
pub struct KeyboardPressEvent<'a> {
    /// The key to be observed.
    pub key: Field<'a, keyboard::Key>,
}

impl<'a> Map<'a> for KeyboardPressEvent<'a> {
    #[inline(always)]
    fn map<'a, B: BackEnd>(&self, back_end: &mut B, command: ||: 'a) -> uint {
        back_end.add_observer(box KeyboardPressEventObserver::new(command, *self.key.get()))
    }
}

struct KeyboardPressEventObserver<'a> {
    command: ||: 'a,
    key: keyboard::Key,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> KeyboardPressEventObserver<'a> {
    pub fn new<'a>(command: ||: 'a, key: keyboard::Key) -> KeyboardPressEventObserver<'a> {
        KeyboardPressEventObserver {
            command: command,
            key: key,
            can_trigger: false,
            is_pressed: false,
        }
    }
}

impl<'a> Observer for KeyboardPressEventObserver<'a> {
    fn can_trigger(&self) -> bool {
        self.can_trigger
    }

    fn trigger(&mut self) {
        (self.command)();
        self.can_trigger = false
    }

    fn on_event(&mut self, e: Event) {
        match e {
            KeyPressed(key) if key == self.key => {
                if !self.is_pressed {
                    self.is_pressed = true;
                    self.can_trigger = true;
                }
            },
            KeyReleased(key) if key == self.key => {
                self.is_pressed = false;
            }
            _ => {}
        }
    }
}

