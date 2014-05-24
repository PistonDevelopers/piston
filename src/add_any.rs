
use Triggered;

pub trait AddAny<'a, T> {
    fn any(&'a self, events: &'a [&'a Triggered<'a>]) -> T;
}

