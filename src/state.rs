
use std;
use input;
use {
    Action,
    After,
    AlwaysSucceed,
    Behavior,
    Failure,
    GenericEvent,
    If,
    Fail,
    Pressed,
    PressEvent,
    Released,
    ReleaseEvent,
    Running,
    Select,
    Sequence,
    Status,
    Success,
    UpdateEvent,
    Wait,
    WaitForever,
    WhenAll,
    WhenAny,
    While,
};

pub static RUNNING: (Status, f64) = (Running, 0.0);

/// Keeps track of a behavior.
#[deriving(Clone, PartialEq)]
pub enum State<A, S> {
    /// Returns `Success` when button is pressed.
    PressedState(input::Button),
    /// Returns `Success` when button is released.
    ReleasedState(input::Button),
    /// Executes an action.
    ActionState(A, Option<S>),
    /// Converts `Success` into `Failure` and vice versa.
    FailState(Box<State<A, S>>),
    /// Ignores failures and always return `Success`.
    AlwaysSucceedState(Box<State<A, S>>),
    /// Keeps track of waiting for a period of time before continuing.
    ///
    /// f64: Total time in seconds to wait
    ///
    /// f64: Time elapsed in seconds
    WaitState(f64, f64),
    /// Waits forever.
    WaitForeverState,
    /// Keeps track of an `If` behavior.
    /// If status is `Running`, then it evaluates the condition.
    /// If status is `Success`, then it evaluates the success behavior.
    /// If status is `Failure`, then it evaluates the failure behavior.
    IfState(Box<Behavior<A>>, Box<Behavior<A>>, Status, Box<State<A, S>>),
    /// Keeps track of a `Select` behavior.
    SelectState(Vec<Behavior<A>>, uint, Box<State<A, S>>),
    /// Keeps track of an `Sequence` behavior.
    SequenceState(Vec<Behavior<A>>, uint, Box<State<A, S>>),
    /// Keeps track of a `While` behavior.
    WhileState(Box<State<A, S>>, Vec<Behavior<A>>, uint, Box<State<A, S>>),
    /// Keeps track of a `WhenAll` behavior.
    WhenAllState(Vec<Option<State<A, S>>>),
    /// Keeps track of a `WhenAny` behavior.
    WhenAnyState(Vec<Option<State<A, S>>>),
    /// Keeps track of an `After` behavior.
    AfterState(uint, Vec<State<A, S>>),
}

// `Sequence` and `Select` share same algorithm.
//
// `Sequence` fails if any fails and succeeds when all succeeds.
// `Select` succeeds if any succeeds and fails when all fails.
fn sequence<A: Clone, S, E: GenericEvent>(
    select: bool,
    upd: Option<f64>,
    seq: &Vec<Behavior<A>>,
    i: &mut uint,
    cursor: &mut Box<State<A, S>>,
    e: &E,
    f: |e: &E, dt: f64, action: &A, state: &mut Option<S>| -> (Status, f64)
) -> (Status, f64) {
    let (status, inv_status) = if select {
        // `Select`
        (Failure, Success)
    } else {
        // `Sequence`
        (Success, Failure)
    };
    let mut remaining_dt = upd.unwrap_or(0.0);
    let mut remaining_e;
    while *i < seq.len() {
        match cursor.event(
            match upd {
                Some(_) => {
                    remaining_e = UpdateEvent::from_dt(remaining_dt).unwrap();
                    &remaining_e
                }
                _ => e
            },
            |e, dt, a, s| f(e, dt, a, s)) {
            (Running, _) => { break; },
            (s, new_dt) if s == inv_status => {
                return (inv_status, new_dt);
            }
            (s, new_dt) if s == status => {
                remaining_dt = match upd {
                    // Change update event with remaining delta time.
                    Some(_) => new_dt,
                    // Other events are 'consumed' and not passed to next.
                    // If this is the last event, then the sequence succeeded.
                    _ => if *i == seq.len() - 1 {
                            return (status, new_dt)
                        } else {
                            return RUNNING
                        }
                }
            }
            _ => unreachable!()
        };
        *i += 1;
        // If end of sequence,
        // return the 'dt' that is left.
        if *i >= seq.len() { return (status, remaining_dt); }
        // Create a new cursor for next event.
        // Use the same pointer to avoid allocation.
        **cursor  = State::new(seq[*i].clone());
    }
    RUNNING
}

// `WhenAll` and `WhenAny` share same algorithm.
//
// `WhenAll` fails if any fails and succeeds when all succeeds.
// `WhenAny` succeeds if any succeeds and fails when all fails.
fn when_all<A: Clone, S, E: GenericEvent>(
    any: bool,
    upd: Option<f64>,
    cursors: &mut Vec<Option<State<A, S>>>,
    e: &E,
    f: |e: &E, dt: f64, action: &A, state: &mut Option<S>| -> (Status, f64)
) -> (Status, f64) {
    let (status, inv_status) = if any {
        // `WhenAny`
        (Failure, Success)
    } else {
        // `WhenAll`
        (Success, Failure)
    };
    // Get the least delta time left over.
    let mut min_dt = std::f64::MAX_VALUE;
    // Count number of terminated events.
    let mut terminated = 0;
    for cur in cursors.iter_mut() {
        match *cur {
            None => {}
            Some(ref mut cur) => {
                match cur.event(e, |e, dt, a, s| f(e, dt, a, s)) {
                    (Running, _) => { continue; },
                    (s, new_dt) if s == inv_status => {
                        // Fail for `WhenAll`.
                        // Succeed for `WhenAny`.
                        return (inv_status, new_dt);
                    }
                    (s, new_dt) if s == status => {
                        min_dt = min_dt.min(new_dt);
                    }
                    _ => unreachable!()
                }
            }
        }

        terminated += 1;
        *cur = None;
    }
    match terminated {
        // If there are no events, there is a whole 'dt' left.
        0 if cursors.len() == 0 => (status, match upd {
                Some(dt) => dt,
                // Other kind of events happen instantly.
                _ => 0.0
            }),
        // If all events terminated, the least delta time is left.
        n if cursors.len() == n => (status, min_dt),
        _ => RUNNING
    }
}

impl<A: Clone, S> State<A, S> {
    /// Creates a state from a behavior.
    pub fn new(behavior: Behavior<A>) -> State<A, S> {
        match behavior {
            Pressed(button) => PressedState(button),
            Released(button) => ReleasedState(button),
            Action(action) => ActionState(action, None),
            Fail(ev) => FailState(box State::new(*ev)),
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
                => WhenAllState(all.into_iter().map(
                    |ev| Some(State::new(ev))).collect()),
            WhenAny(all)
                => WhenAnyState(all.into_iter().map(
                    |ev| Some(State::new(ev))).collect()),
            After(seq)
                => AfterState(0, seq.into_iter().map(
                    |ev| State::new(ev)).collect()),
        }
    }

    /// Updates the cursor that tracks an event.
    ///
    /// The action need to return status and remaining delta time.
    /// Returns status and the remaining delta time.
    pub fn event<E: GenericEvent>(
        &mut self,
        e: &E,
        f: |e: &E, dt: f64, action: &A, state: &mut Option<S>| -> (Status, f64)
    ) -> (Status, f64) {
        let upd = e.update(|args| Some(args.dt)).unwrap_or(None);
        match (upd, self) {
            (None, &PressedState(button)) => {
                e.press(|button_pressed| {
                    if button_pressed != button { return RUNNING; }

                    // Button press is considered to happen instantly.
                    // There is no remaining delta time because
                    // this is input event.
                    (Success, 0.0)
                }).unwrap_or(RUNNING)
            }
            (None, &ReleasedState(button)) => {
                e.release(|button_released| {
                    if button_released != button { return RUNNING; }

                    // Button release is considered to happen instantly.
                    // There is no remaining delta time because
                    // this is input event.
                    (Success, 0.0)
                }).unwrap_or(RUNNING)
            }
            (_, &ActionState(ref action, ref mut state)) => {
                // Execute action.
                f(e, upd.unwrap_or(0.0), action, state)
            }
            (_, &FailState(ref mut cur)) => {
                match cur.event(e, f) {
                    (Running, dt) => (Running, dt),
                    (Failure, dt) => (Success, dt),
                    (Success, dt) => (Failure, dt),
                }
            }
            (_, &AlwaysSucceedState(ref mut cur)) => {
                match cur.event(e, f) {
                    (Running, dt) => (Running, dt),
                    (_, dt) => (Success, dt),
                }
            }
            (Some(dt), &WaitState(wait_t, ref mut t)) => {
                if *t + dt >= wait_t {
                    let remaining_dt = *t + dt - wait_t;
                    *t = wait_t;
                    (Success, remaining_dt)
                } else {
                    *t += dt;
                    RUNNING
                }
            }
            (_, &IfState(ref success, ref failure,
                         ref mut status, ref mut state)) => {
                let mut remaining_dt = upd.unwrap_or(0.0);
                let mut remaining_e;
                // Run in a loop to evaluate success or failure with
                // remaining delta time after condition.
                loop {
                    *status = match *status {
                        Running => {
                            match state.event(e, |e, dt, a, s| f(e, dt, a, s)) {
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
                            return state.event(match upd {
                                Some(_) => {
                                    remaining_e = UpdateEvent::from_dt(
                                        remaining_dt).unwrap();
                                    &remaining_e
                                }
                                _ => e
                            }, |e, dt, a, s| f(e, dt, a, s));
                        }
                    }
                }
            }
            (_, &SelectState(ref seq, ref mut i, ref mut cursor)) => {
                let select = true;
                sequence(select, upd, seq, i, cursor, e, f)
            }
            (_, &SequenceState(ref seq, ref mut i, ref mut cursor)) => {
                let select = false;
                sequence(select, upd, seq, i, cursor, e, f)
            }
            (_, &WhileState(ref mut ev_cursor, ref rep, ref mut i,
                            ref mut cursor)) => {
                // If the event terminates, do not execute the loop.
                match ev_cursor.event(e, |e, dt, a, s| f(e, dt, a, s)) {
                    (Running, _) => {}
                    x => return x,
                };
                let cur = cursor;
                let mut remaining_dt = upd.unwrap_or(0.0);
                let mut remaining_e;
                loop {
                    match cur.event(match upd {
                            Some(_) => {
                                remaining_e = UpdateEvent::from_dt(
                                    remaining_dt).unwrap();
                                &remaining_e
                            }
                            _ => e
                        },
                        |e, dt, a, s| f(e, dt, a, s)) {
                        (Failure, x) => return (Failure, x),
                        (Running, _) => { break },
                        (Success, new_dt) => {
                            remaining_dt = match upd {
                                // Change update event with remaining delta time.
                                Some(_) => new_dt,
                                // Other events are 'consumed' and not passed to next.
                                _ => return RUNNING
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
                RUNNING
            }
            (_, &WhenAllState(ref mut cursors)) => {
                let any = false;
                when_all(any, upd, cursors, e, f)
            }
            (_, &WhenAnyState(ref mut cursors)) => {
                let any = true;
                when_all(any, upd, cursors, e, f)
            }
            (_, &AfterState(ref mut i, ref mut cursors)) => {
                // Get the least delta time left over.
                let mut min_dt = std::f64::MAX_VALUE;
                for j in range(*i, cursors.len()) {
                    match cursors.get_mut(j).event(
                        e, |e, dt, a, s| f(e, dt, a, s)
                    ) {
                        (Running, _) => { min_dt = 0.0; }
                        (Success, new_dt) => {
                            // Remaining delta time must be less to succeed.
                            if *i == j && new_dt < min_dt {
                                *i += 1;
                                min_dt = new_dt;
                            } else {
                                // Return least delta time because
                                // that is when failure is detected.
                                return (Failure, min_dt.min(new_dt));
                            }
                        }
                        (Failure, new_dt) => {
                            return (Failure, new_dt);
                        }
                    };
                }
                if *i == cursors.len() {
                    (Success, min_dt)
                } else {
                    RUNNING
                }
            }
            _ => RUNNING
        }
    }
}
