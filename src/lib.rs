#![crate_name = "piston"]
#![deny(missing_docs)]
#![warn(dead_code)]
#![feature(default_type_params)]
#![feature(globs)]

//! A user friendly game engine written in Rust.

#[cfg(feature = "include_gfx")]
extern crate gfx;
#[cfg(feature = "include_gfx")]
extern crate gfx_graphics;
extern crate opengl_graphics;
extern crate sdl2;
extern crate sdl2_window;

// Crates used to reexport.
extern crate "ai_behavior" as ai_behavior_lib;
extern crate "vecmath" as vecmath_lib;
extern crate "shader_version" as shader_version_lib;
extern crate "image" as image_lib;
extern crate "graphics" as graphics_lib;
extern crate "input" as input_lib;
extern crate "event" as event_lib;
extern crate "window" as window_lib;
extern crate "cam" as cam_lib;
extern crate "current" as current_lib;
extern crate "fps_counter" as fps_counter_lib;
extern crate "drag_controller" as drag_controller_lib;
extern crate "read_color" as read_color_lib;
extern crate "select_color" as select_color_lib;

// Reexports.
pub use current_lib as current;
pub use ai_behavior_lib as ai_behavior;
pub use shader_version_lib as shader_version;
pub use image_lib as image;
pub use graphics_lib as graphics;
pub use vecmath_lib as vecmath;
pub use input_lib as input;
pub use event_lib as event;
pub use window_lib as window;
pub use cam_lib as cam;
pub use fps_counter_lib as fps_counter;
pub use drag_controller_lib as drag_controller;

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

#[cfg(feature = "include_gfx")]
use gfx_graphics::G2D;
#[cfg(feature = "include_gfx")]
use gfx::{ DeviceHelper };
use opengl_graphics::Gl;
use fps_counter::FPSCounter;

pub mod color {
    //! Rexported libraries for working with colors
    pub use read_color_lib as read_color;
    pub use select_color_lib as select_color;
}

fn start_window(
    opengl: shader_version::OpenGL,
    window_settings: WindowSettings,
    f: ||
) {
    let mut window = WindowBackEnd::new(
        opengl,
        window_settings,
    );

    let mut gl = Gl::new(opengl);
    let mut fps_counter = FPSCounter::new();

    let window_guard = CurrentGuard::new(&mut window);
    let gl_guard = CurrentGuard::new(&mut gl);
    let fps_counter_guard = CurrentGuard::new(&mut fps_counter);

    f();

    drop(window_guard);
    drop(gl_guard);
    drop(fps_counter_guard);
}

#[cfg(feature = "include_gfx")]
fn start_gfx(f: ||) {
    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let mut g2d = G2D::new(&mut device);
    let mut renderer = device.create_renderer();
    let event::window::Size([w, h]) = window.get(); 
    let mut frame = gfx::Frame::new(w as u16, h as u16);

    let device_guard = CurrentGuard::new(&mut device);
    let g2d_guard = CurrentGuard::new(&mut g2d);
    let renderer_guard = CurrentGuard::new(&mut renderer);
    let frame_guard = CurrentGuard::new(&mut frame);

    f();
    
    drop(g2d_guard);
    drop(renderer_guard);
    drop(frame_guard);
    drop(device_guard);
}

#[cfg(not(feature = "include_gfx"))]
fn start_gfx(f: ||) {
    f();
}

/// Initializes window and sets up current objects.
pub fn start(
    opengl: shader_version::OpenGL,
    window_settings: WindowSettings,
    f: ||
) {
    start_window(opengl, window_settings, || {
        if cfg!(feature = "include_gfx") {
            start_gfx(|| f());
        } else {
            f();
        }
    });
}

/// The current window
pub unsafe fn current_window() -> Current<WindowBackEnd> { Current::new() }
/// The current Gfx device
#[cfg(feature = "include_gfx")]
pub unsafe fn current_gfx_device() -> Current<gfx::GlDevice> { Current::new() }
/// The current opengl_graphics back-end
pub unsafe fn current_gl() -> Current<Gl> { Current::new() }
/// The current gfx_graphics back-end
#[cfg(feature = "include_gfx")]
pub unsafe fn current_g2d() -> Current<G2D> { Current::new() }
/// The current Gfx renderer
#[cfg(feature = "include_gfx")]
pub unsafe fn current_renderer() -> Current<gfx::Renderer<gfx::GlCommandBuffer>> { Current::new() }
/// The current Gfx frame
#[cfg(feature = "include_gfx")]
pub unsafe fn current_frame() -> Current<gfx::Frame> { Current::new() }
/// The current FPS counter
pub unsafe fn current_fps_counter() -> Current<FPSCounter> { Current::new() }

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

/// Returns true if the current window should be closed.
pub fn should_close() -> bool {
    use window::ShouldClose;

    unsafe {
        let ShouldClose(val) = current_window().get();
        val
    }
}

/// Renders 2D graphics using Gfx.
///
/// ### DANGER
///
/// This function should not be called nested within the closure.
/// Doing so will lead to mutable aliased references to the graphics back-end.
#[cfg(feature = "include_gfx")]
pub fn render_2d_gfx(
    _: current::DANGER,
    bg_color: Option<[f32, ..4]>, 
    f: |graphics::Context, 
        &mut gfx_graphics::GraphicsBackEnd<gfx::GlCommandBuffer>|
) {
    use gfx::Device;    

    unsafe {
        current_g2d().draw(
            &mut *current_renderer(),
            &*current_frame(), 
            |c, g| {
                if let Some(bg_color) = bg_color {
                    graphics::clear(bg_color, g);
                }
                f(c, g);
            });
        current_gfx_device().submit(current_renderer().as_buffer());
        current_renderer().reset();
    }
}

/// Renders 2D graphics using OpenGL.
///
/// ### DANGER
///
/// This function should not be called nested within the closure.
/// Doing so will lead to mutable aliased references to the graphics back-end.
pub fn render_2d_opengl(
    _: current::DANGER,
    bg_color: Option<[f32, ..4]>,
    f: |graphics::Context,
        &mut opengl_graphics::Gl|
) {
    unsafe {
        let gl = &mut *current_gl();
        let window::Size([w, h]) = current_window().get();
        gl.draw([0, 0, w as i32, h as i32], |c, g| {
            use graphics::*;
            if let Some(bg_color) = bg_color {
                graphics::clear(bg_color, g);
            }
            f(c, g);
        });
    }
}

