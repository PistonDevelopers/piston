
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{Input, TextEvent};

#[bench]
fn bench_input_text(bencher: &mut Bencher) {
    let e = Input::Text("".to_string());
    bencher.iter(|| { let _: Option<Input> = TextEvent::from_text("hello", &e); });
}
