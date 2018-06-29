mod temperature;

use self::temperature::{Location, TemperatureReader};
use config;
use futures::Future;
use slog::Logger;
use state::SharedState;
use std::thread;
use std::time::Duration;
use tokio;
use w1::thermometer::ds18b20::DS18B20;

/// Start Tokio runtime and spawn tasks.
///
/// # Arguments
///
/// * `state` - shared application state
///
/// # Note
///
/// This function blocks and will not return immediately. It must be spawned
/// on another thread than main (UI).
fn background_thread(state: SharedState, logger: Logger) {
    let inside_reader = TemperatureReader::new(
        Box::new(DS18B20::new(config::CONFIG.inside_thermometer_device())),
        Location::Inside,
        Duration::from_millis(config::CONFIG.temperature_interval()),
        state.clone(),
        logger.clone(),
    );

    let outside_reader = TemperatureReader::new(
        Box::new(DS18B20::new(config::CONFIG.outside_thermometer_device())),
        Location::Outside,
        Duration::from_millis(config::CONFIG.temperature_interval()),
        state,
        logger,
    );

    let handler = inside_reader.select(outside_reader).then(|_| Ok(()));

    // TODO: Add some shutdown logic (SIGTERM), especially for Docker image on resinOS
    tokio::run(Box::new(handler));
}

/// Spawn new thread with Tokio.
///
/// # Arguments
///
/// * `state` - shared application state
pub fn spawn_background_thread(state: SharedState, logger: Logger) {
    thread::spawn(move || background_thread(state, logger));
}
