# Piston Tutorial

## Introduction

Piston is a game engine focusing on user friendliness, back-end agnostic interface and research game development using the Rust language. With Piston you should be able get something up on the screen quickly, make a prototype, load images and sounds, polish it to look like a real game and port it to other platforms. Our goal is to provide the tools for creative game development, build the best 2D graphics API for Rust and make it easy to use with other libraries for game development in the Rust community. 3D can be achieved through OpenGL.

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

## Getting started

Linux or Mac OSX with developer tools:

1. Clone https://github.com/PistonDevelopers/piston-workspace and follow the instructions to build Piston
2. Copy 'Makefile' from https://github.com/bvssvni/rust-empty and type `make exe` to set up project
3. Copy symlinks from the piston-symlinks folder in piston-workspace to the target/cpu-vendor-os/lib folder

## The `Game` trait

## The `AssetStore` object

### Loading images

### Loading sounds

## Rendering

### Drawing rectangles and ellipses

### Drawing lines

### Drawing images

### Transformations

### Drawing with OpenGL

## User input

### Handling keyboard events

### Handling mouse events

## What's next?

