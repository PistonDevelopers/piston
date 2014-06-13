
/// Implemented by all actions.
pub trait StartState<S> {
    /// Creates a state from action, which tracks the state.
    fn start_state(&self) -> S;
}

