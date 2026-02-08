use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    #[default]
    None,
    Percent,
    Volt,
    Ampere,
    Degree,
    Radian,
    Celsius,
    Kelvin,
    Metre,
    Kilogram,
    Second,
}
