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
    const OXYGEN: &'static str = "oxygen";
    const CARBON_DIOXIDE: &'static str = "carbon_dioxide";
    const NITROGEN: &'static str = "nitrogen";
    const TOXINS: &'static str = "toxins";
    const SLEEPING_AGENT: &'static str = "sleeping_agent";
    const AGENT_B: &'static str = "agent_b";
    const TEMPERATURE: &'static str = "temperature";

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
}
