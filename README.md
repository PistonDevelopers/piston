# Piston [![Build Status](https://travis-ci.org/PistonDevelopers/piston.svg)](https://travis-ci.org/PistonDevelopers/piston) [![Crates.io](https://img.shields.io/crates/v/piston.svg?style=flat-square)](https://crates.io/crates/piston) [![Crates.io](https://img.shields.io/crates/l/piston.svg)](https://github.com/PistonDevelopers/piston/blob/master/LICENSE)

A modular game engine written in Rust

Maintainers: @bvssvni

[Examples](https://github.com/pistondevelopers/piston-examples)

[List of features](https://github.com/PistonDevelopers/piston/issues/668)

[List of games made with Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston)

[Piston online docs](http://docs.piston.rs/piston/piston/)

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)

## Start new project with Piston

You should know how to build "hello world" with Rust, see http://www.rust-lang.org/.

[How to install FreeType](https://github.com/PistonDevelopers/piston/issues/912)
Piston uses FreeType for font rendering.

### Drawing a red rectangle

Add [piston_window](https://crates.io/crates/piston_window) to your Cargo.toml, for example:

```
[dependencies]
piston_window = "0.15.0"
```

In "src/main.rs", type the following code:

```Rust
extern crate piston_window;

use piston_window::*;

fn main() {
    let window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
```

Use `cargo run` to start the application. It should clear the screen in white color and draw a red rectangle.

![red-rectangle](./images/red-rectangle.png)

## Goals

The Piston project is a large collaboration among many developers.
There are libraries for 2D, 3D, event programming, AI, image processing etc.
By sharing the maintenance, we get more time to build new stuff.

Piston is as much a community project as it is a collection of libraries.
Writing and maintaining code is expensive, and by sharing this cost we reach our goals faster.
We believe that seeking personal goals and ambitions, while helping each other, results in higher quality.

* Our main goal is to free up time for maintainers and the people involved
* ... such that we can create new amazing stuff and reach our personal goals
* ... by making more people use Rust for game development and become engaged in open source

In addition we do research or plan to in the following areas:

* Graphics, 2D and 3D
* Idiomatic Rust game design
* Interactive applications
* AI programming
* Animation
* Sound and music
* Network

### Dependency graph

![dependencies](./Cargo.png)

