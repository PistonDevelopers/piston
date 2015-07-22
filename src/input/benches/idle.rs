
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{ Event, IdleArgs, IdleEvent };

#[bench]
fn bench_event_idle(bencher: &mut Bencher) {
    let e = Event::Idle(IdleArgs { dt: 1.0 });
    let args = IdleArgs {
        dt: 1.0,
    };
    bencher.iter(|| {
        let _: Option<Event> = IdleEvent::from_idle_args(&args, &e);
    });
}
