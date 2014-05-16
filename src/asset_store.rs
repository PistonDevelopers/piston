
//! Storing sounds, textures, animations etc.

// Extern crates.
use graphics::*;
use gl = opengles::gl2;
use HashMap = collections::HashMap;

// Local crate.
use png;

/// Represents a texture in Piston.
pub struct Texture {
    id: gl::GLuint,
    width: u32,
    height: u32,
}

/// A place to store sounds, textures, animations etc.
///
/// The idea is to have one object which the app can use
/// to load assets for the game with a simple interface.
pub struct AssetStore {
    // The folder to load assets from.
    assets_folder: Option<~str>,
    // List of OpenGL textures.
    textures: Vec<Texture>,
    // Contains names of loaded textures.
    texture_files: HashMap<~str, uint>,
}

impl AssetStore {
    /// Creates a new `AssetStore` from an assets folder.
    pub fn from_folder(assets_folder: &str) -> AssetStore {
        AssetStore {
            assets_folder: Some(assets_folder.to_owned()),
            textures: Vec::new(),
            texture_files: HashMap::new(),
        }
    }

    /// Creates an empty `AssetStore` with no assets.
    pub fn empty() -> AssetStore {
        AssetStore {
            assets_folder: None,
            textures: Vec::new(),
            texture_files: HashMap::new(),
        }
    }

    /// Gets OpenGL texture from texture id.
    pub fn get_texture(&self, texture_id: uint) -> gl::GLuint {
        self.textures.get(texture_id).id
    }

    /// Loads image by relative file name to the asset root.
    pub fn load_image(&mut self, file: &str) -> Image {
        match self.texture_files.find_equiv(&file) {
            None => {},
            Some(&texture_id) => {
                let texture = self.textures.get(texture_id);
                return Image {
                    texture_id: texture_id,
                    texture_width: texture.width,
                    texture_height: texture.height,
                    source_rect: [0, 0, texture.width, texture.height],
                }
            },
        };

        let folder = self.assets_folder.as_ref().unwrap();
        let path = Path::new(format!("{}/{}", folder, file));
        let img = match png::load_png(&path) {
            Ok(img) => img,
            Err(msg) => fail!("Could not load '{}': {}", file, msg),
        };

        match img.color_type {
            png::RGBA8 => {},
            t => fail!("Unsupported color type {:?} in png", t),
        };

        let id: gl::GLuint = *gl::gen_textures(1).get(0);
        gl::bind_texture(gl::TEXTURE_2D, id);
        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::tex_image_2d(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width as i32,
            img.height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            Some(img.pixels.as_slice())
        );

        let texture = Texture {
            id: id,
            width: img.width,
            height: img.height,
        };
        self.textures.push(texture);
        let texture_id = self.textures.len() - 1;

        self.texture_files.insert(file.to_owned(), texture_id);
        Image {
            texture_id: texture_id,
            texture_width: texture.width,
            texture_height: texture.height,
            source_rect: [0, 0, texture.width, texture.height],
        }
    }
}

