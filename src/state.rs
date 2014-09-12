
use std;
use piston::{
    Event,
    Input,
    Update,
    UpdateArgs,
};
use piston::input;
use {
    Behavior,
    StartState,
    Status,
    Failure,
    Success,
    Running,
    Pressed,
    Released,
    WhenAll,
    While,
    Sequence,
    Select,
    Wait,
    Invert,
    Action,
};

/// Keeps track of an event.
pub enum State<'a, A: 'a, S> {
    /// Keeps track of whether a button was pressed.
    PressedState(input::Button),
    /// Keeps track of whether a button was released.
    ReleasedState(input::Button),
    /// Keeps track of an event where you have a state of an action.
    ActionState(&'a A, S),
    /// Keeps track of `Success` <=> `Failure`.
    InvertState(Box<State<'a, A, S>>),
    /// Keeps track of an event where you wait and do nothing.
    WaitState(f64, f64),
    /// Keeps track of a `Select` event.
    SelectState(&'a Vec<Behavior<A>>, uint, Box<State<'a, A, S>>),
    /// Keeps track of an event where sub events happens sequentially.
    SequenceState(&'a Vec<Behavior<A>>, uint, Box<State<'a, A, S>>),
    /// Keeps track of an event where sub events are repeated sequentially.
    WhileState(Box<State<'a, A, S>>, &'a Vec<Behavior<A>>, uint, Box<State<'a, A, S>>),
    /// Keeps track of an event where all sub events must happen.
    WhenAllState(Vec<Option<State<'a, A, S>>>),
}

impl<'a, A: StartState<S>, S> State<'a, A, S> {
    /// Creates a state from a behavior.
    pub fn new(behavior: &'a Behavior<A>) -> State<'a, A, S> {
        match *behavior {
            Pressed(button)
                => PressedState(button),
            Released(button)
                => ReleasedState(button),
            Action(ref action)
                => ActionState(action, action.start_state()),
            Invert(ref ev)
                => InvertState(box State::new(&**ev)),
            Wait(dt)
                => WaitState(dt, 0.0),
            Select(ref sel)
                => SelectState(sel, 0, box State::new(&sel[0])),
            Sequence(ref seq)
                => SequenceState(seq, 0, box State::new(&seq[0])),
            While(ref ev, ref rep)
                => WhileState(box State::new(&**ev), rep, 0, box State::new(&rep[0])),
            WhenAll(ref all)
                => WhenAllState(all.iter().map(
                    |ev| Some(State::new(ev))).collect()),
        }
    }

    /// Updates the cursor that tracks an event.
    ///
    /// The action need to return status and remaining delta time.
    /// Returns status and the remaining delta time.
    pub fn update(
        &mut self,
        e: &Event,
        f: |dt: f64, action: &'a A, state: &mut S| -> (Status, f64)
    ) -> (Status, f64) {
        match (e, self) {
            (&Input(input::Press(button_pressed)), &PressedState(button))
            if button_pressed == button => {
                // Button press is considered to happen instantly.
                // There is no remaining delta time because this is input event.
                (Success, 0.0)
            },
            (&Input(input::Release(button_released)), &ReleasedState(button))
            if button_released == button => {
                // Button release is considered to happen instantly.
                // There is no remaining delta time because this is input event.
                (Success, 0.0)
            },
            (&Update(UpdateArgs { dt }), &ActionState(action, ref mut state)) => {
                // Call the function that updates the state.
                f(dt, action, state)
            },
            (_, &InvertState(ref mut cur)) => {
                // Invert `Success` <=> `Failure`.
                match cur.update(e, |dt, action, state| f(dt, action, state)) {
                    (Running, dt) => (Running, dt),
                    (Failure, dt) => (Success, dt),
                    (Success, dt) => (Failure, dt),
                }
            },
            (&Update(UpdateArgs { dt }), &WaitState(wait_t, ref mut t)) => {
                if *t + dt >= wait_t {
                    let remaining_dt = *t + dt - wait_t;
                    *t = wait_t;
                    (Success, remaining_dt)
                } else {
                    *t += dt;
                    (Running, 0.0)
                }
            },
            (_, &SelectState(
                seq,
                ref mut i,
                ref mut cursor
            )) => {
                let mut remaining_dt = match *e {
                        Update(UpdateArgs { dt }) => dt,
                        _ => 0.0,
                    };
                let mut remaining_e;
                while *i < seq.len() {
                    match cursor.update(
                        match *e {
                            Update(_) => {
                                remaining_e = Update(UpdateArgs { dt: remaining_dt });
                                &remaining_e
                            }
                            _ => e
                        },
                        |dt, action, state| f(dt, action, state)) {
                        (Success, x) => { return (Success, x) }
                        (Running, _) => { break }
                        (Failure, new_dt) => { remaining_dt = new_dt }
                    };
                    *i += 1;
                    // If end of sequence,
                    // return the 'dt' that is left.
                    if *i >= seq.len() { return (Failure, remaining_dt); }
                    // Create a new cursor for next event.
                    // Use the same pointer to avoid allocation.
                    **cursor = State::new(&seq[*i]);
                }
                (Running, 0.0)
            },
            (_, &SequenceState(
                seq,
                ref mut i,
                ref mut cursor
            )) => {
                let cur = cursor;
                let mut remaining_dt = match *e {
                        Update(UpdateArgs { dt }) => dt,
                        _ => 0.0,
                    };
                let mut remaining_e;
                while *i < seq.len() {
                    match cur.update(match *e {
                            Update(_) => {
                                remaining_e = Update(UpdateArgs { dt: remaining_dt });
                                &remaining_e
                            }
                            _ => e
                        },
                        |dt, action, state| f(dt, action, state)) {
                        (Failure, x) => return (Failure, x),
                        (Running, _) => { break },
                        (Success, new_dt) => {
                            remaining_dt = match *e {
                                // Change update event with remaining delta time.
                                Update(_) => new_dt,
                                // Other events are 'consumed' and not passed to next.
                                // If this is the last event, then the sequence succeeded.
                                _ => if *i == seq.len() - 1 {
                                        return (Success, new_dt)
                                    } else {
                                        return (Running, 0.0)
                                    }
                            }
                        }
                    };
                    *i += 1;
                    // If end of sequence,
                    // return the 'dt' that is left.
                    if *i >= seq.len() { return (Success, remaining_dt); }
                    // Create a new cursor for next event.
                    // Use the same pointer to avoid allocation.
                    **cur = State::new(&seq[*i]);
                }
                (Running, 0.0)
            },
            (_, &WhileState(
                ref mut ev_cursor,
                rep,
                ref mut i,
                ref mut cursor
            )) => {
                // If the event terminates, do not execute the loop.
                match ev_cursor.update(e, |dt, action, state| f(dt, action, state)) {
                    (Running, _) => {}
                    x => return x,
                };
                let cur = cursor;
                let mut remaining_dt = match *e {
                        Update(UpdateArgs { dt }) => dt,
                        _ => 0.0,
                    };
                let mut remaining_e;
                loop {
                    match cur.update(match *e {
                            Update(_) => {
                                remaining_e = Update(UpdateArgs { dt: remaining_dt });
                                &remaining_e
                            }
                            _ => e
                        },
                        |dt, action, state| f(dt, action, state)) {
                        (Failure, x) => return (Failure, x),
                        (Running, _) => { break },
                        (Success, new_dt) => {
                            remaining_dt = match *e {
                                // Change update event with remaining delta time.
                                Update(_) => new_dt,
                                // Other events are 'consumed' and not passed to next.
                                _ => return (Running, 0.0)
                            }
                        }
                    };
                    *i += 1;
                    // If end of repeated events,
                    // start over from the first one.
                    if *i >= rep.len() { *i = 0; }
                    // Create a new cursor for next event.
                    // Use the same pointer to avoid allocation.
                    **cur = State::new(&rep[*i]);
                }
                (Running, 0.0)
            },
            (_, &WhenAllState(ref mut cursors)) => {
                // Get the least delta time left over.
                let mut min_dt = std::f64::MAX_VALUE;
                // Count number of terminated events.
                let mut terminated = 0;
                for cur in cursors.mut_iter() {
                    match *cur {
                        None => terminated += 1,
                        Some(ref mut cur) => {
                            match cur.update(
                                e,
                                |dt, action, state| f(dt, action, state)
                            ) {
                                (Running, _) => {},
                                (Failure, new_dt) => return (Failure, new_dt),
                                (Success, new_dt) => {
                                    min_dt = min_dt.min(new_dt);
                                    terminated += 1;
                                }
                            }
                        }
                    }
                }
                match terminated {
                    // If there are no events, there is a whole 'dt' left.
                    0 if cursors.len() == 0 => (Success, match *e {
                            Update(UpdateArgs { dt }) => dt,
                            // Other kind of events happen instantly.
                            _ => 0.0
                        }),
                    // If all events terminated, the least delta time is left.
                    n if cursors.len() == n => (Success, min_dt),
                    _ => (Running, 0.0)
                }
            },
            _ => (Running, 0.0)
        }
    }
}
