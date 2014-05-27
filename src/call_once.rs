
use {
    EventCenter,
    Triggered,
};

/// Implemented by all contexts that can call a command once.
pub trait CallOnce<'a, A> {
    /// Returns a identifier for user to remove the mapping later.
    fn call_once(&'a self, ec: &mut EventCenter<A>, command: |&mut A|: 'static) -> uint;
}

impl<'a, A, E: Triggered> CallOnce<'a, A> for E {
    fn call_once(&'a self, ec: &mut EventCenter<A>, command: |&mut A|: 'static) -> uint {
        ec.add_observer_call_once(self.get_observer(), command)
    }
}

