use super::texture;
use piston_window::types::Rectangle;
use piston_window::Image as PistonImage;
use piston_window::{Context, DrawState, G2dTexture, GfxFactory, Graphics, ImageSize, Position};

/// Texture wrapper to simplify drawing.
pub struct Image {
    texture: G2dTexture,
}

impl Image {
    /// Create new `Image` from `G2dTexture`.
    ///
    /// # Arguments
    ///
    /// * `texture` - texture
    pub fn new(texture: G2dTexture) -> Image {
        Image { texture }
    }

    /// Image width.
    pub fn width(&self) -> u32 {
        self.texture.get_width()
    }

    /// Image height.
    pub fn height(&self) -> u32 {
        self.texture.get_height()
    }

    /// Create new `Image` from asset file name.
    ///
    /// # Arguments
    ///
    /// * `name` - asset file name with extension
    /// * `factory` - Gfx backend factory
    ///
    /// # Note
    ///
    /// Function panics if asset is not found or can't be loaded.
    pub fn asset(name: &str, factory: &mut GfxFactory) -> Image {
        Image::new(texture(name, factory, None))
    }

    /// Draw self at given point.
    ///
    /// # Arguments
    ///
    /// * `position` - top / left position
    /// * `c` - drawing context
    /// * `g` - backend graphics
    pub fn draw_at<P, G>(&self, position: P, c: &Context, g: &mut G)
    where
        P: Into<Position>,
        G: Graphics<Texture = G2dTexture>,
    {
        let p = position.into();
        self.draw_rect(
            [
                p.x as f64,
                p.y as f64,
                self.texture.get_width() as f64,
                self.texture.get_height() as f64,
            ],
            c,
            g,
        );
    }

    /// Draw self in the given rectangle.
    ///
    /// # Arguments
    ///
    /// * `rect` - rectangle to draw image in
    /// * `c` - drawing context
    /// * `g` - backend graphics
    pub fn draw_rect<R, G>(&self, rect: R, c: &Context, g: &mut G)
    where
        R: Into<Rectangle>,
        G: Graphics<Texture = G2dTexture>,
    {
        let image = PistonImage::new().rect(rect);
        image.draw(&self.texture, &DrawState::default(), c.transform, g);
    }
}
