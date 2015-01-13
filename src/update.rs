use input::Input;

use Event;
use UpdateArgs;

/// When the application state should be updated
pub trait UpdateEvent: Sized {
    /// Creates an update event.
    fn from_update_args(args: &UpdateArgs) -> Option<Self>;
    /// Creates an update event with delta time.
    fn from_dt(dt: f64) -> Option<Self> {
        UpdateEvent::from_update_args(&UpdateArgs { dt: dt })
    }
    /// Calls closure if this is an update event.
    fn update<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U;
    /// Returns update arguments.
    fn update_args(&self) -> Option<UpdateArgs> {
        self.update(|args| args.clone())
    }
}

impl UpdateEvent for Input {
    fn from_update_args(_: &UpdateArgs) -> Option<Self> {
        None
    }

    fn update<U, F>(&self, _: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U
    {
        None
    }
}

impl<I> UpdateEvent for Event<I> {
    fn from_update_args(args: &UpdateArgs) -> Option<Self> {
        Some(Event::Update(args.clone()))
    }

    fn update<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U
    {
        if let &Event::Update(ref args) = self {
            Some(f(args))
        } else {
            None
        }
    }
}
