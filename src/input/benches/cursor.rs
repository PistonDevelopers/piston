#![feature(test)]

extern crate input;
extern crate test;

use input::{CursorEvent, Input};
use test::Bencher;

#[bench]
fn bench_input_cursor(bencher: &mut Bencher) {
    let e = Input::Cursor(false);
    bencher.iter(|| {
        let _: Option<Input> = CursorEvent::from_cursor(true, &e);
    });
}
