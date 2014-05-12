Piston
======

A user friendly game engine written in Rust

Test project: [rust-snake](https://github.com/bvssvni/rust-snake)

[Piston online docs](http://bvssvni.github.io/docs/piston/piston/)  

| Dependency | Online Docs |
|---------|------|------------|
| [rust-graphics](https://github.com/bvssvni/rust-graphics) | [rust-graphics docs](http://bvssvni.github.io/docs/rust-graphics/graphics/) |
| [glfw-rs](https://github.com/bjz/glfw-rs) | [glfw-rs docs](http://bvssvni.github.io/docs/glfw-rs/glfw/) |
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

* Test the design and performance of Rust-Graphics
* Experiment with Rust-ish game design and collect feedback
* Actor/events AI modelling
* Make more people use Rust for game development
* Multi-platform game development
