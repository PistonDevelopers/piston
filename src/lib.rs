#![crate_name = "piston"]
#![deny(missing_docs)]
#![warn(dead_code)]
#![feature(default_type_params)]
#![feature(globs)]
#![feature(if_let)]

//! A user friendly game engine written in Rust.

extern crate sync;
extern crate gfx;
extern crate gfx_graphics;
extern crate opengl_graphics;
extern crate sdl2;
extern crate sdl2_window;
extern crate window;

// Crates used to reexport.
extern crate "vecmath" as vecmath_lib;
extern crate "shader_version" as shader_version_lib;
extern crate "image" as image_lib;
extern crate "graphics" as graphics_lib;
extern crate "input" as input_lib;
extern crate "event" as event_lib;
extern crate "cam" as cam_lib;
extern crate "noise" as noise_lib;
extern crate "genmesh" as genmesh_lib;
extern crate "sprite" as sprite_lib;
extern crate "current" as current_lib;
extern crate "fps_counter" as fps_counter_lib;
extern crate "wavefront-obj" as wavefront_obj_lib;
extern crate "drag_controller" as drag_controller_lib;
extern crate "read_color" as read_color_lib;
extern crate "select_color" as select_color_lib;
extern crate "texture_packer" as texture_packer_lib;
extern crate "wire" as wire_lib;
extern crate "astar" as astar_lib;
extern crate "img_hash" as img_hash_lib;
extern crate "nalgebra" as nalgebra_lib;
extern crate "ncollide" as ncollide_lib;

// Reexports.
pub use shader_version_lib as shader_version;
pub use image_lib as image;
pub use graphics_lib as graphics;
pub use vecmath_lib as vecmath;
pub use input_lib as input;
pub use event_lib as event;
pub use cam_lib as cam;
pub use noise_lib as noise;
pub use genmesh_lib as genmesh;
pub use sprite_lib as sprite;
pub use current_lib as current;
pub use fps_counter_lib as fps_counter;
pub use wavefront_obj_lib as wavefront_obj;
pub use drag_controller_lib as drag_controller;
pub use texture_packer_lib as texture_packer;
pub use wire_lib as wire;
pub use astar_lib as astar;
pub use img_hash_lib as img_hash;
pub use nalgebra_lib as nalgebra;
pub use ncollide_lib as ncollide;

pub use sdl2_window::Sdl2Window as WindowBackEnd;
pub use event::{
    Event,
    Events,
    NoWindow,
    RenderArgs,
    UpdateArgs,
    Window,
    WindowSettings,
};
pub use current::{
    Get,
    Set,
    Modifier,
    Current,
    CurrentGuard,
};

use gfx_graphics::G2D;
use opengl_graphics::Gl;
use fps_counter::FPSCounter;
use gfx::{ DeviceHelper };

pub mod color {
    //! Rexported libraries for working with colors
    pub use read_color_lib as read_color;
    pub use select_color_lib as select_color;
}


/// Initializes window and sets up current objects.
pub fn start(
    opengl: shader_version::opengl::OpenGL,
    window_settings: WindowSettings,
    f: ||
) {
    let mut window = WindowBackEnd::new(
        opengl,
        window_settings,
    );

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let mut gl = Gl::new(opengl);
    let mut g2d = G2D::new(&mut device);
    let mut renderer = device.create_renderer();
    let event::window::Size([w, h]) = window.get();
    let mut frame = gfx::Frame::new(w as u16, h as u16);
    let mut fps_counter = FPSCounter::new();

    let window_guard = CurrentGuard::new(&mut window);
    let device_guard = CurrentGuard::new(&mut device);
    let gl_guard = CurrentGuard::new(&mut gl);
    let g2d_guard = CurrentGuard::new(&mut g2d);
    let renderer_guard = CurrentGuard::new(&mut renderer);
    let frame_guard = CurrentGuard::new(&mut frame);
    let fps_counter_guard = CurrentGuard::new(&mut fps_counter);

    f();

    drop(window_guard);
    drop(device_guard);
    drop(gl_guard);
    drop(g2d_guard);
    drop(renderer_guard);
    drop(frame_guard);
    drop(fps_counter_guard);
}

/// The current window
pub unsafe fn current_window() -> Current<WindowBackEnd> { Current }
/// The current Gfx device
pub unsafe fn current_gfx_device() -> Current<gfx::GlDevice> { Current }
/// The current opengl_graphics back-end
pub unsafe fn current_gl() -> Current<Gl> { Current }
/// The current gfx_graphics back-end
pub unsafe fn current_g2d() -> Current<G2D> { Current }
/// The current Gfx renderer
pub unsafe fn current_renderer() -> Current<gfx::Renderer<gfx::GlCommandBuffer>> { Current }
/// The current Gfx frame
pub unsafe fn current_frame() -> Current<gfx::Frame> { Current }
/// The current FPS counter
pub unsafe fn current_fps_counter() -> Current<FPSCounter> { Current }

/// Returns an event iterator for the event loop
pub fn events() -> event::Events<Current<WindowBackEnd>> {
    unsafe {
        Events::new(current_window())
    }
}

/// Updates the FPS counter and gets the frames per second.
pub fn fps_tick() -> uint {
    unsafe {
        current_fps_counter().tick()
    }
}

/// Sets title of the current window.
pub fn set_title(text: String) {
    unsafe {
        current_window().set_mut(window::Title(text));
    }
}

/// Renders 2D graphics using Gfx.
pub fn render_2d_gfx(
    bg_color: Option<[f32, ..4]>, 
    f: |&graphics::Context, 
        &mut gfx_graphics::GraphicsBackEnd<gfx::GlCommandBuffer>|
) {
    use gfx::Device;    

    unsafe {
        current_g2d().draw(
            &mut *current_renderer(),
            &*current_frame(), 
            |c, g| {
                use graphics::*;
                if let Some(bg_color) = bg_color {
                    c.color(bg_color).draw(g);
                }
                f(&c, g);
            });
        current_gfx_device().submit(current_renderer().as_buffer());
        current_renderer().reset();
    }
}

/// Renders 2D graphics using OpenGL.
pub fn render_2d_opengl(
    bg_color: Option<[f32, ..4]>,
    f: |&graphics::Context,
        &mut opengl_graphics::Gl|
) {
    unsafe {
        use graphics::*;
        let gl = &mut *current_gl();
        let window::Size([w, h]) = current_window().get();
        gl.viewport(0, 0, w as i32, h as i32);
        gl.clear_program();
        let c = Context::abs(w as f64, h as f64);
        if let Some(bg_color) = bg_color {
            c.color(bg_color).draw(gl);
        }
        f(&c, gl);
    }
}

