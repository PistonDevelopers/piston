#![feature(test)]

extern crate input;
extern crate test;

use input::{FocusEvent, Input};
use test::Bencher;

#[bench]
fn bench_input_focus(bencher: &mut Bencher) {
    let e = Input::Focus(false);
    bencher.iter(|| {
        let _: Option<Input> = FocusEvent::from_focused(true, &e);
    });
}
