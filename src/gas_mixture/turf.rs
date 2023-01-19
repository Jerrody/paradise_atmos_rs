use auxtools::{StringRef, Value};

use crate::string_ref;

pub struct Turf {
    pub oxygen: f32,
    pub carbon_dioxide: f32,
    pub nitrogen: f32,
    pub toxins: f32,
    pub sleeping_agent: f32,
    pub agent_b: f32,
    pub temperature: f32,
}

impl Turf {
    const OXYGEN: &str = "oxygen";
    const CARBON_DIOXIDE: &str = "carbon_dioxide";
    const NITROGEN: &str = "nitrogen";
    const TOXINS: &str = "toxins";
    const SLEEPING_AGENT: &str = "sleeping_agent";
    const AGENT_B: &str = "agent_b";
    const TEMPERATURE: &str = "temperature";
    const THERMAL_CONDUCTIVITY: &str = "thermal_conductivity";
    const HEAT_CAPACITY: &str = "heat_capacity";

    #[inline(always)]
    #[must_use]
    pub unsafe fn new(turf: &Value) -> Self {
        Self {
            oxygen: turf
                .get_number(string_ref!(Self::OXYGEN))
                .unwrap_unchecked(),
            carbon_dioxide: turf
                .get_number(string_ref!(Self::CARBON_DIOXIDE))
                .unwrap_unchecked(),
            nitrogen: turf
                .get_number(string_ref!(Self::NITROGEN))
                .unwrap_unchecked(),
            toxins: turf
                .get_number(string_ref!(Self::TOXINS))
                .unwrap_unchecked(),
            sleeping_agent: turf
                .get_number(string_ref!(Self::SLEEPING_AGENT))
                .unwrap_unchecked(),
            agent_b: turf
                .get_number(string_ref!(Self::AGENT_B))
                .unwrap_unchecked(),
            temperature: turf
                .get_number(string_ref!(Self::TEMPERATURE))
                .unwrap_unchecked(),
        }
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_thermal_conductivity(turf: &Value) -> f32 {
        turf.get_number(string_ref!(Self::THERMAL_CONDUCTIVITY))
            .unwrap_unchecked()
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_heat_capacity(turf: &Value) -> f32 {
        turf.get_number(string_ref!(Self::HEAT_CAPACITY))
            .unwrap_unchecked()
    }
}
