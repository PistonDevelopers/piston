#![feature(globs)]

extern crate collections;

extern crate piston;
extern crate event;

use piston::*;
use event::*;

pub struct App {
    number: int,
    e: Event,
    ec: EventCenter,
}

impl App {
    pub fn new() -> App {
        App {
            number: 0,
            e: Event::new(),
            ec: EventCenter::new(),
        }
    }
}

impl Game for App {
    fn load(&mut self, _asset_store: &mut AssetStore) {
        self.e.press(&keyboard::Left).call(&mut self.ec, || {
            println!("Oops! You pressed keyboard::Left");
        });

        self.e.press(&keyboard::Right).release().call(&mut self.ec, || {
            println!("Oops! You release keyboard::Right");
        });

        self.e.press(&mouse::Left).call(&mut self.ec, || {
            println!("Oops! You pressed mouse::Left");
        });

        self.e.interval(10.0).call(&mut self.ec, || {
            println!("ELAPSED 10.0 SECOND");
        });

        self.e.interval(20.0).call_once(&mut self.ec, || {
            println!("ELAPSED 20.0 SECOND, AND THIS WILL BE CALLED ONLY ONCE!!!");
        });

        let key_up = keyboard::Up;
        let key_down = keyboard::Down;
        let a = self.e.press(&key_up);
        let b = self.e.press(&key_down);
        let b = b.release();
        self.e.any([&a as &Triggered, &b as &Triggered]).call(&mut self.ec, || {
            println!("Wow! You pressed keyboard::Up OR released keyboard::Down");
        });
    }

    fn update(&mut self, dt: f64, _asset_store: &mut AssetStore) {
        self.ec.update(dt);
    }


    // re-wrap those events to Event, a good way to do this is in the GameWindow
    // implementions.
    fn key_press(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        self.ec.receive_event(&event::KeyPressed(key));
    }

    fn key_release(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        self.ec.receive_event(&event::KeyReleased(key));
    }

    fn mouse_press(
        &mut self,
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        self.ec.receive_event(&event::MouseButtonPressed(button));
    }

    fn mouse_release(
        &mut self,
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        self.ec.receive_event(&event::MouseButtonReleased(button));
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

    app.run(&mut game_window, &mut asset_store);
}

