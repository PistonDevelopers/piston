#![feature(globs)]

extern crate collections;

extern crate piston;
extern crate event;

use piston::*;
use event::*;

pub struct App<'a> {
    number: int,
    e: Event<'a>,
    ec: EventCenter,
}

impl<'a> App<'a> {
    pub fn new() -> App {
        App {
            number: 0,
            e: Event::new(),
            ec: EventCenter::new(),
        }
    }
}

impl<'a> Game for App<'a> {
    fn load(&mut self, _asset_store: &mut AssetStore) {
        self.e.keyboard().press(keyboard::Up).call(&mut self.ec, || {
            println!("Oops! You pressed keyboard::Up");
        });

        let e = self.e.keyboard().pressing(keyboard::Up);

        let i = e.call(&mut self.ec, || {
            println!("Wow! You are pressing keyboard::Up");
        });

        e.lasting(1.0).call(&mut self.ec, || {
            println!("Wooooooow! You are pressing keybaord::Up at least 1.0 second!!");
        });
        self.ec.remove_observer(i);

        self.e.keyboard().release(keyboard::Up).call(&mut self.ec, || {
            println!("Hmm! You released keyboard::Up");
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
        self.ec.receive_event(event::KeyPressed(key));
    }

    fn key_release(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        self.ec.receive_event(event::KeyReleased(key));
    }

    fn mouse_press(
        &mut self,
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        self.ec.receive_event(event::MouseButtonPressed(button));
    }

    fn mouse_release(
        &mut self,
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        self.ec.receive_event(event::MouseButtonReleased(button));
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

