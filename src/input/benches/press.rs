
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{Button, Key, Input, PressEvent};

#[bench]
fn bench_input_press(bencher: &mut Bencher) {
    let e = Input::Press(Button::Keyboard(Key::S));
    let button = Button::Keyboard(Key::A);
    bencher.iter(|| { let _: Option<Input> = PressEvent::from_button(button, &e); });
}
