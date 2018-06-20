extern crate bytes;

use std::thread::sleep;
use std::time::Duration;

mod config;
mod error;
mod w1;

fn main() {
    let cfg = config::Config::new();
    let thermometers = cfg.thermometers();

    loop {
        for thermometer in thermometers {
            match thermometer.temperature() {
                Ok(temperature) => {
                    println!("{:.3} degrees celsius in {}", temperature.celsius(), thermometer.name());
                },
                Err(error) => {
                    println!("Failed to read temperature for {}: {}", thermometer.name(), error);
                }
            }
        }
        sleep(Duration::from_millis(1_000));
    }
}
