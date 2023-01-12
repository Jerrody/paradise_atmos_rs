use super::constants::*;
use super::mixtures::{Mixture, MixtureNative};
use crate::value_with;
use auxtools::*;

#[hook("/datum/gas_mixture/proc/heat_capacity")]
pub fn heat_capacity() {
    value_with!(MixtureNative::get_heat_capacity(src))
}

#[hook("/datum/gas_mixture/proc/heat_capacity_archived")]
pub fn heat_capacity_archived() {
    value_with!(MixtureNative::get_heat_capacity(src))
}

#[hook("/datum/gas_mixture/proc/total_moles")]
pub fn total_moles() {
    value_with!(MixtureNative::get_total_moles(src))
}

#[hook("/datum/gas_mixture/proc/return_pressure")]
pub fn return_pressure() {
    if MixtureNative::get_volume(src) > 0.0 {
        return Ok(Value::from(
            MixtureNative::get_total_moles(src)
                * crate::gas_mixture::constants::R_IDEAL_GAS_EQUATION
                * MixtureNative::get_temperature(src)
                / MixtureNative::get_volume(src),
        ));
    }

    Ok(Value::from(f32::default()))
}

#[hook("/datum/gas_mixture/proc/return_volume")]
pub fn return_volume() {
    Ok(Value::from(
        f32::default().max(MixtureNative::get_volume(src)),
    ))
}

#[hook("/datum/gas_mixture/proc/thermal_energy")]
pub fn thermal_energy() {
    value_with!(MixtureNative::get_temperature(src) * MixtureNative::get_heat_capacity(src))
}

#[hook("/datum/gas_mixture/proc/react")]
pub fn react() {
    let mut reacting = false; //set to 1 if a notable reaction occured (used by pipe_network)

    let mut mixture = Mixture::new(src);

    if mixture.temperature > 900.0
        && mixture.toxins > MINIMUM_HEAT_CAPACITY
        && mixture.carbon_dioxide > MINIMUM_HEAT_CAPACITY
    {
        let gases = [
            mixture.carbon_dioxide * 0.75,
            mixture.toxins * 0.25,
            mixture.agent_b * 0.05,
        ];
        let reaction_rate = gases
            .into_iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();

        mixture.carbon_dioxide -= reaction_rate;
        mixture.oxygen += reaction_rate;
        mixture.agent_b -= reaction_rate * 0.05;
        mixture.temperature += (reaction_rate * 20_000.0) / mixture.heat_capacity();

        reacting = true;
    }

    MixtureNative::set_fuel_burnt(src, 0.0);
    if mixture.temperature <= FIRE_MINIMUM_TEMPERATURE_TO_EXIST {
        MixtureNative::set_carbon_dioxide(src, mixture.carbon_dioxide);
        MixtureNative::set_oxygen(src, mixture.oxygen);
        MixtureNative::set_agent_b(src, mixture.agent_b);
        MixtureNative::set_temperature(src, mixture.temperature);

        return value_with!(Value::from(reacting));
    }

    let fuel_burnt = mixture.fire();
    if fuel_burnt > 0.0 {
        reacting = true;
    }

    MixtureNative::set_fuel_burnt(src, fuel_burnt);
    MixtureNative::set_carbon_dioxide(src, mixture.carbon_dioxide);
    MixtureNative::set_oxygen(src, mixture.oxygen);
    MixtureNative::set_agent_b(src, mixture.agent_b);
    MixtureNative::set_temperature(src, mixture.temperature);
    MixtureNative::set_toxins(src, mixture.toxins);

    value_with!(reacting)
}
