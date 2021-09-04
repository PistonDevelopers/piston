#![feature(test)]

extern crate input;
extern crate test;

use input::{Input, TextEvent};
use test::Bencher;

#[bench]
fn bench_input_text(bencher: &mut Bencher) {
    let e = Input::Text("".to_string());
    bencher.iter(|| {
        let _: Option<Input> = TextEvent::from_text("hello", &e);
    });
}
