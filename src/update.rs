use std::any::{ Any, TypeId };
use std::hash::{ hash, SipHasher };

use GenericEvent;

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

impl<T: GenericEvent> UpdateEvent for T {
    fn from_update_args(args: &UpdateArgs) -> Option<Self> {
        let id = hash::<_, SipHasher>(&TypeId::of::<Box<UpdateEvent>>());
        GenericEvent::from_args(id, args as &Any)
    }

    fn update<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&UpdateArgs) -> U
    {
        let id = hash::<_, SipHasher>(&TypeId::of::<Box<UpdateEvent>>());
        if self.event_id() != id {
            return None;
        }
        self.with_args(|any| {
            if let Some(args) = any.downcast_ref::<UpdateArgs>() {
                Some(f(args))
            } else {
                panic!("Expected UpdateArgs")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_event_update() {
        use Event;
        use UpdateArgs;

        let x: Option<Event> = UpdateEvent::from_update_args(
            &UpdateArgs {
                dt: 1.0,
            }
        );
        let y: Option<Event> = x.clone().unwrap().update(|args|
            UpdateEvent::from_update_args(args)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_update(bencher: &mut Bencher) {
        use Event;
        use UpdateArgs;

        let args = UpdateArgs {
            dt: 1.0,
        };
        bencher.iter(|| {
            let _: Option<Event> = UpdateEvent::from_update_args(&args);
        });
    }
}
