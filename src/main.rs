extern crate chrono;
extern crate clap;
extern crate find_folder;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate piston_window;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_fs;
extern crate tokio_timer;

mod app;
mod config;
mod error;
mod log;
mod processing;
mod state;
mod w1;

fn main() {
    let logger = log::create_root_logger();
    let shared_state = state::SharedState::new();

    info!(logger, "Spawning background thread for processing");
    processing::spawn_background_thread(shared_state.clone(), logger.clone());

    // TODO: Add some shutdown logic (SIGTERM), especially for Docker image on resinOS
    info!(logger, "Launching UI");
    app::run(shared_state);
}
