
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
    /// An event where some action is performed.
    Action(A),
    /// Returns `Success` <=> `Failure`.
    Invert(Box<Behavior<A>>),
    /// An event that succeeds if any sub event succeeds.
    ///
    /// If a sub event fails it will try the next one.
    Select(Vec<Behavior<A>>),
    /// An event waiting for time in seconds to expire.
    ///
    /// This event never fails.
    Wait(f64),
    /// An event where sub events are happening sequentially.
    ///
    /// The sequence fails if one of the sub events fails.
    /// The sequence succeeds if all the sub events succeeds.
    /// Can be used as a short-circuited logical AND block.
    Sequence(Vec<Behavior<A>>),
    /// While an event is executing, run a sequence of events in a loop..
    While(Box<Behavior<A>>, Vec<Behavior<A>>),
    /// An event where all sub events happen.
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
