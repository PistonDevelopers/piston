
use Triggered;

/// Implemented by all contexts that can triggered by ANY event in parameters.
pub trait AddAny<'a, T> {
    /// Observe any event in parameters.
    fn any(&'a self, events: &'a [&'a Triggered<'a>]) -> T;
}

