use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    #[default]
    None,
    Volt,
    Ampere,
    Degree,
    CelsiusDegree,
    Radian,
    Meter,
}
