use {Button, Event, Input};

/// Stores button state.
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub enum ButtonState {
    /// Button was pressed.
    Press,
    /// Button was released.
    Release,
}

/// Button arguments
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct ButtonArgs {
    /// New state of the button.
    pub state: ButtonState,
    /// The button that changed state.
    pub button: Button,
    /// An optional scancode that tells the physical layout of a keyboard key.
    /// For other devices than keyboard, this is set to `None`.
    ///
    /// Scancode follows SDL (https://wiki.libsdl.org/SDL_Scancode).
    ///
    /// This is stored here to make `Button` equality check work with keyboard layouts.
    ///
    /// Some window backends might not support scancodes.
    /// To test a window backend, use https://github.com/PistonDevelopers/piston-examples/tree/master/user_input
    pub scancode: Option<i32>,
}

/// Changed button state.
pub trait ButtonEvent: Sized {
    /// Creates a button event.
    fn from_button_args(args: ButtonArgs, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a button event.
    fn button<U, F>(&self, f: F) -> Option<U> where F: FnMut(ButtonArgs) -> U;
    /// Returns button arguments.
    fn button_args(&self) -> Option<ButtonArgs> {
        self.button(|args| args)
    }
}

impl ButtonEvent for Event {
    fn from_button_args(args: ButtonArgs, _old_event: &Self) -> Option<Self> {
        Some(Event::Input(Input::Button(args)))
    }
    fn button<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(ButtonArgs) -> U
    {
        match *self {
            Event::Input(Input::Button(args)) => Some(f(args)),
            _ => None,
        }
    }
}

/// The press of a button
pub trait PressEvent: Sized {
    /// Creates a press event.
    ///
    /// Preserves scancode from original button event, if any.
    fn from_button(button: Button, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a press event.
    fn press<U, F>(&self, f: F) -> Option<U> where F: FnMut(Button) -> U;
    /// Returns press arguments.
    fn press_args(&self) -> Option<Button> {
        self.press(|button| button)
    }
}

impl<T> PressEvent for T where T: ButtonEvent {
    fn from_button(button: Button, old_event: &Self) -> Option<Self> {
        if let Some(mut args) = old_event.button_args() {
            args.state = ButtonState::Press;
            args.button = button;
            ButtonEvent::from_button_args(args, old_event)
        } else {
            ButtonEvent::from_button_args(ButtonArgs {
                state: ButtonState::Press,
                button: button,
                scancode: None
            }, old_event)
        }
    }

    fn press<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if let Some(ButtonArgs {
            state: ButtonState::Press, button, ..
        }) = self.button_args() {
            Some(f(button))
        } else {
            None
        }
    }
}

/// The release of a button
pub trait ReleaseEvent: Sized {
    /// Creates a release event.
    ///
    /// Preserves scancode from original button event, if any.
    fn from_button(button: Button, old_event: &Self) -> Option<Self>;
    /// Calls closure if this is a release event.
    fn release<U, F>(&self, f: F) -> Option<U> where F: FnMut(Button) -> U;
    /// Returns release arguments.
    fn release_args(&self) -> Option<Button> {
        self.release(|button| button)
    }
}

impl<T> ReleaseEvent for T where T: ButtonEvent {
    fn from_button(button: Button, old_event: &Self) -> Option<Self> {
        if let Some(mut args) = old_event.button_args() {
            args.state = ButtonState::Release;
            args.button = button;
            ButtonEvent::from_button_args(args, old_event)
        } else {
            ButtonEvent::from_button_args(ButtonArgs {
                state: ButtonState::Release,
                button: button,
                scancode: None
            }, old_event)
        }
    }

    fn release<U, F>(&self, mut f: F) -> Option<U>
        where F: FnMut(Button) -> U
    {
        if let Some(ButtonArgs {
            state: ButtonState::Release, button, ..
        }) = self.button_args() {
            Some(f(button))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_press() {
        use super::super::{Button, Key};

        let e: Event = ButtonArgs {
            state: ButtonState::Press,
            button: Key::S.into(),
            scancode: None,
        }.into();
        let button = Button::Keyboard(Key::A);
        let x: Option<Event> = PressEvent::from_button(button, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .press(|button| PressEvent::from_button(button, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }

    #[test]
    fn test_input_release() {
        use super::super::{Button, Key};

        let e: Event = ButtonArgs {
            state: ButtonState::Release,
            button: Key::S.into(),
            scancode: None,
        }.into();
        let button = Button::Keyboard(Key::A);
        let x: Option<Event> = ReleaseEvent::from_button(button, &e);
        let y: Option<Event> = x.clone()
            .unwrap()
            .release(|button| ReleaseEvent::from_button(button, x.as_ref().unwrap()))
            .unwrap();
        assert_eq!(x, y);
    }
}
