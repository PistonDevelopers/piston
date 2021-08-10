#![crate_name = "piston"]
#![deny(
    rust_2018_compatibility,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    unused,
    clippy::all,
    clippy::doc_markdown,
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations
)]

//! A modular game engine written in Rust.
//!
//! This is the core library of the Piston Game engine.
//! The `Piston` core library reexports the core modules.
//!
//! If you are looking for a convenient window wrapper,
//! see [`piston_window`](https://github.com/pistondevelopers/piston_window).
//!
//! For examples, see [piston-examples](https://github.com/pistondevelopers/piston-examples).
//!
//! For more information and an overview, see [Piston's README in the core repository](https://github.com/pistondevelopers/piston).
//!
//! ### Design
//!
//! The Piston core is a thin and modular abstraction for user input, window and event loop.
//! This functionality is separated into 3 core modules.
//!
//! The core modules are intended to be used directly by generic libraries.
//! By depending directly on core modules, it is easier to maintain the ecosystem.
//!
//! This library is intended to be used in application code.
//! When you write application code, it is common to separate reusable code,
//! which depends on various abstractions, from platform specific code.
//! The reusable code that you write for applications might use the Piston core.
//!
//! The default programming pattern in Piston is Model-View-Controller:
//!
//! - A controller handles events and manipulates a model
//! - A view renders a model on the screen
//!
//! For more information about this pattern, see [Model-View-Controller (Wikipedia)](https://en.wikipedia.org/wiki/Model%E2%80%93view%E2%80%93controller).
//!
//! The most important traits in Piston are the following:
//!
//! - [`GenericEvent`](input::GenericEvent) (allows handling of events for controllers)
//! - [Window](window::Window) (allows polling of events)
//!
//! ### Link to documentation for core modules
//!
//! - [pistoncore-input](https://docs.rs/pistoncore-input)
//!   (User input and event handling)
//! - [pistoncore-window](https://docs.rs/pistoncore-window)
//!   (Window abstraction)
//! - [pistoncore-event_loop](https://docs.rs/pistoncore-event_loop)
//!   (Event loop)
//!
//! ### Points vs Pixels
//!
//! Since some computer screens have higher resolution than others,
//! it is convenient to use two kinds of coordinate systems:
//!
//! - A pixel is a single square on the screen
//! - A point is a unit used by window events and 2D graphics
//!
//! For example, the mouse cursor position events are measured in points.
//!
//! It is common to use points for 2D graphics to match window coordinates.
//!
//! Unintentional blurring, e.g. of rendered text, might be a side effect incorrect sampling.
//!
//! ### About Piston as a Game Engine
//!
//! Piston is a modular game engine with a minimal core abstraction.
//! The core connects input events, window and event loop.
//!
//! Piston is designed for optimal modularity,
//! making it optional to even use the core modules in many cases.
//! The goal is to have as little abstraction as possible,
//! while making larger libraries as independent as possible.
//! The motivation is to encourage diversity and experimentation with various abstractions,
//! without getting tied up to a fixed set of platforms, abstractions or vendors.
//! You can combine Piston with any other library in Rust's ecosystem.
//! This design has worked very well so far.
//!
//! For example (a few libraries, there are many more):
//!
//! - [Image](https://github.com/pistondevelopers/image) library is standalone
//! from both the core and the 2D graphics library,
//! only connected through the 2D graphics backends.
//! - [Piston's 2D graphics](https://github.com/pistondevelopers/graphics) is optional and can be used without a window backend.
//! The window backend can be used without a 2D graphics backend, and so on.
//! - For image processing, see [Imageproc](https://github.com/pistondevelopers/imageproc).
//! - [Dyon](https://github.com/pistondevelopers/dyon) is a Rusty dynamically typed scripting language,
//! using a lifetime checker without garbage collection.
//!
//! For more information and an overview, see [Piston's README in the core repository](https://github.com/pistondevelopers/piston).
//!
//! When writing a library, please depend directly on the core module needed.
//! This makes it less likely that the library will break.
//!
//! When writing an application, it is acceptable to use the `Piston` core.
//! To use it you usually need a window backend:
//!
//! - [pistoncore-glutin_window](https://github.com/pistondevelopers/glutin_window)
//! - [pistoncore-sdl2_window](https://github.com/pistondevelopers/sdl2_window)
//! - [pistoncore-glfw_window](https://github.com/pistondevelopers/glfw_window)
//!
//! There are a [few other window backends as well](https://crates.io/search?q=piston%20window).
//!
//! Plus a 2D graphics backend (optional):
//!
//! - [piston2d-opengl_graphics](https://github.com/pistondevelopers/opengl_graphics)
//! - [piston2d-gfx_graphics](https://github.com/pistondevelopers/gfx_graphics)
//! - [piston2d-glium_graphics](https://github.com/pistondevelopers/glium_graphics)
//!
//! There are a [few other graphics backends as well](https://crates.io/search?q=piston%20graphics).
//!
//! You will find examples of how to get started in each 2D graphics backend repository.
//!
//! ### About Piston as a Project
//!
//! The Piston project is a huge collaboration across many projects,
//! mainly focused on maintenance of libraries and research.
//! Since this has been going on since 2014, there is too much out there
//! to summarize here, but roughly the project is organized into two open source organizations:
//!
//! - [`PistonDevelopers`](https://github.com/pistondevelopers/) - everything game engine related
//! - [`AdvancedResearch`](https://github.com/advancedresearch/) - everything advanced math related
//!
//! In addition, we collaborate across organizations with other projects, mainly:
//!
//! - [Gfx-rs](https://github.com/gfx-rs/) - everything 3D graphics related
//! - [`RustAudio`](https://github.com/rustaudio) - everything audio related
//!
//! In addition, there are many other projects and organizations.
//!
//! For more information and an overview, see [Piston's README in the core repository](https://github.com/pistondevelopers/piston).

// Reexported crates.
pub use event_loop::{self, *};
pub use input::{self, *};
pub use window::{self, *};
