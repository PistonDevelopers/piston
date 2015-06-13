
#![feature(test)]

extern crate test;
extern crate event;
extern crate input;

use test::Bencher;
use event::{ Event, ResizeEvent };
use input::Input;

#[bench]
fn bench_input_resize(bencher: &mut Bencher) {
    let e = Input::Resize(0, 0);
    bencher.iter(|| {
        let _: Option<Input> = ResizeEvent::from_width_height(100, 100, &e);
    });
}

#[bench]
fn bench_event_resize(bencher: &mut Bencher) {
    let e = Event::Input(Input::Resize(0, 0));
    bencher.iter(|| {
        let _: Option<Event> = ResizeEvent::from_width_height(100, 100, &e);
    });
}
