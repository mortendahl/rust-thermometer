mod assets;
mod model;
mod view;

use self::model::Model;
use self::view::View;
use chrono::{Local, SecondsFormat};
use config;
use piston_window::{
    Event, EventLoop, OpenGL, PistonWindow, RenderEvent, UpdateArgs, UpdateEvent, Window, WindowSettings,
};
use state::{SharedState, State};

/// Convert `State` into view `Model` with current date & time.
impl From<State> for Model {
    fn from(state: State) -> Model {
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

        let formatted = Local::now().to_rfc3339_opts(SecondsFormat::Secs, false);

        let date: &str = &formatted[..10]; // YYYY-MM-DD
        let time: &str = &formatted[11..19]; // HH:MM:SS

        Model::new(inside_temperature, outside_temperature, time, date)
    }
}

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
        // Do not use draw_size() here, because it returns 1600x960 in case of retina displays,
        // 800x480 on Rpi. size() always returns 800x480.
        let size = window.size();
        window.draw_2d(e, |c, g| {
            // Just call draw on our view, it will render itself
            self.view.draw(size, c, g);
        });
    }

    /// Update application state.
    ///
    /// # Arguments
    ///
    /// * `_args` - update arguments (not used)
    #[cfg_attr(feature = "cargo-clippy", allow(trivially_copy_pass_by_ref))]
    fn update(&mut self, _args: &UpdateArgs) {
        self.view.set_model(self.shared_state.state());
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

    window.set_max_fps(config::CONFIG.max_fps());

    let view = View::new(&mut window.factory);
    let mut app = App::new(shared_state, view);

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            app.render(&mut window, &e);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
