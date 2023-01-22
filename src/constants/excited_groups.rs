use super::*;

// TODO: Make a documentation for these constants.
/// Number of FULL air controller ticks before an excited group breaks down (averages gas contents across turfs)
pub const EXCITED_GROUP_BREAKDOWN_CYCLES: f32 = 10.0;
/// Number of FULL air controller ticks before an excited group dismantles and removes its turfs from active.
pub const EXCITED_GROUP_DISMANTLE_CYCLES: f32 = 20.0;
/// Minimum ratio of air that must move to/from a tile to suspend group processing.
pub const MINIMUM_AIR_RATIO_TO_SUSPEND: f32 = 0.005;
/// Minimum amount of air that has to move before a group processing can be suspended.
pub const MINIMUM_AIR_TO_SUSPEND: f32 = MOLES_CELLSTANDARD * MINIMUM_AIR_RATIO_TO_SUSPEND;
/// Can be active [`MINIMUM_MOLES_DELTA_TO_MOVE`] or [`MINIMUM_TEMPERATURE_TO_MOVE`] or both of them.
pub const MINIMUM_MOLES_DELTA_TO_MOVE: f32 = MOLES_CELLSTANDARD * MINIMUM_AIR_RATIO_TO_SUSPEND;
/// Can be active [`MINIMUM_TEMPERATURE_TO_MOVE`] or [`MINIMUM_MOLES_DELTA_TO_MOVE`] or both of them.
pub const MINIMUM_TEMPERATURE_TO_MOVE: f32 = T20C + 100.0;
pub const MINIMUM_TEMPERATURE_RATIO_TO_SUSPEND: f32 = 0.012;
pub const MINIMUM_TEMPERATURE_DELTA_TO_SUSPEND: f32 = 4.0; //Minimum temperature difference before group processing is suspended
pub const MINIMUM_TEMPERATURE_DELTA_TO_CONSIDER: f32 = 0.5; //Minimum temperature difference before the gas temperatures are just set to be equal
pub const MINIMUM_TEMPERATURE_FOR_SUPERCONDUCTION: f32 = T20C + 10.0;
pub const MINIMUM_TEMPERATURE_START_SUPERCONDUCTION: f32 = T20C + 200.0;
