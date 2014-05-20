
use BackEnd;

/// Implemented by all contexts that can map a command.
pub trait Map<'a> {
    // the parameters of the closure `command` need more discussions.
    /// Maps event context with a command.
    /// Returns a identifier for user to remove the mapping later.
    fn map<'a, B: BackEnd>(&self, back_end: &mut B, command: ||: 'a) -> uint;
}

