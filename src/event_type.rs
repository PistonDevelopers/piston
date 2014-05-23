
pub trait KeyType {
    fn id(&self) -> uint { 0 }
}

pub trait EventType {
    fn is_press_key(&self, _key: &KeyType) -> bool { false }
    fn is_release_key(&self, _key: &KeyType) -> bool { false }
}

