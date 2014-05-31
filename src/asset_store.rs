
//! Storing sounds, textures, animations etc.

// Extern crates.
use gl;
use gl::types::GLuint;
use libc::c_void;
use std::os::self_exe_path;
use png;

// Local crate.
use Texture;

/// A place to store sounds, textures, animations etc.
///
/// The idea is to have one object which the app can use
/// to load assets for the game with a simple interface.
pub struct AssetStore {
    // The folder to load assets from.
    assets_folder: Option<String>,
}

impl AssetStore {
    /// Creates a new `AssetStore` from an assets folder.
    pub fn from_folder(assets_folder: &str) -> AssetStore {
        AssetStore {
            assets_folder: Some(assets_folder.to_string()),
        }
    }

    /// Creates an empty `AssetStore` with no assets.
    pub fn empty() -> AssetStore {
        AssetStore {
            assets_folder: None,
        }
    }

    /// Loads image by relative file name to the asset root.
    pub fn load_image(&mut self, file: &str) -> Result<Texture, String> {
        let folder = self.assets_folder.as_ref().unwrap();
        let exe_path = self_exe_path();
        let exe_path = match exe_path {
            Some(path) => path,
            None => return Err("Could not get the path to executable".to_string()),
        };
        let path = exe_path.join(Path::new(folder.as_slice())).join(Path::new(file));
        let img = match png::load_png(&path) {
            Ok(img) => img,
            Err(msg) => return Err(format!("Could not load '{}': {}", file, msg)),
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

