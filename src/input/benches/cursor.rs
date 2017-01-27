#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{CursorEvent, Input};

#[bench]
fn bench_input_cursor(bencher: &mut Bencher) {
    let e = Input::Cursor(false);
    bencher.iter(|| { let _: Option<Input> = CursorEvent::from_cursor(true, &e); });
}
