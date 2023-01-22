//Must be between 0 and 1. Values closer to 1 equalize temperature faster
//Should not exceed 0.4 else strange heat flow occur
pub const FLOOR_HEAT_TRANSFER_COEFFICIENT: f32 = 0.15;
pub const WALL_HEAT_TRANSFER_COEFFICIENT: f32 = 0.0;
pub const OPEN_HEAT_TRANSFER_COEFFICIENT: f32 = 0.4;
pub const WINDOW_HEAT_TRANSFER_COEFFICIENT: f32 = 0.1; //a hack for now
pub const HEAT_CAPACITY_VACUUM: f32 = 700000.0; // a hack to help make vacuums "cold", sacrificing realism for gameplay
