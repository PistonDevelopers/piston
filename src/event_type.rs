
/// A trait that every key must implement.
pub trait KeyType {
    /// Returns an unique identifier for this key.
    fn id(&self) -> uint;
}

/// A trait that event event must implement.
pub trait EventType {
    /// Returns true if the event is press certain key event.
    fn is_press_key(&self, _key: &KeyType) -> bool { false }
    /// Returns true if the event is release certain key event.
    fn is_release_key(&self, _key: &KeyType) -> bool { false }
}

