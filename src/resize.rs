use std::intrinsics::TypeId;
use std::any::Any;

use GenericEvent;

/// When the window is resized
pub trait ResizeEvent {
    /// Creates a resize event.
    fn from_width_height(w: u32, h: u32) -> Option<Self>;
    /// Calls closure if this is a resize event.
    fn resize<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U;
    /// Returns resize arguments.
    fn resize_args(&self) -> Option<[u32; 2]> {
        self.resize(|x, y| [x, y])
    }
}

impl<T: GenericEvent> ResizeEvent for T {
    fn from_width_height(w: u32, h: u32) -> Option<Self> {
        GenericEvent::from_args(
            TypeId::of::<Box<ResizeEvent>>().hash(),
            &(w, h) as &Any
        )
    }

    fn resize<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U
    {
        if self.event_id() != TypeId::of::<Box<ResizeEvent>>().hash() {
            return None;
        }
        self.with_args(|any| {
            if let Some(&(w, h)) = any.downcast_ref::<(u32, u32)>() {
                Some(f(w, h))
            } else {
                panic!("Expected (u32, u32)")
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_input_resize() {
        use input::Input;

        let x: Option<Input> = ResizeEvent::from_width_height(100, 100);
        let y: Option<Input> = x.clone().unwrap().resize(|w, h|
            ResizeEvent::from_width_height(w, h)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_input_resize(bencher: &mut Bencher) {
        use input::Input;

        bencher.iter(|| {
            let _: Option<Input> = ResizeEvent::from_width_height(100, 100);
        });
    }

    #[test]
    fn test_event_resize() {
        use Event;

        let x: Option<Event> = ResizeEvent::from_width_height(100, 100);
        let y: Option<Event> = x.clone().unwrap().resize(|w, h|
            ResizeEvent::from_width_height(w, h)).unwrap();
        assert_eq!(x, y);
    }

    #[bench]
    fn bench_event_resize(bencher: &mut Bencher) {
        use Event;

        bencher.iter(|| {
            let _: Option<Event> = ResizeEvent::from_width_height(100, 100);
        });
    }
}
