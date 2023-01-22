mod getters;
mod setters;

use crate::constants::*;
use crate::turf::Turf;
use crate::utils::*;
use crate::{profile, string_ref};

use once_cell::unsync::Lazy;

use auxtools::{StringRef, Value};

/// ### Description
/// SOA (struct of arrays). Used for efficient cache utilization.
/// ### Why?
/// You would say that it's less comfortable to use `gas_mixture` and why this unnecessary complexity.
/// The answer is to load components (aka `oxygen`, `carbon_dioxide`, and others) that need to use, with no extra load.
/// Imagine that you use simple `Vec<Mixture>` and need to use the `return_volume` method, so, you index an instance of `Mixture`.
/// So, you load to the cache whole **68** `bytes`! And use only 4 bytes :) == waste CPU time on useless action.
/// I hope I answered your question!
pub static mut MIXTURES: Lazy<Mixture> = Lazy::new(Mixture::new);

#[derive(Debug, Default)]
pub struct Mixture {
    oxygen: Vec<f32>,
    carbon_dioxide: Vec<f32>,
    nitrogen: Vec<f32>,
    toxins: Vec<f32>,
    sleeping_agent: Vec<f32>,
    agent_b: Vec<f32>,
    volume: Vec<f32>,
    temperature: Vec<f32>, //in Kelvin
    last_share: Vec<f32>,
    oxygen_archived: Vec<f32>,
    carbon_dioxide_archived: Vec<f32>,
    nitrogen_archived: Vec<f32>,
    toxins_archived: Vec<f32>,
    sleeping_agent_archived: Vec<f32>,
    agent_b_archived: Vec<f32>,
    temperature_archived: Vec<f32>,
    fuel_burnt: Vec<f32>,
    is_initialized: Vec<bool>,
}

impl Mixture {
    /// ### Description
    /// Pre-allocates a 1_000_000 `gas_mixtures`.
    /// Needs for avoding a reallocation a whole [`GAS_MIXTURES`] with each creaing of an instance of `gas_mixture`.
    /// ### Size
    /// Initial size of [`MIXTURES`] is 68 MBs.
    const DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT: usize = 1_000_000;
    /// #### Description
    /// Liters in a cell.
    const CELL_VOLUME: f32 = 2500.0;

    /// TODO: Add an `cfg_attr` to disable an `inline` when `profile` or `profile_proc` enabled.
    #[inline(always)]
    #[must_use]
    fn new() -> Self {
        let vec_of_zeros = vec![f32::default(); Self::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT];
        let vec_of_cell_volumes =
            vec![Self::CELL_VOLUME; Self::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT];
        let vec_of_bools = vec![Default::default(); Self::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT];

        Self {
            oxygen: vec_of_zeros.clone(),
            carbon_dioxide: vec_of_zeros.clone(),
            nitrogen: vec_of_zeros.clone(),
            toxins: vec_of_zeros.clone(),
            sleeping_agent: vec_of_zeros.clone(),
            agent_b: vec_of_zeros.clone(),
            volume: vec_of_cell_volumes,
            temperature: vec_of_zeros.clone(),
            last_share: vec_of_zeros.clone(),
            oxygen_archived: vec_of_zeros.clone(),
            carbon_dioxide_archived: vec_of_zeros.clone(),
            nitrogen_archived: vec_of_zeros.clone(),
            toxins_archived: vec_of_zeros.clone(),
            sleeping_agent_archived: vec_of_zeros.clone(),
            agent_b_archived: vec_of_zeros.clone(),
            temperature_archived: vec_of_zeros.clone(),
            fuel_burnt: vec_of_zeros,
            is_initialized: vec_of_bools,
        }
    }

    #[inline(always)]
    pub unsafe fn register(&mut self, src: &Value) {
        // FIXME: Won't work as expected due to `inline` fix with adding additional `cfg_attr`.
        profile!("register");

        let id = src.raw.data.id as usize;

        *self.is_initialized.get_unchecked_mut(id) = true;
    }

    #[inline(always)]
    pub unsafe fn unregister(&mut self, src: &Value) {
        profile!("unregister");

        let id = src.raw.data.id as usize;

        *self.is_initialized.get_unchecked_mut(id) = false;

        self.set_oxygen(id, 0.0);
        self.set_carbon_dioxide(id, 0.0);
        self.set_nitrogen(id, 0.0);
        self.set_toxins(id, 0.0);
        self.set_sleeping_agent(id, 0.0);
        self.set_agent_b(id, 0.0);
        self.set_volume(id, Self::CELL_VOLUME);
        self.set_temperature(id, 0.0); //in Kelvin
        self.set_last_share(id, 0.0);
        self.set_oxygen_archived(id, 0.0);
        self.set_carbon_dioxide_archived(id, 0.0);
        self.set_nitrogen_archived(id, 0.0);
        self.set_toxins_archived(id, 0.0);
        self.set_sleeping_agent_archived(id, 0.0);
        self.set_agent_b_archived(id, 0.0);
        self.set_temperature_archived(id, 0.0);
        self.set_fuel_burnt(id, 0.0);
    }

    #[inline(always)]
    pub fn unregister_by_id(&mut self, id: usize) {
        profile!("unregister");

        unsafe {
            *self.is_initialized.get_unchecked_mut(id) = false;
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn get_is_initialized(&self, id: usize) -> bool {
        profile!("get_oxygen");

        unsafe { *self.is_initialized.get_unchecked(id) }
    }

    // PROCS
    // ============================================================================================

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_heat_capacity(&self, id: usize) -> f32 {
        profile!("get_heat_capacity");

        crate::utils::calculate_heat_capacity(
            self.get_carbon_dioxide(id),
            self.get_oxygen(id),
            self.get_nitrogen(id),
            self.get_toxins(id),
            self.get_sleeping_agent(id),
            self.get_agent_b(id),
        )
    }

    #[inline(always)]
    #[must_use]
    unsafe fn get_heat_capacity_archived(&self, id: usize) -> f32 {
        profile!("get_heat_capacity_archived");

        crate::utils::calculate_heat_capacity(
            self.get_carbon_dioxide_archived(id),
            self.get_oxygen_archived(id),
            self.get_nitrogen_archived(id),
            self.get_toxins_archived(id),
            self.get_sleeping_agent_archived(id),
            self.get_agent_b_archived(id),
        )
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_total_moles(&self, id: usize) -> f32 {
        profile!("get_total_moles");

        self.get_oxygen(id)
            + self.get_carbon_dioxide(id)
            + self.get_nitrogen(id)
            + self.get_toxins(id)
            + self.get_sleeping_agent(id)
            + self.get_agent_b(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_total_trace_moles(&self, id: usize) -> f32 {
        profile!("get_total_trace_moles");

        self.get_sleeping_agent(id) + self.get_agent_b(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_pressure(&self, id: usize) -> f32 {
        profile!("get_pressure");

        let volume = self.get_volume(id);
        if volume > 0.0 {
            return self.get_total_moles(id) * R_IDEAL_GAS_EQUATION * self.get_temperature(id)
                / volume;
        }

        0.0
    }

    // I'm not sure that this thing was made by a person with good mental health in DM.
    // Anyway, it could cause, potentially, unexpected behavior.
    #[inline(always)]
    #[must_use]
    pub unsafe fn return_volume(&self, id: usize) -> f32 {
        profile!("return_volume");

        0.0_f32.max(self.get_volume(id))
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_thermal_energy(&self, id: usize) -> f32 {
        profile!("get_thermal_energy");

        self.get_temperature(id) * self.get_heat_capacity(id)
    }

    #[inline(always)]
    pub unsafe fn react(&mut self, id: usize) -> bool {
        profile!("react");

        let mut reacting = false; //set to 1 if a notable reaction occured (used by pipe_network)

        if self.get_agent_b(id) != 0.0
            && self.get_temperature(id) > 900.0
            && self.get_toxins(id) > MINIMUM_HEAT_CAPACITY
            && self.get_carbon_dioxide(id) > MINIMUM_HEAT_CAPACITY
        {
            let gases = [
                self.get_carbon_dioxide(id) * 0.75,
                self.get_toxins(id) * 0.25,
                self.get_agent_b(id) * 0.05,
            ];

            let reaction_rate = unsafe {
                gases
                    .into_iter()
                    .min_by(|a, b| a.total_cmp(b))
                    .unwrap_unchecked()
            };

            self.carbon_dioxide[id] -= reaction_rate;
            self.oxygen[id] += reaction_rate;
            self.agent_b[id] -= reaction_rate * 0.05;
            self.temperature[id] += (reaction_rate * 20_000.0) / self.get_heat_capacity(id);

            reacting = true;
        }

        if self.temperature[id] > FIRE_MINIMUM_TEMPERATURE_TO_EXIST && self.fire(id) > 0.0 {
            reacting = true;
        }

        reacting
    }

    #[inline(always)]
    #[must_use]
    unsafe fn fire(&mut self, id: usize) -> f32 {
        profile!("fire");

        self.fuel_burnt[id] = 0.0;
        let mut energy_released = 0.0;
        let old_heat_capacity = self.get_heat_capacity(id);

        //Handle plasma burning
        if self.toxins[id] > MINIMUM_HEAT_CAPACITY {
            let plasma_burn_rate;
            let oxygen_burn_rate;

            let temperature_scale = if self.temperature[id] > PLASMA_UPPER_TEMPERATURE {
                1.0
            } else {
                (self.temperature[id] - PLASMA_MINIMUM_BURN_TEMPERATURE)
                    / (PLASMA_UPPER_TEMPERATURE - PLASMA_MINIMUM_BURN_TEMPERATURE)
            };

            if temperature_scale > 0.0 {
                oxygen_burn_rate = OXYGEN_BURN_RATE_BASE - temperature_scale;

                let toxins = self.get_toxins(id);
                if self.get_oxygen(id) > toxins * PLASMA_OXYGEN_FULLBURN {
                    plasma_burn_rate = (toxins * temperature_scale) / PLASMA_BURN_RATE_DELTA
                } else {
                    plasma_burn_rate = (temperature_scale
                        * (self.get_oxygen(id) / PLASMA_OXYGEN_FULLBURN))
                        / PLASMA_BURN_RATE_DELTA;
                }

                if plasma_burn_rate > MINIMUM_HEAT_CAPACITY {
                    self.set_toxins(id, self.get_toxins(id) - plasma_burn_rate);
                    self.set_oxygen(
                        id,
                        self.get_oxygen(id) - plasma_burn_rate * oxygen_burn_rate,
                    );
                    self.set_carbon_dioxide(id, self.get_carbon_dioxide(id) + plasma_burn_rate);

                    energy_released += FIRE_PLASMA_ENERGY_RELEASED * (plasma_burn_rate);

                    self.set_fuel_burnt(
                        id,
                        self.get_fuel_burnt(id) + ((plasma_burn_rate) * (1.0 + oxygen_burn_rate)),
                    );
                }
            }
        }

        if energy_released > 0.0 {
            let new_heat_capacity = self.get_heat_capacity(id);

            if new_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.set_temperature(
                    id,
                    (self.get_temperature(id) * old_heat_capacity + energy_released)
                        / new_heat_capacity,
                );
            }
        }

        self.get_fuel_burnt(id)
    }

    #[inline(always)]
    pub unsafe fn archive(&mut self, id: usize) {
        profile!("archive");

        self.set_oxygen_archived(id, self.get_oxygen(id));
        self.set_carbon_dioxide_archived(id, self.get_carbon_dioxide(id));
        self.set_nitrogen_archived(id, self.get_nitrogen(id));
        self.set_toxins_archived(id, self.get_toxins(id));
        self.set_sleeping_agent_archived(id, self.get_sleeping_agent(id));
        self.set_agent_b_archived(id, self.get_agent_b(id));
        self.set_temperature_archived(id, self.get_temperature(id));
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub unsafe  fn merge(&mut self, id: usize, giver_id: usize) {
        profile!("get_oxygen");

        if !self.get_is_initialized(giver_id) {
            return;
        }

        if (self.get_temperature(id) - self.get_temperature(giver_id)).abs()
            > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER
        {
            let self_heat_capacity = self.get_heat_capacity(id);
            let giver_heat_capacity = self.get_heat_capacity(giver_id);
            let combined_heat_capacity = giver_heat_capacity + self_heat_capacity;
            if combined_heat_capacity != 0.0 {
                self.set_temperature(
                    id,
                    (self.get_temperature(giver_id) * giver_heat_capacity
                        + self.get_temperature(id) * self_heat_capacity)
                        / combined_heat_capacity,
                );
            }
        }

        self.set_oxygen(id, self.get_oxygen(id) + self.get_oxygen(giver_id));
        self.set_carbon_dioxide(id, self.get_carbon_dioxide(id) + self.get_carbon_dioxide(giver_id));
        self.set_nitrogen(id, self.get_nitrogen(id) + self.get_nitrogen(giver_id));
        self.set_toxins(id, self.get_toxins(id) + self.get_toxins(giver_id));
        self.set_sleeping_agent(id, self.get_sleeping_agent(id) + self.get_sleeping_agent(giver_id));
        self.set_agent_b(id, self.get_agent_b(id) + self.get_agent_b(giver_id));
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub unsafe  fn remove(&mut self, id: usize, removed_id: usize, mut amount: f32) {
        profile!("remove");

        let sum = self.get_total_moles(id);
        amount = amount.min(sum); // Can not take more air than tile has!

        if amount <= 0.0 {
            self.unregister_by_id(removed_id);

            return;
        }

        self.set_oxygen(removed_id, quantize((self.get_oxygen(id) / sum) * amount));
        self.set_nitrogen(removed_id, quantize((self.get_nitrogen(id) / sum) * amount));
        self.set_carbon_dioxide(removed_id,quantize((self.get_carbon_dioxide(id) / sum) * amount),);
        self.set_toxins(removed_id, quantize((self.get_toxins(id) / sum) * amount));
        self.set_sleeping_agent(removed_id,quantize((self.get_sleeping_agent(id) / sum) * amount),);
        self.set_agent_b(removed_id, quantize((self.get_agent_b(id) / sum) * amount));
        self.set_temperature(removed_id, self.get_temperature(id));

        self.set_oxygen(id, self.get_oxygen(id) - self.get_oxygen(removed_id));
        self.set_nitrogen(id, self.get_nitrogen(id) - self.get_nitrogen(removed_id));
        self.set_carbon_dioxide(id, self.get_carbon_dioxide(id) - self.get_carbon_dioxide(removed_id));
        self.set_toxins(id, self.get_toxins(id) - self.get_toxins(removed_id));
        self.set_sleeping_agent(id, self.get_sleeping_agent(id) - self.get_sleeping_agent(removed_id));
        self.set_agent_b(id, self.get_agent_b(id) - self.get_agent_b(removed_id));
    }

    #[rustfmt::skip]
    #[inline(always)]
    pub  unsafe fn remove_ratio(&mut self, id: usize, removed_id: usize, mut ratio: f32) {
        profile!("remove_ratio");

        if ratio <= 0.0 {
            self.unregister_by_id(removed_id);

            return;
        }

        ratio = ratio.min(1.0);

        self.set_oxygen(removed_id, quantize(self.get_oxygen(id) * ratio));
        self.set_nitrogen(removed_id, quantize(self.get_nitrogen(id) * ratio));
        self.set_carbon_dioxide(removed_id, quantize(self.get_carbon_dioxide(id) * ratio));
        self.set_toxins(removed_id, quantize(self.get_toxins(id) * ratio));
        self.set_sleeping_agent(removed_id, quantize(self.get_sleeping_agent(id) * ratio));
        self.set_agent_b(removed_id, quantize(self.get_agent_b(id) * ratio));
        self.set_temperature(removed_id, self.get_temperature(id));

        self.set_oxygen(id, self.get_oxygen(id) - self.get_oxygen(removed_id));
        self.set_nitrogen(id, self.get_oxygen(id) - self.get_nitrogen(removed_id));
        self.set_carbon_dioxide(id, self.get_oxygen(id) - self.get_carbon_dioxide(removed_id));
        self.set_toxins(id, self.get_oxygen(id) - self.get_toxins(removed_id));
        self.set_sleeping_agent(id, self.get_oxygen(id) - self.get_sleeping_agent(removed_id));
        self.set_agent_b(id, self.get_oxygen(id) - self.get_agent_b(removed_id));
    }

    #[inline(always)]
    pub unsafe fn copy_from(&mut self, id: usize, sample_id: usize) {
        profile!("copy_from");

        self.set_oxygen(id, self.get_oxygen(sample_id));
        self.set_carbon_dioxide(id, self.get_carbon_dioxide(sample_id));
        self.set_nitrogen(id, self.get_nitrogen(sample_id));
        self.set_toxins(id, self.get_toxins(sample_id));
        self.set_sleeping_agent(id, self.get_sleeping_agent(sample_id));
        self.set_agent_b(id, self.get_agent_b(sample_id));
        self.set_temperature(id, self.get_temperature(sample_id));
    }

    // TODO: Make a method looks much more minimalistic.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub unsafe fn copy_from_turf(
        &mut self,
        id: usize,
        turf_model: Turf,
        initial_model_temperature: f32,
        initial_model_parent_temperature: f32,
    ) {
        profile!("copy_from_turf");

        self.set_oxygen(id, turf_model.oxygen);
        self.set_carbon_dioxide(id, turf_model.carbon_dioxide);
        self.set_nitrogen(id, turf_model.nitrogen);
        self.set_toxins(id, turf_model.toxins);
        self.set_sleeping_agent(id, turf_model.sleeping_agent);
        self.set_agent_b(id, turf_model.agent_b);

        if turf_model.temperature != initial_model_temperature
            || turf_model.temperature != initial_model_parent_temperature
        {
            self.set_temperature(id, turf_model.temperature);
        }
    }

    // TODO: Make a method looks much more minimalistic.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub unsafe fn check_turf(
        &self,
        id: usize,
        turf_model: Turf,
        atmos_adjacent_turfs: f32,
    ) -> bool {
        profile!("check_turf");

        let delta_oxygen =
            (self.get_oxygen_archived(id) - turf_model.oxygen) / (atmos_adjacent_turfs + 1.0);
        let delta_carbon_dioxide = (self.get_carbon_dioxide_archived(id)
            - turf_model.carbon_dioxide)
            / (atmos_adjacent_turfs + 1.0);
        let delta_nitrogen =
            (self.get_nitrogen_archived(id) - turf_model.nitrogen) / (atmos_adjacent_turfs + 1.0);
        let delta_toxins =
            (self.get_toxins_archived(id) - turf_model.toxins) / (atmos_adjacent_turfs + 1.0);
        let delta_sleeping_agent = (self.get_sleeping_agent_archived(id)
            - turf_model.sleeping_agent)
            / (atmos_adjacent_turfs + 1.0);
        let delta_agent_b =
            (self.get_agent_b_archived(id) - turf_model.agent_b) / (atmos_adjacent_turfs + 1.0);
        let delta_temperature = self.get_temperature_archived(id) - turf_model.temperature;

        // FIXME: Potentially, can be minimized and etc., anyway, this is 💀
        if ((delta_oxygen.abs() > MINIMUM_AIR_TO_SUSPEND)
            && (delta_oxygen.abs() >= self.get_oxygen_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_carbon_dioxide.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_carbon_dioxide.abs()
                    >= self.get_carbon_dioxide_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_nitrogen.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_nitrogen.abs()
                    >= self.get_nitrogen_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_toxins.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_toxins.abs()
                    >= self.get_toxins_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_sleeping_agent.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_sleeping_agent.abs()
                    >= self.get_sleeping_agent_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_agent_b.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_agent_b.abs()
                    >= self.get_agent_b_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND
        {
            return false;
        }

        true
    }

    // TODO: Make a method looks much more minimalistic.
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    pub unsafe fn check_turf_total(&self, id: usize, turf_model: Turf) -> bool {
        profile!("check_turf_total");

        let delta_oxygen = self.get_oxygen(id) - turf_model.oxygen;
        let delta_carbon_dioxide = self.get_carbon_dioxide(id) - turf_model.carbon_dioxide;
        let delta_nitrogen = self.get_nitrogen(id) - turf_model.nitrogen;
        let delta_toxins = self.get_toxins(id) - turf_model.toxins;
        let delta_sleeping_agent = self.get_sleeping_agent(id) - turf_model.sleeping_agent;
        let delta_agent_b = self.get_agent_b(id) - turf_model.agent_b;
        let delta_temperature = self.get_temperature(id) - turf_model.temperature;

        // FIXME: Potentially, can be minimized and etc., anyway, this is 💀
        if ((delta_oxygen.abs() > MINIMUM_AIR_TO_SUSPEND)
            && (delta_oxygen.abs() >= self.get_oxygen_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_carbon_dioxide.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_carbon_dioxide.abs()
                    >= self.get_carbon_dioxide_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_nitrogen.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_nitrogen.abs()
                    >= self.get_nitrogen_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_toxins.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_toxins.abs()
                    >= self.get_toxins_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_sleeping_agent.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_sleeping_agent.abs()
                    >= self.get_sleeping_agent_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || ((delta_agent_b.abs() > MINIMUM_AIR_TO_SUSPEND)
                && (delta_agent_b.abs()
                    >= self.get_agent_b_archived(id) * MINIMUM_AIR_RATIO_TO_SUSPEND))
            || delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND
        {
            return false;
        }

        true
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn share(&mut self, id: usize, sharer_id: usize, atmos_adjacent_turfs: f32) -> f32 {
        profile!("share");

        if !self.get_is_initialized(sharer_id) {
            return 0.0;
        }

        if self.get_oxygen_archived(id) == self.get_oxygen_archived(sharer_id)
            && self.get_carbon_dioxide_archived(id) == self.get_carbon_dioxide_archived(sharer_id)
            && self.get_nitrogen_archived(id) == self.get_nitrogen_archived(sharer_id)
            && self.get_toxins_archived(id) == self.get_toxins_archived(sharer_id)
            && self.get_sleeping_agent_archived(id) == self.get_sleeping_agent_archived(sharer_id)
            && self.get_agent_b_archived(id) == self.get_agent_b_archived(sharer_id)
            && self.get_temperature_archived(id) == self.get_temperature_archived(sharer_id)
        {
            return 0.0;
        }

        let delta_oxygen =
            quantize(self.get_oxygen_archived(id) - self.get_oxygen_archived(sharer_id))
                / (atmos_adjacent_turfs + 1.0);
        let delta_carbon_dioxide = quantize(
            self.get_carbon_dioxide_archived(id) - self.get_carbon_dioxide_archived(sharer_id),
        ) / (atmos_adjacent_turfs + 1.0);
        let delta_nitrogen =
            quantize(self.get_nitrogen_archived(id) - self.get_nitrogen_archived(sharer_id))
                / (atmos_adjacent_turfs + 1.0);
        let delta_toxins =
            quantize(self.get_toxins_archived(id) - self.get_toxins_archived(sharer_id))
                / (atmos_adjacent_turfs + 1.0);
        let delta_sleeping_agent = quantize(
            self.get_sleeping_agent_archived(id) - self.get_sleeping_agent_archived(sharer_id),
        ) / (atmos_adjacent_turfs + 1.0);
        let delta_agent_b =
            quantize(self.get_agent_b_archived(id) - self.get_agent_b_archived(sharer_id))
                / (atmos_adjacent_turfs + 1.0);
        let delta_temperature =
            self.get_temperature_archived(id) - self.get_temperature_archived(sharer_id);

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

            old_self_heat_capacity = self.get_heat_capacity(id);
            old_sharer_heat_capacity = self.get_heat_capacity(sharer_id);
        }

        self.set_oxygen(id, self.get_oxygen(id) - delta_oxygen);
        self.set_oxygen(sharer_id, self.get_oxygen(sharer_id) + delta_oxygen);

        self.set_carbon_dioxide(id, self.get_carbon_dioxide(id) - delta_carbon_dioxide);
        self.set_carbon_dioxide(
            sharer_id,
            self.get_carbon_dioxide(sharer_id) + delta_carbon_dioxide,
        );

        self.set_nitrogen(id, self.get_nitrogen(id) - delta_nitrogen);
        self.set_nitrogen(sharer_id, self.get_nitrogen(sharer_id) + delta_nitrogen);

        self.set_toxins(id, self.get_toxins(id) - delta_toxins);
        self.set_toxins(sharer_id, self.get_toxins(sharer_id) + delta_toxins);

        self.set_sleeping_agent(id, self.get_sleeping_agent(id) - delta_sleeping_agent);
        self.set_sleeping_agent(
            sharer_id,
            self.get_sleeping_agent(sharer_id) + delta_sleeping_agent,
        );

        self.set_agent_b(id, self.get_agent_b(id) - delta_agent_b);
        self.set_agent_b(sharer_id, self.get_agent_b(sharer_id) + delta_agent_b);

        let moved_moles = delta_oxygen
            + delta_carbon_dioxide
            + delta_nitrogen
            + delta_toxins
            + delta_sleeping_agent
            + delta_agent_b;
        self.set_last_share(
            id,
            delta_oxygen.abs()
                + delta_carbon_dioxide.abs()
                + delta_nitrogen.abs()
                + delta_toxins.abs()
                + delta_sleeping_agent.abs()
                + delta_agent_b.abs(),
        );

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let new_self_heat_capacity = old_self_heat_capacity + heat_capacity_sharer_to_self
                - heat_capacity_self_to_sharer;
            let new_sharer_heat_capacity = old_sharer_heat_capacity + heat_capacity_self_to_sharer
                - heat_capacity_sharer_to_self;

            if new_self_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.set_temperature(
                    id,
                    (old_self_heat_capacity * self.get_temperature(id)
                        - heat_capacity_self_to_sharer * self.get_temperature_archived(id)
                        + heat_capacity_sharer_to_self * self.get_temperature_archived(sharer_id))
                        / new_self_heat_capacity,
                );
            }

            if new_sharer_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.set_temperature(
                    sharer_id,
                    (old_sharer_heat_capacity * self.get_temperature(sharer_id)
                        - heat_capacity_sharer_to_self * self.get_temperature_archived(sharer_id)
                        + heat_capacity_self_to_sharer * self.get_temperature_archived(id))
                        / new_sharer_heat_capacity,
                );
            }

            // <10% change in sharer heat capacity
            if old_sharer_heat_capacity.abs() > MINIMUM_HEAT_CAPACITY
                && (new_sharer_heat_capacity / old_sharer_heat_capacity - 1.0).abs() < 0.10
                && (new_sharer_heat_capacity / old_sharer_heat_capacity - 1.0).abs() < 0.10
            {
                self.temperature_share(id, sharer_id, OPEN_HEAT_TRANSFER_COEFFICIENT);
            }
        }

        if (delta_temperature > MINIMUM_TEMPERATURE_TO_MOVE)
            || moved_moles.abs() > MINIMUM_MOLES_DELTA_TO_MOVE
        {
            let delta_pressure = self.get_temperature_archived(id)
                * (self.get_total_moles(id) + moved_moles)
                - self.get_temperature_archived(sharer_id)
                    * (self.get_total_moles(sharer_id) - moved_moles);
            delta_pressure * R_IDEAL_GAS_EQUATION / self.get_volume(id)
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub unsafe fn temperature_share(
        &mut self,
        id: usize,
        sharer_id: usize,
        conduction_coefficient: f32,
    ) {
        profile!("temperature_share");

        let delta_temperature =
            self.get_temperature_archived(id) - self.get_temperature_archived(sharer_id);

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity_archived(id);
            let sharer_heat_capacity = self.get_heat_capacity_archived(sharer_id);

            if sharer_heat_capacity > MINIMUM_HEAT_CAPACITY
                && (self_heat_capacity > MINIMUM_HEAT_CAPACITY)
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * sharer_heat_capacity
                        / (self_heat_capacity + sharer_heat_capacity));

                self.set_temperature(id, self.get_temperature(id) - heat / self_heat_capacity);
                self.set_temperature(
                    sharer_id,
                    self.get_temperature(sharer_id) + heat / sharer_heat_capacity,
                );
            }
        }
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn mimic(
        &mut self,
        id: usize,
        turf_model: Turf,
        model_thermal_conductivity: f32,
        model_heat_capacity: f32,
        atmos_adjacent_turfs: f32,
    ) -> f32 {
        profile!("mimic");

        let delta_oxygen = quantize(self.get_oxygen_archived(id) - turf_model.oxygen)
            / (atmos_adjacent_turfs + 1.0);
        let delta_carbon_dioxide =
            quantize(self.get_carbon_dioxide_archived(id) - turf_model.carbon_dioxide)
                / (atmos_adjacent_turfs + 1.0);
        let delta_nitrogen = quantize(self.get_nitrogen_archived(id) - turf_model.nitrogen)
            / (atmos_adjacent_turfs + 1.0);
        let delta_toxins = quantize(self.get_toxins_archived(id) - turf_model.toxins)
            / (atmos_adjacent_turfs + 1.0);
        let delta_sleeping_agent =
            quantize(self.get_sleeping_agent_archived(id) - turf_model.sleeping_agent)
                / (atmos_adjacent_turfs + 1.0);
        let delta_agent_b = quantize(self.get_agent_b_archived(id) - turf_model.agent_b)
            / (atmos_adjacent_turfs + 1.0);
        let delta_temperature = self.get_temperature_archived(id) - turf_model.temperature;

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

            old_self_heat_capacity = self.get_heat_capacity(id);
        }

        self.set_oxygen(id, self.get_oxygen(id) - delta_oxygen);
        self.set_carbon_dioxide(id, self.get_oxygen(id) - delta_carbon_dioxide);
        self.set_nitrogen(id, self.get_nitrogen(id) - delta_nitrogen);
        self.set_toxins(id, self.get_toxins(id) - delta_toxins);
        self.set_sleeping_agent(id, self.get_sleeping_agent(id) - delta_sleeping_agent);
        self.set_agent_b(id, self.get_agent_b(id) - delta_agent_b);

        let moved_moles = delta_oxygen
            + delta_carbon_dioxide
            + delta_nitrogen
            + delta_toxins
            + delta_sleeping_agent
            + delta_agent_b;
        self.set_last_share(
            id,
            delta_oxygen.abs()
                + delta_carbon_dioxide.abs()
                + delta_nitrogen.abs()
                + delta_toxins.abs()
                + delta_sleeping_agent.abs()
                + delta_agent_b.abs(),
        );

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let new_self_heat_capacity = old_self_heat_capacity - heat_capacity_transferred;
            if new_self_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.set_temperature(
                    id,
                    (old_self_heat_capacity * self.get_temperature(id)
                        - heat_capacity_transferred * self.get_temperature_archived(id))
                        / new_self_heat_capacity,
                );
            }

            self.temperature_mimic(
                id,
                turf_model.temperature,
                model_heat_capacity,
                model_thermal_conductivity,
            );
        }

        if (delta_temperature > MINIMUM_TEMPERATURE_TO_MOVE)
            || moved_moles.abs() > MINIMUM_MOLES_DELTA_TO_MOVE
        {
            let delta_pressure = self.get_temperature_archived(id)
                * (self.get_total_moles(id) + moved_moles)
                - turf_model.temperature
                    * (turf_model.oxygen
                        + turf_model.carbon_dioxide
                        + turf_model.nitrogen
                        + turf_model.toxins
                        + turf_model.sleeping_agent
                        + turf_model.agent_b);
            delta_pressure * R_IDEAL_GAS_EQUATION / self.get_volume(id)
        } else {
            0.0
        }
    }

    #[inline(always)]
    pub unsafe fn temperature_mimic(
        &mut self,
        id: usize,
        model_temperature: f32,
        model_heat_capacity: f32,
        conduction_coefficient: f32,
    ) {
        profile!("temperature_mimic");

        let delta_temperature = self.get_temperature(id) - model_temperature;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity(id);

            if (model_heat_capacity > MINIMUM_HEAT_CAPACITY)
                && (self_heat_capacity > MINIMUM_HEAT_CAPACITY)
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * model_heat_capacity
                        / (self_heat_capacity + model_heat_capacity));

                self.set_temperature(id, self.get_temperature(id) - heat / self_heat_capacity);
            }
        }
    }

    #[inline(always)]
    pub unsafe fn temperature_turf_share(
        &mut self,
        id: usize,
        turf_sharer: &Value,
        conduction_coefficient: f32,
    ) {
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

        let delta_temperature = self.get_temperature_archived(id) - turf_sharer_temperature;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity(id);

            if (turf_sharer_heat_capacity > MINIMUM_HEAT_CAPACITY)
                && (self_heat_capacity > MINIMUM_HEAT_CAPACITY)
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * turf_sharer_heat_capacity
                        / (self_heat_capacity + turf_sharer_heat_capacity));

                self.set_temperature(id, self.get_temperature(id) - heat / self_heat_capacity);
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
    pub unsafe fn compare(&self, id: usize, sample_id: usize) -> bool {
        profile!("compare");

        let oxygen = self.get_oxygen(id);
        let sample_oxygen = self.get_oxygen(sample_id);
        if ((oxygen - self.get_oxygen(sample_id)).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((oxygen < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_oxygen)
                || (oxygen > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_oxygen))
        {
            return false;
        }

        let nitrogen = self.get_nitrogen(id);
        let sample_nitrogen = self.get_nitrogen(sample_id);
        if ((nitrogen - sample_nitrogen).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((nitrogen < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_nitrogen)
                || (nitrogen > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_nitrogen))
        {
            return false;
        }

        let carbon_dioxide = self.get_carbon_dioxide(id);
        let sample_carbon_dioxide = self.get_carbon_dioxide(sample_id);
        if ((carbon_dioxide - sample_carbon_dioxide).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((carbon_dioxide < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_carbon_dioxide)
                || (carbon_dioxide > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_carbon_dioxide))
        {
            return false;
        }

        let toxins = self.get_toxins(id);
        let sample_toxins = self.get_toxins(sample_id);
        if ((toxins - sample_toxins).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((toxins < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_toxins)
                || (toxins > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_toxins))
        {
            return false;
        }

        let sleeping_agent = self.get_sleeping_agent(id);
        let sample_sleeping_agent = self.get_sleeping_agent(sample_id);
        if ((sleeping_agent - sample_sleeping_agent).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((sleeping_agent < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_sleeping_agent)
                || (sleeping_agent > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_sleeping_agent))
        {
            return false;
        }

        let agent_b = self.get_agent_b(id);
        let sample_agent_b = self.get_agent_b(sample_id);
        if ((agent_b - sample_agent_b).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((agent_b < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_agent_b)
                || (agent_b > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_agent_b))
        {
            return false;
        }

        let temperature = self.get_temperature(id);
        let sample_temperature = self.get_temperature(sample_id);
        if self.get_total_moles(id) > MINIMUM_AIR_TO_SUSPEND
            && ((temperature - sample_temperature).abs() > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND)
            && ((temperature < (1.0 - MINIMUM_TEMPERATURE_RATIO_TO_SUSPEND) * sample_temperature)
                || (temperature
                    > (1.0 + MINIMUM_TEMPERATURE_RATIO_TO_SUSPEND) * sample_temperature))
        {
            return false;
        }

        true
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_breath_partial_pressure(&self, id: usize, gas_pressure: f32) -> f32 {
        profile!("get_breath_partial_pressure");

        (gas_pressure * R_IDEAL_GAS_EQUATION * self.get_temperature(id)) / BREATH_VOLUME
    }

    //Reverse of the above
    #[inline(always)]
    #[must_use]
    pub unsafe fn get_true_breath_pressure(&self, id: usize, breath_pp: f32) -> f32 {
        profile!("get_true_breath_pressure");

        (breath_pp * BREATH_VOLUME) / (R_IDEAL_GAS_EQUATION * self.get_temperature(id))
    }
}
