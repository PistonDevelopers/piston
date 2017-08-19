# Piston Guide

## Getting Help

*Notice! Piston is  being actively developed. Please let us know if anything in this tutorial is outdated.*

If you experience problems with content covered in this tutorial, or would like help in general, ask on the [#rust-gamedev](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-gamedev) IRC channel or open an issue in the [Piston](https://github.com/PistonDevelopers/piston/) repository.

## Introduction

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

To install Rust and Cargo (Rust's package manager) on your machine visit this page and install the appropriate package for your platform.

* [Install Rust](https://www.rust-lang.org/en-US/install.html)

Once you're done, try these commands to confirm that each have been installed correctly.

`rustc --version`
`cargo --version`

Things are moving fast in Rust-land, so be sure to keep your machine up to date.

## Hello World

[Getting Started](https://github.com/PistonDevelopers/Piston-Tutorials/tree/master/getting-started)

Now that you have Rust and Cargo installed, check out the Getting Started tutorial above.
It will take you through creating your first Piston project, rendering a box to a window, all in Rust!

## Next Steps

### More of Piston in Action

[Piston-Examples](https://github.com/pistondevelopers/piston-examples)
To see small samples of Piston's features, check out the Piston Examples repo.

[Piston-Tutorials](https://github.com/PistonDevelopers/Piston-Tutorials)
You're already familiar with our Tutorials repo. Check it out for more in-depth guides to creating projects.

### Piston Community

* Join the [#rust-gamedev](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-gamedev) IRC channel.
* Browse current issues in the [Piston](https://github.com/PistonDevelopers/piston/issues) repo.

### Contributing

Check out How To Contribute [How To Contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md).

### Show Us Your Games

[Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston)

We'd love to add whatever you make with Piston to the examples.

You can edit the page when you are added as a collaborator to the Piston project.
Alternatively, you can also open up an issue to get your game submitted.
