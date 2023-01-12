// NOTE: All constants is constant evaluated that means that every constant with op is a compile-time computation => no overhead at the runtime.

pub mod fire;
pub mod heat;
pub mod plasma;
pub use fire::*;
pub use heat::*;
pub use plasma::*;

const T0C: f32 = 273.15;

pub const R_IDEAL_GAS_EQUATION: f32 = 8.31;
/// #### Description
/// Liters in a cell.
pub const CELL_VOLUME: f32 = 2500.0;
