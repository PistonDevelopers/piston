
/// Implemented by all contexts that can observe keyboard event.
pub trait AddKeyboard<'a, T> {
    /// Observe a keyboard event
    fn keyboard(&'a self) -> T;
}

