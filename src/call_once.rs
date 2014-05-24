
use {
    EventCenter,
    Triggered,
};

/// Implemented by all contexts that can call a command once.
pub trait CallOnce<'a> {
    /// Returns a identifier for user to remove the mapping later.
    fn call_once(&'a self, ec: &mut EventCenter, command: ||: 'static) -> uint;
}

impl<'a, E: Triggered<'a>> CallOnce<'a> for E {
    fn call_once(&'a self, ec: &mut EventCenter, command: ||: 'static) -> uint {
        ec.add_observer_call_once(self.get_observer(), command)
    }
}

