
/// Settings for window behavior.
pub struct GameWindowSettings {
    /// Title of the window.
    pub title: String,
    /// The size of the window
    pub size: [u32, ..2],
    /// If true, the window is fullscreen.
    pub fullscreen: bool,
    /// If true, exit when pressing Esc.
    pub exit_on_esc: bool,
    /// The background color.
    pub background_color: [f32, ..4],
}

impl GameWindowSettings {
    /// Gets default settings.
    ///
    /// This exits the window when pressing `Esc`.
    /// The background color is set to black.
    pub fn default() -> GameWindowSettings {
        GameWindowSettings {
            title: "Piston".to_owned(),
            size: [640, 480],
            fullscreen: false,
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Creates a new Settings.
    pub fn new(
        title: String,
        size: [u32, ..2],
        fullscreen: bool,
        exit_on_esc: bool,
        background_color: [f32, ..4]
    ) -> GameWindowSettings {

        GameWindowSettings {
            title: title,
            size: size,
            fullscreen: fullscreen,
            exit_on_esc: exit_on_esc,
            background_color: background_color,
        }
    }
}
