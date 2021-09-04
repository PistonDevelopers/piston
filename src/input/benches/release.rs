#![feature(test)]

extern crate input;
extern crate test;

use input::{Button, Input, Key, ReleaseEvent};
use test::Bencher;

#[bench]
fn bench_input_release(bencher: &mut Bencher) {
    let e = Input::Release(Button::Keyboard(Key::S));
    let button = Button::Keyboard(Key::A);
    bencher.iter(|| {
        let _: Option<Input> = ReleaseEvent::from_button(button, &e);
    });
}
