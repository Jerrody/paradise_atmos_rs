use crate::gas_mixture::{constants::*, mixtures::MixtureNative};
use auxtools::Value;

pub struct Mixture {
    pub oxygen: f32,
    pub carbon_dioxide: f32,
    pub nitrogen: f32,
    pub toxins: f32,
    pub sleeping_agent: f32,
    pub agent_b: f32,
    /// In Kelvin.
    pub temperature: f32,
}

impl Mixture {
    #[inline(always)]
    #[must_use]
    pub fn new(gas_mixture: &Value) -> Self {
        Self {
            oxygen: MixtureNative::get_oxygen(gas_mixture),
            carbon_dioxide: MixtureNative::get_carbon_dioxide(gas_mixture),
            nitrogen: MixtureNative::get_nitrogen(gas_mixture),
            toxins: MixtureNative::get_toxins(gas_mixture),
            sleeping_agent: MixtureNative::get_sleeping_agent(gas_mixture),
            agent_b: MixtureNative::get_agent_b(gas_mixture),
            temperature: MixtureNative::get_temperature(gas_mixture),
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn fire(&mut self) -> f32 {
        let mut fuel_burnt = 0.0;
        let mut energy_released = 0.0;
        let old_heat_capacity = self.heat_capacity();

        //Handle plasma burning
        if self.toxins > MINIMUM_HEAT_CAPACITY {
            let plasma_burn_rate;
            let oxygen_burn_rate;
            let temperature_scale;

            if self.temperature > PLASMA_UPPER_TEMPERATURE {
                temperature_scale = 1.0;
            } else {
                temperature_scale = (self.temperature - PLASMA_MINIMUM_BURN_TEMPERATURE)
                    / (PLASMA_UPPER_TEMPERATURE - PLASMA_MINIMUM_BURN_TEMPERATURE);
            }

            if temperature_scale > 0.0 {
                oxygen_burn_rate = OXYGEN_BURN_RATE_BASE - temperature_scale;

                if self.oxygen > self.toxins * PLASMA_OXYGEN_FULLBURN {
                    plasma_burn_rate = (self.toxins * temperature_scale) / PLASMA_BURN_RATE_DELTA
                } else {
                    plasma_burn_rate = (temperature_scale * (self.oxygen / PLASMA_OXYGEN_FULLBURN))
                        / PLASMA_BURN_RATE_DELTA;
                }

                if plasma_burn_rate > MINIMUM_HEAT_CAPACITY {
                    self.toxins -= plasma_burn_rate;
                    self.oxygen -= plasma_burn_rate * oxygen_burn_rate;
                    self.carbon_dioxide += plasma_burn_rate;

                    energy_released += FIRE_PLASMA_ENERGY_RELEASED * (plasma_burn_rate);

                    fuel_burnt += (plasma_burn_rate) * (1.0 + oxygen_burn_rate);
                }
            }
        }

        if energy_released > 0.0 {
            let new_heat_capacity = self.heat_capacity();

            if new_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.temperature =
                    (self.temperature * old_heat_capacity + energy_released) / new_heat_capacity;
            }
        }

        fuel_burnt
    }

    #[inline(always)]
    #[must_use]
    pub fn heat_capacity(&self) -> f32 {
        self.carbon_dioxide * SPECIFIC_HEAT_CDO
            + (self.oxygen + self.nitrogen) * SPECIFIC_HEAT_AIR
            + self.toxins * SPECIFIC_HEAT_TOXIN
            + self.sleeping_agent * SPECIFIC_HEAT_N2O
            + self.agent_b * SPECIFIC_HEAT_AGENT_B
    }
}
