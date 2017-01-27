use Input;

/// After render arguments.
#[derive(Copy, Clone, PartialEq, Debug, RustcDecodable, RustcEncodable)]
pub struct AfterRenderArgs;

/// After rendering and buffers are swapped.
pub trait AfterRenderEvent: Sized {
    /// Creates an after render event.
    fn from_after_render_args(args: &AfterRenderArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is an after render event.
    fn after_render<U, F>(&self, f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U;
    /// Returns after render arguments.
    fn after_render_args(&self) -> Option<AfterRenderArgs> {
        self.after_render(|args| args.clone())
    }
}

impl AfterRenderEvent for Input {
    fn from_after_render_args(args: &AfterRenderArgs, _old_event: &Input) -> Option<Self> {
        Some(Input::AfterRender(*args))
    }

    fn after_render<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(&AfterRenderArgs) -> U
    {
        match *self {
            Input::AfterRender(ref args) => Some(f(args)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_after_render() {
        use Input;
        use AfterRenderArgs;

        let e = Input::AfterRender(AfterRenderArgs);
        let x: Option<Input> = AfterRenderEvent::from_after_render_args(
            &AfterRenderArgs, &e);
        let y: Option<Input> = x.clone().unwrap().after_render(|args|
            AfterRenderEvent::from_after_render_args(args, x.as_ref().unwrap())).unwrap();
        assert_eq!(x, y);
    }
}
