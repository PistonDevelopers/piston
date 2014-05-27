
use Observer;

/// A triat that event context should implement if it can be triggered.
pub trait Triggered {
    /// Returns the event context corresponding observer.
    fn get_observer(&self) -> Box<Observer>;
}

