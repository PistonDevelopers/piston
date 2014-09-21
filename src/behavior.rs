
use input;

/// Describes a behavior.
#[deriving(Clone, PartialEq)]
pub enum Behavior<A> {
    /// A button was pressed.
    Pressed(input::Button),
    /// A button was released.
    Released(input::Button),
    /// A high level description of an action.
    Action(A),
    /// Converts `Success` into `Failure` and vice versa.
    Fail(Box<Behavior<A>>),
    /// Ignores failures and returns `Success`.
    AlwaysSucceed(Box<Behavior<A>>),
    /// Succeeds if any sub behavior succeeds.
    ///
    /// If a sub behavior fails it will try the next one.
    /// Can be thought of a short-circuited logical OR gate.
    Select(Vec<Behavior<A>>),
    /// Waits an amount of time before continuing.
    ///
    /// f64: Time in seconds
    Wait(f64),
    /// Wait forever.
    WaitForever,
    /// `If(condition, success, failure)`
    If(Box<Behavior<A>>, Box<Behavior<A>>, Box<Behavior<A>>),
    /// Runs sub behaviors in sequence.
    ///
    /// The sequence fails if a sub behavior fails.
    /// The sequence succeeds if all the sub behavior succeeds.
    /// Can be thought of as a short-circuited logical AND gate.
    Sequence(Vec<Behavior<A>>),
    /// Loops while conditional behavior is running.
    /// Succeeds if the conditional behavior succeeds.
    /// Fails if the conditional behavior fails,
    /// or if any behavior in the loop body fails.
    While(Box<Behavior<A>>, Vec<Behavior<A>>),
    /// Runs all behaviors in parallel.
    WhenAll(Vec<Behavior<A>>),
}
