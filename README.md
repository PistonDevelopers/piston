
rust-event
==========

An experimental library using context types for event logic

Dependences
-----------
`rust-event` is a pure rust library, but can be used with following library:

| Library | Online Docs |
|---------|------|------------|
| [piston](https://github.com/PistonDevelopers/piston) | [piston docs](http://pistondevelopers.github.io/docs/piston/piston/) |

Examples
--------

Here are some examples integrated with [piston](https://github.com/PistonDevelopers/piston):
```
e.press(keyboard::Up).call(event_center, || {
    println!("Oops! You pressed keyboard::Up");
});

e.press(mouse::Left).release().call(event_center, || {
    println!("Oops! You just releaseed mouse::Left");
});
```

