use super::Location;
use error::Error;
use futures::{Async, Future, Poll};
use slog::Logger;
use state::SharedState;
use std::time::Duration;
use tokio_timer::{sleep, Delay};
use w1::thermometer::{Temperature, Thermometer, Units};

pub struct TemperatureReader {
    thermometer: Box<dyn Thermometer + Send>,
    location: Location,
    interval: Duration,
    shared_state: SharedState,
    logger: Logger,
    delay_handler: Option<Delay>,
    reader_handler: Option<Box<dyn Future<Item = Temperature, Error = Error> + Send>>,
}

impl TemperatureReader {
    pub fn new(
        thermometer: Box<dyn Thermometer + Send>,
        location: Location,
        interval: Duration,
        shared_state: SharedState,
        logger: Logger,
    ) -> TemperatureReader {
        TemperatureReader {
            thermometer,
            shared_state,
            location,
            interval,
            logger,
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
                    error!(self.logger, "Timer failed"; "error" => %e);
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
                    debug!(self.logger, "Temperature";
                        "device" => self.thermometer.device_path(),
                        "celsius" => t.to_string(Units::Celsius),
                        "fahrenheit" => t.to_string(Units::Fahrenheit));

                    self.update_temperature(t);
                }
                Err(e) => {
                    error!(self.logger, "Failed to read temperature";
                        "error" => %e,
                        "device" => self.thermometer.device_path());
                }
            };

            self.delay_handler = Some(sleep(self.interval));
        } else {
            self.reader_handler = Some(self.thermometer.temperature());
        }

        self.poll()
    }
}
