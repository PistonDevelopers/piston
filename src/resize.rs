use input::Input;

use Event;

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

impl ResizeEvent for Input {
    fn from_width_height(w: u32, h: u32) -> Option<Self> {
        Some(Input::Resize(w, h))
    }

    fn resize<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U
    {
        if let &Input::Resize(w, h) = self {
            Some(f(w, h))
        } else {
            None
        }
    }
}

impl<I> ResizeEvent for Event<I>
    where I: ResizeEvent
{
    fn from_width_height(w: u32, h: u32) -> Option<Self> {
        if let Some(input) = ResizeEvent::from_width_height(w, h) {
            Some(Event::Input(input))
        } else {
            None
        }
    }

    fn resize<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(u32, u32) -> U
    {
        if let &Event::Input(ref input) = self {
            input.resize(f)
        } else {
            None
        }
    }
}
