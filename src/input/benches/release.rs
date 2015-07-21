
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{ Button, Event, Key, Input, ReleaseEvent };

#[bench]
fn bench_input_release(bencher: &mut Bencher) {
    let e = Input::Release(Button::Keyboard(Key::S));
    let button = Button::Keyboard(Key::A);
    bencher.iter(|| {
        let _: Option<Input> = ReleaseEvent::from_button(button, &e);
    });
}

#[bench]
fn bench_event_release(bencher: &mut Bencher) {
    let e = Event::Input(Input::Release(Button::Keyboard(Key::S)));
    let button = Button::Keyboard(Key::A);
    bencher.iter(|| {
        let _: Option<Event> = ReleaseEvent::from_button(button, &e);
    });
}
