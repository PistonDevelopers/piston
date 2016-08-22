use { Event, Input };

/// When the window is resized
pub trait ResizeEvent: Sized {
    /// Creates a resize event.
    fn from_width_height(w: u32, h: u32, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a resize event.
    fn resize<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U;
    /// Returns resize arguments.
    fn resize_args(&self) -> Option<[u32; 2]> {
        self.resize(|x, y| [x, y])
    }
}

/* Enable when specialization gets stable.
impl<T: GenericEvent> ResizeEvent for T {
    fn from_width_height(w: u32, h: u32, old_event: &Self) -> Option<Self> {
        GenericEvent::from_args(RESIZE, &(w, h) as &Any, old_event)
    }

    fn resize<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U
    {
        if self.event_id() != RESIZE {
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
*/

impl ResizeEvent for Input {
    fn from_width_height(w: u32, h: u32, _old_event: &Self) -> Option<Self> {
        Some(Input::Resize(w, h))
    }

    fn resize<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U
    {
        match *self {
            Input::Resize(w, h) => Some(f(w, h)),
            _ => None
        }
    }
}

impl<I: ResizeEvent> ResizeEvent for Event<I> {
    fn from_width_height(w: u32, h: u32, old_event: &Self) -> Option<Self> {
        if let &Event::Input(ref old_input) = old_event {
            <I as ResizeEvent>::from_width_height(w, h, old_input)
                .map(|x| Event::Input(x))
        } else {
            None
        }
    }

    fn resize<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U
    {
        match *self {
            Event::Input(ref x) => x.resize(f),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_resize() {
        use super::super::Input;

        let e = Input::Resize(0, 0);
        let x: Option<Input> = ResizeEvent::from_width_height(100, 100, &e);
        let y: Option<Input> = x.clone().unwrap().resize(|w, h|
            ResizeEvent::from_width_height(w, h, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn test_event_resize() {
        use Event;
        use super::super::Input;

        let e = Event::Input(Input::Resize(0, 0));
        let x: Option<Event> = ResizeEvent::from_width_height(100, 100, &e);
        let y: Option<Event> = x.clone().unwrap().resize(|w, h|
            ResizeEvent::from_width_height(w, h, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
