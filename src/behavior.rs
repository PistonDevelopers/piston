
use piston::input;
use {
    State,
    PressedState,
    SelectState,
    SequenceState,
    StartState,
    ActionState,
    InvertState,
    WaitState,
    WhenAllState,
    WhileState,
};

/// Describes a behavior.
pub enum Behavior<A> {
    /// A button was pressed.
    Pressed(input::Button),
    /// A high level description of an action.
    Action(A),
    /// Converts `Success` into `Failure` and vice versa.
    Invert(Box<Behavior<A>>),
    /// Succeeds if any sub behavior succeeds.
    ///
    /// If a sub behavior fails it will try the next one.
    /// Can be thought of a short-circuited logical OR gate.
    Select(Vec<Behavior<A>>),
    /// Waits the number of seconds to expire.
    Wait(f64),
    /// Runs sub behaviors in sequence.
    ///
    /// The sequence fails if a sub behavior fails.
    /// The sequence succeeds if all the sub behavior succeeds.
    /// Can be thought of as a short-circuited logical AND gate.
    Sequence(Vec<Behavior<A>>),
    /// Loops while conditional behavior is running.
    While(Box<Behavior<A>>, Vec<Behavior<A>>),
    /// Runs all behaviors in parallel.
    WhenAll(Vec<Behavior<A>>),
}

impl<A: StartState<S>, S> Behavior<A> {
    /// Creates a cursor structure from an event structure.
    ///
    /// The cursor structure keeps track of the state.
    /// You can define your own actions and use the combinations
    /// to create more complex states.
    pub fn to_state<'a>(&'a self) -> State<'a, A, S> {
        match *self {
            Pressed(button)
                => PressedState(button),
            Action(ref action)
                => ActionState(action, action.start_state()),
            Invert(ref ev)
                => InvertState(box ev.to_state()),
            Wait(dt)
                => WaitState(dt, 0.0),
            Select(ref sel)
                => SelectState(sel, 0, box sel[0].to_state()),
            Sequence(ref seq)
                => SequenceState(seq, 0, box seq[0].to_state()),
            While(ref ev, ref rep)
                => WhileState(box ev.to_state(), rep, 0, box rep[0].to_state()),
            WhenAll(ref all)
                => WhenAllState(all.iter().map(|ev| Some(ev.to_state())).collect()),
        }
    }
}
