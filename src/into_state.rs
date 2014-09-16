
/// Implemented by objects describing something that can be executed.
pub trait IntoState<S: Clone> {
    /// Creates a state from a description.
    fn into_state(self) -> S;
}
