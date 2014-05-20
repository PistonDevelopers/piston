rust-event
==========

An experimental library using context types for event logic

Examples
--------

Maps "press event" with a certain keyboard key:
```
e.keyboard().press(keyboard::Up).map(back_end, || {
    println!("Oops! You pressed keyboard::Up");
});
```

Maps "pressing and lasting certain time event" with a keyboard key:
```
e.keyboard().pressing(keyboard::Up).lasting(1.0).map(back_end, || {
    println!("Wooooooow! You are pressing keybaord::Up at least 1.0 second!!");
});
```

Dependences
-----------
| Dependency | Online Docs |
|---------|------|------------|
| [rust-graphics](https://github.com/PistonDevelopers/rust-graphics) | [rust-graphics docs](http://bvssvni.github.io/docs/rust-graphics/graphics/) |
| [piston](https://github.com/PistonDevelopers/piston) | [piston docs](http://bvssvni.github.io/docs/piston/piston/) |

