use event::{
    Action,
    Event,
    State,
    Sequence,
    Success,
    Update,
    UpdateArgs,
    Wait,
    WhenAll,
    While,
};

/// Some test actions.
#[deriving(Clone)]
pub enum TestActions {
    /// Increment accumulator.
    Inc,
    /// Decrement accumulator.
    Dec,
}

// A test state machine that can increment and decrement.
fn exec(mut acc: u32, dt: f64, state: &mut State<TestActions, ()>) -> u32 {
    let e: Event = Update(UpdateArgs { dt: dt });
    state.event(&e, |_, dt, action, _| {
        match *action {
            Inc => { acc += 1; (Success, dt) },
            Dec => { acc -= 1; (Success, dt) },
        }
    });
    acc
}

// Each action that terminates immediately
// consumes a time of 0.0 seconds.
// This makes it possible to execute one action
// after another without delay or waiting for next update.
#[test]
fn print_2() {
    let a: u32 = 0;
    let seq = Sequence(vec![Action(Inc), Action(Inc)]);
    let mut state = State::new(seq);
    let a = exec(a, 0.0, &mut state);
    assert_eq!(a, 2);
}

// If you wait the exact amount before to execute an action,
// it will execute. This behavior makes it easy to predict
// when an action will run.
#[test]
fn wait_sec() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(1.0), Action(Inc)]);
    let mut state = State::new(seq);
    let a = exec(a, 1.0, &mut state);
    assert_eq!(a, 1);
}

// When we execute half the time and then the other half,
// then the action should be executed.
#[test]
fn wait_half_sec() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(1.0), Action(Inc)]);
    let mut state = State::new(seq);
    let a = exec(a, 0.5, &mut state);
    assert_eq!(a, 0);
    let a = exec(a, 0.5, &mut state);
    assert_eq!(a, 1);
}

// A sequence of one event is like a bare event.
#[test]
fn sequence_of_one_event() {
    let a: u32 = 0;
    let seq = Sequence(vec![Action(Inc)]);
    let mut state = State::new(seq);
    let a = exec(a, 1.0, &mut state);
    assert_eq!(a, 1);
}

// A sequence of wait events is the same as one wait event.
#[test]
fn wait_two_waits() {
    let a: u32 = 0;
    let seq = Sequence(vec![Wait(0.5), Wait(0.5), Action(Inc)]);
    let mut state = State::new(seq);
    let a = exec(a, 1.0, &mut state);
    assert_eq!(a, 1);
}

// Increase counter ten times.
#[test]
fn loop_ten_times() {
    let a: u32 = 0;
    let rep = While(box Wait(50.0), vec![Wait(0.5), Action(Inc), Wait(0.5)]);
    let mut state = State::new(rep);
    let a = exec(a, 10.0, &mut state);
    assert_eq!(a, 10);
}

#[test]
fn when_all_wait() {
    let a: u32 = 0;
    let all = Sequence(vec![
            // Wait in parallel.
            WhenAll(vec![Wait(0.5), Wait(1.0)]),
            Action(Inc)
        ]);
    let mut state = State::new(all);
    let a = exec(a, 0.5, &mut state);
    assert_eq!(a, 0);
    let a = exec(a, 0.5, &mut state);
    assert_eq!(a, 1);
}
