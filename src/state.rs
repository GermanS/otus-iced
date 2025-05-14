use std::{error::Error, fmt::Display, ops::Deref, str::FromStr};

#[derive(Debug, Default)]
pub struct DeviceState(bool);

impl DeviceState {
    pub fn get(&self) -> bool {
        self.0
    }
}
impl FromStr for DeviceState {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self(true)),
            _ => Ok(Self(false)),
        }
    }
}

impl Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = if **self { "on" } else { "off" };

        write!(f, "{}", state)
    }
}

impl Deref for DeviceState {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
