#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{ Event, CursorEvent, Input };

#[bench]
fn bench_input_cursor(bencher: &mut Bencher) {
    let e = Input::Cursor(false);
    bencher.iter(|| {
        let _: Option<Input> = CursorEvent::from_cursor(true, &e);
    });
}

#[bench]
fn bench_event_cursor(bencher: &mut Bencher) {
    let e = Event::Input(Input::Cursor(false));
    bencher.iter(|| {
        let _: Option<Event> = CursorEvent::from_cursor(true, &e);
    });
}
