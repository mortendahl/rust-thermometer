use super::glyphs;
use piston_window::text;
use piston_window::types::{Color, FontSize};
use piston_window::{Context, DrawState, G2dTexture, GfxFactory, Glyphs, Graphics, Position, Transformed};

/// Glyphs wrapper to simplify text rendering.
pub struct Font {
    glyphs: Glyphs,
}

impl Font {
    /// Create new `Font` from `Glyphs`.
    ///
    /// # Arguments
    ///
    /// * `glyphs` - glyphs
    pub fn new(glyphs: Glyphs) -> Font {
        Font { glyphs }
    }

    /// Create new `Font` from asset file name.
    ///
    /// # Arguments
    ///
    /// * `name` - asset file name with extension
    /// * `factory` - Gfx backend factory
    ///
    /// # Note
    ///
    /// Function panics if asset is not found or can't be loaded.
    ///
    /// TTF fonts work. OTF and other fonts untested.
    pub fn asset(name: &str, factory: GfxFactory) -> Font {
        Font::new(glyphs(name, factory, None))
    }

    /// Render text at given position.
    ///
    /// # Arguments
    ///
    /// * `position` - top / left position
    /// * `text` - text to render
    /// * `size` - font size
    /// * `color` - text color
    /// * `c` - drawing context
    /// * `g` - backend graphics
    pub fn draw_at<P, G>(&mut self, position: P, text: &str, size: FontSize, color: Color, c: &Context, g: &mut G)
    where
        G: Graphics<Texture = G2dTexture>,
        P: Into<Position>,
    {
        let p = position.into();
        text::Text::new_color(color, size)
            .draw(
                text,
                &mut self.glyphs,
                &DrawState::default(),
                c.transform.trans(f64::from(p.x), f64::from(p.y)),
                g,
            )
            .unwrap();
    }
}
