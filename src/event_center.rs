
use collections::treemap::TreeMap;
use Observer;
use EventType;

pub struct EventCenter {
    observers: TreeMap<uint, Box<Observer>>,
    commands_call: TreeMap<uint, ||: 'static>,
    commands_call_once: TreeMap<uint, ||: 'static>,
    count: uint,
}

impl EventCenter {
    /// Returns a new event center
    pub fn new() -> EventCenter {
        EventCenter {
            observers: TreeMap::<uint, Box<Observer>>::new(),
            commands_call: TreeMap::<uint, ||: 'static>::new(),
            commands_call_once: TreeMap::<uint, ||: 'static>::new(),
            count: 0,
        }
    }

    /// Add an observer to the event center so that the it will notify the
    /// observer when there is a event occuring.
    ///
    /// This will continuing trigger command until the observer is removed.
    pub fn add_observer_call(&mut self, ob: Box<Observer>, command: ||: 'static) -> uint {
        let i = self.get_empty_id();
        self.observers.insert(i, ob);
        self.commands_call.insert(i, command);
        i
    }

    pub fn add_observer_call_once(&mut self, ob: Box<Observer>, command: ||: 'static) -> uint {
        let i = self.get_empty_id();
        self.observers.insert(i, ob);
        self.commands_call_once.insert(i, command);
        i
    }

    /// Remove an observer so that it will not be triggered again.
    pub fn remove_observer(&mut self, i: uint) {
        self.observers.remove(&i);
        self.commands_call.remove(&i);
        self.commands_call_once.remove(&i);
    }

    /// Update the event center for every game loop.
    pub fn update(&mut self, dt: f64) {
        self.update_observers_with_commands_call(dt);
        self.update_observers_with_commands_call_once(dt);
    }

    /// Notify the event_center that there is a event occuring.
    pub fn receive_event(&mut self, e: &EventType) {
        for (_, ob) in self.observers.mut_iter() {
            ob.on_event(e);
        }
    }

    fn get_empty_id(&mut self) -> uint {
        self.count += 1;
        self.count - 1
    }

    fn update_observers_with_commands_call(&mut self, dt: f64) {
        for (id, command) in self.commands_call.mut_iter() {
            let mut ob = self.observers.find_mut(id);
            let ob = ob.get_mut_ref();
            ob.update(dt);

            if ob.can_trigger() {
                (*command)();
                ob.after_trigger();
            }
        }
    }

    fn update_observers_with_commands_call_once(&mut self, dt: f64) {
        let mut id_to_removed = Vec::<uint>::new();
        for (id, command) in self.commands_call_once.mut_iter() {
            let mut ob = self.observers.find_mut(id);
            let ob = ob.get_mut_ref();
            ob.update(dt);

            if ob.can_trigger() {
                (*command)();
                ob.after_trigger();
                id_to_removed.push(*id);
            }
        }
        for id in id_to_removed.iter() {
            self.remove_observer(*id);
        }
    }
}

