use state::SharedState;
use std::thread;
use std::time::Duration;
use w1::thermometer::Thermometer;

/// Thermometer location.
#[derive(Clone, Copy)]
pub enum Location {
    Inside,
    Outside,
}

/// Spawn thread with temperature reader.
///
/// # Arguments
///
/// * `shared_state` - shared thermometer state
/// * `location` - thermometer location
/// * `thermometer` - thermometer device
/// * `interval` - interval in which we try to read temperature
pub fn spawn_temperature_reader(
    shared_state: SharedState,
    location: Location,
    thermometer: Box<Thermometer>,
    interval: Duration,
) {
    thread::spawn(move || loop {
        match (thermometer.read_temperature(), location) {
            (Ok(temp), Location::Inside) => {
                shared_state.set_inside_temperature(temp);
            }
            (Ok(temp), Location::Outside) => {
                shared_state.set_outside_temperature(temp);
            }
            (Err(e), _) => {
                eprintln!("Failed to read temperature: {}", e);
            }
        }
        thread::sleep(interval);
    });
}
