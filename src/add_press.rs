
/// Implemented by all contexts that can observe press event.
pub trait AddPress<'a, K, T> {
    /// Observe a press event on certain key.
    fn press(&'a self, key: K) -> T;
}

