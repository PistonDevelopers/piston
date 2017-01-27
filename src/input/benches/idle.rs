
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{Input, IdleArgs, IdleEvent};

#[bench]
fn bench_input_idle(bencher: &mut Bencher) {
    let e = Input::Idle(IdleArgs { dt: 1.0 });
    let args = IdleArgs { dt: 1.0 };
    bencher.iter(|| { let _: Option<Input> = IdleEvent::from_idle_args(&args, &e); });
}
