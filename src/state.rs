
use std;
use piston::{
    Event,
    Input,
    Update,
    UpdateArgs,
};
use piston::input;
use {
    Action,
    AlwaysSucceed,
    Behavior,
    Failure,
    If,
    Not,
    Pressed,
    Released,
    Running,
    Select,
    Sequence,
    Status,
    Success,
    Wait,
    WaitForever,
    WhenAll,
    While,
};

/// Keeps track of a behavior.
#[deriving(Clone)]
pub enum State<A> {
    /// Returns `Success` when button is pressed.
    PressedState(input::Button),
    /// Returns `Success` when button is released.
    ReleasedState(input::Button),
    /// Executes an action.
    ActionState(A),
    /// Converts `Success` into `Failure` and vice versa.
    NotState(Box<State<A>>),
    /// Ignores failures and always return `Success`.
    AlwaysSucceedState(Box<State<A>>),
    /// Number of seconds we should wait and seconds we have waited.
    WaitState(f64, f64),
    /// Waits forever.
    WaitForeverState,
    /// Keeps track of an `If` behavior.
    /// If status is `Running`, then it evaluates the condition.
    /// If status is `Success`, then it evaluates the success behavior.
    /// If status is `Failure`, then it evaluates the failure behavior.
    IfState(Box<Behavior<A>>, Box<Behavior<A>>, Status, Box<State<A>>),
    /// Keeps track of a `Select` behavior.
    SelectState(Vec<Behavior<A>>, uint, Box<State<A>>),
    /// Keeps track of an `Sequence` behavior.
    SequenceState(Vec<Behavior<A>>, uint, Box<State<A>>),
    /// Keeps track of a `While` behavior.
    WhileState(Box<State<A>>, Vec<Behavior<A>>, uint, Box<State<A>>),
    /// Keeps track of an `WhenAll` behavior.
    WhenAllState(Vec<Option<State<A>>>),
}

impl<A: Clone> State<A> {
    /// Creates a state from a behavior.
    pub fn new(behavior: Behavior<A>) -> State<A> {
        match behavior {
            Pressed(button) => PressedState(button),
            Released(button) => ReleasedState(button),
            Action(action) => ActionState(action),
            Not(ev) => NotState(box State::new(*ev)),
            AlwaysSucceed(ev) => AlwaysSucceedState(box State::new(*ev)),
            Wait(dt) => WaitState(dt, 0.0),
            WaitForever => WaitForeverState,
            If(condition, success, failure) => {
                let state = State::new(*condition);
                IfState(success, failure, Running, box state)
            }
            Select(sel) => {
                let state = State::new(sel[0].clone());
                SelectState(sel, 0, box state)
            }
            Sequence(seq) => {
                let state = State::new(seq[0].clone());
                SequenceState(seq, 0, box state)
            }
            While(ev, rep) => {
                let state = State::new(rep[0].clone());
                WhileState(box State::new(*ev), rep, 0, box state)
            }
            WhenAll(all)
                => WhenAllState(all.move_iter().map(
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
        f: |dt: f64, action: &A| -> (Status, f64)
    ) -> (Status, f64) {
        match (e, self) {
            (&Input(input::Press(button_pressed)), &PressedState(button))
            if button_pressed == button => {
                // Button press is considered to happen instantly.
                // There is no remaining delta time because this is input event.
                (Success, 0.0)
            }
            (&Input(input::Release(button_released)), &ReleasedState(button))
            if button_released == button => {
                // Button release is considered to happen instantly.
                // There is no remaining delta time because this is input event.
                (Success, 0.0)
            }
            (&Update(UpdateArgs { dt }), &ActionState(ref action)) => {
                // Execute action.
                f(dt, action)
            }
            (_, &NotState(ref mut cur)) => {
                match cur.update(e, f) {
                    (Running, dt) => (Running, dt),
                    (Failure, dt) => (Success, dt),
                    (Success, dt) => (Failure, dt),
                }
            }
            (_, &AlwaysSucceedState(ref mut cur)) => {
                match cur.update(e, f) {
                    (Running, dt) => (Running, dt),
                    (_, dt) => (Success, dt),
                }
            }
            (&Update(UpdateArgs { dt }), &WaitState(wait_t, ref mut t)) => {
                if *t + dt >= wait_t {
                    let remaining_dt = *t + dt - wait_t;
                    *t = wait_t;
                    (Success, remaining_dt)
                } else {
                    *t += dt;
                    (Running, 0.0)
                }
            }
            (_, &IfState(ref success, ref failure,
                         ref mut status, ref mut state)) => {
                let mut remaining_dt = match *e {
                        Update(UpdateArgs { dt }) => dt,
                        _ => 0.0,
                    };
                let mut remaining_e;
                // Run in a loop to evaluate success or failure with
                // remaining delta time after condition.
                loop {
                    *status = match *status {
                        Running => {
                            match state.update(e, |dt, a| f(dt, a)) {
                                (Running, dt) => { return (Running, dt); },
                                (Success, dt) => {
                                    **state = State::new((**success).clone());
                                    remaining_dt = dt;
                                    Success
                                }
                                (Failure, dt) => {
                                    **state = State::new((**failure).clone());
                                    remaining_dt = dt;
                                    Failure
                                }
                            }
                        }
                        _ => {
                            return state.update(match *e {
                                Update(_) => {
                                    remaining_e = Update(UpdateArgs {
                                            dt: remaining_dt
                                        });
                                    &remaining_e
                                }
                                _ => e
                            }, |dt, a| f(dt, a));
                        }
                    }
                }
            }
            (_, &SelectState(
                ref seq,
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
                                remaining_e = Update(UpdateArgs {
                                        dt: remaining_dt
                                    });
                                &remaining_e
                            }
                            _ => e
                        },
                        |dt, a| f(dt, a)) {
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
                    **cursor = State::new(seq[*i].clone());
                }
                (Running, 0.0)
            }
            (_, &SequenceState(ref seq, ref mut i, ref mut cursor)) => {
                let cur = cursor;
                let mut remaining_dt = match *e {
                        Update(UpdateArgs { dt }) => dt,
                        _ => 0.0,
                    };
                let mut remaining_e;
                while *i < seq.len() {
                    match cur.update(match *e {
                            Update(_) => {
                                remaining_e = Update(UpdateArgs {
                                        dt: remaining_dt
                                    });
                                &remaining_e
                            }
                            _ => e
                        },
                        |dt, a| f(dt, a)) {
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
                    **cur = State::new(seq[*i].clone());
                }
                (Running, 0.0)
            }
            (_, &WhileState(ref mut ev_cursor, ref rep, ref mut i,
                            ref mut cursor)) => {
                // If the event terminates, do not execute the loop.
                match ev_cursor.update(e, |dt, a| f(dt, a)) {
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
                                remaining_e = Update(UpdateArgs {
                                        dt: remaining_dt
                                    });
                                &remaining_e
                            }
                            _ => e
                        },
                        |dt, a| f(dt, a)) {
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
                    **cur = State::new(rep[*i].clone());
                }
                (Running, 0.0)
            }
            (_, &WhenAllState(ref mut cursors)) => {
                // Get the least delta time left over.
                let mut min_dt = std::f64::MAX_VALUE;
                // Count number of terminated events.
                let mut terminated = 0;
                for cur in cursors.mut_iter() {
                    match *cur {
                        None => terminated += 1,
                        Some(ref mut cur) => {
                            match cur.update(e, |dt, a| f(dt, a)) {
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
            }
            _ => (Running, 0.0)
        }
    }
}
