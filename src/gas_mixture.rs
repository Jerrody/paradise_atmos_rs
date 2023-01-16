mod constants;
pub mod procs;
mod utils;

use constants::*;

use auxtools::Value;
use dashmap::DashMap;
use once_cell::sync::Lazy;

type MixtureRef<'a> = dashmap::mapref::one::Ref<'a, usize, Mixture>;
type MixtureRefMut<'a> = dashmap::mapref::one::RefMut<'a, usize, Mixture>;

static GAS_MIXTURES: Lazy<DashMap<usize, Mixture>> =
    Lazy::new(|| DashMap::with_capacity(Mixture::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT));

#[derive(Debug, Default)]
struct Mixture {
    oxygen: f32,
    carbon_dioxide: f32,
    nitrogen: f32,
    toxins: f32,
    sleeping_agent: f32,
    agent_b: f32,
    volume: f32,
    temperature: f32, //in Kelvin
    last_share: f32,
    oxygen_archived: f32,
    carbon_dioxide_archived: f32,
    nitrogen_archived: f32,
    toxins_archived: f32,
    sleeping_agent_archived: f32,
    agent_b_archived: f32,
    temperature_archived: f32,
    fuel_burnt: f32,
}

impl Mixture {
    const DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT: usize = 1_000_000;
    /// #### Description
    /// Liters in a cell.
    const CELL_VOLUME: f32 = 2500.0;

    #[inline(always)]
    fn new() -> Self {
        Self {
            volume: Self::CELL_VOLUME,
            ..Default::default()
        }
    }

    #[inline(always)]
    pub fn register(id: usize) {
        GAS_MIXTURES.insert(id, Self::new());
    }

    #[inline(always)]
    #[must_use]
    pub fn get_mixture(id: &Value) -> MixtureRef {
        let id = unsafe { id.as_number().unwrap_unchecked() as usize };

        unsafe { GAS_MIXTURES.get(&id).unwrap_unchecked() }
    }

    #[inline(always)]
    #[must_use]
    pub fn get_mixture_mut(id: &Value) -> MixtureRefMut {
        let id = unsafe { id.as_number().unwrap_unchecked() as usize };

        unsafe { GAS_MIXTURES.get_mut(&id).unwrap_unchecked() }
    }

    #[inline(always)]
    #[must_use]
    pub fn get_oxygen(id: &Value) -> f32 {
        Self::get_mixture(id).oxygen
    }

    #[inline(always)]
    #[must_use]
    pub fn get_carbon_dioxide(id: &Value) -> f32 {
        Self::get_mixture(id).carbon_dioxide
    }

    #[inline(always)]
    #[must_use]
    pub fn get_nitrogen(id: &Value) -> f32 {
        Self::get_mixture(id).nitrogen
    }

    #[inline(always)]
    #[must_use]
    pub fn get_toxins(id: &Value) -> f32 {
        Self::get_mixture(id).toxins
    }

    #[inline(always)]
    #[must_use]
    pub fn get_sleeping_agent(id: &Value) -> f32 {
        Self::get_mixture(id).sleeping_agent
    }

    #[inline(always)]
    #[must_use]
    pub fn get_agent_b(id: &Value) -> f32 {
        Self::get_mixture(id).agent_b
    }

    #[inline(always)]
    #[must_use]
    pub fn get_volume(id: &Value) -> f32 {
        Self::get_mixture(id).volume
    }

    #[inline(always)]
    #[must_use]
    pub fn get_temperature(id: &Value) -> f32 {
        Self::get_mixture(id).temperature
    }

    #[inline(always)]
    #[must_use]
    pub fn get_last_share(id: &Value) -> f32 {
        Self::get_mixture(id).last_share
    }

    #[inline(always)]
    pub fn set_oxygen(id: &Value, oxygen: f32) {
        Self::get_mixture_mut(id).oxygen = oxygen;
    }

    #[inline(always)]
    pub fn set_carbon_dioxide(id: &Value, carbon_dioxide: f32) {
        Self::get_mixture_mut(id).carbon_dioxide = carbon_dioxide;
    }

    #[inline(always)]
    pub fn set_nitrogen(id: &Value, nitrogen: f32) {
        Self::get_mixture_mut(id).nitrogen = nitrogen;
    }

    #[inline(always)]
    pub fn set_toxins(id: &Value, toxins: f32) {
        Self::get_mixture_mut(id).toxins = toxins;
    }

    #[inline(always)]
    pub fn set_sleeping_agent(id: &Value, sleeping_agent: f32) {
        Self::get_mixture_mut(id).sleeping_agent = sleeping_agent;
    }

    #[inline(always)]
    pub fn set_agent_b(id: &Value, agent_b: f32) {
        Self::get_mixture_mut(id).agent_b = agent_b;
    }

    #[inline(always)]
    pub fn set_volume(id: &Value, volume: f32) {
        Self::get_mixture_mut(id).volume = volume;
    }

    #[inline(always)]
    pub fn set_temperature(id: &Value, temperature: f32) {
        Self::get_mixture_mut(id).temperature = temperature;
    }

    #[inline(always)]
    pub fn set_last_share(id: &Value, last_share: f32) {
        Self::get_mixture_mut(id).last_share = last_share;
    }

    #[inline(always)]
    #[must_use]
    fn get_heat_capacity(&self) -> f32 {
        utils::calculate_heat_capacity(
            self.carbon_dioxide,
            self.oxygen,
            self.nitrogen,
            self.toxins,
            self.sleeping_agent,
            self.agent_b,
        )
    }

    #[inline(always)]
    #[must_use]
    fn get_heat_capacity_archived(&self) -> f32 {
        utils::calculate_heat_capacity(
            self.carbon_dioxide_archived,
            self.oxygen_archived,
            self.nitrogen_archived,
            self.toxins_archived,
            self.sleeping_agent_archived,
            self.agent_b_archived,
        )
    }

    #[inline(always)]
    #[must_use]
    pub fn get_total_moles(&self) -> f32 {
        self.oxygen
            + self.carbon_dioxide
            + self.nitrogen
            + self.toxins
            + self.sleeping_agent
            + self.agent_b
    }

    #[inline(always)]
    #[must_use]
    pub fn get_total_trace_moles(&self) -> f32 {
        self.sleeping_agent + self.agent_b
    }

    #[inline(always)]
    #[must_use]
    pub fn get_pressure(&self) -> f32 {
        if self.volume > ZERO {
            return self.get_total_moles() * R_IDEAL_GAS_EQUATION * self.temperature / self.volume;
        }

        ZERO
    }

    // I'm not sure that this thing was made by a person with good mental health in DM.
    // Anyway, it could cause, potentially, unexpected behavior.
    #[inline(always)]
    #[must_use]
    pub fn return_volume(&self) -> f32 {
        ZERO.max(self.volume)
    }

    #[inline(always)]
    #[must_use]
    pub fn get_thermal_energy(&self) -> f32 {
        self.temperature * self.get_heat_capacity()
    }

    #[inline(always)]
    pub fn react(&mut self) -> bool {
        let mut reacting = false; //set to 1 if a notable reaction occured (used by pipe_network)

        if self.temperature > 900.0
            && self.toxins > MINIMUM_HEAT_CAPACITY
            && self.carbon_dioxide > MINIMUM_HEAT_CAPACITY
        {
            let gases = [
                self.carbon_dioxide * 0.75,
                self.toxins * 0.25,
                self.agent_b * 0.05,
            ];

            let reaction_rate = unsafe {
                gases
                    .into_iter()
                    .min_by(|a, b| a.total_cmp(b))
                    .unwrap_unchecked()
            };

            self.carbon_dioxide -= reaction_rate;
            self.oxygen += reaction_rate;
            self.agent_b -= reaction_rate * 0.05;
            self.temperature += (reaction_rate * 20_000.0) / self.get_heat_capacity();

            reacting = true;
        }

        if self.temperature > FIRE_MINIMUM_TEMPERATURE_TO_EXIST {
            if self.fire() > 0.0 {
                reacting = true;
            }
        }

        reacting
    }

    #[inline(always)]
    #[must_use]
    fn fire(&mut self) -> f32 {
        self.fuel_burnt = 0.0;
        let mut energy_released = 0.0;
        let old_heat_capacity = self.get_heat_capacity();

        //Handle plasma burning
        if self.toxins > MINIMUM_HEAT_CAPACITY {
            let plasma_burn_rate;
            let oxygen_burn_rate;

            let temperature_scale = if self.temperature > PLASMA_UPPER_TEMPERATURE {
                1.0
            } else {
                (self.temperature - PLASMA_MINIMUM_BURN_TEMPERATURE)
                    / (PLASMA_UPPER_TEMPERATURE - PLASMA_MINIMUM_BURN_TEMPERATURE)
            };

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

                    self.fuel_burnt += (plasma_burn_rate) * (1.0 + oxygen_burn_rate);
                }
            }
        }

        if energy_released > 0.0 {
            let new_heat_capacity = self.get_heat_capacity();

            if new_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.temperature =
                    (self.temperature * old_heat_capacity + energy_released) / new_heat_capacity;
            }
        }

        self.fuel_burnt
    }

    #[inline(always)]
    pub fn archive(&mut self) {
        self.oxygen_archived = self.oxygen;
        self.carbon_dioxide_archived = self.carbon_dioxide;
        self.nitrogen_archived = self.nitrogen;
        self.toxins_archived = self.toxins;
        self.sleeping_agent_archived = self.sleeping_agent;
        self.agent_b_archived = self.agent_b;
        self.temperature_archived = self.temperature;
    }

    #[inline(always)]
    pub fn merge(&mut self, giver: MixtureRefMut) {
        if (self.temperature - giver.temperature).abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity();
            let giver_heat_capacity = giver.get_heat_capacity();
            let combined_heat_capacity = giver_heat_capacity + self_heat_capacity;
            if combined_heat_capacity != 0.0 {
                self.temperature = (giver.temperature * giver_heat_capacity
                    + self.temperature * self_heat_capacity)
                    / combined_heat_capacity
            }
        }

        self.oxygen += giver.oxygen;
        self.carbon_dioxide += giver.carbon_dioxide;
        self.nitrogen += giver.nitrogen;
        self.toxins += giver.toxins;
        self.sleeping_agent += giver.sleeping_agent;
        self.agent_b += giver.agent_b;
    }
}
