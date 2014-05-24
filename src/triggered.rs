
use Observer;

pub trait Triggered<'a> {
    fn get_observer(&'a self) -> Box<Observer>;
}

