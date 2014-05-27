
use {
    EventCenter,
    Triggered,
};

/// Implemented by all contexts that can call a command.
pub trait Call<'a, A> {
    /// Returns a identifier for user to remove the mapping later.
    fn call(&'a self, ec: &mut EventCenter<A>, command: |&mut A|: 'static) -> uint;
}

impl<'a, A, E: Triggered> Call<'a, A> for E {
    fn call(&'a self, ec: &mut EventCenter<A>, command: |&mut A|: 'static) -> uint {
        ec.add_observer_call(self.get_observer(), command)
    }
}

