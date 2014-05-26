#![feature(globs)]

extern crate collections;

extern crate piston;
extern crate event;

use piston::*;
use event::*;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }
}

impl EventGame for App {
    fn register_event(&mut self, ec: &mut EventCenter) {
        let e = Event::new();

        e.press(&keyboard::Left).call(ec, || {
            println!("Oops! You pressed keyboard::Left");
        });

        e.press(&keyboard::Right).release().call(ec, || {
            println!("Oops! You release keyboard::Right");
        });

        e.press(&mouse::Left).call(ec, || {
            println!("Oops! You pressed mouse::Left");
        });

        e.interval(10.0).call(ec, || {
            println!("ELAPSED 10.0 SECOND");
        });

        e.interval(20.0).call_once(ec, || {
            println!("ELAPSED 20.0 SECOND, AND THIS WILL BE CALLED ONLY ONCE!!!");
        });

        let key_up = keyboard::Up;
        let key_down = keyboard::Down;
        let a = e.press(&key_up);
        let b = e.press(&key_down);
        let b = b.release();
        e.any([&a as &Triggered, &b as &Triggered]).call(ec, || {
            println!("Wow! You pressed keyboard::Up OR released keyboard::Down");
        });
    }
}

type GameWindowBackEnd = GameWindowSDL2;

fn main() {
    let mut game_window: GameWindowBackEnd = GameWindow::new(
        GameWindowSettings::new (
            "Piston-Lab".to_owned(),
            [300, 300],
            false,
            true,
            [1.0, 1.0, 1.0, 1.0]
        )
    );

    let mut asset_store = AssetStore::from_folder("assets");
    let mut app = App::new();

    app.run_with_event(&mut game_window, &mut asset_store);
}

