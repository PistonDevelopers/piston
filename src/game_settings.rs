
/// Basic settings for window behavior.
pub struct GameSettings {
    /// If true, exit when pressing Esc.
    pub exit_on_esc: bool,
    /// The color to use as background.
    pub background_color: [f32, ..4],
}

impl GameSettings {
    /// Gets default settings.
    pub fn default() -> GameSettings {
        GameSettings {
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Creates a new Settings.
    pub fn new(exit_on_esc: bool, background_color: [f32, ..4]) -> GameSettings {
        GameSettings {
            exit_on_esc: exit_on_esc,
            background_color: background_color,
        }
    }
}
