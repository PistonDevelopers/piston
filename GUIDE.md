# Piston Guide

## Getting Help

*Notice! Piston is  being actively developed. Please let us know if anything in this tutorial is outdated.*

If you experience problems with content covered in this tutorial, or would like help in general, ask on the [#rust-gamedev](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-gamedev) IRC channel or open an issue in the [Piston](https://github.com/PistonDevelopers/piston/) repository.

## Introduction

#### [RFC]
    Someone more familiar with project would be able to give a more accurate explanation for Why Piston
### Why Piston?

Piston is a game engine focusing on user-friendliness, back-end agnostic interface and game development research using the Rust language.

With Piston you should be able get something up on the screen quickly, make a prototype, load images and sounds, polish it to look like a real game and port it to other platforms.

Our goal is to provide the tools for creative game development, build the best 2D graphics API for Rust and make it easy to use with other libraries for game development in the Rust community. 3D can be achieved through OpenGL and [Gfx](https://github.com/gfx-rs/gfx-rs).

### Why Rust?

Rust is a programming language with a focus on type safety, memory safety, concurrency and performance. It is intended for writing large-scale, high-performance software that is free from several classes of common errors. Rust has a sophisticated memory model that encourages efficient data structures and safe concurrency patterns, forbidding invalid memory accesses that would otherwise cause segmentation faults. It is statically typed and compiled ahead of time. For more information see http://www.rust-lang.org/

Some of the things that make Rust suitable for game development:

* Garbage collection is optional which makes it easier to reason about performance
* The Rust enum type is suitable for state machines which are commonly used in game logic
* Exhaustive match helps you cover all states
* Allows low level programming and reuse of existing C libraries
* Link libraries statically to create standalone executables
* Generics & traits makes it possible to build zero cost abstractions
* No null pointer exceptions
* Functional iterator patterns that are optimized at compile time
* Immutability by default which makes it easier to read Rust code
* Flexible module system for organizing large projects
* Active and friendly community with a rapidly growing open source ecosystem

## Installing Rust and Cargo

### Linux or Mac OSX (with developer tools)

First you have to set up a new Rust project for executable:

1. Create a new folder for your game project
2. Copy 'Makefile' from https://github.com/bvssvni/rust-empty
3. If `rustc --version` gives an error, type `make nightly-install` to install Rust nightly
4. In the Terminal window, type `make exe` and `make git-ignore`

You are now ready for writing Rust programs!
Before you can start coding, you need to build Piston:

#### [FIXME]
   Replace piston-workspace with instructions for installing cargo

1. Clone https://github.com/PistonDevelopers/piston-workspace and follow the instructions to build Piston
2. Copy symlinks from the 'piston-symlinks' folder in piston-workspace to your project's 'target/cpu-vendor-os/lib' folder

If you don't have the 'target' folder, use the command `make target-dir`.

You are now ready to make a game in Rust!

### Windows
#### [TODO]

## Hello World

Now that you have Rust and Cargo installed, check out the [Getting Started](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started) tutorial.
It will take you through creating your first Piston project and rendering to a window, all in Rust!

## Examples

Here are some examples showing Piston in action.

[Piston-Examples](https://github.com/pistondevelopers/piston-examples)

## Next Steps

### Piston Community

* Join the [#rust-gamedev](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-gamedev) IRC channel.
* Browse current issues in the [Piston](https://github.com/PistonDevelopers/piston/issues) repo.

### Contributing

Check out How To Contribute [How To Contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md).

### Show Us Your Games

[Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston)
We'd love to add whatever you make with Piston to the examples.
[TODO] Explain how to submit a game.

