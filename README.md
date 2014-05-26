
rust-event
==========

An experimental library using context types for event logic

Dependences
-----------
`rust-event` is a pure rust library, but can be used with following library:

| Library | Online Docs |
|---------|------|------------|
| [piston](https://github.com/PistonDevelopers/piston) | [piston docs](http://pistondevelopers.github.io/docs/piston/piston/) |

How to integrate with Piston
----------------------------
```
impl EventGame for App {
    fn register_event(&mut self, ec: &mut EventCenter<App>) {
        // event logic goes here, see examples below.
    }

    // other methods are same with Piston except there are no long
    // `key_press`, `key_release`, `mouse_press`, `mouse_release`,
    // `mouse_move`, `mouse_relative_move`, and the update now has
    // EventCenter method is:
    fn update(&mut self, dt: f64, event_center: &mut EventCenter, asset_store: &mut AssetStore) {
        // game logic goes here.
    }
}
```

Examples
--------

Here are some examples integrated with [piston](https://github.com/PistonDevelopers/piston):

Pressed a key to modify application state:
```
e.press(keyboard::Up).call(event_center, |app| {
    app.count += 1;
    println!("Oops! You pressed keyboard::Up for {} times", app.count);
});
```

Released a key:
```
e.press(mouse::Left).release().call(event_center, |_| {
    println!("Oops! You just releaseed mouse::Left");
});
```

Time interval:
```
e.interval(1.0).call(event_center, |_| {
    println!("ELAPSED 10.0 SECOND");
});
```

Call once:
```
self.e.interval(20.0).call_once(&mut self.ec, |_| {
    println!("ELAPSED 20.0 SECOND, AND THIS WILL BE CALLED ONLY ONCE!!!");
});
```

Any events happened:
```
let key_up = keyboard::Up;
let key_down = keyboard::Down;
let a = self.e.press(&key_up);
let b = self.e.press(&key_down);
let b = b.release();
self.e.any(&[&a as &Triggered, &b as &Triggered]).call(&mut self.ec, |_| {
    println!("Wow! You pressed keyboard::Up OR released keyboard::Down");
});
```

