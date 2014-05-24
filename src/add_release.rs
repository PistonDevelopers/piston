
/// Implemented by all contexts that can observe release event.
pub trait AddRelease<'a, T> {
    /// Observe a release event on certain key.
    fn release(&'a self) -> T;
}

