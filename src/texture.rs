use png;
use gl;
use gl::types::GLuint;
use libc::c_void;
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
    
    /// Loads image by relative file name to the asset root.
    pub fn from_path(path: &Path) -> Result<Texture, String> {
        let img = match png::load_png(path) {
            Ok(img) => img,
            Err(msg) => return Err(format!("Could not load '{}': {}", path.filename_str().unwrap(), msg)),
        };

        match img.color_type {
            png::RGBA8 => {},
            t => fail!("Unsupported color type {:?} in png", t),
        };

        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width as i32,
                img.height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.pixels.as_ptr() as *c_void
            );
        }
        
        Ok(Texture::new(id, img.width, img.height))
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
