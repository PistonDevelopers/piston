use graphics::{
    Image,
};

/// Wraps OpenGL texture data.
pub struct Texture {
    /// Texture id.
    pub texture_id: uint,
    /// Texture width.
    pub texture_width: u32,
    /// Texture height.
    pub texture_height: u32,
}

impl Image for Texture {
    fn get_size(&self) -> (u32, u32) {
        (self.texture_width, self.texture_height)
    }
}
