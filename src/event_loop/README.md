event_loop [![Crates.io](https://img.shields.io/crates/v/piston.svg?style=flat-square)](https://crates.io/crates/piston)
==========

A generic event loop for games and interactive applications

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

