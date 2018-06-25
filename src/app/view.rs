use super::assets::{Font, Image};
use super::model::Model;
use piston_window::{clear, color, Context, G2d, GfxFactory, Position, Size};

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

    fn rust_logo_position(&self, size: &Size) -> Position {
        [
            (size.width - self.rust_logo.width() - 15) as i32,
            (size.height - self.rust_logo.height() - 15) as i32,
        ].into()
    }

    fn time_position(&self, _size: &Size) -> Position {
        [15, 22].into()
    }

    fn date_position(&self, size: &Size) -> Position {
        [15, size.height as i32 - 20].into()
    }

    fn inside_temperature_position(&self, _size: &Size) -> Position {
        [250, 200].into()
    }

    fn outside_temperature_position(&self, _size: &Size) -> Position {
        [250, 250].into()
    }

    /// Render view.
    ///
    /// # Arguments
    ///
    /// * `c` - context
    /// * `g` - graphics
    pub fn draw(&mut self, size: Size, c: Context, g: &mut G2d) {
        clear(color::BLACK, g);

        let logo_position = self.rust_logo_position(&size);
        self.rust_logo.draw_at(logo_position, &c, g);

        if let Some(ref model) = self.model {
            let position = self.time_position(&size);
            self.mono_regular_font
                .draw_at(position, model.time(), 14, color::WHITE, &c, g);

            let position = self.date_position(&size);
            self.mono_regular_font
                .draw_at(position, model.date(), 14, color::WHITE, &c, g);

            let position = self.inside_temperature_position(&size);
            self.mono_regular_font
                .draw_at(position, model.inside_temperature(), 14, color::WHITE, &c, g);

            let position = self.outside_temperature_position(&size);
            self.mono_regular_font
                .draw_at(position, model.outside_temperature(), 14, color::WHITE, &c, g);
        }
    }
}
