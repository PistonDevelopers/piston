# Piston [![Build Status](https://travis-ci.org/PistonDevelopers/piston.svg)](https://travis-ci.org/PistonDevelopers/piston)

A user friendly game engine written in Rust

Maintainers: @bvssvni, @Coeuvre

[Examples](https://github.com/pistondevelopers/piston-examples)

[List of games made with Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston)

[Piston online docs](http://www.rust-ci.org/PistonDevelopers/piston/doc/piston/)

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)

| Back-ends |
|--------------------|
| [sdl2_game_window](https://github.com/pistondevelopers/sdl2_game_window) |
| [glfw_game_window](https://github.com/pistondevelopers/glfw_game_window) |

## Start new project with Piston

If you are starting a new project, [Rust-Empty](https://github.com/bvssvni/rust-empty) will automate the setup of directories for you.

When you have set up your project, you can build Piston.
Piston can either be compiled as a static or dynamic library (or both).
One way to simplify working with multiple projects on the same machine is to symlink the '.rlib' directly into the next project with the command `ln -s <from> <to>`.

## How to build Piston

1. Install [Cargo](https://github.com/rust-lang/cargo)
2. Open up the Terminal window and type:

```
git clone https://github.com/PistonDevelopers/piston
cd piston
cargo build
```

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
