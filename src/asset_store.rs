
//! Storing sounds, textures, animations etc.

/// A place to store sounds, textures, animations etc.
///
/// The idea is to have one object which the app can use  
/// to load assets for the game with a simple interface.  
pub struct AssetStore {
    assets_folder: Option<~str>
}

impl AssetStore {
    /// Creates a new `AssetStore` from an assets folder.  
    pub fn from_folder(assets_folder: &str) -> AssetStore {
        AssetStore {
            assets_folder: Some(assets_folder.to_owned()),
        }
    }

    /// Creates an empty `AssetStore` with no assets.
    pub fn empty() -> AssetStore {
        AssetStore {
            assets_folder: None,
        }
    }
}

