use std::sync::{Arc, Mutex};
use w1::thermometer::Temperature;

/// Application state.
#[derive(Clone)]
pub struct State {
    pub inside_temperature: Option<Temperature>,
    pub outside_temperature: Option<Temperature>,
}

impl Default for State {
    fn default() -> State {
        State {
            inside_temperature: None,
            outside_temperature: None,
        }
    }
}

/// Shared cloneable application state.
#[derive(Clone)]
pub struct SharedState {
    state: Arc<Mutex<State>>,
}

impl SharedState {
    /// Create new `SharedState`.
    pub fn new() -> SharedState {
        SharedState {
            state: Arc::new(Mutex::new(State::default())),
        }
    }

    /// Set inside temperature.
    ///
    /// # Arguments
    ///
    /// * `value` - new inside temperature
    pub fn set_inside_temperature(&self, value: Temperature) {
        self.state.lock().unwrap().inside_temperature = Some(value);
    }

    /// Set outside temperature
    ///
    /// # Arguments
    ///
    /// * `value` - new outside temperature
    pub fn set_outside_temperature(&self, value: Temperature) {
        self.state.lock().unwrap().outside_temperature = Some(value);
    }

    /// `State` snapshot.
    pub fn state(&self) -> State {
        self.state.lock().unwrap().clone()
    }
}
