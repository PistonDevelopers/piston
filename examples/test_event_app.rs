#![feature(globs)]

extern crate collections;

extern crate piston;
extern crate event;

use piston::*;
use event::*;

pub struct App {
    count: int,
}

impl App {
    pub fn new() -> App {
        App {
            count: 0
        }
    }
}

impl EventGame for App {
    fn register_event(&mut self, ec: &mut EventCenter<App>) {
        let e = Event::new();

        e.press(&keyboard::Left).call(ec, |app| {
            app.count += 1;
            println!("Oops! You pressed keyboard::Left for {} times", app.count);
        });

        e.press(&keyboard::Right).release().call(ec, |_| {
            println!("Oops! You release keyboard::Right");
        });

        e.press(&mouse::Left).call(ec, |_| {
            println!("Oops! You pressed mouse::Left");
        });

        e.interval(10.0).call(ec, |_| {
            println!("ELAPSED 10.0 SECOND");
        });

        e.interval(20.0).call_once(ec, |_| {
            println!("ELAPSED 20.0 SECOND, AND THIS WILL BE CALLED ONLY ONCE!!!");
        });

        e.any([&e.press(&keyboard::Up) as &Triggered,
               &e.press(&keyboard::Down).release() as &Triggered,
              ])
         .call(ec, |_| {
             println!("Wow! You pressed keyboard::Up OR released keyboard::Down");
         });

        e.all([&e.press(&keyboard::Q) as &Triggered,
               &e.press(&keyboard::W) as &Triggered,
               &e.press(&keyboard::E) as &Triggered,
              ])
         .call(ec, |_| {
             println!("You have pressed Q, W and E!");
         });

        e.press(&keyboard::S).after(&e.press(&keyboard::A)).call(ec, |_| {
            println!("You pressed keyboard::S AFTER you pressed keyboard::A.");
        });
    }
}

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings {
            title: "Rust Event Examples".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [1.0, 1.0, 1.0, 1.0],
        }
    );

    let mut asset_store = AssetStore::from_folder("assets");
    let mut app = App::new();

    app.run(&mut game_window, &mut asset_store);
}

