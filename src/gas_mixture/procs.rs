// use super::constants::*;
// use crate::value;
// use auxtools::*;

pub use super::GAS_MIXTURES;

// // #[hook("/datum/gas_mixture/proc/heat_capacity_rs")]
// // pub fn heat_capacity() {
// //     value!(MixtureNative::get_heat_capacity(src))
// // }

// // #[hook("/datum/gas_mixture/proc/heat_capacity_archived_rs")]
// // pub fn heat_capacity_archived() {
// //     value!(MixtureNative::get_heat_capacity(src))
// // }

// // #[hook("/datum/gas_mixture/proc/total_moles_rs")]
// // pub fn total_moles() {
// //     value!(MixtureNative::get_total_moles(src))
// // }

// // #[hook("/datum/gas_mixture/proc/return_pressure_rs")]
// // pub fn return_pressure() {
// //     let volume = MixtureNative::get_volume(src);

// //     if volume > 0.0 {
// //         return Ok(Value::from(
// //             MixtureNative::get_total_moles(src)
// //                 * crate::gas_mixture::constants::R_IDEAL_GAS_EQUATION
// //                 * MixtureNative::get_temperature(src)
// //                 / volume,
// //         ));
// //     }

// //     Ok(Value::from(f32::default()))
// // }

// // #[hook("/datum/gas_mixture/proc/thermal_energy_rs")]
// // pub fn thermal_energy() {
// //     // TODO: Make it procces vai struct `Mixture`.
// //     value!(MixtureNative::get_temperature(src) * MixtureNative::get_heat_capacity(src))
// // }

// #[hook("/datum/gas_mixture/proc/react_rs")]
// pub fn react() {
//     let mut reacting = 0.0; //set to 1 if a notable reaction occured (used by pipe_network)

//     let mut mixture = Mixture::new(src);

//     if mixture.temperature > 900.0
//         && mixture.toxins > MINIMUM_HEAT_CAPACITY
//         && mixture.carbon_dioxide > MINIMUM_HEAT_CAPACITY
//     {
//         // let gases = [
//         //     mixture.carbon_dioxide * 0.75,
//         //     mixture.toxins * 0.25,
//         //     mixture.agent_b * 0.05,
//         // ];

//         let reaction_rate = unsafe {
//             [
//                 mixture.carbon_dioxide * 0.75,
//                 mixture.toxins * 0.25,
//                 mixture.agent_b * 0.05,
//             ]
//             .into_iter()
//             .min_by(|a, b| a.total_cmp(b))
//             .unwrap_unchecked()
//         };

//         mixture.carbon_dioxide -= reaction_rate;
//         mixture.oxygen += reaction_rate;
//         mixture.agent_b -= reaction_rate * 0.05;
//         mixture.temperature += (reaction_rate * 20_000.0) / mixture.heat_capacity();

//         reacting = 1.0;
//     }

//     if mixture.temperature > FIRE_MINIMUM_TEMPERATURE_TO_EXIST {
//         let fuel_burnt = mixture.fire();
//         MixtureNative::set_fuel_burnt(src, fuel_burnt);
//         MixtureNative::set_toxins(src, mixture.toxins);

//         if fuel_burnt > 0.0 {
//             reacting = 1.0;
//         }
//     }

//     if reacting > 0.0 {
//         MixtureNative::set_carbon_dioxide(src, mixture.carbon_dioxide);
//         MixtureNative::set_oxygen(src, mixture.oxygen);
//         MixtureNative::set_agent_b(src, mixture.agent_b);
//         MixtureNative::set_temperature(src, mixture.temperature);
//     }

//     value!(reacting)
// }
