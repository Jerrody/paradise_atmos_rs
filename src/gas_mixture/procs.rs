use auxtools::*;

use super::{helpers, GasMixture as GS};

#[hook("/datum/gas_mixture/proc/heat_capacity")]
pub fn heat_capacity(self_gas_mixture: Value) {
    let oxygen = GS::get_oxygen(self_gas_mixture);
    let carbon_dioxide = GS::get_oxygen(self_gas_mixture);
    let nitrogen = GS::get_oxygen(self_gas_mixture);
    let toxins = GS::get_oxygen(self_gas_mixture);
    let sleeping_agent = GS::get_oxygen(self_gas_mixture);
    let agent_b = GS::get_oxygen(self_gas_mixture);

    Ok(Value::from(helpers::heat_capacity_calculation(
        oxygen,
        carbon_dioxide,
        nitrogen,
        toxins,
        sleeping_agent,
        agent_b,
    )))
}

#[hook("/datum/gas_mixture/proc/heat_capacity_archived")]
pub fn heat_capacity_archived(self_gas_mixture: Value) {
    let oxygen_archived = GS::get_oxygen_archived(self_gas_mixture);
    let carbon_dioxide_archived = GS::get_oxygen(self_gas_mixture);
    let nitrogen_archived = GS::get_nitrogen_archived(self_gas_mixture);
    let toxins_archived = GS::get_toxins_archived(self_gas_mixture);
    let sleeping_agent_archived = GS::get_sleeping_agent_archived(self_gas_mixture);
    let agent_b_archived = GS::get_agent_b_archived(self_gas_mixture);

    Ok(Value::from(helpers::heat_capacity_calculation(
        oxygen_archived,
        carbon_dioxide_archived,
        nitrogen_archived,
        toxins_archived,
        sleeping_agent_archived,
        agent_b_archived,
    )))
}

#[hook("/datum/gas_mixture/proc/total_moles")]
pub fn total_moles(self_gas_mixture: Value) {
    let moles = GS::get_oxygen(self_gas_mixture)
        + GS::get_carbon_dioxide(self_gas_mixture)
        + GS::get_nitrogen(self_gas_mixture)
        + GS::get_toxins(self_gas_mixture)
        + GS::get_sleeping_agent(self_gas_mixture)
        + GS::get_agent_b(self_gas_mixture);

    Ok(Value::from(moles))
}

// /datum/gas_mixture/proc/heat_capacity()
// 	return HEAT_CAPACITY_CALCULATION(oxygen, carbon_dioxide, nitrogen, toxins, sleeping_agent, agent_b)

// /datum/gas_mixture/proc/heat_capacity_archived()
// 	return HEAT_CAPACITY_CALCULATION(oxygen_archived, carbon_dioxide_archived, nitrogen_archived, toxins_archived, sleeping_agent_archived, agent_b_archived)

// /datum/gas_mixture/proc/total_moles()
// 	var/moles = oxygen + carbon_dioxide + nitrogen + toxins + sleeping_agent + agent_b
// 	return moles

// /datum/gas_mixture/proc/total_trace_moles()
// 	var/moles = sleeping_agent + agent_b
// 	return moles

// /datum/gas_mixture/proc/return_pressure()
// 	if(volume > 0)
// 		return total_moles() * R_IDEAL_GAS_EQUATION * temperature / volume
// 	return 0

// /datum/gas_mixture/proc/return_temperature()
// 	return temperature

// /datum/gas_mixture/proc/return_volume()
// 	return max(0, volume)

// /datum/gas_mixture/proc/thermal_energy()
// 	return temperature * heat_capacity()
