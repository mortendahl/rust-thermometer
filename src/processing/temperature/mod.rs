/// Thermometer location.
#[derive(Clone, Copy)]
pub enum Location {
    Inside,
    Outside,
}

//
// Real temperature reader
//

#[cfg(not(feature = "simulate-temperature"))]
mod real;

#[cfg(not(feature = "simulate-temperature"))]
pub use self::real::TemperatureReader;

//
// Temperature simulation
//

#[cfg(feature = "simulate-temperature")]
mod simulation;

#[cfg(feature = "simulate-temperature")]
pub use self::simulation::TemperatureReader;
