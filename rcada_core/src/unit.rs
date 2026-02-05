#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Unit {
    Volt,
    Ampere,
    Degree,
    CelsiusDegree,
    Radian,
    Meter,

    #[default]
    None,
}
