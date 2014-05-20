
/// Implemented by all contexts that can observe a event lasting certain time.
pub trait AddLasting<'a, T> {
    /// Observe a event lasting certain time in second
    fn lasting(&'a self, time: f64) -> T;
}

