use auxtools::*;

use super::GasMixture as GS;

#[hook("/datum/gas_mixture/proc/heat_capacity")]
pub fn heat_capacity() {
    Ok(Value::from(GS::get_heat_capacity(src)))
}

#[hook("/datum/gas_mixture/proc/heat_capacity_archived")]
pub fn heat_capacity_archived() {
    Ok(Value::from(GS::get_heat_capacity(src)))
}

#[hook("/datum/gas_mixture/proc/total_moles")]
pub fn total_moles() {
    Ok(Value::from(GS::get_total_moles(src)))
}

#[hook("/datum/gas_mixture/proc/total_trace_moles")]
pub fn total_trace_moles() {
    Ok(Value::from(
        GS::get_sleeping_agent(src) + GS::get_agent_b(src),
    ))
}

#[hook("/datum/gas_mixture/proc/return_pressure")]
pub fn return_pressure() {
    if GS::get_volume(src) > 0.0 {
        return Ok(Value::from(
            GS::get_total_moles(src)
                * crate::gas_mixture::R_IDEAL_GAS_EQUATION
                * GS::get_temperature(src)
                / GS::get_volume(src),
        ));
    }

    Ok(Value::from(f32::default()))
}

#[hook("/datum/gas_mixture/proc/return_volume")]
pub fn return_volume() {
    Ok(Value::from(f32::default().max(GS::get_volume(src))))
}

#[hook("/datum/gas_mixture/proc/thermal_energy")]
pub fn thermal_energy() {
    Ok(Value::from(
        GS::get_temperature(src) * GS::get_heat_capacity(src),
    ))
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
