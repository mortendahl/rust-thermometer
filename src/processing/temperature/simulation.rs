use super::Location;
use futures::{Async, Future, Poll};
use slog::Logger;
use state::SharedState;
use std::time::Duration;
use tokio_timer::{sleep, Delay};
use w1::thermometer::{Temperature, Thermometer};

const MIN_TEMPERATURE: i64 = -10_000;
const MAX_TEMPERATURE: i64 = 10_000;

pub struct TemperatureReader {
    thermometer: Box<dyn Thermometer + Send>,
    location: Location,
    interval: Duration,
    shared_state: SharedState,
    logger: Logger,
    delay_handler: Option<Delay>,
    temperature: i64,
    temperature_step: i64,
}

impl TemperatureReader {
    pub fn new(
        thermometer: Box<dyn Thermometer + Send>,
        location: Location,
        interval: Duration,
        shared_state: SharedState,
        logger: Logger,
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
            logger,
            delay_handler: None,
            temperature,
            temperature_step,
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
