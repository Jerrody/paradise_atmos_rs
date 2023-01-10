use auxtools::*;

use super::constants::{fire::*, heat::*};
use super::mixtures::mixtures_native::MixtureNative as GSN;

#[hook("/datum/gas_mixture/proc/heat_capacity")]
pub fn heat_capacity() {
    Ok(Value::from(GSN::get_heat_capacity(src)))
}

#[hook("/datum/gas_mixture/proc/heat_capacity_archived")]
pub fn heat_capacity_archived() {
    Ok(Value::from(GSN::get_heat_capacity(src)))
}

#[hook("/datum/gas_mixture/proc/total_moles")]
pub fn total_moles() {
    Ok(Value::from(GSN::get_total_moles(src)))
}

#[hook("/datum/gas_mixture/proc/total_trace_moles")]
pub fn total_trace_moles() {
    Ok(Value::from(
        GSN::get_sleeping_agent(src) + GSN::get_agent_b(src),
    ))
}

#[hook("/datum/gas_mixture/proc/return_pressure")]
pub fn return_pressure() {
    if GSN::get_volume(src) > 0.0 {
        return Ok(Value::from(
            GSN::get_total_moles(src)
                * crate::gas_mixture::constants::R_IDEAL_GAS_EQUATION
                * GSN::get_temperature(src)
                / GSN::get_volume(src),
        ));
    }

    Ok(Value::from(f32::default()))
}

#[hook("/datum/gas_mixture/proc/return_volume")]
pub fn return_volume() {
    Ok(Value::from(f32::default().max(GSN::get_volume(src))))
}

#[hook("/datum/gas_mixture/proc/thermal_energy")]
pub fn thermal_energy() {
    Ok(Value::from(
        GSN::get_temperature(src) * GSN::get_heat_capacity(src),
    ))
}

#[hook("/datum/gas_mixture/proc/react")]
pub fn react() {
    let mut reacting: f32 = 0.0; //set to 1 if a notable reaction occured (used by pipe_network)

    let mut agent_b = GSN::get_agent_b(src);
    let mut temperature = GSN::get_temperature(src);
    let toxins = GSN::get_toxins(src);
    let mut carbon_dioxide = GSN::get_carbon_dioxide(src);
    let mut oxygen = GSN::get_oxygen(src);

    if temperature > 900.0
        && toxins > MINIMUM_HEAT_CAPACITY
        && carbon_dioxide > MINIMUM_HEAT_CAPACITY
    {
        let gases = [carbon_dioxide * 0.75, toxins * 0.25, agent_b * 0.05];
        let reaction_rate = gases
            .into_iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();

        carbon_dioxide -= reaction_rate;
        oxygen += reaction_rate;
        agent_b -= reaction_rate;
        temperature += (reaction_rate * 20_000.0) / GSN::get_heat_capacity(src);

        reacting = 1.0;
    }

    GSN::set_fuel_burnt(src, 0.0);
    if temperature > FIRE_MINIMUM_TEMPERATURE_TO_EXIST {
        let fire = unsafe {
            src.call("fire", Default::default())
                .unwrap_unchecked()
                .as_number()
                .unwrap_unchecked()
        };

        if fire > 0.0 {
            reacting = 1.0;
        }
    }

    GSN::set_carbon_dioxide(src, carbon_dioxide);
    GSN::set_oxygen(src, oxygen);
    GSN::set_agent_b(src, agent_b);
    GSN::set_temperature(src, temperature);

    Ok(Value::from(reacting))
}

#[hook("/datum/gas_mixture/proc/fire")]
pub fn fire() {
    Ok(Value::null())
}

// /datum/gas_mixture/proc/react()
// 	var/reacting = 0 //set to 1 if a notable reaction occured (used by pipe_network)

// 	if(agent_b && temperature > 900)
// 		if(toxins > MINIMUM_HEAT_CAPACITY && carbon_dioxide > MINIMUM_HEAT_CAPACITY)
// 			var/reaction_rate = min(carbon_dioxide * 0.75, toxins * 0.25, agent_b * 0.05)

// 			carbon_dioxide -= reaction_rate
// 			oxygen += reaction_rate

// 			agent_b -= reaction_rate * 0.05

// 			temperature += (reaction_rate * 20000) / heat_capacity()

// 			reacting = 1

// 	fuel_burnt = 0
// 	if(temperature > FIRE_MINIMUM_TEMPERATURE_TO_EXIST)
// 		if(fire() > 0)
// 			reacting = 1

// 	return reacting
