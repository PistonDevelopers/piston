#![crate_name = "piston"]
#![deny(missing_docs)]
#![warn(dead_code)]

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
extern crate "quack" as quack_lib;
extern crate "fps_counter" as fps_counter_lib;
extern crate "drag_controller" as drag_controller_lib;
extern crate "read_color" as read_color_lib;
extern crate "select_color" as select_color_lib;

// Reexports.
pub use current_lib as current;
pub use quack_lib as quack;
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
    WindowSettings,
};

pub use quack::{
    Action,
    ActOn,
    Get,
    GetFrom,
    Set,
    SetAt,
};
pub use current::{
    Current,
    CurrentGuard,
};

#[cfg(feature = "include_gfx")]
use gfx_graphics::G2D;
#[cfg(feature = "include_gfx")]
use gfx::{ DeviceHelper };
use opengl_graphics::Gl;
use fps_counter::FPSCounter;

use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;

pub mod color {
    //! Rexported libraries for working with colors
    pub use read_color_lib as read_color;
    pub use select_color_lib as select_color;
}

fn start_window<F>(
    opengl: shader_version::OpenGL,
    window_settings: WindowSettings,
    mut f: F
)
    where
        F: FnMut()
{
    let mut window = Rc::new(RefCell::new(WindowBackEnd::new(
        opengl,
        window_settings,
    )));
    let mut gl = Rc::new(RefCell::new(Gl::new(opengl)));
    let mut fps_counter = Rc::new(RefCell::new(FPSCounter::new()));

    let window_guard = CurrentGuard::new(&mut window);
    let gl_guard = CurrentGuard::new(&mut gl);
    let fps_counter_guard = CurrentGuard::new(&mut fps_counter);

    f();

    drop(window_guard);
    drop(gl_guard);
    drop(fps_counter_guard);
}

#[cfg(feature = "include_gfx")]
fn start_gfx<F>(mut f: F)
    where
        F: FnMut()
{
    let window = current_window();

    let mut device = Rc::new(RefCell::new(gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    })));
    let mut g2d = Rc::new(RefCell::new(G2D::new(&mut *device.borrow_mut())));
    let mut renderer = Rc::new(RefCell::new(device.borrow_mut().create_renderer()));
    let event::window::Size([w, h]) = window.get(); 
    let mut frame = Rc::new(RefCell::new(gfx::Frame::new(w as u16, h as u16)));

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
fn start_gfx<F>(mut f: F)
    where
        F: FnMut()
{
    f();
}

/// Initializes window and sets up current objects.
pub fn start<F>(
    opengl: shader_version::OpenGL,
    window_settings: WindowSettings,
    mut f: F
)
    where
        F: FnMut()
{
    start_window(opengl, window_settings, || {
        if cfg!(feature = "include_gfx") {
            start_gfx(|| f());
        } else {
            f();
        }
    });
}

/// The current window
pub fn current_window() -> Rc<RefCell<WindowBackEnd>> {
    unsafe {
        Current::<Rc<RefCell<WindowBackEnd>>>::new().clone()
    }
}
/// The current Gfx device
#[cfg(feature = "include_gfx")]
pub fn current_gfx_device() -> Rc<RefCell<gfx::GlDevice>> {
    unsafe {
        Current::<Rc<RefCell<gfx::GlDevice>>>::new().clone()
    }
}
/// The current opengl_graphics back-end
pub fn current_gl() -> Rc<RefCell<Gl>> {
    unsafe {
        Current::<Rc<RefCell<Gl>>>::new().clone()
    }
}
/// The current gfx_graphics back-end
#[cfg(feature = "include_gfx")]
pub fn current_g2d() -> Rc<RefCell<G2D>> {
    unsafe {
        Current::<Rc<RefCell<G2D>>>::new().clone()
    }
}
/// The current Gfx renderer
#[cfg(feature = "include_gfx")]
pub fn current_renderer() -> Rc<RefCell<gfx::Renderer<gfx::GlCommandBuffer>>> {
    unsafe {
        Current::<Rc<RefCell<gfx::Renderer<gfx::GlCommandBuffer>>>>::new().clone()
    }
}
/// The current Gfx frame
#[cfg(feature = "include_gfx")]
pub fn current_frame() -> Rc<RefCell<gfx::Frame>> {
    unsafe {
        Current::<Rc<RefCell<gfx::Frame>>>::new().clone()
    }
}
/// The current FPS counter
pub fn current_fps_counter() -> Rc<RefCell<FPSCounter>> {
    unsafe {
        Current::<Rc<RefCell<FPSCounter>>>::new().clone()
    }
}

/// Returns an event iterator for the event loop
pub fn events() -> event::Events<Rc<RefCell<WindowBackEnd>>, input::Input, event::Event> {
    event::events(current_window())
}

/// Updates the FPS counter and gets the frames per second.
pub fn fps_tick() -> usize {
    current_fps_counter().borrow_mut().tick()
}

/// Sets title of the current window.
pub fn set_title(text: String) {
    current_window().set_mut(window::Title(text));
}

/// Returns true if the current window should be closed.
pub fn should_close() -> bool {
    use window::ShouldClose;
    let ShouldClose(val) = current_window().get();
    val
}

/// Renders 2D graphics using Gfx.
#[cfg(feature = "include_gfx")]
pub fn render_2d_gfx<F>(
    bg_color: Option<[f32; 4]>, 
    mut f: F
)
    where
        F: FnMut(graphics::Context, 
            &mut gfx_graphics::GraphicsBackEnd<gfx::GlCommandBuffer>)
{
    use gfx::Device;    

    let renderer = current_renderer();
    let mut renderer = renderer.borrow_mut();
    let renderer = &mut *renderer;
    current_g2d().borrow_mut().draw(
        renderer,
        &mut *current_frame().borrow_mut(), 
        |c, g| {
            if let Some(bg_color) = bg_color {
                graphics::clear(bg_color, g);
            }
            f(c, g);
        });
    current_gfx_device().borrow_mut().submit(renderer.as_buffer());
    renderer.reset();
}

/// Renders 2D graphics using OpenGL.
///
/// Panics if called nested within the closure
/// to prevent mutable aliases to the graphics back-end.
pub fn render_2d_opengl<F>(
    bg_color: Option<[f32; 4]>,
    mut f: F
)
    where
        F: FnMut(graphics::Context, &mut opengl_graphics::Gl)
{
    use std::ops::Deref;

    let window::Size([w, h]) = current_window().borrow().deref().get();
    current_gl().borrow_mut().draw([0, 0, w as i32, h as i32], |c, g| {
        use graphics::*;
        if let Some(bg_color) = bg_color {
            graphics::clear(bg_color, g);
        }
        f(c, g);
    });
}

