#![feature(globs)]

extern crate piston;
extern crate event;

use piston::*;
use event::{
    AddKeyboard,
    AddLasting,
    AddPress,
    AddPressing,

    Event,

    Map,

    BackEnd,
    Observer,
};

pub struct App<'a> {
    e: Event<'a>,
    back_end: TestBackEnd<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> App {
        App {
            e: Event::new(),
            back_end: TestBackEnd::new(),
        }
    }
}

impl<'a> Game for App<'a> {
    fn load(&mut self, _asset_store: &mut AssetStore) {
        self.e.keyboard().press(keyboard::Up).map(&mut self.back_end, || {
            println!("Oops! You pressed keyboard::Up");
        });

        let e = self.e.keyboard().pressing(keyboard::Up);
        e.map(&mut self.back_end, || {
            println!("Wow! You are pressing keyboard::Up");
        });

        e.lasting(1.0).map(&mut self.back_end, || {
            println!("Wooooooow! You are pressing keybaord::Up at least 1.0 second!!");
        });
    }

    fn update(&mut self, dt: f64, _asset_store: &mut AssetStore) {
        self.back_end.update(dt);
    }


    // re-wrap those events to Event, a good way to do this is in the GameWindow
    // implementions.
    fn key_press(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        self.back_end.on_event(event::KeyPressed(key));
    }

    fn key_release(
        &mut self,
        key: keyboard::Key,
        _asset_store: &mut AssetStore
    ) {
        self.back_end.on_event(event::KeyReleased(key));
    }

    fn mouse_press(
        &mut self,
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        self.back_end.on_event(event::MouseButtonPressed(button));
    }

    fn mouse_release(
        &mut self,
        button: mouse::Button,
        _asset_store: &mut AssetStore
    ) {
        self.back_end.on_event(event::MouseButtonReleased(button));
    }
}

struct TestBackEnd<'a> {
    observers: Vec<Box<Observer>>,
}

impl<'a> TestBackEnd<'a> {
    pub fn new() -> TestBackEnd {
        TestBackEnd {
            observers: Vec::<Box<Observer>>::new(),
        }
    }
}

impl<'a> BackEnd for TestBackEnd<'a> {
    fn add_observer(&mut self, ob: Box<Observer>) -> uint {
        self.observers.push(ob);
        self.observers.len() - 1
    }

    fn update(&mut self, dt: f64) {
        for i in range(0, self.observers.len()) {
            let ob = self.observers.get_mut(i);

            ob.update(dt);

            if ob.can_trigger() {
                ob.trigger();
            }
        }
    }

    fn on_event(&mut self, e: event::Event) {
        for ob in self.observers.mut_iter() {
            ob.on_event(e);
        }
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


