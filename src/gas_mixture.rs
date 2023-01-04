mod heat_constants;
mod helpers;
mod procs;

use auxtools::*;

use crate::string_ref;

struct GasMixture;

impl GasMixture {
    // Field names
    const OXYGEN_NAME: &str = "oxygen";
    const CARBON_DIOXIDE_NAME: &str = "carbon_dioxide";
    const NITROGEN_NAME: &str = "nitrogen";
    const TOXINS_NAME: &str = "toxins";
    const SLEEPING_AGENT_NAME: &str = "sleeping_agent";
    const AGENT_B_NAME: &str = "agent_b";
    const VOLUME_NAME: &str = "volume";
    const TEMPERATURE_NAME: &str = "temperature";
    const LAST_SHARE_NAME: &str = "last_share";
    const FUEL_BURNT_NAME: &str = "fuel_burnt";
    const OXYGEN_ARCHIVED_NAME: &str = "oxygen_archived";
    const CARBON_DIOXIDE_ARCHIVED_NAME: &str = "carbon_dioxide_archived";
    const NITROGEN_ARCHIVED_NAME: &str = "nitrogen_archived";
    const TOXINS_ARCHIVED_NAME: &str = "toxins_archived";
    const SLEEPING_AGENT_ARCHIVED_NAME: &str = "sleeping_agent_archived";
    const AGENT_B_ARCHIVED_NAME: &str = "agent_b_archived";
    const TEMPERATURE_ARCHIVED_NAME: &str = "temperature_archived";

    #[inline(always)]
    pub fn get_oxygen(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::OXYGEN_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_carbon_dioxide(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::CARBON_DIOXIDE_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_nitrogen(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::NITROGEN_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_toxins(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::TOXINS_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_sleeping_agent(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::SLEEPING_AGENT_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_agent_b(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::AGENT_B_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_volume(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::VOLUME_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_temperature(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::TEMPERATURE_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_last_share(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::LAST_SHARE_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_fuel_burnt(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::FUEL_BURNT_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_oxygen_archived(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::OXYGEN_ARCHIVED_NAME))
                .unwrap_unchecked()
        }
    }
    #[inline(always)]
    fn get_carbon_dioxide_archived(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::CARBON_DIOXIDE_ARCHIVED_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_nitrogen_archived(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::NITROGEN_ARCHIVED_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_toxins_archived(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::TOXINS_ARCHIVED_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_sleeping_agent_archived(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::SLEEPING_AGENT_ARCHIVED_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_agent_b_archived(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::AGENT_B_ARCHIVED_NAME))
                .unwrap_unchecked()
        }
    }

    #[inline(always)]
    fn get_temperature_archived(gas_mixture: &Value) -> f32 {
        unsafe {
            gas_mixture
                .get_number(string_ref!(Self::TEMPERATURE_ARCHIVED_NAME))
                .unwrap_unchecked()
        }
    }
}
