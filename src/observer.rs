
use EventType;

/// Implemented by all observers.
pub trait Observer {
    /// Whether the observer can be triggered
    fn can_trigger(&self) -> bool;
    /// Called after the observer was triggered.
    fn after_trigger(&mut self) {}
    /// Update for every game loop.
    fn update(&mut self, _dt: f64) {}
    /// Notify the observer that there is a event occuring.
    fn on_event(&mut self, _e: &EventType) {}
}

