use auxtools::*;

use crate::gas_mixture::MIXTURES;
use crate::turf::Turf;
use crate::{id, null, profile_proc, value};

const DEFAULT_ATMOS_ADJACENT_TURFS: f32 = 4.0;

#[cfg(any(feature = "profile", feature = "profile_proc"))]
#[hook("/proc/enable_tracy")]
pub fn enable_tracy() {
    use tracing::Level;
    use tracing_subscriber::layer::SubscriberExt;

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::default().add_directive(Level::DEBUG.into()))
            .with(tracing_tracy::TracyLayer::new()),
    )
    .expect("set up the subscriber");

    null!()
}

#[hook("/datum/gas_mixture/proc/register")]
pub fn register() {
    profile_proc!("register");

    unsafe { MIXTURES.register(src) };

    null!()
}

#[hook("/datum/gas_mixture/proc/unregister")]
pub fn unregister() {
    profile_proc!("unregister");

    unsafe { MIXTURES.unregister(src) };

    null!()
}

#[hook("/datum/gas_mixture/proc/get_oxygen")]
pub fn get_oxygen() {
    profile_proc!("get_oxygen");

    value!(unsafe { MIXTURES.get_oxygen(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_carbon_dioxide")]
pub fn get_carbon_dioxide() {
    profile_proc!("get_carbon_dioxide");

    value!(unsafe { MIXTURES.get_carbon_dioxide(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_nitrogen")]
pub fn get_nitrogen() {
    profile_proc!("get_nitrogen");

    value!(unsafe { MIXTURES.get_nitrogen(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_toxins")]
pub fn get_toxins() {
    profile_proc!("get_toxins");

    value!(unsafe { MIXTURES.get_toxins(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_sleeping_agent")]
pub fn get_sleeping_agent() {
    profile_proc!("get_sleeping_agent");

    value!(unsafe { MIXTURES.get_sleeping_agent(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_agent_b")]
pub fn get_agent_b() {
    profile_proc!("get_agent_b");

    value!(unsafe { MIXTURES.get_agent_b(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_volume")]
pub fn get_volume() {
    profile_proc!("get_volume");

    value!(unsafe { MIXTURES.get_volume(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_temperature")]
pub fn get_temperature() {
    profile_proc!("get_temperature");

    value!(unsafe { MIXTURES.get_temperature(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/get_last_share")]
pub fn get_last_share() {
    profile_proc!("get_last_share");

    value!(unsafe { MIXTURES.get_last_share(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/set_oxygen")]
pub fn set_oxygen(value: &Value) {
    profile_proc!("set_oxygen");

    unsafe { MIXTURES.set_oxygen(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_carbon_dioxide")]
pub fn set_carbon_dioxide(value: &Value) {
    profile_proc!("set_carbon_dioxide");

    unsafe { MIXTURES.set_carbon_dioxide(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_nitrogen")]
pub fn set_nitrogen(value: &Value) {
    profile_proc!("set_nitrogen");

    unsafe { MIXTURES.set_nitrogen(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_toxins")]
pub fn set_toxins(value: &Value) {
    profile_proc!("set_toxins");

    unsafe { MIXTURES.set_toxins(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_sleeping_agent")]
pub fn set_sleeping_agent(value: &Value) {
    profile_proc!("set_sleeping_agent");

    unsafe { MIXTURES.set_sleeping_agent(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_agent_b")]
pub fn set_agent_b(value: &Value) {
    profile_proc!("set_agent_b");

    unsafe { MIXTURES.set_agent_b(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_volume")]
pub fn set_volume(value: &Value) {
    profile_proc!("set_volume");

    unsafe { MIXTURES.set_volume(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_temperature")]
pub fn set_temperature(value: &Value) {
    profile_proc!("set_temperature");

    unsafe { MIXTURES.set_temperature(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_last_share")]
pub fn set_last_share(value: &Value) {
    profile_proc!("set_last_share");

    unsafe { MIXTURES.set_last_share(id!(src), value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/heat_capacity")]
pub fn get_heat_capacity() {
    profile_proc!("get_heat_capacity");

    value!(unsafe { MIXTURES.get_heat_capacity(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/total_moles")]
pub fn get_total_moles() {
    profile_proc!("get_total_moles");

    value!(unsafe { MIXTURES.get_total_moles(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/total_trace_moles")]
pub fn get_total_trace_moles() {
    profile_proc!("get_total_trace_moles");

    value!(unsafe { MIXTURES.get_total_trace_moles(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/return_pressure")]
pub fn get_pressure() {
    profile_proc!("get_pressure");

    value!(unsafe { MIXTURES.get_pressure(id!(src)) })
}

// I'm not sure that this thing was made by a person with good mental health in DM.
// Anyway, it could cause, potentially, unexpected behavior.
#[hook("/datum/gas_mixture/proc/return_volume")]
pub fn return_volume() {
    profile_proc!("return_volume");

    value!(unsafe { MIXTURES.return_volume(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/thermal_energy")]
pub fn get_thermal_energy() {
    profile_proc!("get_thermal_energy");

    value!(unsafe { MIXTURES.get_thermal_energy(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/react")]
pub fn react() {
    profile_proc!("react");

    value!(unsafe { MIXTURES.react(id!(src)) })
}

#[hook("/datum/gas_mixture/proc/archive")]
pub fn archive() {
    profile_proc!("archive");

    unsafe { MIXTURES.archive(id!(src)) }

    null!()
}

#[hook("/datum/gas_mixture/proc/merge")]
pub fn merge(giver: &Value) {
    profile_proc!("merge");

    unsafe { MIXTURES.merge(id!(src), id!(giver)) }

    null!()
}

#[hook("/datum/gas_mixture/proc/remove")]
pub fn remove(removed: &Value, amount: &Value) {
    profile_proc!("remove");

    unsafe {
        MIXTURES.remove(
            id!(src),
            id!(removed),
            amount.as_number().unwrap_unchecked(),
        );
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/remove_ratio")]
pub fn remove_ratio(removed: u32, mut ratio: &Value) {
    profile_proc!("remove_ratio");

    unsafe {
        MIXTURES.remove_ratio(id!(src), id!(removed), ratio.as_number().unwrap_unchecked());
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/copy_from")]
pub fn copy_from(sample: &Value) {
    profile_proc!("copy_from");

    unsafe {
        MIXTURES.copy_from(id!(src), id!(sample));
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/copy_from_turf")]
pub fn copy_from_turf(
    turf_model: &Value,
    initial_model_temperature: &Value,
    initial_model_parent_temperature: &Value,
) {
    profile_proc!("copy_from_turf");

    let turf_model = unsafe { Turf::new(turf_model) };
    let initial_model_temperature =
        unsafe { initial_model_temperature.as_number().unwrap_unchecked() };
    let initial_model_parent_temperature = unsafe {
        initial_model_parent_temperature
            .as_number()
            .unwrap_unchecked()
    };

    unsafe {
        MIXTURES.copy_from_turf(
            id!(src),
            turf_model,
            initial_model_temperature,
            initial_model_parent_temperature,
        )
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/check_turf")]
pub fn check_turf(turf_model: &Value, atmos_adjacent_turfs: &Value) {
    profile_proc!("check_turf");

    let turf_model = unsafe { Turf::new(turf_model) };
    let atmos_adjacent_turfs = atmos_adjacent_turfs
        .as_number()
        .unwrap_or(DEFAULT_ATMOS_ADJACENT_TURFS);

    value!(unsafe {
        MIXTURES.check_turf(src.raw.data.id as usize, turf_model, atmos_adjacent_turfs)
    })
}

#[hook("/datum/gas_mixture/proc/check_turf_total")]
pub fn check_turf_total(turf_model: &Value) {
    profile_proc!("check_turf_total");

    let turf_model = unsafe { Turf::new(turf_model) };

    value!(unsafe { MIXTURES.check_turf_total(id!(src), turf_model) })
}

#[hook("/datum/gas_mixture/proc/share")]
pub fn share(sharer: &Value, atmos_adjacent_turfs: &Value) {
    profile_proc!("share");

    let atmos_adjacent_turfs = atmos_adjacent_turfs
        .as_number()
        .unwrap_or(DEFAULT_ATMOS_ADJACENT_TURFS);

    value!(unsafe { MIXTURES.share(id!(src), id!(sharer), atmos_adjacent_turfs) })
}

#[hook("/datum/gas_mixture/proc/temperature_share")]
pub fn temperature_share(sharer: &Value, conduction_coefficient: &Value) {
    profile_proc!("temperature_share");

    let conduction_coefficient = unsafe { conduction_coefficient.as_number().unwrap_unchecked() };

    unsafe {
        MIXTURES.temperature_share(id!(src), id!(sharer), conduction_coefficient);
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/mimic")]
pub fn mimic(
    turf_model: &Value,
    model_thermal_conductivity: &Value,
    model_heat_capacity: &Value,
    atmos_adjacent_turfs: &Value,
) {
    profile_proc!("mimic");

    let turf_model = unsafe { Turf::new(turf_model) };
    let model_thermal_conductivity =
        unsafe { model_thermal_conductivity.as_number().unwrap_unchecked() };
    let model_heat_capacity = unsafe { model_heat_capacity.as_number().unwrap_unchecked() };
    let atmos_adjacent_turfs = atmos_adjacent_turfs
        .as_number()
        .unwrap_or(DEFAULT_ATMOS_ADJACENT_TURFS);

    value!(unsafe {
        MIXTURES.mimic(
            id!(src),
            turf_model,
            model_thermal_conductivity,
            model_heat_capacity,
            atmos_adjacent_turfs,
        )
    })
}

#[hook("/datum/gas_mixture/proc/temperature_mimic")]
pub fn temperature_mimic(
    model_temperature: &Value,
    model_heat_capacity: &Value,
    conduction_coefficient: &Value,
) {
    profile_proc!("temperature_mimic");

    let model_temperature = unsafe { model_temperature.as_number().unwrap_unchecked() };
    let model_heat_capacity = unsafe { model_heat_capacity.as_number().unwrap_unchecked() };
    let conduction_coefficient = unsafe { conduction_coefficient.as_number().unwrap_unchecked() };

    unsafe {
        MIXTURES.temperature_mimic(
            id!(src),
            model_temperature,
            model_heat_capacity,
            conduction_coefficient,
        );
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/temperature_turf_share")]
pub fn temperature_turf_share(turf_sharer: &Value, conduction_coefficient: &Value) {
    profile_proc!("temperature_turf_share");

    let conduction_coefficient = unsafe { conduction_coefficient.as_number().unwrap_unchecked() };

    unsafe {
        MIXTURES.temperature_turf_share(id!(src), turf_sharer, conduction_coefficient);
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/compare")]
pub fn compare(sample: &Value) {
    profile_proc!("compare");

    value!(unsafe { MIXTURES.compare(id!(src), id!(sample)) })
}

#[hook("/datum/gas_mixture/proc/get_breath_partial_pressure")]
pub fn get_breath_partial_pressure(gas_pressure: &Value) {
    profile_proc!("get_breath_partial_pressure");

    value!(unsafe {
        MIXTURES.get_breath_partial_pressure(id!(src), gas_pressure.as_number().unwrap_unchecked())
    })
}

//Reverse of the above
#[hook("/datum/gas_mixture/proc/get_true_breath_pressure")]
pub fn get_true_breath_pressure(breath_pp: &Value) {
    profile_proc!("get_true_breath_pressure");

    value!(unsafe {
        MIXTURES.get_true_breath_pressure(id!(src), breath_pp.as_number().unwrap_unchecked())
    })
}
