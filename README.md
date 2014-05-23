rust-event
==========

An experimental library using context types for event logic

Examples
--------

Call a command when press with a certain keyboard key:
```
e.press(keyboard::Up).call(event_center, || {
    println!("Oops! You pressed keyboard::Up");
});
```

Dependences
-----------
`rust-event` is a pure rust library, but can be used with following library:
| Library | Online Docs |
|---------|-------------|
| [piston](https://github.com/PistonDevelopers/piston) | [piston docs](http://pistondevelopers.github.io/docs/piston/piston/) |

