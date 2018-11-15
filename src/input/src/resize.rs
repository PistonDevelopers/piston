use {Event, Input};

/// When the window is resized
pub trait ResizeEvent: Sized {
    /// Creates a resize event.
    fn from_width_height(w: f64, h: f64, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a resize event.
    fn resize<U, F>(&self, f: F) -> Option<U> where F: FnMut(f64, f64) -> U;
    /// Returns resize arguments.
    fn resize_args(&self) -> Option<[f64; 2]> {
        self.resize(|x, y| [x, y])
    }
}

impl ResizeEvent for Event {
    fn from_width_height(w: f64, h: f64, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Resize(w, h)))
    }

    fn resize<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(f64, f64) -> U
    {
        match *self {
            Event::Input(Input::Resize(w, h)) => Some(f(w, h)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_resize() {
        use super::super::Input;

        let e: Event = Input::Resize(0.0, 0.0).into();
        let x: Option<Event> = ResizeEvent::from_width_height(100.0, 100.0, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .resize(|w, h| ResizeEvent::from_width_height(w, h, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
