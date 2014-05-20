
/// Implemented by all contexts that can observe pressing event.
pub trait AddPressing<'a, K, T> {
    /// Observe a pressing event on certain key.
    fn pressing(&'a self, key: K) -> T;
}

