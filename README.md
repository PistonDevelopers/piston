Piston
======

A user friendly game engine written in Rust

[Tutorial](https://github.com/PistonDevelopers/piston/blob/master/learning%20materials/tutorial.md)

[List of games made with Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston)

[Piston online docs](http://www.piston.rs/docs/piston/piston)

[How to contribute](https://github.com/PistonDevelopers/piston/issues/70)

| Dependency | Online Docs |
|---------|------|------------|
| [rust-graphics](https://github.com/PistonDevelopers/rust-graphics) | [rust-graphics docs](http://www.piston.rs/docs/rust-graphics/graphics) |
| [rust-image](https://github.com/PistonDevelopers/rust-image) | [rust-image docs](http://www.piston.rs/docs/rust-image/image/)
| [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) | [rust-sdl2 docs](http://www.piston.rs/docs/rust-sdl2/sdl2/) |
| [rust-sdl2_mixer](https://github.com/andelf/rust-sdl2_mixer) | [rust-sdl2_mixer docs](http://www.piston.rs/docs/rust-sdl2_mixer/sdl2_mixer/) |
| [glfw-rs](https://github.com/bjz/glfw-rs) | [glfw-rs docs](http://www.piston.rs/docs/glfw-rs/glfw/) |
| [gl-rs](https://github.com/bjz/gl-rs) | [gl-rs docs](http://www.piston.rs/docs/gl-rs/gl/) |
| [hgl-rs](https://github.com/cmr/hgl-rs) | [hgl-rs docs](http://www.piston.rs/docs/hgl-rs/hgl/) |

## Start new project with Piston

If you are starting a new project, [Rust-Empty](https://github.com/bvssvni/rust-empty) will automate the setup of directories for you.

When you have set up your project, you can build Piston.
Piston can either be compiled as a static or dynamic library (or both).
One way to simplify working with multiple projects on the same machine is to symlink the '.rlib' directly into the next project with the command `ln -s <from> <to>`.

## How to build Piston

You can now use https://github.com/PistonDevelopers/piston-workspace repository to download and build Piston without having to set up symlinks manually.

## Add libraries to your project

After building Piston, you need to add the '.rlib' files to your '/target/cpu-vendor-os/lib/' folder.

The folder is created first time you build. If you do not see the folder, type `make exe`.

## Goals

2D will be first priority, but we encourage people to experiment with 3D as well.

* Test the design and performance of Rust-Graphics
* Experiment with Rust-ish game design and collect feedback
* Experiment with actor/events AI modelling
* Experiment with multiplayer architectures
* Make more people use Rust for game development
* Multi-platform game development
