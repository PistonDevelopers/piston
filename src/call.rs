
use EventCenter;

/// Implemented by all contexts that can call a command.
pub trait Call<'a> {
    /// Returns a identifier for user to remove the mapping later.
    fn call(&'a self, ec: &mut EventCenter, command: ||: 'static) -> uint;
}

