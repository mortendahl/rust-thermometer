/// Model for view containing ready to render / draw values.
pub struct Model {
    inside_temperature: String,
    outside_temperature: String,
    time: String,
    date: String,
}

impl Model {
    /// Create new view `Model`.
    ///
    /// # Arguments
    ///
    /// * `inside_temperature` - formatted inside temperature
    /// * `outside_temperature` - formatted outside temperature
    /// * `time` - formatted time
    /// * `date` - formatted date
    pub fn new<S1, S2, S3, S4>(inside_temperature: S1, outside_temperature: S2, time: S3, date: S4) -> Model
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Model {
            inside_temperature: inside_temperature.into(),
            outside_temperature: outside_temperature.into(),
            time: time.into(),
            date: date.into(),
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

    /// Current time.
    pub fn time(&self) -> &str {
        &self.time
    }

    /// Current date.
    pub fn date(&self) -> &str {
        &self.date
    }
}
