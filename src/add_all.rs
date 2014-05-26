
use Triggered;

/// Implemented by all contexts that can triggered by ALL event in parameters.
pub trait AddAll<'a, T> {
    /// Observe all the event in parameters.
    fn all(&'a self, events: &'a [&'a Triggered]) -> T;
}

