
/// Settings for window behavior.
pub struct GameWindowSettings {
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
            exit_on_esc: true,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Creates a new Settings.
    pub fn new(
        exit_on_esc: bool, 
        background_color: [f32, ..4]
    ) -> GameWindowSettings {
        
        GameWindowSettings {
            exit_on_esc: exit_on_esc,
            background_color: background_color,
        }
    }
}
