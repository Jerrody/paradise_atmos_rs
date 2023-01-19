use auxtools::*;

use crate::{null, profile_proc, value};

use super::{turf::Turf, Mixture};

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

    unsafe { Mixture::register(src) };

    null!()
}

#[hook("/datum/gas_mixture/proc/unregister")]
pub fn unregister() {
    profile_proc!("unregister");

    unsafe { Mixture::unregister(src) };

    null!()
}

#[hook("/datum/gas_mixture/proc/get_oxygen")]
pub fn get_oxygen() {
    profile_proc!("get_oxygen");

    value!(unsafe { Mixture::get_oxygen(src) })
}

#[hook("/datum/gas_mixture/proc/get_carbon_dioxide")]
pub fn get_carbon_dioxide() {
    profile_proc!("get_carbon_dioxide");

    value!(unsafe { Mixture::get_carbon_dioxide(src) })
}

#[hook("/datum/gas_mixture/proc/get_nitrogen")]
pub fn get_nitrogen() {
    profile_proc!("get_nitrogen");

    value!(unsafe { Mixture::get_nitrogen(src) })
}

#[hook("/datum/gas_mixture/proc/get_toxins")]
pub fn get_toxins() {
    profile_proc!("get_toxins");

    value!(unsafe { Mixture::get_toxins(src) })
}

#[hook("/datum/gas_mixture/proc/get_sleeping_agent")]
pub fn get_sleeping_agent() {
    profile_proc!("get_sleeping_agent");

    value!(unsafe { Mixture::get_sleeping_agent(src) })
}

#[hook("/datum/gas_mixture/proc/get_agent_b")]
pub fn get_agent_b() {
    profile_proc!("get_agent_b");

    value!(unsafe { Mixture::get_agent_b(src) })
}

#[hook("/datum/gas_mixture/proc/get_volume")]
pub fn get_volume() {
    profile_proc!("get_volume");

    value!(unsafe { Mixture::get_volume(src) })
}

#[hook("/datum/gas_mixture/proc/get_temperature")]
pub fn get_temperature() {
    profile_proc!("get_temperature");

    value!(unsafe { Mixture::get_temperature(src) })
}

#[hook("/datum/gas_mixture/proc/get_last_share")]
pub fn get_last_share() {
    profile_proc!("get_last_share");

    value!(unsafe { Mixture::get_last_share(src) })
}

#[hook("/datum/gas_mixture/proc/set_oxygen")]
pub fn set_oxygen(value: &Value) {
    profile_proc!("set_oxygen");

    unsafe { Mixture::set_oxygen(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_carbon_dioxide")]
pub fn set_carbon_dioxide(value: &Value) {
    profile_proc!("set_carbon_dioxide");

    unsafe { Mixture::set_carbon_dioxide(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_nitrogen")]
pub fn set_nitrogen(value: &Value) {
    profile_proc!("set_nitrogen");

    unsafe { Mixture::set_nitrogen(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_toxins")]
pub fn set_toxins(value: &Value) {
    profile_proc!("set_toxins");

    unsafe { Mixture::set_toxins(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_sleeping_agent")]
pub fn set_sleeping_agent(value: &Value) {
    profile_proc!("set_sleeping_agent");

    unsafe { Mixture::set_sleeping_agent(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_agent_b")]
pub fn set_agent_b(value: &Value) {
    profile_proc!("set_agent_b");

    unsafe { Mixture::set_agent_b(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_volume")]
pub fn set_volume(value: &Value) {
    profile_proc!("set_volume");

    unsafe { Mixture::set_volume(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_temperature")]
pub fn set_temperature(value: &Value) {
    profile_proc!("set_temperature");

    unsafe { Mixture::set_temperature(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/set_last_share")]
pub fn set_last_share(value: &Value) {
    profile_proc!("set_last_share");

    unsafe { Mixture::set_last_share(src, value.as_number().unwrap_unchecked()) }

    null!()
}

#[hook("/datum/gas_mixture/proc/heat_capacity")]
pub fn get_heat_capacity() {
    profile_proc!("get_heat_capacity");

    let heat_capacity = unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .get_heat_capacity()
    };

    value!(heat_capacity)
}

#[hook("/datum/gas_mixture/proc/total_moles")]
pub fn get_total_moles() {
    profile_proc!("get_total_moles");

    let total_moles = unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .get_total_moles()
    };

    value!(total_moles)
}

#[hook("/datum/gas_mixture/proc/total_trace_moles")]
pub fn get_total_trace_moles() {
    profile_proc!("get_total_trace_moles");

    let total_trace_moles = unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .get_total_trace_moles()
    };

    value!(total_trace_moles)
}

#[hook("/datum/gas_mixture/proc/return_pressure")]
pub fn get_pressure() {
    profile_proc!("get_pressure");

    let pressure = unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .get_pressure()
    };

    value!(pressure)
}

// I'm not sure that this thing was made by a person with good mental health in DM.
// Anyway, it could cause, potentially, unexpected behavior.
#[hook("/datum/gas_mixture/proc/return_volume")]
pub fn return_volume() {
    profile_proc!("return_volume");

    let volume = unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .return_volume()
    };

    value!(volume)
}

#[hook("/datum/gas_mixture/proc/thermal_energy")]
pub fn get_thermal_energy() {
    profile_proc!("get_thermal_energy");

    let thermal_energy = unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .get_thermal_energy()
    };

    value!(thermal_energy)
}

#[hook("/datum/gas_mixture/proc/react")]
pub fn react() {
    profile_proc!("react");

    value!(unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .react()
    })
}

#[hook("/datum/gas_mixture/proc/archive")]
pub fn archive() {
    profile_proc!("archive");

    unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .archive()
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/merge")]
pub fn merge(giver: &Value) {
    profile_proc!("merge");

    unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .merge(Mixture::get_mixture_mut_by_src(giver))
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/remove")]
pub fn remove(removed: &Value, amount: &Value) {
    profile_proc!("remove");

    unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .remove(removed.raw.data.id, amount.as_number().unwrap_unchecked())
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/remove_ratio")]
pub fn remove_ratio(removed: u32, mut ratio: &Value) {
    profile_proc!("remove_ratio");

    unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .remove_ratio(removed.raw.data.id, ratio.as_number().unwrap_unchecked())
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/copy_from")]
pub fn copy_from(sample: &Value) {
    profile_proc!("copy_from");

    unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .copy_from(Mixture::get_mixture_by_src(sample).unwrap_unchecked())
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
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .copy_from_turf(
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
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .check_turf(turf_model, atmos_adjacent_turfs)
    })
}

#[hook("/datum/gas_mixture/proc/check_turf_total")]
pub fn check_turf_total(turf_model: &Value) {
    profile_proc!("check_turf_total");

    let turf_model = unsafe { Turf::new(turf_model) };

    value!(unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .check_turf_total(turf_model)
    })
}

#[hook("/datum/gas_mixture/proc/share")]
pub fn share(sharer: &Value, atmos_adjacent_turfs: &Value) {
    profile_proc!("share");

    let atmos_adjacent_turfs = atmos_adjacent_turfs
        .as_number()
        .unwrap_or(DEFAULT_ATMOS_ADJACENT_TURFS);

    value!(unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .share(
                Mixture::get_mixture_mut_by_src(sharer),
                atmos_adjacent_turfs,
            )
    })
}

#[hook("/datum/gas_mixture/proc/temperature_share")]
pub fn temperature_share(sharer: &Value, conduction_coefficient: &Value) {
    profile_proc!("temperature_share");

    let conduction_coefficient = unsafe { conduction_coefficient.as_number().unwrap_unchecked() };

    unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .temperature_share(
                &mut Mixture::get_mixture_mut_by_src(sharer).unwrap_unchecked(),
                conduction_coefficient,
            )
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
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .mimic(
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
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .temperature_mimic(
                model_temperature,
                model_heat_capacity,
                conduction_coefficient,
            )
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/temperature_turf_share")]
pub fn temperature_turf_share(turf_sharer: &Value, conduction_coefficient: &Value) {
    profile_proc!("temperature_turf_share");

    let conduction_coefficient = unsafe { conduction_coefficient.as_number().unwrap_unchecked() };

    unsafe {
        Mixture::get_mixture_mut_by_src(src)
            .unwrap_unchecked()
            .temperature_turf_share(turf_sharer, conduction_coefficient)
    }

    null!()
}

#[hook("/datum/gas_mixture/proc/compare")]
pub fn compare(sample: &Value) {
    profile_proc!("compare");

    value!(unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .compare(Mixture::get_mixture_by_src(sample).unwrap_unchecked())
    })
}

#[hook("/datum/gas_mixture/proc/get_breath_partial_pressure")]
pub fn get_breath_partial_pressure(gas_pressure: &Value) {
    profile_proc!("get_breath_partial_pressure");

    value!(unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .get_breath_partial_pressure(gas_pressure.as_number().unwrap_unchecked())
    })
}

//Reverse of the above
#[hook("/datum/gas_mixture/proc/get_true_breath_pressure")]
pub fn get_true_breath_pressure(breath_pp: &Value) {
    profile_proc!("get_true_breath_pressure");

    value!(unsafe {
        Mixture::get_mixture_by_src(src)
            .unwrap_unchecked()
            .get_true_breath_pressure(breath_pp.as_number().unwrap_unchecked())
    })
}
