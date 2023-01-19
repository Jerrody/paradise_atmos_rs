mod constants;
pub mod procs;
mod turf;
mod utils;

use constants::*;
use utils::*;

use auxtools::{StringRef, Value};
use dashmap::DashMap;
use once_cell::sync::Lazy;

use crate::{profile, string_ref};

use self::turf::Turf;

type MixtureRef<'a> = dashmap::mapref::one::Ref<'a, u32, Mixture, ahash::RandomState>;
type MixtureRefMut<'a> = dashmap::mapref::one::RefMut<'a, u32, Mixture, ahash::RandomState>;

pub static GAS_MIXTURES: Lazy<DashMap<u32, Mixture, ahash::RandomState>> = Lazy::new(|| {
    let hasher = ahash::RandomState::new();
    DashMap::with_capacity_and_hasher(Mixture::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT, hasher)
});

#[derive(Debug, Default)]
pub struct Mixture {
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
    /// ### Description
    /// Pre-allocates a 1_000_000 `gas_mixtures`.
    /// Needs for avoding a reallocation a whole [`GAS_MIXTURES`] with each creaing of an instance of `gas_mixture`.
    /// ### Size
    /// Initial size of [`GAS_MIXTURES`] is 68 MBs.
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
    pub unsafe fn register(src: &Value) {
        profile!("register");

        let id = src.raw.data.id;

        GAS_MIXTURES.insert(id, Self::new());
    }

    #[inline(always)]
    pub unsafe fn unregister(src: &Value) {
        profile!("unregister");

        let id = src.raw.data.id;

        GAS_MIXTURES.remove(&id).unwrap_unchecked();
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_mixture_by_src(src: &Value) -> Option<MixtureRef> {
        profile!("get_mixture_by_value");

        let id = src.raw.data.id;

        GAS_MIXTURES.get(&id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_mixture_mut_by_src(src: &Value) -> Option<MixtureRefMut> {
        profile!("get_mixture_mut_by_value");

        let id = src.raw.data.id;

        GAS_MIXTURES.get_mut(&id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_mixture_mut_by_id<'a>(id: u32) -> Option<MixtureRefMut<'a>> {
        profile!("get_mixture_mut_by_id");

        GAS_MIXTURES.get_mut(&id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_oxygen(src: &Value) -> f32 {
        profile!("get_oxygen");

        Self::get_mixture_by_src(src).unwrap_unchecked().oxygen
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_carbon_dioxide(src: &Value) -> f32 {
        profile!("get_carbon_dioxide");

        Self::get_mixture_by_src(src)
            .unwrap_unchecked()
            .carbon_dioxide
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_nitrogen(src: &Value) -> f32 {
        profile!("get_nitrogen");

        Self::get_mixture_by_src(src).unwrap_unchecked().nitrogen
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_toxins(src: &Value) -> f32 {
        profile!("get_toxins");

        Self::get_mixture_by_src(src).unwrap_unchecked().toxins
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_sleeping_agent(src: &Value) -> f32 {
        profile!("get_sleeping_agent");

        Self::get_mixture_by_src(src)
            .unwrap_unchecked()
            .sleeping_agent
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_agent_b(src: &Value) -> f32 {
        profile!("get_agent_b");

        Self::get_mixture_by_src(src).unwrap_unchecked().agent_b
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_volume(src: &Value) -> f32 {
        profile!("get_volume");

        Self::get_mixture_by_src(src).unwrap_unchecked().volume
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_temperature(id: &Value) -> f32 {
        profile!("get_temperature");

        Self::get_mixture_by_src(id).unwrap_unchecked().temperature
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_last_share(src: &Value) -> f32 {
        profile!("get_last_share");

        Self::get_mixture_by_src(src).unwrap_unchecked().last_share
    }

    #[inline(always)]
    pub unsafe fn set_oxygen(src: &Value, oxygen: f32) {
        profile!("set_oxygen");

        Self::get_mixture_mut_by_src(src).unwrap_unchecked().oxygen = oxygen;
    }

    #[inline(always)]
    pub unsafe fn set_carbon_dioxide(src: &Value, carbon_dioxide: f32) {
        profile!("set_carbon_dioxide");

        Self::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .carbon_dioxide = carbon_dioxide;
    }

    #[inline(always)]
    pub unsafe fn set_nitrogen(src: &Value, nitrogen: f32) {
        profile!("set_nitrogen");

        Self::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .nitrogen = nitrogen;
    }

    #[inline(always)]
    pub unsafe fn set_toxins(src: &Value, toxins: f32) {
        profile!("set_toxins");

        Self::get_mixture_mut_by_src(src).unwrap_unchecked().toxins = toxins;
    }

    #[inline(always)]
    pub unsafe fn set_sleeping_agent(src: &Value, sleeping_agent: f32) {
        profile!("set_sleeping_agent");

        Self::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .sleeping_agent = sleeping_agent;
    }

    #[inline(always)]
    pub unsafe fn set_agent_b(src: &Value, agent_b: f32) {
        profile!("set_agent_b");

        Self::get_mixture_mut_by_src(src).unwrap_unchecked().agent_b = agent_b;
    }

    #[inline(always)]
    pub unsafe fn set_volume(src: &Value, volume: f32) {
        profile!("set_volume");

        Self::get_mixture_mut_by_src(src).unwrap_unchecked().volume = volume;
    }

    #[inline(always)]
    pub unsafe fn set_temperature(src: &Value, temperature: f32) {
        profile!("set_temperature");

        Self::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .temperature = temperature;
    }

    #[inline(always)]
    pub unsafe fn set_last_share(src: &Value, last_share: f32) {
        profile!("set_last_share");

        Self::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .last_share = last_share;
    }

    #[inline(always)]
    #[must_use]
    fn get_heat_capacity(&self) -> f32 {
        profile!("get_heat_capacity");

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
        profile!("get_heat_capacity_archived");

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
        profile!("get_total_moles");

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
        profile!("get_total_trace_moles");

        self.sleeping_agent + self.agent_b
    }

    #[inline(always)]
    #[must_use]
    pub fn get_pressure(&self) -> f32 {
        profile!("get_pressure");

        if self.volume > 0.0 {
            return self.get_total_moles() * R_IDEAL_GAS_EQUATION * self.temperature / self.volume;
        }

        0.0
    }

    // I'm not sure that this thing was made by a person with good mental health in DM.
    // Anyway, it could cause, potentially, unexpected behavior.
    #[inline(always)]
    #[must_use]
    pub fn return_volume(&self) -> f32 {
        profile!("return_volume");

        0.0_f32.max(self.volume)
    }

    #[inline(always)]
    #[must_use]
    pub fn get_thermal_energy(&self) -> f32 {
        profile!("get_thermal_energy");

        self.temperature * self.get_heat_capacity()
    }

    #[inline(always)]
    pub fn react(&mut self) -> bool {
        profile!("react");

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

        if self.temperature > FIRE_MINIMUM_TEMPERATURE_TO_EXIST && self.fire() > 0.0 {
            reacting = true;
        }

        reacting
    }

    #[inline(always)]
    #[must_use]
    fn fire(&mut self) -> f32 {
        profile!("fire");

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
        profile!("archive");

        self.oxygen_archived = self.oxygen;
        self.carbon_dioxide_archived = self.carbon_dioxide;
        self.nitrogen_archived = self.nitrogen;
        self.toxins_archived = self.toxins;
        self.sleeping_agent_archived = self.sleeping_agent;
        self.agent_b_archived = self.agent_b;
        self.temperature_archived = self.temperature;
    }

    #[inline(always)]
    pub fn merge(&mut self, giver: Option<MixtureRefMut>) {
        profile!("get_oxygen");

        let giver = match giver {
            Some(giver) => giver,
            None => return,
        };

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

    #[inline(always)]
    pub fn remove(&mut self, removed_id: u32, mut amount: f32) {
        profile!("remove");

        let sum = self.get_total_moles();
        amount = amount.min(sum); // Can not take more air than tile has!

        let mut removed = match amount <= 0.0 {
            true => {
                unsafe { GAS_MIXTURES.remove(&removed_id).unwrap_unchecked() };

                return;
            }
            false => unsafe { Self::get_mixture_mut_by_id(removed_id).unwrap_unchecked() },
        };

        removed.oxygen = quantize((self.oxygen / sum) * amount);
        removed.nitrogen = quantize((self.nitrogen / sum) * amount);
        removed.carbon_dioxide = quantize((self.carbon_dioxide / sum) * amount);
        removed.toxins = quantize((self.toxins / sum) * amount);
        removed.sleeping_agent = quantize((self.sleeping_agent / sum) * amount);
        removed.agent_b = quantize((self.agent_b / sum) * amount);
        removed.temperature = self.temperature;

        self.oxygen -= removed.oxygen;
        self.nitrogen -= removed.nitrogen;
        self.carbon_dioxide -= removed.carbon_dioxide;
        self.toxins -= removed.toxins;
        self.sleeping_agent -= removed.sleeping_agent;
        self.agent_b -= removed.agent_b;
    }

    #[inline(always)]
    pub fn remove_ratio(&mut self, removed_id: u32, mut ratio: f32) {
        profile!("remove_ratio");

        let mut removed = unsafe {
            match ratio <= 0.0 {
                true => {
                    GAS_MIXTURES.remove(&removed_id).unwrap_unchecked();

                    return;
                }
                false => Self::get_mixture_mut_by_id(removed_id).unwrap_unchecked(),
            }
        };

        ratio = ratio.min(1.0);

        removed.oxygen = quantize(self.oxygen * ratio);
        removed.nitrogen = quantize(self.nitrogen * ratio);
        removed.carbon_dioxide = quantize(self.carbon_dioxide * ratio);
        removed.toxins = quantize(self.toxins * ratio);
        removed.sleeping_agent = quantize(self.sleeping_agent * ratio);
        removed.agent_b = quantize(self.agent_b * ratio);
        removed.temperature = self.temperature;

        self.oxygen -= removed.oxygen;
        self.nitrogen -= removed.nitrogen;
        self.carbon_dioxide -= removed.carbon_dioxide;
        self.toxins -= removed.toxins;
        self.sleeping_agent -= removed.sleeping_agent;
        self.agent_b -= removed.agent_b;
    }

    #[inline(always)]
    pub fn copy_from(&mut self, sample: MixtureRef) {
        profile!("copy_from");

        self.oxygen = sample.oxygen;
        self.carbon_dioxide = sample.carbon_dioxide;
        self.nitrogen = sample.nitrogen;
        self.toxins = sample.toxins;
        self.sleeping_agent = sample.sleeping_agent;
        self.agent_b = sample.agent_b;
        self.temperature = sample.temperature;
    }

    // TODO: Make a method looks much more minimalistic.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn copy_from_turf(
        &mut self,
        turf_model: Turf,
        initial_model_temperature: f32,
        initial_model_parent_temperature: f32,
    ) {
        profile!("copy_from_turf");

        self.oxygen = turf_model.oxygen;
        self.carbon_dioxide = turf_model.carbon_dioxide;
        self.nitrogen = turf_model.nitrogen;
        self.toxins = turf_model.toxins;
        self.sleeping_agent = turf_model.sleeping_agent;
        self.agent_b = turf_model.agent_b;

        if turf_model.temperature != initial_model_temperature
            || turf_model.temperature != initial_model_parent_temperature
        {
            self.temperature = turf_model.temperature;
        }
    }

    // TODO: Make a method looks much more minimalistic.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn check_turf(&self, turf_model: Turf, atmos_adjacent_turfs: f32) -> bool {
        profile!("check_turf");

        let delta_oxygen =
            (self.oxygen_archived - turf_model.oxygen) / (atmos_adjacent_turfs + 1.0);
        let delta_carbon_dioxide = (self.carbon_dioxide_archived - turf_model.carbon_dioxide)
            / (atmos_adjacent_turfs + 1.0);
        let delta_nitrogen =
            (self.nitrogen_archived - turf_model.nitrogen) / (atmos_adjacent_turfs + 1.0);
        let delta_toxins =
            (self.toxins_archived - turf_model.toxins) / (atmos_adjacent_turfs + 1.0);
        let delta_sleeping_agent = (self.sleeping_agent_archived - turf_model.sleeping_agent)
            / (atmos_adjacent_turfs + 1.0);
        let delta_agent_b =
            (self.agent_b_archived - turf_model.agent_b) / (atmos_adjacent_turfs + 1.0);
        let delta_temperature = self.temperature_archived - turf_model.temperature;

        // FIXME: Potentially, can be minimized and etc., anyway, this is 💀
        if ((delta_oxygen.abs() > MINIMUM_AIR_TO_SUSPEND)
            && (delta_oxygen.abs() >= self.oxygen_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_carbon_dioxide.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_carbon_dioxide.abs()
                    >= self.carbon_dioxide_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_nitrogen.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_nitrogen.abs() >= self.nitrogen_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_toxins.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_toxins.abs() >= self.toxins_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_sleeping_agent.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_sleeping_agent.abs()
                    >= self.sleeping_agent_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_agent_b.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_agent_b.abs() >= self.agent_b_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND
        {
            return false;
        }

        true
    }

    // TODO: Make a method looks much more minimalistic.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub fn check_turf_total(&self, turf_model: Turf) -> bool {
        profile!("check_turf_total");

        let delta_oxygen = self.oxygen - turf_model.oxygen;
        let delta_carbon_dioxide = self.carbon_dioxide - turf_model.carbon_dioxide;
        let delta_nitrogen = self.nitrogen - turf_model.nitrogen;
        let delta_toxins = self.toxins - turf_model.toxins;
        let delta_sleeping_agent = self.sleeping_agent - turf_model.sleeping_agent;
        let delta_agent_b = self.agent_b - turf_model.agent_b;
        let delta_temperature = self.temperature - turf_model.temperature;

        // FIXME: Potentially, can be minimized and etc., anyway, this is 💀
        if ((delta_oxygen.abs() > MINIMUM_AIR_TO_SUSPEND)
            && (delta_oxygen.abs() >= self.oxygen_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_carbon_dioxide.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_carbon_dioxide.abs()
                    >= self.carbon_dioxide_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_nitrogen.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_nitrogen.abs() >= self.nitrogen_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_toxins.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_toxins.abs() >= self.toxins_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_sleeping_agent.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_sleeping_agent.abs()
                    >= self.sleeping_agent_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_agent_b.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_agent_b.abs() >= self.agent_b_archived * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND
        {
            return false;
        }

        true
    }

    #[inline(always)]
    #[must_use]
    pub fn share(&mut self, sharer: Option<MixtureRefMut>, atmos_adjacent_turfs: f32) -> f32 {
        profile!("share");

        let mut sharer = match sharer {
            Some(sharer) => sharer,
            None => return 0.0,
        };

        if self.oxygen_archived == sharer.oxygen_archived
            && self.carbon_dioxide_archived == sharer.carbon_dioxide_archived
            && self.nitrogen_archived == sharer.nitrogen_archived
            && self.toxins_archived == sharer.toxins_archived
            && self.sleeping_agent_archived == sharer.sleeping_agent_archived
            && self.agent_b_archived == sharer.agent_b_archived
            && self.temperature_archived == sharer.temperature_archived
        {
            return 0.0;
        }

        let delta_oxygen =
            quantize(self.oxygen_archived - sharer.oxygen_archived) / (atmos_adjacent_turfs + 1.0);
        let delta_carbon_dioxide =
            quantize(self.carbon_dioxide_archived - sharer.carbon_dioxide_archived)
                / (atmos_adjacent_turfs + 1.0);
        let delta_nitrogen = quantize(self.nitrogen_archived - sharer.nitrogen_archived)
            / (atmos_adjacent_turfs + 1.0);
        let delta_toxins =
            quantize(self.toxins_archived - sharer.toxins_archived) / (atmos_adjacent_turfs + 1.0);
        let delta_sleeping_agent =
            quantize(self.sleeping_agent_archived - sharer.sleeping_agent_archived)
                / (atmos_adjacent_turfs + 1.0);
        let delta_agent_b = quantize(self.agent_b_archived - sharer.agent_b_archived)
            / (atmos_adjacent_turfs + 1.0);
        let delta_temperature = self.temperature_archived - sharer.temperature_archived;

        let mut old_self_heat_capacity = 0.0;
        let mut old_sharer_heat_capacity = 0.0;

        let mut heat_capacity_self_to_sharer = 0.0;
        let mut heat_capacity_sharer_to_self = 0.0;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let delta_air = delta_oxygen + delta_nitrogen;
            if delta_air != 0.0 {
                let air_heat_capacity = SPECIFIC_HEAT_AIR * delta_air;
                if delta_air > 0.0 {
                    heat_capacity_self_to_sharer += air_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= air_heat_capacity;
                }
            }

            if delta_carbon_dioxide != 0.0 {
                let carbon_dioxide_heat_capacity = SPECIFIC_HEAT_CDO * delta_carbon_dioxide;
                if delta_carbon_dioxide > 0.0 {
                    heat_capacity_self_to_sharer += carbon_dioxide_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= carbon_dioxide_heat_capacity;
                }
            }

            if delta_toxins != 0.0 {
                let toxins_heat_capacity = SPECIFIC_HEAT_TOXIN * delta_toxins;
                if delta_toxins > 0.0 {
                    heat_capacity_self_to_sharer += toxins_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= toxins_heat_capacity;
                }
            }

            if delta_sleeping_agent != 0.0 {
                let sleeping_agent_heat_capacity = SPECIFIC_HEAT_N2O * delta_sleeping_agent;
                if delta_sleeping_agent > 0.0 {
                    heat_capacity_self_to_sharer += sleeping_agent_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= sleeping_agent_heat_capacity;
                }
            }

            if delta_agent_b != 0.0 {
                let agent_b_heat_capacity = SPECIFIC_HEAT_AGENT_B * delta_agent_b;
                if delta_agent_b > 0.0 {
                    heat_capacity_self_to_sharer += agent_b_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= agent_b_heat_capacity;
                }
            }

            old_self_heat_capacity = self.get_heat_capacity();
            old_sharer_heat_capacity = sharer.get_heat_capacity();
        }

        self.oxygen -= delta_oxygen;
        sharer.oxygen += delta_oxygen;

        self.carbon_dioxide -= delta_carbon_dioxide;
        sharer.carbon_dioxide += delta_carbon_dioxide;

        self.nitrogen -= delta_nitrogen;
        sharer.nitrogen += delta_nitrogen;

        self.toxins -= delta_toxins;
        sharer.toxins += delta_toxins;

        self.sleeping_agent -= delta_sleeping_agent;
        sharer.sleeping_agent += delta_sleeping_agent;

        self.agent_b -= delta_agent_b;
        sharer.agent_b += delta_agent_b;

        let moved_moles = delta_oxygen
            + delta_carbon_dioxide
            + delta_nitrogen
            + delta_toxins
            + delta_sleeping_agent
            + delta_agent_b;
        self.last_share = delta_oxygen.abs()
            + delta_carbon_dioxide.abs()
            + delta_nitrogen.abs()
            + delta_toxins.abs()
            + delta_sleeping_agent.abs()
            + delta_agent_b.abs();

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let new_self_heat_capacity = old_self_heat_capacity + heat_capacity_sharer_to_self
                - heat_capacity_self_to_sharer;
            let new_sharer_heat_capacity = old_sharer_heat_capacity + heat_capacity_self_to_sharer
                - heat_capacity_sharer_to_self;

            if new_self_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.temperature = (old_self_heat_capacity * self.temperature
                    - heat_capacity_self_to_sharer * self.temperature_archived
                    + heat_capacity_sharer_to_self * sharer.temperature_archived)
                    / new_self_heat_capacity;
            }

            if new_sharer_heat_capacity > MINIMUM_HEAT_CAPACITY {
                sharer.temperature = (old_sharer_heat_capacity * sharer.temperature
                    - heat_capacity_sharer_to_self * sharer.temperature_archived
                    + heat_capacity_self_to_sharer * self.temperature_archived)
                    / new_sharer_heat_capacity
            }

            // <10% change in sharer heat capacity
            if old_sharer_heat_capacity.abs() > MINIMUM_HEAT_CAPACITY
                && (new_sharer_heat_capacity / old_sharer_heat_capacity - 1.0).abs() < 0.10
                && (new_sharer_heat_capacity / old_sharer_heat_capacity - 1.0).abs() < 0.10
            {
                self.temperature_share(&mut sharer, OPEN_HEAT_TRANSFER_COEFFICIENT);
            }
        }

        if (delta_temperature > MINIMUM_TEMPERATURE_TO_MOVE)
            || moved_moles.abs() > MINIMUM_MOLES_DELTA_TO_MOVE
        {
            let delta_pressure = self.temperature_archived * (self.get_total_moles() + moved_moles)
                - sharer.temperature_archived * (sharer.get_total_moles() - moved_moles);
            delta_pressure * R_IDEAL_GAS_EQUATION / self.volume
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub fn temperature_share(&mut self, sharer: &mut MixtureRefMut, conduction_coefficient: f32) {
        profile!("temperature_share");

        let delta_temperature = self.temperature_archived - sharer.temperature_archived;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity_archived();
            let sharer_heat_capacity = sharer.get_heat_capacity_archived();

            if sharer_heat_capacity > MINIMUM_HEAT_CAPACITY
                && (self_heat_capacity > MINIMUM_HEAT_CAPACITY)
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * sharer_heat_capacity
                        / (self_heat_capacity + sharer_heat_capacity));

                self.temperature -= heat / self_heat_capacity;
                sharer.temperature += heat / sharer_heat_capacity;
            }
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn mimic(
        &mut self,
        turf_model: Turf,
        model_thermal_conductivity: f32,
        model_heat_capacity: f32,
        atmos_adjacent_turfs: f32,
    ) -> f32 {
        profile!("mimic");

        let delta_oxygen =
            quantize(self.oxygen_archived - turf_model.oxygen) / (atmos_adjacent_turfs + 1.0);
        let delta_carbon_dioxide =
            quantize(self.carbon_dioxide_archived - turf_model.carbon_dioxide)
                / (atmos_adjacent_turfs + 1.0);
        let delta_nitrogen =
            quantize(self.nitrogen_archived - turf_model.nitrogen) / (atmos_adjacent_turfs + 1.0);
        let delta_toxins =
            quantize(self.toxins_archived - turf_model.toxins) / (atmos_adjacent_turfs + 1.0);
        let delta_sleeping_agent =
            quantize(self.sleeping_agent_archived - turf_model.sleeping_agent)
                / (atmos_adjacent_turfs + 1.0);
        let delta_agent_b =
            quantize(self.agent_b_archived - turf_model.agent_b) / (atmos_adjacent_turfs + 1.0);
        let delta_temperature = self.temperature_archived - turf_model.temperature;

        let mut old_self_heat_capacity = 0.0;
        let mut heat_capacity_transferred = 0.0;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let delta_air = delta_oxygen + delta_nitrogen;
            if delta_air != 0.0 {
                let air_heat_capacity = SPECIFIC_HEAT_AIR * delta_air;
                heat_capacity_transferred -= air_heat_capacity;
            }

            if delta_carbon_dioxide != 0.0 {
                let carbon_dioxide_heat_capacity = SPECIFIC_HEAT_CDO * delta_carbon_dioxide;
                heat_capacity_transferred -= carbon_dioxide_heat_capacity;
            }

            if delta_toxins != 0.0 {
                let toxins_heat_capacity = SPECIFIC_HEAT_TOXIN * delta_toxins;
                heat_capacity_transferred -= toxins_heat_capacity;
            }

            if delta_sleeping_agent != 0.0 {
                let sleeping_agent_heat_capacity = SPECIFIC_HEAT_N2O * delta_sleeping_agent;
                heat_capacity_transferred -= sleeping_agent_heat_capacity;
            }

            if delta_agent_b != 0.0 {
                let agent_b_heat_capacity = SPECIFIC_HEAT_AGENT_B * delta_agent_b;
                heat_capacity_transferred -= agent_b_heat_capacity;
            }

            old_self_heat_capacity = self.get_heat_capacity();
        }

        self.oxygen -= delta_oxygen;
        self.carbon_dioxide -= delta_carbon_dioxide;
        self.nitrogen -= delta_nitrogen;
        self.toxins -= delta_toxins;
        self.sleeping_agent -= delta_sleeping_agent;
        self.agent_b -= delta_agent_b;

        let moved_moles = delta_oxygen
            + delta_carbon_dioxide
            + delta_nitrogen
            + delta_toxins
            + delta_sleeping_agent
            + delta_agent_b;
        self.last_share = delta_oxygen.abs()
            + delta_carbon_dioxide.abs()
            + delta_nitrogen.abs()
            + delta_toxins.abs()
            + delta_sleeping_agent.abs()
            + delta_agent_b.abs();

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let new_self_heat_capacity = old_self_heat_capacity - heat_capacity_transferred;
            if new_self_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.temperature = (old_self_heat_capacity * self.temperature
                    - heat_capacity_transferred * self.temperature_archived)
                    / new_self_heat_capacity;
            }

            self.temperature_mimic(
                turf_model.temperature,
                model_heat_capacity,
                model_thermal_conductivity,
            );
        }

        if (delta_temperature > MINIMUM_TEMPERATURE_TO_MOVE)
            || moved_moles.abs() > MINIMUM_MOLES_DELTA_TO_MOVE
        {
            let delta_pressure = self.temperature_archived * (self.get_total_moles() + moved_moles)
                - turf_model.temperature
                    * (turf_model.oxygen
                        + turf_model.carbon_dioxide
                        + turf_model.nitrogen
                        + turf_model.toxins
                        + turf_model.sleeping_agent
                        + turf_model.agent_b);
            delta_pressure * R_IDEAL_GAS_EQUATION / self.volume
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub fn temperature_mimic(
        &mut self,
        model_temperature: f32,
        model_heat_capacity: f32,
        conduction_coefficient: f32,
    ) {
        profile!("temperature_mimic");

        let delta_temperature = self.temperature - model_temperature;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity();

            if (model_heat_capacity > MINIMUM_HEAT_CAPACITY)
                && (self_heat_capacity > MINIMUM_HEAT_CAPACITY)
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * model_heat_capacity
                        / (self_heat_capacity + model_heat_capacity));

                self.temperature -= heat / self_heat_capacity
            }
        }
    }

    #[inline(always)]
    pub fn temperature_turf_share(&mut self, turf_sharer: &Value, conduction_coefficient: f32) {
        profile!("temperature_turf_share");

        let temperature_name = unsafe { string_ref!("temperature") };

        let turf_sharer_heat_capacity = unsafe {
            turf_sharer
                .get_number(string_ref!("heat_capacity"))
                .unwrap_unchecked()
        };
        let turf_sharer_temperature = unsafe {
            turf_sharer
                .get_number(temperature_name.clone())
                .unwrap_unchecked()
        };

        let delta_temperature = self.temperature_archived - turf_sharer_temperature;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity();

            if (turf_sharer_heat_capacity > MINIMUM_HEAT_CAPACITY)
                && (self_heat_capacity > MINIMUM_HEAT_CAPACITY)
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * turf_sharer_heat_capacity
                        / (self_heat_capacity + turf_sharer_heat_capacity));

                self.temperature -= heat / self_heat_capacity;
                unsafe {
                    turf_sharer
                        .set(
                            temperature_name,
                            turf_sharer_temperature + heat / turf_sharer_heat_capacity,
                        )
                        .unwrap_unchecked()
                };
            }
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn compare(&self, sample: MixtureRef) -> bool {
        profile!("compare");

        if ((self.oxygen - sample.oxygen).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((self.oxygen < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.oxygen)
                || (self.oxygen > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.oxygen))
        {
            return false;
        }

        if ((self.nitrogen - sample.nitrogen).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((self.nitrogen < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.nitrogen)
                || (self.nitrogen > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.nitrogen))
        {
            return false;
        }

        if ((self.carbon_dioxide - sample.carbon_dioxide).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((self.carbon_dioxide
                < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.carbon_dioxide)
                || (self.carbon_dioxide
                    > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.carbon_dioxide))
        {
            return false;
        }

        if ((self.toxins - sample.toxins).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((self.toxins < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.toxins)
                || (self.toxins > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.toxins))
        {
            return false;
        }

        if ((self.sleeping_agent - sample.sleeping_agent).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((self.sleeping_agent
                < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.sleeping_agent)
                || (self.sleeping_agent
                    > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.sleeping_agent))
        {
            return false;
        }

        if ((self.agent_b - sample.agent_b).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((self.agent_b < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.agent_b)
                || (self.agent_b > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample.agent_b))
        {
            return false;
        }

        if self.get_total_moles() > MINIMUM_AIR_TO_SUSPEND
            && ((self.temperature - sample.temperature).abs()
                > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND)
            && ((self.temperature
                < (1.0 - MINIMUM_TEMPERATURE_RATIO_TO_SUSPEND) * sample.temperature)
                || (self.temperature
                    > (1.0 + MINIMUM_TEMPERATURE_RATIO_TO_SUSPEND) * sample.temperature))
        {
            return false;
        }

        true
    }

    #[inline(always)]
    #[must_use]
    pub fn get_breath_partial_pressure(&self, gas_pressure: f32) -> f32 {
        profile!("get_breath_partial_pressure");

        (gas_pressure * R_IDEAL_GAS_EQUATION * self.temperature) / BREATH_VOLUME
    }

    //Reverse of the above
    #[inline(always)]
    #[must_use]
    pub fn get_true_breath_pressure(&self, breath_pp: f32) -> f32 {
        profile!("get_true_breath_pressure");

        (breath_pp * BREATH_VOLUME) / (R_IDEAL_GAS_EQUATION * self.temperature)
    }
}
