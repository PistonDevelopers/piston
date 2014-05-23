
use EventCenter;

/// Implemented by all contexts that can call a command.
pub trait Call {
    /// Returns a identifier for user to remove the mapping later.
    fn call(&self, ec: &mut EventCenter, command: ||: 'static) -> uint;
}

