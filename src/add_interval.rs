
/// Implemented by all contexts that can observe time interval.
pub trait AddInterval<T> {
    /// Observe a time interval event on certain time in seconds.
    fn interval(&self, seconds: f64) -> T;
}

