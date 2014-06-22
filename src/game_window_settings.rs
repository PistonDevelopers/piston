
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
}

impl GameWindowSettings {
    /// Gets default settings.
    ///
    /// This exits the window when pressing `Esc`.
    /// The background color is set to black.
    pub fn default() -> GameWindowSettings {
        GameWindowSettings {
            title: "Piston".to_string(),
            size: [640, 480],
            fullscreen: false,
            exit_on_esc: true,
        }
    }
}
