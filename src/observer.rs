
use piston::event::Event;

/// Implemented by all observers.
pub trait Observer {
    /// Whether the observer can trigger a command.
    fn can_trigger(&self) -> bool { false }
    /// Trigger the observer's command.
    fn trigger(&mut self) {}
    // one way to optimize is to support following method for back-end to
    // query, so it will not call `update` on the observer which don't need.
    //fn need_to_update(&self) -> bool { false; }
    /// Update for every game loop.
    fn update(&mut self, _dt: f64) {}
    /// Notify the observer that there is a event occuring.
    fn on_event(&mut self, _e: Event) {}
}

