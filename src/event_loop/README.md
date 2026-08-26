event_loop [![Crates.io](https://img.shields.io/crates/v/piston.svg?style=flat-square)](https://crates.io/crates/piston)
==========

A Piston event loop for games and interactive applications.

### Design

Piston's architecture decouples backend-specific code from abstraction.
When you design a library for the Piston ecosystem,
you write code using generics and traits, without depending on backends.
Backends implement the traits and are plugged in at application level of coding.

This design fits with Rust's type system and improves ecosystem stability.
The default programming pattern is Model-View-Controller (MVP).
However, there are no traits needed to use this pattern.

- A model is simply some data structure, database etc.
- A view is how something is rendered
- A controller stores the state, transforms input events into other events or actions, or deals with application logic in general.

Piston's philosophy is "less is more".
As much as possible of the ecosystem is decoupled from the core
and shared with the broader Rust ecosystem.
This blurs the boundary between what is Piston and what is just Rust,
but it also helps developers write the code they need for some specific
application, without inventing entire new frameworks to solve their problems.

Remember to take regular breaks, ideally with some physical exercise.

Developers who use Piston typically maintain their code bases for long periods
of time, where the main problem is reducing the costs of maintenance.
Such code bases often use multiple programming languages, not just Rust.
Patterns to solve small technical problems can reduce cognitive load,
but if they do not work well with the rest of the ecosystem,
such patterns increase maintenance costs.

You might find Piston's architecture boring,
which is a good thing: A design optimized for overall quality of life.

### Async mode

When the Cargo feature "async" is enabled,
there is an `Events::async_next` method,
using the "tokio" crate.
This can be used to run your game loop in asynchronous Rust code.

### Spin sleep

When the Cargo feature "spin_sleep" is enabled,
the event loop gets more accurate timing in synchronous Rust code,
using the "spin_sleep" crate.
However, this will also lead to higher CPU usage overall.

[How to contribute](https://github.com/PistonDevelopers/piston/blob/master/CONTRIBUTING.md)
