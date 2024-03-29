use crate::constants::*;

/// #### Description
/// Creates a `DM`'s `null` via [`auxtools::Value`] wrapped with [`Result::Ok`].
#[macro_export]
macro_rules! null {
    () => {
        Ok(Default::default())
    };
}

/// #### Description
/// Creates a  wrapped with [`Result::Ok`] a [`auxtools::Value`] from passed value.
#[macro_export]
macro_rules! value {
    ($value:expr) => {
        Ok(ByondValue::from($value))
    };
}

/// #### Description
/// Gets `id` from value.
/// Returns `usize`.
#[macro_export]
macro_rules! id {
    ($value:expr) => {
        $value.0.data.ref_ as usize
    };
}

#[macro_export]
macro_rules! profile {
    ($s:expr) => {
        #[cfg(feature = "profile")]
        let span = span!(tracing::Level::INFO, $s);
        #[cfg(feature = "profile")]
        let _enter = span.enter();
    };
}

#[macro_export]
macro_rules! profile_proc {
    ($s:expr) => {
        #[cfg(feature = "profile_proc")]
        let span = span!(tracing::Level::INFO, $s);
        #[cfg(feature = "profile_proc")]
        let _enter = span.enter();
    };
}

// FIXME: Can be possible reason of unexpected behaviour. Check if result is expected.
#[must_use]
#[inline(always)]
pub fn quantize(value: f32) -> f32 {
    const QUANTIZE_NEAREST: f32 = 0.0001;

    ((value + (QUANTIZE_NEAREST / 2.0)).div_euclid(QUANTIZE_NEAREST)) * QUANTIZE_NEAREST
}

#[must_use]
#[inline(always)]
pub fn calculate_heat_capacity(
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
