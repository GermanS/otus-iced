use std::{error::Error, fmt::Display, str::FromStr};

use regex::Regex;

use crate::{power::Power, state::DeviceState};

#[derive(Debug, Default)]
pub struct Socket {
    power: Power,
    state: DeviceState,
}

impl Socket {
    pub fn new(power: Power, state: DeviceState) -> Self {
        Self { power, state }
    }

    pub fn power(&self) -> &Power {
        &self.power
    }

    pub fn power_mut(&mut self) -> &mut Power {
        &mut self.power
    }

    pub fn state(&self) -> &DeviceState {
        &self.state
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Socket {}W State: {}", self.power(), self.state())
    }
}

impl FromStr for Socket {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Socket\s+(\d+((\.\d)*)?)W\s+State:\s+(on|off)").unwrap();

        match re.captures(s) {
            Some(caps) => {
                let power = caps[1].parse::<Power>().unwrap_or_default();

                let state = caps[4].parse::<DeviceState>().unwrap_or_default();

                Ok(Self::new(power, state))
            }
            None => Err("does not look like message from socket".into()),
        }
    }
}
