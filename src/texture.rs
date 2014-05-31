use gl;
use gl::types::GLuint;
use graphics::{
    Image,
};

/// Wraps OpenGL texture data.
/// The texture gets deleted when running out of scope.
pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    /// Creates a new texture.
    #[inline(always)]
    pub fn new(id: GLuint, width: u32, height: u32) -> Texture {
        Texture {
            id: id,
            width: width,
            height: height,
        }
    }

    /// Gets the OpenGL id of the texture.
    #[inline(always)]
    pub fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}

impl Image for Texture {
    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
