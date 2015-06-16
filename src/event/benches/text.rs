
#![feature(test)]

extern crate test;
extern crate event;
extern crate input;

use test::Bencher;
use event::{ Event, TextEvent };
use input::Input;

#[bench]
fn bench_input_text(bencher: &mut Bencher) {
    let e = Input::Text("".to_string());
    bencher.iter(|| {
        let _: Option<Input> = TextEvent::from_text("hello", &e);
    });
}

#[bench]
fn bench_event_text(bencher: &mut Bencher) {
    let e = Event::Input(Input::Text("".to_string()));
    bencher.iter(|| {
        let _: Option<Event> = TextEvent::from_text("hello", &e);
    });
}
