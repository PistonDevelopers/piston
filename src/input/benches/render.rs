#![feature(test)]

extern crate input;
extern crate test;

use input::{Input, RenderArgs, RenderEvent};
use test::Bencher;

#[bench]
fn bench_input_render(bencher: &mut Bencher) {
    let e = Input::Render(RenderArgs {
        ext_dt: 0.0,
        width: 0,
        height: 0,
        draw_width: 0,
        draw_height: 0,
    });
    let args = RenderArgs {
        ext_dt: 1.0,
        width: 10,
        height: 10,
        draw_width: 10,
        draw_height: 10,
    };
    bencher.iter(|| {
        let _: Option<Input> = RenderEvent::from_render_args(&args, &e);
    });
}
