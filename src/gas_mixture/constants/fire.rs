use super::T0C;

pub const FIRE_MINIMUM_TEMPERATURE_TO_SPREAD: f32 = 150.0 + T0C;
pub const FIRE_MINIMUM_TEMPERATURE_TO_EXIST: f32 = 100.0 + T0C;
pub const FIRE_SPREAD_RADIOSITY_SCALE: f32 = 0.85;
pub const FIRE_CARBON_ENERGY_RELEASED: f32 = 500000.0;
pub const FIRE_PLASMA_ENERGY_RELEASED: f32 = 3000000.0;
pub const FIRE_GROWTH_RATE: f32 = 40000.0;
