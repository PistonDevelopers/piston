rust-event
==========

An experimental library using context types for event logic

Examples
--------

Call a command when press with a certain keyboard key:
```
e.keyboard().press(keyboard::Up).call(&mut back_end, || {
    println!("Oops! You pressed keyboard::Up");
});
```

Call a command when pressing with a keyboard key and lasting it with certain time:
```
e.keyboard().pressing(keyboard::Up).lasting(1.0).call(&mut back_end, || {
    println!("Wooooooow! You are pressing keyboard::Up at least 1.0 second!!");
});
```

Dependences
-----------
| Dependency | Online Docs |
|---------|------|------------|
| [rust-graphics](https://github.com/PistonDevelopers/rust-graphics) | [rust-graphics docs](http://bvssvni.github.io/docs/rust-graphics/graphics/) |
| [piston](https://github.com/PistonDevelopers/piston) | [piston docs](http://bvssvni.github.io/docs/piston/piston/) |

