extern crate chrono;
extern crate clap;
extern crate find_folder;
extern crate futures;
#[macro_use]
extern crate lazy_static;
extern crate piston_window;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_fs;
extern crate tokio_timer;

mod app;
mod config;
mod error;
mod processing;
mod state;
mod w1;

fn main() {
    let shared_state = state::SharedState::new();

    processing::spawn_background_thread(shared_state.clone());

    // TODO: Add some shutdown logic (SIGTERM), especially for Docker image on resinOS
    app::run(shared_state);
}
