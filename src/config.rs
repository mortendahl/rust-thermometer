use w1::thermometer::ds18b20::DS18B20;
use w1::thermometer::Thermometer;

pub struct Config {
    thermometers: Vec<Box<Thermometer>>,
}

impl Config {
    pub fn new() -> Config {
        let living_room = DS18B20::new("Living room", "28-000009e8f6e7");
        let bedroom = DS18B20::new("Bedroom", "28-000009e9b786");

        Config { thermometers: vec![Box::new(living_room), Box::new(bedroom)] }
    }

    pub fn thermometers(&self) -> &[Box<Thermometer>] {
        &self.thermometers
    }
}
