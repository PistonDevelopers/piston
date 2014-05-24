
use KeyType;

/// Implemented by all contexts that can observe press event.
pub trait AddPress<'a, T> {
    /// Observe a press event on certain key.
    fn press(&'a self, key: &'a KeyType) -> T;
}

