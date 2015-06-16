
#![feature(test)]

extern crate test;
extern crate event;
extern crate input;

use test::Bencher;
use event::{ Event, FocusEvent };
use input::Input;

#[bench]
fn bench_input_focus(bencher: &mut Bencher) {
    let e = Input::Focus(false);
    bencher.iter(|| {
        let _: Option<Input> = FocusEvent::from_focused(true, &e);
    });
}

#[bench]
fn bench_event_focus(bencher: &mut Bencher) {
    let e = Event::Input(Input::Focus(false));
    bencher.iter(|| {
        let _: Option<Event> = FocusEvent::from_focused(true, &e);
    });
}
