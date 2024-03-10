use byondapi::byond_string;
use byondapi::value::ByondValue;

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
}

#[byondapi::bind("/datum/gas_mixture/proc/")]
pub fn register(id: ByondValue) {
    profile_proc!("register");

    unsafe { MIXTURES.register(id!(id)) };

    null!()
}

#[byondapi::bind]
pub fn unregister(id: ByondValue) {
    profile_proc!("unregister");

    unsafe { MIXTURES.unregister(id!(id)) };

    null!()
}

#[byondapi::bind]
pub fn get_is_initialized(src: ByondValue) {
    profile_proc!("unregister");

    value!(unsafe { MIXTURES.get_is_initialized(id!(src)) })
}

#[byondapi::bind]
pub fn get_oxygen(src: ByondValue) {
    profile_proc!("get_oxygen");

    value!(unsafe { MIXTURES.get_oxygen(id!(src)) })
}

#[byondapi::bind]
pub fn get_carbon_dioxide(src: ByondValue) {
    profile_proc!("get_carbon_dioxide");

    value!(unsafe { MIXTURES.get_carbon_dioxide(id!(src)) })
}

#[byondapi::bind]
pub fn get_nitrogen(src: ByondValue) {
    profile_proc!("get_nitrogen");

    value!(unsafe { MIXTURES.get_nitrogen(id!(src)) })
}

#[byondapi::bind]
pub fn get_toxins(src: ByondValue) {
    profile_proc!("get_toxins");

    value!(unsafe { MIXTURES.get_toxins(id!(src)) })
}

#[byondapi::bind]
pub fn get_sleeping_agent(src: ByondValue) {
    profile_proc!("get_sleeping_agent");

    value!(unsafe { MIXTURES.get_sleeping_agent(id!(src)) })
}

#[byondapi::bind]
pub fn get_agent_b(src: ByondValue) {
    profile_proc!("get_agent_b");

    value!(unsafe { MIXTURES.get_agent_b(id!(src)) })
}

#[byondapi::bind]
pub fn get_volume(src: ByondValue) {
    profile_proc!("get_volume");

    value!(unsafe { MIXTURES.get_volume(id!(src)) })
}

#[byondapi::bind]
pub fn get_temperature(src: ByondValue) {
    profile_proc!("get_temperature");

    value!(unsafe { MIXTURES.get_temperature(id!(src)) })
}

#[byondapi::bind]
pub fn get_last_share(src: ByondValue) {
    profile_proc!("get_last_share");

    value!(unsafe { MIXTURES.get_last_share(id!(src)) })
}

#[byondapi::bind]
pub fn set_oxygen(src: ByondValue) {
    profile_proc!("set_oxygen");

    unsafe { MIXTURES.set_oxygen(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_carbon_dioxide(src: ByondValue) {
    profile_proc!("set_carbon_dioxide");

    unsafe { MIXTURES.set_carbon_dioxide(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_nitrogen(src: ByondValue) {
    profile_proc!("set_nitrogen");

    unsafe { MIXTURES.set_nitrogen(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_toxins(src: ByondValue) {
    profile_proc!("set_toxins");

    unsafe { MIXTURES.set_toxins(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_sleeping_agent(src: ByondValue) {
    profile_proc!("set_sleeping_agent");

    unsafe { MIXTURES.set_sleeping_agent(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_agent_b(src: ByondValue) {
    profile_proc!("set_agent_b");

    unsafe { MIXTURES.set_agent_b(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_volume(src: ByondValue) {
    profile_proc!("set_volume");

    unsafe { MIXTURES.set_volume(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_temperature(src: ByondValue) {
    profile_proc!("set_temperature");

    unsafe { MIXTURES.set_temperature(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn set_last_share(src: ByondValue) {
    profile_proc!("set_last_share");

    unsafe { MIXTURES.set_last_share(id!(src), src.get_number().unwrap_unchecked()) }

    null!()
}

#[byondapi::bind]
pub fn get_heat_capacity(src: ByondValue) {
    profile_proc!("get_heat_capacity");

    value!(unsafe { MIXTURES.heat_capacity(id!(src)) })
}

#[byondapi::bind]
pub fn get_total_moles(src: ByondValue) {
    profile_proc!("get_total_moles");

    value!(unsafe { MIXTURES.total_moles(id!(src)) })
}

#[byondapi::bind]
pub fn get_total_trace_moles(src: ByondValue) {
    profile_proc!("get_total_trace_moles");

    value!(unsafe { MIXTURES.get_total_trace_moles(id!(src)) })
}

#[byondapi::bind]
pub fn get_pressure(src: ByondValue) {
    profile_proc!("get_pressure");

    value!(unsafe { MIXTURES.return_pressure(id!(src)) })
}

// I'm not sure that this thing was made by a person with good mental health in DM.
// Anyway, it could cause, potentially, unexpected behavior.
#[byondapi::bind]
pub fn return_volume(src: ByondValue) {
    profile_proc!("return_volume");

    value!(unsafe { MIXTURES.return_volume(id!(src)) })
}

#[byondapi::bind]
pub fn get_thermal_energy(src: ByondValue) {
    profile_proc!("get_thermal_energy");

    value!(unsafe { MIXTURES.thermal_energy(id!(src)) })
}

#[byondapi::bind]
pub fn react(src: ByondValue) {
    profile_proc!("react");

    value!(unsafe { MIXTURES.react(id!(src)) })
}

#[byondapi::bind]
pub fn archive(src: ByondValue) {
    profile_proc!("archive");

    unsafe { MIXTURES.archive(id!(src)) }

    null!()
}

#[byondapi::bind]
pub fn merge(src: ByondValue, giver: ByondValue) {
    profile_proc!("merge");

    value!(unsafe { MIXTURES.merge(id!(src), id!(giver)) })
}

#[byondapi::bind]
pub fn remove(src: ByondValue, removed: ByondValue, amount: ByondValue) {
    profile_proc!("remove");

    unsafe {
        MIXTURES.remove(
            id!(src),
            id!(removed),
            amount.get_number().unwrap_unchecked(),
        );
    }

    null!()
}

#[byondapi::bind]
pub fn remove_ratio(src: ByondValue, removed: ByondValue, ratio: ByondValue) {
    profile_proc!("remove_ratio");

    unsafe {
        MIXTURES.remove_ratio(id!(src), id!(removed), ratio.get_number().unwrap_unchecked());
    }

    null!()
}

#[byondapi::bind]
pub fn copy_from(src: ByondValue, sample: ByondValue) {
    profile_proc!("copy_from");

    unsafe {
        MIXTURES.copy_from(id!(src), id!(sample));
    }

    null!()
}

#[byondapi::bind]
pub fn check_turf(src: ByondValue, turf_model: ByondValue, atmos_adjacent_turfs: ByondValue) {
    profile_proc!("check_turf");

    let turf_model = unsafe { Turf::new(turf_model) };
    let atmos_adjacent_turfs = atmos_adjacent_turfs
        .get_number()
        .unwrap_or(DEFAULT_ATMOS_ADJACENT_TURFS);

    value!(unsafe {
        MIXTURES.check_turf(id!(src), turf_model, atmos_adjacent_turfs)
    })
}

#[byondapi::bind]
pub fn check_turf_total(src: ByondValue,turf_model: ByondValue) {
    profile_proc!("check_turf_total");

    let turf_model = unsafe { Turf::new(turf_model) };

    value!(unsafe { MIXTURES.check_turf_total(id!(src), turf_model) })
}

#[byondapi::bind]
pub fn share(src: ByondValue, sharer: ByondValue, atmos_adjacent_turfs: ByondValue) {
    profile_proc!("share");

    let atmos_adjacent_turfs = atmos_adjacent_turfs
        .get_number()
        .unwrap_or(DEFAULT_ATMOS_ADJACENT_TURFS);

    value!(unsafe { MIXTURES.share(id!(src), id!(sharer), atmos_adjacent_turfs) })
}

#[byondapi::bind]
pub fn temperature_share(src: ByondValue, sharer: ByondValue, conduction_coefficient: ByondValue) {
    profile_proc!("temperature_share");

    let conduction_coefficient = unsafe { conduction_coefficient.get_number().unwrap_unchecked() };

    unsafe {
        let id = id!(src);
        let sharer_id = id!(sharer);

        MIXTURES.temperature_share(
            id,
            id!(sharer),
            MIXTURES.get_temperature_archived(id),
            MIXTURES.get_temperature_archived(sharer_id),
            conduction_coefficient,
        );
    }

    null!()
}

#[byondapi::bind]
pub fn mimic(
    src: ByondValue,
    turf_model: ByondValue,
    model_thermal_conductivity: ByondValue,
    model_heat_capacity: ByondValue,
    atmos_adjacent_turfs: ByondValue,
) {
    profile_proc!("mimic");

    let turf_model = unsafe { Turf::new(turf_model) };
    let model_thermal_conductivity =
        unsafe { model_thermal_conductivity.get_number().unwrap_unchecked() };
    let model_heat_capacity = unsafe { model_heat_capacity.get_number().unwrap_unchecked() };
    let atmos_adjacent_turfs = atmos_adjacent_turfs
        .get_number()
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

#[byondapi::bind]
pub fn temperature_mimic(
    src: ByondValue,
    model_temperature: ByondValue,
    model_heat_capacity: ByondValue,
    conduction_coefficient: ByondValue,
) {
    profile_proc!("temperature_mimic");

    let model_temperature = unsafe { model_temperature.get_number().unwrap_unchecked() };
    let model_heat_capacity = unsafe { model_heat_capacity.get_number().unwrap_unchecked() };
    let conduction_coefficient = unsafe { conduction_coefficient.get_number().unwrap_unchecked() };

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

#[byondapi::bind]
pub fn temperature_turf_share(src: ByondValue, mut turf_sharer: ByondValue, conduction_coefficient: ByondValue) {
    profile_proc!("temperature_turf_share");

    let conduction_coefficient = unsafe { conduction_coefficient.get_number().unwrap_unchecked() };

    unsafe {
        MIXTURES.temperature_turf_share(id!(src), &mut turf_sharer, conduction_coefficient);
    }

    null!()
}

#[byondapi::bind]
pub fn compare(src: ByondValue, sample: ByondValue) {
    profile_proc!("compare");

    value!(unsafe { MIXTURES.compare(id!(src), id!(sample)) })
}

#[byondapi::bind]
pub fn get_breath_partial_pressure(src: ByondValue, gas_pressure: ByondValue) {
    profile_proc!("get_breath_partial_pressure");

    value!(unsafe {
        MIXTURES.get_breath_partial_pressure(id!(src), gas_pressure.get_number().unwrap_unchecked())
    })
}

//Reverse of the above
#[byondapi::bind]
pub fn get_true_breath_pressure(src: ByondValue, breath_pp: ByondValue) {
    profile_proc!("get_true_breath_pressure");

    value!(unsafe {
        MIXTURES.get_true_breath_pressure(id!(src), breath_pp.get_number().unwrap_unchecked())
    })
}
