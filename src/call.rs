
use {
    EventCenter,
    Triggered,
};

/// Implemented by all contexts that can call a command.
pub trait Call<'a> {
    /// Returns a identifier for user to remove the mapping later.
    fn call(&'a self, ec: &mut EventCenter, command: ||: 'static) -> uint;
}

impl<'a, E: Triggered<'a>> Call<'a> for E {
    fn call(&'a self, ec: &mut EventCenter, command: ||: 'static) -> uint {
        ec.add_observer_call(self.get_observer(), command)
    }
}

