
use collections::treemap::TreeMap;
use piston;
use Observer;

/// A event center to handle user's request
pub struct EventCenter {
    observers: TreeMap<uint, Box<Observer>>,
    count: uint,
}

impl EventCenter {
    /// Returns a new event center
    pub fn new() -> EventCenter {
        EventCenter {
            observers: TreeMap::<uint, Box<Observer>>::new(),
            count: 0,
        }
    }

    /// Add an observer to the event center so that the it will notify the
    /// observer when there is a event occuring.
    pub fn add_observer(&mut self, ob: Box<Observer>) -> uint {
        let i = self.count;
        self.count += 1;
        self.observers.insert(i, ob);
        i
    }

    /// Remove an observer so that it will not be triggered again.
    pub fn remove_observer(&mut self, i: uint) {
        self.observers.remove(&i);
    }

    /// Update the event center for every game loop.
    pub fn update(&mut self, dt: f64) {
        for (_, ob) in self.observers.mut_iter() {
            ob.update(dt);

            if ob.can_trigger() {
                ob.trigger();
            }
        }
    }

    /// Notify the event_center that there is a event occuring.
    pub fn receive_event(&mut self, e: piston::event::Event) {
        for (_, ob) in self.observers.mut_iter() {
            ob.on_event(e);
        }
    }
}

