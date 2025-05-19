use std::{error::Error, fmt::Display, str::FromStr};

use regex::Regex;

use crate::{state::DeviceState, temperature::Temperature};

#[derive(Debug, Default)]
pub struct Termometer {
    temperature: Temperature,
    state: DeviceState,
}

impl Termometer {
    pub fn new(temperature: Temperature, state: DeviceState) -> Self {
        Self { temperature, state }
    }

    pub fn temperature(&self) -> &Temperature {
        &self.temperature
    }

    pub fn temperature_mut(&mut self) -> &mut Temperature {
        &mut self.temperature
    }

    pub fn state(&self) -> &DeviceState {
        &self.state
    }
}

impl Display for Termometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Termometer {}C State: {}",
            self.temperature(),
            self.state()
        )
    }
}

impl FromStr for Termometer {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_temperature =
            Regex::new(r"^Termometer(\s)+(\d+((\.\d+)*)?)C\s+State:\s+(on|off)").unwrap();

        match re_temperature.captures(s) {
            Some(caps) => {
                let temperature = caps[2].parse::<Temperature>().unwrap_or_default();

                let state = caps[5].parse::<DeviceState>().unwrap_or_default();

                Ok(Self::new(temperature, state))
            }
            None => Err("does not look like message from termometer".into()),
        }
    }
}
