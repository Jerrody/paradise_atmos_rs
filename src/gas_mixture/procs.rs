use super::Mixture;
use crate::{constants::*, turf::*, utils::*};
use crate::profile;
use byondapi::value::ByondValue;

impl Mixture {
    #[must_use]
    #[inline(always)]
    pub unsafe fn heat_capacity(&self, id: usize) -> f32 {
        crate::utils::calculate_heat_capacity(
            self.get_oxygen(id),
            self.get_carbon_dioxide(id),
            self.get_nitrogen(id),
            self.get_toxins(id),
            self.get_sleeping_agent(id),
            self.get_agent_b(id),
        )
    }

    #[must_use]
    #[inline(always)]
    unsafe fn get_heat_capacity_archived(&self, id: usize) -> f32 {
        crate::utils::calculate_heat_capacity(
            self.get_oxygen_archived(id),
            self.get_carbon_dioxide_archived(id),
            self.get_nitrogen_archived(id),
            self.get_toxins_archived(id),
            self.get_sleeping_agent_archived(id),
            self.get_agent_b_archived(id),
        )
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn total_moles(&self, id: usize) -> f32 {
        self.get_oxygen(id)
            + self.get_carbon_dioxide(id)
            + self.get_nitrogen(id)
            + self.get_toxins(id)
            + self.get_sleeping_agent(id)
            + self.get_agent_b(id)
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn get_total_trace_moles(&self, id: usize) -> f32 {
        self.get_sleeping_agent(id) + self.get_agent_b(id)
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn return_pressure(&self, id: usize) -> f32 {
        let volume = self.get_volume(id);
        if volume > Default::default() {
            return self.total_moles(id) * R_IDEAL_GAS_EQUATION * self.get_temperature(id) / volume;
        }

        Default::default()
    }

    // I'm not sure that this thing was made by a person with good mental health in DM.
    // Anyway, it could cause, potentially, unexpected behavior.
    #[must_use]
    #[inline(always)]
    pub unsafe fn return_volume(&self, id: usize) -> f32 {
        self.get_volume(id).max(Default::default())
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn thermal_energy(&self, id: usize) -> f32 {
        self.get_temperature(id) * self.heat_capacity(id)
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn react(&mut self, id: usize) -> bool {
        profile!("react");

        let mut reacting = Default::default(); //set to 1 if a notable reaction occured (used by pipe_network)

        let temperature = self.get_temperature(id);
        let agent_b = self.get_agent_b(id);
        let toxins = self.get_toxins(id);
        let carbon_dioxide = self.get_carbon_dioxide(id);

        if agent_b != 0.0
            && temperature > 900.0
            && toxins > MINIMUM_HEAT_CAPACITY
            && carbon_dioxide > MINIMUM_HEAT_CAPACITY
        {
            let gases = [carbon_dioxide * 0.75, toxins * 0.25, agent_b * 0.05];

            let reaction_rate = gases
                .into_iter()
                .min_by(|a, b| a.total_cmp(b))
                .unwrap_unchecked();

            self.sub_carbon_dioxide(id, reaction_rate);
            self.add_oxygen(id, reaction_rate);
            self.sub_agent_b(id, reaction_rate * 0.05);
            self.add_temperature(id, reaction_rate * 20_000.0 / self.heat_capacity(id));

            reacting = true;
        }

        self.set_fuel_burnt(id, 0.0);
        if self.get_temperature(id) > FIRE_MINIMUM_TEMPERATURE_TO_EXIST && self.fire(id) > 0.0 {
            reacting = true;
        }

        reacting
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    unsafe fn fire(&mut self, id: usize) -> f32 {
        profile!("fire");

        let mut energy_released = 0.0;
        let old_heat_capacity = self.heat_capacity(id);

        // Handle plasma burning
        if self.get_toxins(id) > MINIMUM_HEAT_CAPACITY {
            let plasma_burn_rate;
            let oxygen_burn_rate;
            let temperature = self.get_temperature(id);

            let temperature_scale = if temperature > PLASMA_UPPER_TEMPERATURE {
                1.0
            } else {
                (temperature - PLASMA_MINIMUM_BURN_TEMPERATURE)
                    / (PLASMA_UPPER_TEMPERATURE - PLASMA_MINIMUM_BURN_TEMPERATURE)
            };

            if temperature_scale > Default::default() {
                oxygen_burn_rate = OXYGEN_BURN_RATE_BASE - temperature_scale;

                let toxins = self.get_toxins(id);
                let oxygen = self.get_oxygen(id);

                if oxygen > toxins * PLASMA_OXYGEN_FULLBURN {
                    plasma_burn_rate = (toxins * temperature_scale) / PLASMA_BURN_RATE_DELTA;
                } else {
                    plasma_burn_rate = (temperature_scale * (oxygen / PLASMA_OXYGEN_FULLBURN))
                        / PLASMA_BURN_RATE_DELTA;
                }

                if plasma_burn_rate > MINIMUM_HEAT_CAPACITY {
                    self.sub_toxins(id, plasma_burn_rate);
                    self.sub_oxygen(id, plasma_burn_rate * oxygen_burn_rate);
                    self.add_carbon_dioxide(id, plasma_burn_rate);
                    self.add_fuel_burnt(id, plasma_burn_rate * (oxygen_burn_rate + 1.0));

                    energy_released += FIRE_PLASMA_ENERGY_RELEASED * plasma_burn_rate;
                }
            }
        }

        if energy_released > Default::default() {
            let new_heat_capacity = self.heat_capacity(id);

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
        self.set_oxygen_archived(id, self.get_oxygen(id));
        self.set_carbon_dioxide_archived(id, self.get_carbon_dioxide(id));
        self.set_nitrogen_archived(id, self.get_nitrogen(id));
        self.set_toxins_archived(id, self.get_toxins(id));
        self.set_sleeping_agent_archived(id, self.get_sleeping_agent(id));
        self.set_agent_b_archived(id, self.get_agent_b(id));
        self.set_temperature_archived(id, self.get_temperature(id));
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn merge(&mut self, id: usize, giver_id: usize) -> bool {
        profile!("merge");

        if !self.get_is_initialized(giver_id) {
            return Default::default();
        }

        if (self.get_temperature(id) - self.get_temperature(giver_id)).abs()
            > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER
        {
            let self_heat_capacity = self.heat_capacity(id);
            let giver_heat_capacity = self.heat_capacity(giver_id);
            let combined_heat_capacity = self_heat_capacity + giver_heat_capacity;
            if combined_heat_capacity != 0.0 {
                self.set_temperature(
                    id,
                    (self.get_temperature(giver_id) * giver_heat_capacity
                        + self.get_temperature(id) * self_heat_capacity)
                        / combined_heat_capacity,
                );
            }
        }

        self.add_oxygen(id, self.get_oxygen(giver_id));
        self.add_carbon_dioxide(id, self.get_carbon_dioxide(giver_id));
        self.add_nitrogen(id, self.get_nitrogen(giver_id));
        self.add_toxins(id, self.get_toxins(giver_id));
        self.add_sleeping_agent(id, self.get_sleeping_agent(giver_id));
        self.add_agent_b(id, self.get_agent_b(giver_id));

        true
    }

    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn remove(&mut self, id: usize, removed_id: usize, mut amount: f32) {
        profile!("remove");

        let sum = self.total_moles(id);
        amount = amount.min(sum); // Can not take more air than tile has!

        if amount <= Default::default() {
            self.unregister(removed_id);

            return;
        }

        let removed_oxygen_quantized = quantize(self.get_oxygen(id) / sum * amount);
        let removed_nitrogen_quantized = quantize(self.get_nitrogen(id) / sum * amount);
        let removed_carbon_dioxide_quantized = quantize(self.get_carbon_dioxide(id) / sum * amount);
        let removed_toxins_quantized = quantize(self.get_toxins(id) / sum * amount);
        let removed_sleeping_agent_quantized = quantize(self.get_sleeping_agent(id) / sum * amount);
        let removed_agent_b_quantized = quantize(self.get_agent_b(id) / sum * amount);

        self.set_oxygen(removed_id, removed_oxygen_quantized);
        self.set_nitrogen(removed_id, removed_nitrogen_quantized);
        self.set_carbon_dioxide(removed_id, removed_carbon_dioxide_quantized);
        self.set_toxins(removed_id, removed_toxins_quantized);
        self.set_sleeping_agent(removed_id, removed_sleeping_agent_quantized);
        self.set_agent_b(removed_id, removed_agent_b_quantized);
        self.set_temperature(removed_id, self.get_temperature(id));

        self.sub_oxygen(id, removed_oxygen_quantized);
        self.sub_nitrogen(id, removed_nitrogen_quantized);
        self.sub_carbon_dioxide(id, removed_carbon_dioxide_quantized);
        self.sub_toxins(id, removed_toxins_quantized);
        self.sub_sleeping_agent(id, removed_sleeping_agent_quantized);
        self.sub_agent_b(id, removed_agent_b_quantized);
    }

    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn remove_ratio(&mut self, id: usize, removed_id: usize, mut ratio: f32) {
        profile!("remove_ratio");

        if ratio <= 0.0 {
            self.unregister(removed_id);

            return;
        }

        ratio = ratio.min(1.0);

        let removed_oxygen_quantized = quantize(self.get_oxygen(id) * ratio);
        let removed_nitrogen_quantized = quantize(self.get_nitrogen(id) * ratio);
        let removed_carbon_dioxide_quantized = quantize(self.get_carbon_dioxide(id) * ratio);
        let removed_toxins_quantized = quantize(self.get_toxins(id) * ratio);
        let removed_sleeping_agent_quantized = quantize(self.get_sleeping_agent(id) * ratio);
        let removed_agent_b_quantized = quantize(self.get_agent_b(id) * ratio);

        self.set_oxygen(removed_id, removed_oxygen_quantized);
        self.set_nitrogen(removed_id, removed_nitrogen_quantized);
        self.set_carbon_dioxide(removed_id, removed_carbon_dioxide_quantized);
        self.set_toxins(removed_id, removed_toxins_quantized);
        self.set_sleeping_agent(removed_id, removed_sleeping_agent_quantized);
        self.set_agent_b(removed_id, removed_agent_b_quantized);
        self.set_temperature(removed_id, self.get_temperature(id));

        self.sub_oxygen(id, removed_oxygen_quantized);
        self.sub_nitrogen(id, removed_nitrogen_quantized);
        self.sub_carbon_dioxide(id, removed_carbon_dioxide_quantized);
        self.sub_toxins(id, removed_toxins_quantized);
        self.sub_sleeping_agent(id, removed_sleeping_agent_quantized);
        self.sub_agent_b(id, removed_agent_b_quantized);
    }

    #[inline(always)]
    pub unsafe fn copy_from(&mut self, id: usize, sample_id: usize) {
        self.set_oxygen(id, self.get_oxygen(sample_id));
        self.set_carbon_dioxide(id, self.get_carbon_dioxide(sample_id));
        self.set_nitrogen(id, self.get_nitrogen(sample_id));
        self.set_toxins(id, self.get_toxins(sample_id));
        self.set_sleeping_agent(id, self.get_sleeping_agent(sample_id));
        self.set_agent_b(id, self.get_agent_b(sample_id));
        self.set_temperature(id, self.get_temperature(sample_id));
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn check_turf(
        &self,
        id: usize,
        turf_model: Turf,
        mut atmos_adjacent_turfs: f32,
    ) -> bool {
        profile!("check_turf");

        let oxygen_archived = self.get_oxygen_archived(id);
        let carbon_dioxide_archived = self.get_carbon_dioxide_archived(id);
        let nitrogen_archived = self.get_nitrogen_archived(id);
        let toxins_archived = self.get_toxins_archived(id);
        let sleeping_agent_archived = self.get_sleeping_agent_archived(id);
        let agent_b_archived = self.get_agent_b_archived(id);

        atmos_adjacent_turfs += 1.0;
        let delta_oxygen = (oxygen_archived - turf_model.oxygen) / atmos_adjacent_turfs;
        let delta_carbon_dioxide =
            (carbon_dioxide_archived - turf_model.carbon_dioxide) / atmos_adjacent_turfs;
        let delta_nitrogen = (nitrogen_archived - turf_model.nitrogen) / atmos_adjacent_turfs;
        let delta_toxins = (toxins_archived - turf_model.toxins) / atmos_adjacent_turfs;
        let delta_sleeping_agent =
            (sleeping_agent_archived - turf_model.sleeping_agent) / atmos_adjacent_turfs;
        let delta_agent_b = (agent_b_archived - turf_model.agent_b) / atmos_adjacent_turfs;
        let delta_temperature = self.get_temperature_archived(id) - turf_model.temperature;

        if Self::check_turf_condition(delta_oxygen.abs(), oxygen_archived)
            || Self::check_turf_condition(delta_carbon_dioxide.abs(), carbon_dioxide_archived)
            || Self::check_turf_condition(delta_nitrogen.abs(), nitrogen_archived)
            || Self::check_turf_condition(delta_toxins.abs(), toxins_archived)
            || Self::check_turf_condition(delta_sleeping_agent.abs(), sleeping_agent_archived)
            || Self::check_turf_condition(delta_agent_b.abs(), agent_b_archived)
            || delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND
        {
            return false;
        }

        true
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn check_turf_total(&self, id: usize, turf_model: Turf) -> bool {
        profile!("check_turf_total");

        let oxygen = self.get_oxygen(id);
        let carbon_dioxide = self.get_carbon_dioxide(id);
        let nitrogen = self.get_nitrogen(id);
        let toxins = self.get_toxins(id);
        let sleeping_agent = self.get_sleeping_agent(id);
        let agent_b = self.get_agent_b(id);

        let delta_oxygen = oxygen - turf_model.oxygen;
        let delta_carbon_dioxide = carbon_dioxide - turf_model.carbon_dioxide;
        let delta_nitrogen = nitrogen - turf_model.nitrogen;
        let delta_toxins = toxins - turf_model.toxins;
        let delta_sleeping_agent = sleeping_agent - turf_model.sleeping_agent;
        let delta_agent_b = agent_b - turf_model.agent_b;
        let delta_temperature = self.get_temperature(id) - turf_model.temperature;

        if Self::check_turf_condition(delta_oxygen.abs(), oxygen)
            || Self::check_turf_condition(delta_carbon_dioxide.abs(), carbon_dioxide)
            || Self::check_turf_condition(delta_nitrogen.abs(), nitrogen)
            || Self::check_turf_condition(delta_toxins.abs(), toxins)
            || Self::check_turf_condition(delta_sleeping_agent.abs(), sleeping_agent)
            || Self::check_turf_condition(delta_agent_b.abs(), agent_b)
            || delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND
        {
            return false;
        }

        true
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn share(
        &mut self,
        id: usize,
        sharer_id: usize,
        mut atmos_adjacent_turfs: f32,
    ) -> f32 {
        profile!("share");

        if !self.get_is_initialized(sharer_id) {
            return Default::default();
        }

        let oxygen_archived = self.get_oxygen_archived(id);
        let carbon_dioxide_archived = self.get_carbon_dioxide_archived(id);
        let nitrogen_archived = self.get_nitrogen_archived(id);
        let toxins_archived = self.get_toxins_archived(id);
        let sleeping_agent_archived = self.get_sleeping_agent_archived(id);
        let agent_b_archived = self.get_agent_b_archived(id);
        let temperature_archived = self.get_temperature_archived(id);

        let sharer_oxygen_archived = self.get_oxygen_archived(sharer_id);
        let sharer_carbon_dioxide_archived = self.get_carbon_dioxide_archived(sharer_id);
        let sharer_nitrogen_archived = self.get_nitrogen_archived(sharer_id);
        let sharer_toxins_archived = self.get_toxins_archived(sharer_id);
        let sharer_sleeping_agent_archived = self.get_sleeping_agent_archived(sharer_id);
        let sharer_agent_b_archived = self.get_agent_b_archived(sharer_id);
        let sharer_temperature_archived = self.get_temperature_archived(sharer_id);

        if oxygen_archived == sharer_oxygen_archived
            && carbon_dioxide_archived == sharer_carbon_dioxide_archived
            && nitrogen_archived == sharer_nitrogen_archived
            && toxins_archived == sharer_toxins_archived
            && sleeping_agent_archived == sharer_sleeping_agent_archived
            && agent_b_archived == sharer_agent_b_archived
            && temperature_archived == sharer_temperature_archived
        {
            return Default::default();
        }

        atmos_adjacent_turfs += 1.0;
        let delta_oxygen =
            quantize(oxygen_archived - sharer_oxygen_archived) / atmos_adjacent_turfs;
        let delta_carbon_dioxide =
            quantize(carbon_dioxide_archived - sharer_carbon_dioxide_archived)
                / atmos_adjacent_turfs;
        let delta_nitrogen =
            quantize(nitrogen_archived - sharer_nitrogen_archived) / atmos_adjacent_turfs;
        let delta_toxins =
            quantize(toxins_archived - sharer_toxins_archived) / atmos_adjacent_turfs;
        let delta_sleeping_agent =
            quantize(sleeping_agent_archived - sharer_sleeping_agent_archived)
                / atmos_adjacent_turfs;
        let delta_agent_b =
            quantize(agent_b_archived - sharer_agent_b_archived) / atmos_adjacent_turfs;
        let delta_temperature = temperature_archived - sharer_temperature_archived;

        let mut old_self_heat_capacity = 0.0;
        let mut old_sharer_heat_capacity = 0.0;

        let mut heat_capacity_self_to_sharer = 0.0;
        let mut heat_capacity_sharer_to_self = 0.0;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let delta_air = delta_oxygen + delta_nitrogen;
            if delta_air != Default::default() {
                let air_heat_capacity = SPECIFIC_HEAT_AIR * delta_air;
                if delta_air > Default::default() {
                    heat_capacity_self_to_sharer += air_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= air_heat_capacity;
                }
            }

            if delta_carbon_dioxide != Default::default() {
                let carbon_dioxide_heat_capacity = SPECIFIC_HEAT_CDO * delta_carbon_dioxide;
                if delta_carbon_dioxide > Default::default() {
                    heat_capacity_self_to_sharer += carbon_dioxide_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= carbon_dioxide_heat_capacity;
                }
            }

            if delta_toxins != Default::default() {
                let toxins_heat_capacity = SPECIFIC_HEAT_TOXIN * delta_toxins;
                if delta_toxins > Default::default() {
                    heat_capacity_self_to_sharer += toxins_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= toxins_heat_capacity;
                }
            }

            if delta_sleeping_agent != Default::default() {
                let sleeping_agent_heat_capacity = SPECIFIC_HEAT_N2O * delta_sleeping_agent;
                if delta_sleeping_agent > Default::default() {
                    heat_capacity_self_to_sharer += sleeping_agent_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= sleeping_agent_heat_capacity;
                }
            }

            if delta_agent_b != Default::default() {
                let agent_b_heat_capacity = SPECIFIC_HEAT_AGENT_B * delta_agent_b;
                if delta_agent_b > Default::default() {
                    heat_capacity_self_to_sharer += agent_b_heat_capacity;
                } else {
                    heat_capacity_sharer_to_self -= agent_b_heat_capacity;
                }
            }

            old_self_heat_capacity = self.heat_capacity(id);
            old_sharer_heat_capacity = self.heat_capacity(sharer_id);
        }

        self.sub_oxygen(id, delta_oxygen);
        self.add_oxygen(sharer_id, delta_oxygen);

        self.sub_carbon_dioxide(id, delta_carbon_dioxide);
        self.add_carbon_dioxide(sharer_id, delta_carbon_dioxide);

        self.sub_nitrogen(id, delta_nitrogen);
        self.add_nitrogen(sharer_id, delta_nitrogen);

        self.sub_toxins(id, delta_toxins);
        self.add_toxins(sharer_id, delta_toxins);

        self.sub_sleeping_agent(id, delta_sleeping_agent);
        self.add_sleeping_agent(sharer_id, delta_sleeping_agent);

        self.sub_agent_b(id, delta_agent_b);
        self.add_agent_b(sharer_id, delta_agent_b);

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
                        - heat_capacity_self_to_sharer * temperature_archived
                        + heat_capacity_sharer_to_self * sharer_temperature_archived)
                        / new_self_heat_capacity,
                );
            }

            if new_sharer_heat_capacity > MINIMUM_HEAT_CAPACITY {
                self.set_temperature(
                    sharer_id,
                    (old_sharer_heat_capacity * self.get_temperature(sharer_id)
                        - heat_capacity_sharer_to_self * sharer_temperature_archived
                        + heat_capacity_self_to_sharer * temperature_archived)
                        / new_sharer_heat_capacity,
                );

                // <10% change in sharer heat capacity
                if old_sharer_heat_capacity.abs() > MINIMUM_HEAT_CAPACITY
                    && (new_sharer_heat_capacity / old_sharer_heat_capacity - 1.0).abs() < 0.10
                {
                    self.temperature_share(
                        id,
                        sharer_id,
                        temperature_archived,
                        sharer_temperature_archived,
                        OPEN_HEAT_TRANSFER_COEFFICIENT,
                    );
                }
            }
        }

        if delta_temperature > MINIMUM_TEMPERATURE_TO_MOVE
            || moved_moles.abs() > MINIMUM_MOLES_DELTA_TO_MOVE
        {
            let delta_pressure = temperature_archived * (self.total_moles(id) + moved_moles)
                - sharer_temperature_archived * (self.total_moles(sharer_id) - moved_moles);

            delta_pressure * R_IDEAL_GAS_EQUATION / self.get_volume(id)
        } else {
            Default::default()
        }
    }

    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn temperature_share(
        &mut self,
        id: usize,
        sharer_id: usize,
        temperature_archived: f32,
        sharer_temperature_archived: f32,
        conduction_coefficient: f32,
    ) {
        profile!("temperature_share");

        let delta_temperature = temperature_archived - sharer_temperature_archived;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.get_heat_capacity_archived(id);
            let sharer_heat_capacity = self.get_heat_capacity_archived(sharer_id);

            if sharer_heat_capacity > MINIMUM_HEAT_CAPACITY
                && self_heat_capacity > MINIMUM_HEAT_CAPACITY
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * sharer_heat_capacity
                        / (self_heat_capacity + sharer_heat_capacity));

                self.sub_temperature(id, heat / self_heat_capacity);
                self.add_temperature(sharer_id, heat / sharer_heat_capacity);
            }
        }
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn mimic(
        &mut self,
        id: usize,
        turf_model: Turf,
        model_thermal_conductivity: f32,
        model_heat_capacity: f32,
        mut atmos_adjacent_turfs: f32,
    ) -> f32 {
        profile!("mimic");

        let temperature_archived = self.get_temperature_archived(id);

        atmos_adjacent_turfs += 1.0;
        let delta_oxygen =
            quantize(self.get_oxygen_archived(id) - turf_model.oxygen) / atmos_adjacent_turfs;
        let delta_carbon_dioxide =
            quantize(self.get_carbon_dioxide_archived(id) - turf_model.carbon_dioxide)
                / atmos_adjacent_turfs;
        let delta_nitrogen =
            quantize(self.get_nitrogen_archived(id) - turf_model.nitrogen) / atmos_adjacent_turfs;
        let delta_toxins =
            quantize(self.get_toxins_archived(id) - turf_model.toxins) / atmos_adjacent_turfs;
        let delta_sleeping_agent =
            quantize(self.get_sleeping_agent_archived(id) - turf_model.sleeping_agent)
                / atmos_adjacent_turfs;
        let delta_agent_b =
            quantize(self.get_agent_b_archived(id) - turf_model.agent_b) / atmos_adjacent_turfs;
        let delta_temperature = temperature_archived - turf_model.temperature;

        let mut old_self_heat_capacity = Default::default();
        let mut heat_capacity_transferred: f32 = Default::default();

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let delta_air = delta_oxygen + delta_nitrogen;
            if delta_air != Default::default() {
                let air_heat_capacity = SPECIFIC_HEAT_AIR * delta_air;
                heat_capacity_transferred -= air_heat_capacity;
            }

            if delta_carbon_dioxide != Default::default() {
                let carbon_dioxide_heat_capacity = SPECIFIC_HEAT_CDO * delta_carbon_dioxide;
                heat_capacity_transferred -= carbon_dioxide_heat_capacity;
            }

            if delta_toxins != Default::default() {
                let toxins_heat_capacity = SPECIFIC_HEAT_TOXIN * delta_toxins;
                heat_capacity_transferred -= toxins_heat_capacity;
            }

            if delta_sleeping_agent != Default::default() {
                let sleeping_agent_heat_capacity = SPECIFIC_HEAT_N2O * delta_sleeping_agent;
                heat_capacity_transferred -= sleeping_agent_heat_capacity;
            }

            if delta_agent_b != Default::default() {
                let agent_b_heat_capacity = SPECIFIC_HEAT_AGENT_B * delta_agent_b;
                heat_capacity_transferred -= agent_b_heat_capacity;
            }

            old_self_heat_capacity = self.heat_capacity(id);
        }

        self.sub_oxygen(id, delta_oxygen);
        self.sub_carbon_dioxide(id, delta_carbon_dioxide);
        self.sub_nitrogen(id, delta_nitrogen);
        self.sub_toxins(id, delta_toxins);
        self.sub_sleeping_agent(id, delta_sleeping_agent);
        self.sub_agent_b(id, delta_agent_b);

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
                        - heat_capacity_transferred * temperature_archived)
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
            let delta_pressure = temperature_archived * (self.total_moles(id) + moved_moles)
                - turf_model.temperature
                    * (turf_model.oxygen
                        + turf_model.carbon_dioxide
                        + turf_model.nitrogen
                        + turf_model.toxins
                        + turf_model.sleeping_agent
                        + turf_model.agent_b);

            delta_pressure * R_IDEAL_GAS_EQUATION / self.get_volume(id)
        } else {
            Default::default()
        }
    }

    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn temperature_mimic(
        &mut self,
        id: usize,
        model_temperature: f32,
        model_heat_capacity: f32,
        conduction_coefficient: f32,
    ) {
        profile!("temperature_mimic");

        let temperature = self.get_temperature(id);
        let delta_temperature = temperature - model_temperature;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let self_heat_capacity = self.heat_capacity(id);

            if model_heat_capacity > MINIMUM_HEAT_CAPACITY
                && self_heat_capacity > MINIMUM_HEAT_CAPACITY
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * model_heat_capacity
                        / (self_heat_capacity + model_heat_capacity));

                self.sub_temperature(id, heat / self_heat_capacity);
            }
        }
    }

    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn temperature_turf_share(
        &mut self,
        id: usize,
        turf_sharer: &mut ByondValue,
        conduction_coefficient: f32,
    ) {
        profile!("temperature_turf_share");

        const TEMPERATURE_NAME: &str = "temperature";

        // TODO: Make the setters and getters methods for the turfs.
        let turf_sharer_temperature = turf_sharer
            .read_var(TEMPERATURE_NAME)
            .unwrap_unchecked()
            .get_number()
            .unwrap_unchecked();

        let delta_temperature = self.get_temperature_archived(id) - turf_sharer_temperature;

        if delta_temperature.abs() > MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER {
            let turf_sharer_heat_capacity = turf_sharer
                .read_var("heat_capacity")
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked();
            let self_heat_capacity = self.heat_capacity(id);

            if (turf_sharer_heat_capacity > MINIMUM_HEAT_CAPACITY)
                && (self_heat_capacity > MINIMUM_HEAT_CAPACITY)
            {
                let heat = conduction_coefficient
                    * delta_temperature
                    * (self_heat_capacity * turf_sharer_heat_capacity
                        / (self_heat_capacity + turf_sharer_heat_capacity));

                self.sub_temperature(id, heat / self_heat_capacity);
                turf_sharer
                    .write_var(
                        TEMPERATURE_NAME,
                        &ByondValue::from(
                            turf_sharer_temperature + heat / turf_sharer_heat_capacity,
                        ),
                    )
                    .unwrap_unchecked();
            }
        }
    }

    #[must_use]
    #[cfg_attr(feature = "profile", inline(never))]
    #[cfg_attr(not(feature = "profile"), inline(always))]
    pub unsafe fn compare(&self, id: usize, sample_id: usize) -> bool {
        profile!("compare");

        let oxygen = self.get_oxygen(id);
        let sample_oxygen = self.get_oxygen(sample_id);
        if Self::compare_condition(oxygen, sample_oxygen) {
            return Default::default();
        }

        let nitrogen = self.get_nitrogen(id);
        let sample_nitrogen = self.get_nitrogen(sample_id);
        if Self::compare_condition(nitrogen, sample_nitrogen) {
            return Default::default();
        }

        let carbon_dioxide = self.get_carbon_dioxide(id);
        let sample_carbon_dioxide = self.get_carbon_dioxide(sample_id);
        if Self::compare_condition(carbon_dioxide, sample_carbon_dioxide) {
            return Default::default();
        }

        let toxins = self.get_toxins(id);
        let sample_toxins = self.get_toxins(sample_id);
        if Self::compare_condition(toxins, sample_toxins) {
            return Default::default();
        }

        let sleeping_agent = self.get_sleeping_agent(id);
        let sample_sleeping_agent = self.get_sleeping_agent(sample_id);
        if Self::compare_condition(sleeping_agent, sample_sleeping_agent) {
            return Default::default();
        }

        let agent_b = self.get_agent_b(id);
        let sample_agent_b = self.get_agent_b(sample_id);
        if Self::compare_condition(agent_b, sample_agent_b) {
            return Default::default();
        }

        let temperature = self.get_temperature(id);
        let sample_temperature = self.get_temperature(sample_id);
        if self.total_moles(id) > MINIMUM_AIR_TO_SUSPEND
            && Self::compare_condition(temperature, sample_temperature)
        {
            return Default::default();
        }

        true
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn get_breath_partial_pressure(&self, id: usize, gas_pressure: f32) -> f32 {
        (gas_pressure * R_IDEAL_GAS_EQUATION * self.get_temperature(id)) / BREATH_VOLUME
    }

    // Reverse of the above
    #[must_use]
    #[inline(always)]
    pub unsafe fn get_true_breath_pressure(&self, id: usize, breath_pp: f32) -> f32 {
        (breath_pp * BREATH_VOLUME) / (R_IDEAL_GAS_EQUATION * self.get_temperature(id))
    }
}
