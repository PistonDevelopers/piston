#![crate_name = "piston"]
#![deny(missing_docs)]
#![warn(dead_code)]

//! A modular game engine written in Rust.
//!
//! This is the core library of the Piston Game engine.
//! The `Piston` core library reexports the core modules.
//!
//! If you are looking for a convenient window wrapper,
//! see [piston_window](https://github.com/pistondevelopers/piston_window).
//!
//! For examples, see [piston-examples](https://github.com/pistondevelopers/piston-examples).
//!
//! For more information and an overview, see [Piston's README in the core repository](https://github.com/pistondevelopers/piston).
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
//! to summarize here, but rougly the project is organized into two open source organizations:
//!
//! - [PistonDevelopers](https://github.com/pistondevelopers/) - everything game engine related
//! - [AdvancedResearch](https://github.com/advancedresearch/) - everything advanced math related
//!
//! In addition we collaborate across organizations with other projects, mainly:
//!
//! - [Gfx-rs](https://github.com/gfx-rs/) - everything 3D graphics related
//! - [RustAudio](https://github.com/rustaudio) - everything audio related
//!
//! In addition there are many other projects and organizations.
//!
//! For more information and an overview, see [Piston's README in the core repository](https://github.com/pistondevelopers/piston).

// Reexported crates.
pub extern crate input;
pub extern crate event_loop;
pub extern crate window;
