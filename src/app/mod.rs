mod assets;
mod model;
mod view;

use self::model::Model;
use self::view::View;
use chrono::Local;
use config;
use piston_window::*;
use state::SharedState;

/// Main application (UI).
struct App {
    shared_state: SharedState,
    view: View,
}

impl App {
    /// Create new application.
    ///
    /// # Arguments
    ///
    /// * `shared_state` - shared state
    /// * `view` - main view
    fn new(shared_state: SharedState, view: View) -> App {
        App { shared_state, view }
    }

    /// Render / draw application.
    ///
    /// # Arguments
    ///
    /// * `window` - window
    /// * `e` - event
    fn render(&mut self, window: &mut PistonWindow, e: &Event) {
        window.draw_2d(e, |c, g| {
            // Just call draw on our view, it will render itself
            self.view.draw(c, g);
        });
    }

    /// Update application state.
    ///
    /// # Arguments
    ///
    /// * `_args` - update arguments (not used)
    fn update(&mut self, _args: &UpdateArgs) {
        // Get state snapshot
        let state = self.shared_state.state();

        let inside_temperature = format!(
            "Inside temperature {}",
            state
                .inside_temperature
                .map(|t| t.to_string(config::CONFIG.temperature_units()))
                .unwrap_or_else(|| "N/A".to_string())
        );

        let outside_temperature = format!(
            "Outside temperature {}",
            state
                .outside_temperature
                .map(|t| t.to_string(config::CONFIG.temperature_units()))
                .unwrap_or_else(|| "N/A".to_string())
        );

        let time = Local::now().to_rfc2822();

        self.view
            .set_model(Model::new(inside_temperature, outside_temperature, time));
    }
}

/// Run main application (UI).
///
/// # Arguments
///
/// * `shared_state` - shared state
pub fn run(shared_state: SharedState) {
    let open_gl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(config::PKG_NAME, [800, 480])
        .opengl(open_gl)
        .samples(0)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_max_fps(2);

    let view = View::new(&mut window.factory);
    let mut app = App::new(shared_state, view);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            app.render(&mut window, &e);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
