#![feature(test)]

extern crate input;
extern crate test;

use input::{Button, Input, Key, PressEvent};
use test::Bencher;

#[bench]
fn bench_input_press(bencher: &mut Bencher) {
    let e = Input::Press(Button::Keyboard(Key::S));
    let button = Button::Keyboard(Key::A);
    bencher.iter(|| {
        let _: Option<Input> = PressEvent::from_button(button, &e);
    });
}
