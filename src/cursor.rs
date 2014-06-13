
use std;
use {
    Event,
    StartState,
};

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
    WhenAllCursor(Vec<Option<Cursor<'a, A, S>>>),
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
            WhenAllCursor(
                ref mut cursors
            ) => {
                // Get the least delta time left over.
                let mut min_dt = std::f64::MAX_VALUE;
                // Count number of terminated events.
                let mut terminated = 0;
                for cur in cursors.mut_iter() {
                    match *cur {
                        None => terminated += 1,
                        Some(ref mut cur) => {
                            match cur.update(
                                dt,
                                |action, state| f(action, state)
                            ) {
                                None => {},
                                Some(new_dt) => {
                                    min_dt = min_dt.min(new_dt);
                                    terminated += 1;
                                }
                            }
                        }
                    }
                }
                match terminated {
                    // If there are no events, there is a whole 'dt' left.
                    0 if cursors.len() == 0 => Some(dt),
                    // If all events terminated, the least delta time is left.
                    n if cursors.len() == n => Some(min_dt),
                    _ => None
                }
            }
        }
    }
}

