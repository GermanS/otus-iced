use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, Default)]
pub struct Power(f32);

impl Power {
    pub const MIN_POWER: f32 = 500.0;
    pub const MAX_POWER: f32 = 2000.0;
    pub const GRADUATION: f32 = 2.5;

    pub fn new(power: f32) -> Self {
        Self(power)
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, value: f32) {
        if (Self::MIN_POWER..=Self::MAX_POWER).contains(&value) {
            self.0 = value
        }
    }

    pub fn ratio(power: f32) -> f32 {
        if power >= Self::MIN_POWER {
            return (power - Self::MIN_POWER) / (Self::MAX_POWER - Self::MIN_POWER);
        }

        0.0
    }
}

impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl FromStr for Power {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<u32>() {
            return Ok(Self::new(value as f32));
        }

        if let Ok(value) = s.parse::<f32>() {
            return Ok(Self::new(value));
        }

        Err("Can't parse power".into())
    }
}
