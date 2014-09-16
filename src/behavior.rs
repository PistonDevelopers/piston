
use input;
use IntoState;

/// Describes a behavior.
#[deriving(Clone, PartialEq)]
pub enum Behavior<A: IntoState<S>, S> {
    /// A button was pressed.
    Pressed(input::Button),
    /// A button was released.
    Released(input::Button),
    /// A high level description of an action.
    Action(A),
    /// Converts `Success` into `Failure` and vice versa.
    Not(Box<Behavior<A, S>>),
    /// Ignores failures and returns `Success`.
    AlwaysSucceed(Box<Behavior<A, S>>),
    /// Succeeds if any sub behavior succeeds.
    ///
    /// If a sub behavior fails it will try the next one.
    /// Can be thought of a short-circuited logical OR gate.
    Select(Vec<Behavior<A, S>>),
    /// Waits an amount of time before continuing.
    ///
    /// f64: Time in seconds
    Wait(f64),
    /// Wait forever.
    WaitForever,
    /// `If(condition, success, failure)`
    If(Box<Behavior<A, S>>, Box<Behavior<A, S>>, Box<Behavior<A, S>>),
    /// Runs sub behaviors in sequence.
    ///
    /// The sequence fails if a sub behavior fails.
    /// The sequence succeeds if all the sub behavior succeeds.
    /// Can be thought of as a short-circuited logical AND gate.
    Sequence(Vec<Behavior<A, S>>),
    /// Loops while conditional behavior is running.
    While(Box<Behavior<A, S>>, Vec<Behavior<A, S>>),
    /// Runs all behaviors in parallel.
    WhenAll(Vec<Behavior<A, S>>),
}
