
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{FocusEvent, Input};

#[bench]
fn bench_input_focus(bencher: &mut Bencher) {
    let e = Input::Focus(false);
    bencher.iter(|| { let _: Option<Input> = FocusEvent::from_focused(true, &e); });
}
