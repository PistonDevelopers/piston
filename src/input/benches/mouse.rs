
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{Input, Motion, MouseScrollEvent, MouseRelativeEvent, MouseCursorEvent};

#[bench]
fn bench_input_mouse_cursor(bencher: &mut Bencher) {
    let e = Input::Move(Motion::MouseCursor(0.0, 0.0));
    bencher.iter(|| { let _: Option<Input> = MouseCursorEvent::from_xy(1.0, 0.0, &e); });
}

#[bench]
fn bench_input_mouse_relative(bencher: &mut Bencher) {
    let e = Input::Move(Motion::MouseRelative(0.0, 0.0));
    bencher.iter(|| { let _: Option<Input> = MouseRelativeEvent::from_xy(1.0, 0.0, &e); });
}

#[bench]
fn bench_input_mouse_scroll(bencher: &mut Bencher) {
    let e = Input::Move(Motion::MouseScroll(0.0, 0.0));
    bencher.iter(|| { let _: Option<Input> = MouseScrollEvent::from_xy(1.0, 0.0, &e); });
}
