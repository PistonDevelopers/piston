#![feature(test)]

extern crate input;
extern crate test;

use input::{ControllerAxisArgs, ControllerAxisEvent, Input, Motion};
use test::Bencher;

#[bench]
fn bench_input_controller_axis(bencher: &mut Bencher) {
    let e = Input::Move(Motion::ControllerAxis(ControllerAxisArgs {
        id: 0,
        axis: 0,
        position: 0.0,
    }));
    bencher.iter(|| {
        let _: Option<Input> = ControllerAxisEvent::from_controller_axis_args(
            ControllerAxisArgs {
                id: 0,
                axis: 0,
                position: 1.0,
            },
            &e,
        );
    });
}
