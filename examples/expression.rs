
#![deny(missing_doc)]

//! Example for Rust-Event using expression based design.
//!
//! The idea is to use combinators of events to describe more complex events.
//! An 'Action' is a variant of an event that spans across time.
//! You program the actions like a state machine, controlling how they interact with the world.
//!
//! Assume you have a complete list of the actions.
//! Any event you can construct from these actions has a corresponding `Cursor`.
//! The cursor keeps track of the combinatorial state.
//!
//! This design is useful in environments where all actions can be broken down
//! into simple interacitons while needing complex combinations of those actions.

extern crate debug;

/// Describes an event.
pub enum Event<A> {
    /// An event where some action is performed.
    Action(A),
    /// An event 
    Wait(f64),
    /// An event where sub events are happening sequentially.
    Sequence(Vec<Event<A>>),
    /// An event where sub events are repeated sequentially forever.
    Repeat(Vec<Event<A>>),
    /// An event where any sub event might happen.
    WhenAny(Vec<Event<A>>),
    /// An event where all sub events happen.
    WhenAll(Vec<Event<A>>),
}

/// Keeps track of an event.
pub enum Cursor<'a, A, S> {
    /// Keeps track of an event where you have a state of an action.
    State(&'a A, S),
    /// Keeps track of an event where you wait and do nothing.
    WaitCursor(f64, f64),
    /// Keeps track of an event where sub events happens sequentially.
    SequenceCursor(&'a Vec<Event<A>>, uint, f64, f64, Box<Cursor<'a, A, S>>),
    /// Keeps track of an event where sub events are repeated sequentially.
    RepeatCursor(&'a Vec<Event<A>>, uint, Box<Cursor<'a, A, S>>),
    /// Keeps track of an event where any sub event might happen.
    WhenAnyCursor(&'a Vec<Event<A>>, Vec<Cursor<'a, A, S>>),
    /// Keeps track of an event where all sub events must happen.
    WhenAllCursor(&'a Vec<Event<A>>, Vec<Option<Cursor<'a, A, S>>>),
}

/// Implemented by all actions.
pub trait StartState<S> {
    /// Creates a state from action, which tracks the state.
    fn start_state(&self) -> S;
}

impl<A: StartState<S>, S> Event<A> {
    /// Creates a cursor structure from an event structure.
    ///
    /// The cursor structure keeps track of the state.
    /// You can define your own actions and use the combinations
    /// to create more complex states.
    pub fn to_cursor<'a>(&'a self) -> Cursor<'a, A, S> {
        match *self {
            Action(ref action) 
                => State(action, action.start_state()),
            Wait(dt) 
                => WaitCursor(dt, 0.0),
            Sequence(ref seq) 
                => SequenceCursor(seq, 0, 0.0, 0.0, box seq.get(0).to_cursor()),
            Repeat(ref rep)
                => RepeatCursor(rep, 0, box rep.get(0).to_cursor()),
            WhenAny(ref any)
                => WhenAnyCursor(any, any.iter().map(|ev| ev.to_cursor()).collect()),
            WhenAll(ref all)
                => WhenAllCursor(all, all.iter().map(|ev| Some(ev.to_cursor())).collect()),
        }
    }
}

impl<'a, A: StartState<S>, S> Cursor<'a, A, S> {
    /// Updates the cursor that tracks an event.
    ///
    /// Returns `None` if the action did not terminate.
    /// or `Some(dt)` that tells how much time was consumed by the action.
    pub fn update(
        &mut self, 
        dt: f64, 
        f: |action: &'a A, state: &S| -> Option<S>
    ) -> Option<f64> {
        match *self {
            State(action, ref mut state) => {
                // Call the function that updates the state.
                match f(action, state) {
                    Some(new_state) => {*state = new_state; None},
                    None => Some(0.0),
                }
            },
            WaitCursor(dt, ref mut t) => {
                // Update the time and return 'false' if we completed.
                *t = dt.min(dt + *t);
                if *t < dt { None } else { Some(dt) }
            },
            SequenceCursor(
                seq, 
                ref mut i, 
                ref mut inc_dt, 
                ref mut waited_dt, 
                ref mut cursor
            ) => {
                // Update a sequence of events.
                let next_update = *waited_dt + dt;
                let cur = cursor;
                while *i < seq.len() && *inc_dt <= next_update {
                    // If the sub event terminates,
                    // decrement the delta time for next events.
                    match cur.update(next_update  - *inc_dt, |action, state| f(action, state)) {
                        None => { *waited_dt += dt; break },
                        Some(consumed_dt) => { *waited_dt = 0.0; *inc_dt += consumed_dt },
                    };
                    // Create a new cursor for next event.
                    // Use the same pointer to avoid allocation.
                    *i += 1;
                    if *i >= seq.len() { return Some(*inc_dt); }
                    **cur = seq.get(*i).to_cursor();
                }
                None
            },
            _ => unimplemented!(),
        }
    }
}


/////////////////////////////////////////////////////////////////////////////////

/// Some test actions.
pub enum TestActions {
    /// Increment accumulator.
    Inc,
    /// Decrement accumulator.
    Dec,
}

impl StartState<()> for TestActions {
    fn start_state(&self) {}
}

fn exec(mut acc: u32, dt: f64, cursor: &mut Cursor<TestActions, ()>) -> u32 {
    cursor.update(dt, |action, _| {
        println!("{:?}", action);
        match *action {
            Inc => { acc += 1; None },
            Dec => { acc -= 1; None },
        }
    });
    acc
}

fn print_2() {
    // Prints 2.
    let a: u32 = 0;
    let seq = Sequence(vec![Action(Inc), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 0.0, &mut cursor);
    println!("{}", a);
}

fn wait_sec() {
    // Prints 2.
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(1.0), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 1.0, &mut cursor);
    println!("{}", a);
}

fn main() {
    print_2();
    wait_sec();
}


