
use {
    EventCenter,
    Triggered,
};

/// Implemented by all contexts that can call a command.
pub trait Call<'a, A> {
    /// Returns a identifier for user to remove the mapping later.
    fn call(&self, ec: &mut EventCenter<'a, A>, command: |&mut A|: 'a) -> uint;
}

impl<'a, A, E: Triggered> Call<'a, A> for E {
    fn call(&self, ec: &mut EventCenter<'a, A>, command: |&mut A|: 'a) -> uint {
        ec.add_observer_call(self.get_observer(), command)
    }
}

