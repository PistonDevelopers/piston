
#![feature(test)]

extern crate test;
extern crate input;

use test::Bencher;
use input::{Input, AfterRenderArgs, AfterRenderEvent};

#[bench]
fn bench_input_after_render(bencher: &mut Bencher) {
    let e = Input::AfterRender(AfterRenderArgs);
    let args = AfterRenderArgs;
    bencher.iter(|| {
        let _: Option<Input> = AfterRenderEvent::from_after_render_args(&args, &e);
    });
}
