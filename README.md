Piston
======

A user friendly game engine written in Rust

Test project: [rust-snake](https://github.com/bvssvni/rust-snake)

[Piston online docs](http://bvssvni.github.io/docs/piston/piston/)  

[How to contribute](https://github.com/PistonDevelopers/piston/issues/70)

| Dependency | Online Docs |
|---------|------|------------|
| [rust-graphics](https://github.com/bvssvni/rust-graphics) | [rust-graphics docs](http://bvssvni.github.io/docs/rust-graphics/graphics/) |
| [rust-png](https://github.com/bvssvni/rust-png) | [rust-png docs](http://bvssvni.github.io/docs/rust-png/png/) |
| [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) | [rust-sdl2 docs](http://bvssvni.github.io/docs/rust-sdl2/sdl2/) |
| [glfw-rs](https://github.com/bjz/glfw-rs) | [glfw-rs docs](http://bvssvni.github.io/docs/glfw-rs/glfw/) |
| [gl-rs](https://github.com/bjz/gl-rs) | [gl-rs docs](http://bvssvni.github.io/docs/gl-rs/gl/) |
| [rust-opengles](https://github.com/mozilla-servo/rust-opengles) | [rust-opengles docs](http://bvssvni.github.io/docs/rust-opengles/opengles/) |

## Start new project with Piston

If you are starting a new project, [Rust-Empty](https://github.com/bvssvni/rust-empty) will automate the setup of directories for you.

When you have set up your project, you can build Piston.  
Piston can either be compiled as a static or dynamic library (or both).  
One way to simplify working with multiple projects on the same machine is to symlink the '.rlib' directly into the next project with the command `ln -s <from> <to>`.  

## How to build Piston

Build the dependencies and put the '.rlib' files to '/target/cpu-vendor-os/lib/'.

In the Terminal window, navigate to the project folder and type:

```
make
```

This gives you a new '.rlib' file in the '/target/cpu-vendor-os/lib/' folder to put in your project.

## Compiling dynamic library

The default make command can be modified by setting `DEFAULT = make dylib` in the `Makefile`. 

## Goals

2D will be first priority, but we encourage people to experiment with 3D as well.  

* Test the design and performance of Rust-Graphics
* Experiment with Rust-ish game design and collect feedback
* Experiment with actor/events AI modelling
* Experiment with multiplayer architectures
* Make more people use Rust for game development
* Multi-platform game development
