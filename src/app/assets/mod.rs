//! Asset loading functions.

mod font;
mod image;

pub use self::font::Font;
pub use self::image::Image;

use find_folder;
use piston_window::{Flip, G2dTexture, GfxFactory, Glyphs, Texture, TextureSettings};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

lazy_static! {
    /// Assets folder. All assets must be located directly in this folder (in the root).
    static ref ASSETS_FOLDER: PathBuf = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

    /// Default `TextureSettings` in case `None` is provided as an argument for the `texture()` function.
    static ref DEFAULT_TEXTURE_SETTINGS: TextureSettings = TextureSettings::new();
}

/// Create asset path.
///
/// # Arguments
///
/// * `name` - asset file name with extension
fn asset_path(name: &str) -> OsString {
    let mut path = ASSETS_FOLDER.clone();
    path.push(Path::new(name));
    path.into_os_string()
}

/// Load texture from file.
///
/// # Arguments
///
/// * `name` - asset file name with extension
/// * `factory` -  Gfx backend factory
/// * `settings` - `TextureSettings` or `None` if default settings should be used
///
/// # Note
///
/// Function panics if asset is not found or can't be loaded.
pub fn texture(name: &str, factory: &mut GfxFactory, settings: Option<&TextureSettings>) -> G2dTexture {
    Texture::from_path(
        factory,
        &asset_path(name),
        Flip::None,
        settings.unwrap_or(&DEFAULT_TEXTURE_SETTINGS),
    ).unwrap()
}

/// Load glyphs from file.
///
/// # Arguments
///
/// * `name` - asset file name with extension
/// * `factory` - Gfx backend factory
/// * `settings` - `TextureSettings` or `None` if default settings should be used
///
/// # Note
///
/// Function panics if asset is not found or can't be loaded.
pub fn glyphs(name: &str, factory: GfxFactory, settings: Option<TextureSettings>) -> Glyphs {
    Glyphs::new(asset_path(name), factory, settings.unwrap_or_else(TextureSettings::new)).unwrap()
}
