/// Model for view containing ready to render / draw values.
pub struct Model {
    inside_temperature: String,
    outside_temperature: String,
    time: String,
}

impl Model {
    /// Create new view `Model`.
    ///
    /// # Arguments
    ///
    /// * `inside_temperature` - inside temperature
    /// * `outside_temperature` - outside temperature
    /// * `time` - date & time
    pub fn new<S>(inside_temperature: S, outside_temperature: S, time: S) -> Model
    where
        S: Into<String>,
    {
        Model {
            inside_temperature: inside_temperature.into(),
            outside_temperature: outside_temperature.into(),
            time: time.into(),
        }
    }

    /// Inside temperature.
    pub fn inside_temperature(&self) -> &str {
        &self.inside_temperature
    }

    /// Outside temperature.
    pub fn outside_temperature(&self) -> &str {
        &self.outside_temperature
    }

    /// Current date & time.
    pub fn time(&self) -> &str {
        &self.time
    }
}
