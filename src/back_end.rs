
use piston::event::Event;
use Observer;

/// Implemented by all event handling back-ends.
pub trait BackEnd {
    /// Add an observer to the back-end so that the back-end will notify the
    /// observer when there is a event occuring.
    fn add_observer(&mut self, _ob: Box<Observer>) -> uint { 0 }
    /// Remove an observer so that it will not be triggered again.
    fn remove_observer(&mut self, _id: uint) {}
    /// Update the back-end for every game loop.
    fn update(&mut self, _dt: f64) {}
    /// Notify the back-end that there is a event occuring.
    fn on_event(&mut self, _e: Event) {}
}

