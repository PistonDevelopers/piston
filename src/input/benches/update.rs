#![feature(test)]

extern crate input;
extern crate test;

use input::{Input, UpdateArgs, UpdateEvent};
use test::Bencher;

#[bench]
fn bench_input_update(bencher: &mut Bencher) {
    let e = Input::Update(UpdateArgs { dt: 0.0 });
    let args = UpdateArgs { dt: 1.0 };
    bencher.iter(|| {
        let _: Option<Input> = UpdateEvent::from_update_args(&args, &e);
    });
}
