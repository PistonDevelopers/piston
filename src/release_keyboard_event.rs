
use graphics::Field;
use piston::keyboard;
use piston::event::{
    Event,
    KeyPressed,
    KeyReleased,
};
use {
    Call,
    EventCenter,
    Observer,
};

/// A event context which used to observe whether a certain keyboard key was released.
pub struct ReleaseKeyboardEvent<'a> {
    /// The key to be observed.
    pub key: Field<'a, keyboard::Key>,
}

impl<'a> Call<'a> for ReleaseKeyboardEvent<'a> {
    #[inline(always)]
    fn call<'a>(&self, ec: &mut EventCenter, command: ||: 'a) -> uint {
        ec.add_observer(box ReleaseKeyboardEventObserver::new(command, *self.key.get()))
    }
}

struct ReleaseKeyboardEventObserver<'a> {
    command: ||: 'a,
    key: keyboard::Key,
    can_trigger: bool,
    is_pressed: bool,
}

impl<'a> ReleaseKeyboardEventObserver<'a> {
    pub fn new<'a>(command: ||: 'a, key: keyboard::Key) -> ReleaseKeyboardEventObserver<'a> {
        ReleaseKeyboardEventObserver {
            command: command,
            key: key,
            can_trigger: false,
            is_pressed: false,
        }
    }
}

impl<'a> Observer for ReleaseKeyboardEventObserver<'a> {
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
                self.is_pressed = true;
            },
            KeyReleased(key) if key == self.key => {
                if self.is_pressed {
                    self.is_pressed = false;
                    self.can_trigger = true;
                }
            }
            _ => {}
        }
    }
}

