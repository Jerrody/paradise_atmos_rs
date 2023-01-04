use super::heat_constants::*;
use auxtools::StringRef;

const QUANTIZE_NEAREST: f32 = 0.0001;

#[macro_export]
macro_rules! string_ref {
    ($s:expr) => {
        unsafe { StringRef::new($s).unwrap_unchecked() }
    };
}

#[inline(always)]
pub fn heat_capacity_calculation(
    oxygen: f32,
    carbon_dioxide: f32,
    nitrogen: f32,
    toxins: f32,
    sleeping_agent: f32,
    agent_b: f32,
) -> f32 {
    carbon_dioxide * SPECIFIC_HEAT_CDO
        + (oxygen + nitrogen) * SPECIFIC_HEAT_AIR
        + toxins * SPECIFIC_HEAT_TOXIN
        + sleeping_agent * SPECIFIC_HEAT_N2O
        + agent_b * SPECIFIC_HEAT_AGENT_B
}

// FIXME: Can be possible reason of unexpected behaviour.
#[inline(always)]
pub fn quantize(value: f32) -> f32 {
    QUANTIZE_NEAREST * (value / QUANTIZE_NEAREST).round()
}

pub(crate) use string_ref;
