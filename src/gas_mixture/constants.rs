#![allow(unused)] // FIXME: Later remove it.

// NOTE: All constants is constant evaluated that means that every constant with op is a compile-time computation => no overhead at the runtime.

// TODO: Make a documentation for the all constants.
pub mod excited_groups;
pub mod fire;
pub mod heat;
pub mod heat_transfer_coefficients;
pub mod plasma;

pub use excited_groups::*;
pub use fire::*;
pub use heat::*;
pub use heat_transfer_coefficients::*;
pub use plasma::*;

/// ### Description
/// kPa * L / (K * mol)
pub const R_IDEAL_GAS_EQUATION: f32 = 8.31;
pub const ONE_ATMOSPHERE: f32 = 101.325; //kPa
pub const TCMB: f32 = 2.7; // -270.3degC
pub const TCRYO: f32 = 265.0; // -48.15degC
pub const T0C: f32 = 273.15; // 0degC
pub const T20C: f32 = 293.15; // 20degC
pub const MOLES_CELLSTANDARD: f32 = ONE_ATMOSPHERE * CELL_VOLUME / (T20C * R_IDEAL_GAS_EQUATION); //moles in a 2.5 m^3 cell at 101.325 Pa and 20 degC
pub const M_CELL_WITH_RATIO: f32 = MOLES_CELLSTANDARD * 0.005; //compared against for superconductivity
pub const O2STANDARD: f32 = 0.21; //percentage of oxygen in a normal mixture of air
pub const N2STANDARD: f32 = 0.79; //same but for nitrogen
pub const MOLES_O2STANDARD: f32 = MOLES_CELLSTANDARD * O2STANDARD; // O2 standard value (21%)
pub const MOLES_N2STANDARD: f32 = MOLES_CELLSTANDARD * N2STANDARD; // N2 standard value (79%)
pub const CELL_VOLUME: f32 = 2500.0; //;liters in a cell
                                     //liters in a normal breath
pub const BREATH_VOLUME: f32 = 1.0;
pub const BREATH_PERCENTAGE: f32 = BREATH_VOLUME / CELL_VOLUME;
pub const MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER: f32 = 0.5;

pub const ZERO: f32 = 0.0;
