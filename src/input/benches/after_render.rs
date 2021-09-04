#![feature(test)]

extern crate input;
extern crate test;

use input::{AfterRenderArgs, AfterRenderEvent, Input};
use test::Bencher;

#[bench]
fn bench_input_after_render(bencher: &mut Bencher) {
    let e = Input::AfterRender(AfterRenderArgs);
    let args = AfterRenderArgs;
    bencher.iter(|| {
        let _: Option<Input> = AfterRenderEvent::from_after_render_args(&args, &e);
    });
}
