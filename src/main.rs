extern crate chrono;
extern crate clap;
extern crate find_folder;
extern crate piston_window;
#[macro_use]
extern crate lazy_static;

mod app;
mod config;
mod error;
mod state;
mod temperature;
mod w1;

use state::SharedState;
use std::time::Duration;
use w1::thermometer::ds18b20::DS18B20;

fn main() {
    let state = SharedState::new();

    temperature::spawn_temperature_reader(
        state.clone(),
        temperature::Location::Inside,
        Box::new(DS18B20::new(config::CONFIG.inside_thermometer_device())),
        Duration::from_millis(500),
    );

    temperature::spawn_temperature_reader(
        state.clone(),
        temperature::Location::Outside,
        Box::new(DS18B20::new(config::CONFIG.outside_thermometer_device())),
        Duration::from_millis(500),
    );

    app::run(state);
}
