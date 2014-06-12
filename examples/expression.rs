
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
    /// While an event is executing, run a sequence of events in a loop..
    While(Box<Event<A>>, Vec<Event<A>>),
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
    SequenceCursor(&'a Vec<Event<A>>, uint, Box<Cursor<'a, A, S>>),
    /// Keeps track of an event where sub events are repeated sequentially.
    WhileCursor(Box<Cursor<'a, A, S>>, &'a Vec<Event<A>>, uint, Box<Cursor<'a, A, S>>),
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
                => SequenceCursor(seq, 0, box seq.get(0).to_cursor()),
            While(ref ev, ref rep)
                => WhileCursor(box ev.to_cursor(), rep, 0, box rep.get(0).to_cursor()),
            WhenAll(ref all)
                => WhenAllCursor(all, all.iter().map(|ev| Some(ev.to_cursor())).collect()),
        }
    }
}

impl<'a, A: StartState<S>, S> Cursor<'a, A, S> {
    /// Updates the cursor that tracks an event.
    ///
    /// Returns `None` if the action did not terminate.
    /// or `Some(dt)` that tells how much time is left of the update time.
    pub fn update(
        &mut self, 
        dt: f64, 
        f: |action: &'a A, state: &S| -> Option<S>
    ) -> Option<f64> {
        match *self {
            State(action, ref mut state) => {
                // Call the function that updates the state.
                match f(action, state) {
                    Some(new_state) => {
                        *state = new_state; 
                        None
                    },
                    // Actions are considered instant,
                    // so there is always a full 'dt' left.
                    None => Some(dt),
                }
            },
            WaitCursor(wait_t, ref mut t) => {
                if *t + dt >= wait_t {
                    let remaining_dt = *t + dt - wait_t;
                    *t = wait_t;
                    Some(remaining_dt)
                } else {
                    *t += dt;
                    None
                }
            },
            SequenceCursor(
                seq, 
                ref mut i, 
                ref mut cursor
            ) => {
                let cur = cursor;
                let mut dt = dt;
                while *i < seq.len() {
                    match cur.update(dt, |action, state| f(action, state)) {
                        None => { break },
                        Some(new_dt) => { dt = new_dt; }
                    };
                    *i += 1;
                    // If end of sequence,
                    // return the 'dt' that is left.
                    if *i >= seq.len() { return Some(dt); }
                    // Create a new cursor for next event.
                    // Use the same pointer to avoid allocation.
                    **cur = seq.get(*i).to_cursor();
                }
                None
            },
            WhileCursor(
                ref mut ev_cursor,
                rep, 
                ref mut i, 
                ref mut cursor
            ) => {
                // If the event terminates, do not execute the loop.
                match ev_cursor.update(dt, |action, state| f(action, state)) {
                    Some(new_dt) => return Some(new_dt),
                    None => {}
                };
                let cur = cursor;
                let mut dt = dt;
                loop {
                    match cur.update(dt, |action, state| f(action, state)) {
                        None => { break },
                        Some(new_dt) => {
                            dt = new_dt;
                        }
                    };
                    *i += 1;
                    // If end of repeated events,
                    // start over from the first one.
                    if *i >= rep.len() { *i = 0; }
                    // Create a new cursor for next event.
                    // Use the same pointer to avoid allocation.
                    **cur = rep.get(*i).to_cursor();
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

// A test state machine that can increment and decrement.
fn exec(mut acc: u32, dt: f64, cursor: &mut Cursor<TestActions, ()>) -> u32 {
    cursor.update(dt, |action, _| {
        match *action {
            Inc => { acc += 1; None },
            Dec => { acc -= 1; None },
        }
    });
    acc
}

// Each action that terminates immediately
// consumes a time of 0.0 seconds.
// This makes it possible to execute one action
// after another without delay or waiting for next update.
fn print_2() {
    let a: u32 = 0;
    let seq = Sequence(vec![Action(Inc), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 0.0, &mut cursor);
    assert_eq!(a, 2);
}

// If you wait the exact amount before to execute an action,
// it will execute. This behavior makes it easy to predict
// when an action will run.
fn wait_sec() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(1.0), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 1.0, &mut cursor);
    assert_eq!(a, 1);
}

// When we execute half the time and then the other half,
// then the action should be executed. 
fn wait_half_sec() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(1.0), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 0.5, &mut cursor);
    assert_eq!(a, 0);
    let a = exec(a, 0.5, &mut cursor);
    assert_eq!(a, 1);
}

// A sequence of wait events is the same as one wait event.
fn wait_two_waits() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(0.5), Wait(0.5), Action(Inc)]);
    let mut cursor = seq.to_cursor();
    let a = exec(a, 1.0, &mut cursor);
    assert_eq!(a, 1);
}

// Increase counter ten times.
fn loop_ten_times() {
    let a: u32 = 0;
    let rep = While(box Wait(50.0), vec![Wait(0.5), Action(Inc), Wait(0.5)]);
    let mut cursor = rep.to_cursor();
    let a = exec(a, 10.0, &mut cursor);
    assert_eq!(a, 10);
}

fn main() {
    print_2();
    wait_sec();
    wait_half_sec();
    wait_two_waits();
    loop_ten_times();
}


