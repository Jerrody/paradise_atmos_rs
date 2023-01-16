use crate::gas_mixture::constants::*;

const QUANTIZE_NEAREST: f32 = 0.0001;
pub mod macros {
    /// #### Description
    /// Creates a `BYOND`'s string.
    #[macro_export]
    macro_rules! string_ref {
        ($s:expr) => {
            StringRef::new($s).unwrap_unchecked()
        };
    }

    /// #### Description
    /// Creates a `DM`'s `null` via [`auxtools::Value`] wrapped with [`Result::Ok`].
    #[macro_export]
    macro_rules! null {
        () => {
            Ok(Value::null())
        };
    }

    /// #### Description
    /// Creates a  wrapped with [`Result::Ok`] a [`auxtools::Value`] from passed value.
    #[macro_export]
    macro_rules! value {
        ($value:expr) => {
            Ok(Value::from($value))
        };
    }
}

// FIXME: Can be possible reason of unexpected behaviour. Check if result is expected.
#[inline(always)]
pub fn quantize(value: f32) -> f32 {
    QUANTIZE_NEAREST * (value / QUANTIZE_NEAREST).round()
}

#[inline(always)]
#[must_use]
pub fn calculate_heat_capacity(
    carbon_dioxide: f32,
    oxygen: f32,
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
