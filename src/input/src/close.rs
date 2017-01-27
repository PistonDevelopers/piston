use Input;

/// Close arguments.
#[derive(Copy, Clone, PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub struct CloseArgs;

/// Window is closing.
pub trait CloseEvent: Sized {
    /// Creates a close event from arguments.
    fn from_close_args(args: &CloseArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a close event.
    fn close<U, F>(&self, f: F) -> Option<U> where F: FnMut(&CloseArgs) -> U;
    /// Returns close arguments.
    fn close_args(&self) -> Option<CloseArgs> {
        self.close(|args| args.clone())
    }
}

impl CloseEvent for Input {
    fn from_close_args(args: &CloseArgs, _old_event: &Input) -> Option<Self> {
        Some(Input::Close(*args))
    }

    fn close<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&CloseArgs) -> U
    {
        match *self {
            Input::Close(ref args) => Some(f(args)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_close() {
        use Input;
        use CloseArgs;

        let e = Input::Close(CloseArgs);
        let x: Option<Input> = CloseEvent::from_close_args(&CloseArgs, &e);
        let y: Option<Input> = x.clone()
            .unwrap()
            .close(|args| CloseEvent::from_close_args(args, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
