
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{ Event, UpdateArgs, UpdateEvent };

#[bench]
fn bench_event_update(bencher: &mut Bencher) {
    let e = Event::Update(UpdateArgs { dt: 0.0 });
    let args = UpdateArgs { dt: 1.0 };
    bencher.iter(|| {
        let _: Option<Event> = UpdateEvent::from_update_args(&args, &e);
    });
}
