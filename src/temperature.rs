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
#[cfg(not(feature = "simulate-temperature"))]
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

/// Spawn thread with simulated temperature reader.
///
/// # Arguments
///
/// * `shared_state` - shared thermometer state
/// * `location` - thermometer location
/// * `thermometer` - thermometer device
/// * `interval` - interval in which we try to read temperature
#[cfg(feature = "simulate-temperature")]
pub fn spawn_temperature_reader(
    shared_state: SharedState,
    location: Location,
    _thermometer: Box<Thermometer>,
    interval: Duration,
) {
    use w1::thermometer::Temperature;

    let min = -10_000;
    let max = 10_000;

    thread::spawn(move || loop {
        let (mut temp, mut step) = match location {
            Location::Inside => (min, 1_000),
            Location::Outside => (max, -500),
        };

        loop {
            temp += step;

            if temp == min || temp == max {
                step *= -1;
            }

            let t = Temperature::new(temp);

            match location {
                Location::Inside => shared_state.set_inside_temperature(t),
                Location::Outside => shared_state.set_outside_temperature(t),
            }

            thread::sleep(interval);
        }
    });
}
