#[cfg(not(feature = "simulate-temperature"))]
use error::Error;
use futures::{Async, Future, Poll};
use state::SharedState;
use std::time::Duration;
use tokio_timer::{sleep, Delay};
use w1::thermometer::{Temperature, Thermometer};

/// Thermometer location.
#[derive(Clone, Copy)]
pub enum Location {
    Inside,
    Outside,
}

pub struct TemperatureReader {
    thermometer: Box<dyn Thermometer + Send>,
    location: Location,
    interval: Duration,
    shared_state: SharedState,
    delay_handler: Option<Delay>,

    #[cfg(not(feature = "simulate-temperature"))]
    reader_handler: Option<Box<dyn Future<Item = Temperature, Error = Error> + Send>>,

    #[cfg(feature = "simulate-temperature")]
    temperature: i64,

    #[cfg(feature = "simulate-temperature")]
    temperature_step: i64,
}

impl TemperatureReader {
    #[cfg(not(feature = "simulate-temperature"))]
    pub fn new(
        thermometer: Box<dyn Thermometer + Send>,
        location: Location,
        interval: Duration,
        shared_state: SharedState,
    ) -> TemperatureReader {
        TemperatureReader {
            thermometer,
            shared_state,
            location,
            interval,
            delay_handler: None,
            reader_handler: None,
        }
    }

    fn update_temperature(&self, temperature: Temperature) {
        match self.location {
            Location::Inside => self.shared_state.set_inside_temperature(temperature),
            Location::Outside => self.shared_state.set_outside_temperature(temperature),
        }
    }
}

#[cfg(not(feature = "simulate-temperature"))]
impl Future for TemperatureReader {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(mut delay_handler) = self.delay_handler.take() {
            match delay_handler.poll() {
                Ok(Async::NotReady) => {
                    self.delay_handler = Some(delay_handler);
                    return Ok(Async::NotReady);
                }
                Err(e) => {
                    eprintln!("Timer failed: {}", e);
                }
                _ => {}
            };

            // Read temperature when timer fires or fails

            self.reader_handler = Some(self.thermometer.temperature());
        }

        if let Some(mut reader_handler) = self.reader_handler.take() {
            match reader_handler.poll() {
                Ok(Async::NotReady) => {
                    self.reader_handler = Some(reader_handler);
                    return Ok(Async::NotReady);
                }
                Ok(Async::Ready(t)) => {
                    self.update_temperature(t);
                }
                Err(e) => {
                    eprintln!("Reader failed: {}", e);
                }
            };

            self.delay_handler = Some(sleep(self.interval));
        } else {
            self.reader_handler = Some(self.thermometer.temperature());
        }

        self.poll()
    }
}

//
// Temperature simulation
//
// #[cfg(feature = "simulate-temperature")]
//

#[cfg(feature = "simulate-temperature")]
const MIN_TEMPERATURE: i64 = -10_000;

#[cfg(feature = "simulate-temperature")]
const MAX_TEMPERATURE: i64 = 10_000;

#[cfg(feature = "simulate-temperature")]
impl TemperatureReader {
    pub fn new(
        thermometer: Box<dyn Thermometer + Send>,
        location: Location,
        interval: Duration,
        shared_state: SharedState,
    ) -> TemperatureReader {
        let (temperature, temperature_step) = match location {
            Location::Inside => (MIN_TEMPERATURE, 1_000),
            Location::Outside => (MAX_TEMPERATURE, -500),
        };

        TemperatureReader {
            thermometer,
            shared_state,
            location,
            interval,
            delay_handler: None,
            temperature,
            temperature_step,
        }
    }
}

#[cfg(feature = "simulate-temperature")]
impl Future for TemperatureReader {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(mut delay_handler) = self.delay_handler.take() {
            match delay_handler.poll() {
                Ok(Async::NotReady) => {
                    self.delay_handler = Some(delay_handler);
                    return Ok(Async::NotReady);
                }
                Err(e) => {
                    eprintln!("Timer failed: {}", e);
                }
                _ => {}
            };
        }

        self.temperature += self.temperature_step;
        if self.temperature <= MIN_TEMPERATURE || self.temperature >= MAX_TEMPERATURE {
            self.temperature_step *= -1;
        }
        self.update_temperature(Temperature::new(self.temperature));

        self.delay_handler = Some(sleep(self.interval));
        self.poll()
    }
}
