use super::assets::{Font, Image};
use super::model::Model;
use piston_window::{clear, color, Context, G2d, GfxFactory};

/// Thermometer view.
pub struct View {
    rust_logo: Image,
    mono_regular_font: Font,
    model: Option<Model>,
}

impl View {
    /// Create new `View`.
    ///
    /// # Arguments
    ///
    /// * `factory` - graphics factory
    ///
    /// # Notes
    ///
    /// Panics if assets can not be loaded.
    pub fn new(factory: &mut GfxFactory) -> View {
        let rust_logo = Image::asset("rust-logo.png", factory);
        let mono_regular_font = Font::asset("FiraMono-Regular.ttf", factory.clone());

        View {
            rust_logo,
            mono_regular_font,
            model: None,
        }
    }

    /// Replace view model.
    ///
    /// # Arguments
    ///
    /// * `model` - new view model
    pub fn set_model<M>(&mut self, model: M)
    where
        M: Into<Model>,
    {
        self.model = Some(model.into());
    }

    /// Render view.
    ///
    /// # Arguments
    ///
    /// * `c` - context
    /// * `g` - graphics
    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        clear(color::BLACK, g);

        self.rust_logo.draw_at([15, 15], &c, g);

        if let Some(ref model) = self.model {
            self.mono_regular_font
                .draw_at([15, 75], model.inside_temperature(), 14, color::WHITE, &c, g);

            self.mono_regular_font
                .draw_at([15, 120], model.outside_temperature(), 14, color::WHITE, &c, g);

            self.mono_regular_font
                .draw_at([15, 165], model.time(), 14, color::WHITE, &c, g);
        }
    }
}
