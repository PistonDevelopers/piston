
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{Input, UpdateArgs, UpdateEvent};

#[bench]
fn bench_input_update(bencher: &mut Bencher) {
    let e = Input::Update(UpdateArgs { dt: 0.0 });
    let args = UpdateArgs { dt: 1.0 };
    bencher.iter(|| { let _: Option<Input> = UpdateEvent::from_update_args(&args, &e); });
}
