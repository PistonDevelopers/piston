
/// Implemented by all contexts that can observe release event.
pub trait AddRelease<'a, K, T> {
    /// Observe a release event on certain key.
    fn release(&self, K) -> T;
}

