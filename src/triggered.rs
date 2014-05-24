
use Observer;

/// A triat that event context should implement if it can be triggered.
pub trait Triggered<'a> {
    /// Returns the event context corresponding observer.
    fn get_observer(&'a self) -> Box<Observer>;
}

