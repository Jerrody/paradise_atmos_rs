mod adds;
mod getters;
mod procs;
mod setters;
mod subs;

use crate::constants::*;

use once_cell::unsync::Lazy;

/// ### Description
/// SOA (struct of arrays). Used for efficient cache utilization.
/// ### Why?
/// You would say that it's less comfortable to use `gas_mixture` and why this unnecessary complexity.
/// The answer is to load components (aka `oxygen`, `carbon_dioxide`, and others) that need to use, with no extra load.
/// Imagine that you use simple `Vec<Mixture>` and need to use the `return_volume` method, so, you index an instance of `Mixture`.
/// So, you load to the cache whole **68** `bytes`! And use only 4 bytes :) == waste CPU time on useless action.
/// I hope I answered your question!
pub static mut MIXTURES: Lazy<Mixture> = Lazy::new(Mixture::new);
static mut IS_INITIALIZED_MIXTURES: bool = false;

#[derive(Debug, Default)]
pub struct Mixture {
    oxygen: Vec<f32>,
    carbon_dioxide: Vec<f32>,
    nitrogen: Vec<f32>,
    toxins: Vec<f32>,
    sleeping_agent: Vec<f32>,
    agent_b: Vec<f32>,
    volume: Vec<f32>,
    temperature: Vec<f32>, //in Kelvin
    last_share: Vec<f32>,
    oxygen_archived: Vec<f32>,
    carbon_dioxide_archived: Vec<f32>,
    nitrogen_archived: Vec<f32>,
    toxins_archived: Vec<f32>,
    sleeping_agent_archived: Vec<f32>,
    agent_b_archived: Vec<f32>,
    temperature_archived: Vec<f32>,
    fuel_burnt: Vec<f32>,
    is_initialized: Vec<bool>,
}

impl Mixture {
    /// ### Description
    /// Pre-allocates a 1_000_000 `gas_mixtures`.
    /// Needs for avoding a reallocation a whole [`GAS_MIXTURES`] with each creaing of an instance of `gas_mixture`.
    /// ### Size
    /// Initial size of [`MIXTURES`] is 68 MBs.
    const DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT: usize = 1_000_000;
    /// #### Description
    /// Liters in a cell.
    const CELL_VOLUME: f32 = 2500.0;

    #[must_use]
    #[inline(always)]
    fn new() -> Self {
        if unsafe { IS_INITIALIZED_MIXTURES } {
            panic!("`MIXTURES` is already initialized!");
        }

        let vec_of_zeros = vec![Default::default(); Self::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT];
        let vec_of_cell_volumes =
            vec![Self::CELL_VOLUME; Self::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT];
        let vec_of_bools = vec![Default::default(); Self::DEFAULT_ALLOCATED_GAS_MIXTURES_COUNT];

        unsafe {
            IS_INITIALIZED_MIXTURES = true;
        }

        Self {
            oxygen: vec_of_zeros.clone(),
            carbon_dioxide: vec_of_zeros.clone(),
            nitrogen: vec_of_zeros.clone(),
            toxins: vec_of_zeros.clone(),
            sleeping_agent: vec_of_zeros.clone(),
            agent_b: vec_of_zeros.clone(),
            volume: vec_of_cell_volumes,
            temperature: vec_of_zeros.clone(),
            last_share: vec_of_zeros.clone(),
            oxygen_archived: vec_of_zeros.clone(),
            carbon_dioxide_archived: vec_of_zeros.clone(),
            nitrogen_archived: vec_of_zeros.clone(),
            toxins_archived: vec_of_zeros.clone(),
            sleeping_agent_archived: vec_of_zeros.clone(),
            agent_b_archived: vec_of_zeros.clone(),
            temperature_archived: vec_of_zeros.clone(),
            fuel_burnt: vec_of_zeros,
            is_initialized: vec_of_bools,
        }
    }

    #[inline(always)]
    pub unsafe fn register(&mut self, src: usize) {
        self.set_is_initialized(src, true);
    }

    #[inline(always)]
    pub unsafe fn unregister(&mut self, src: usize) {
        self.set_to_default(src);
    }

    #[inline(always)]
    unsafe fn set_to_default(&mut self, id: usize) {
        self.set_is_initialized(id, Default::default());

        self.set_oxygen(id, Default::default());
        self.set_carbon_dioxide(id, Default::default());
        self.set_nitrogen(id, Default::default());
        self.set_toxins(id, Default::default());
        self.set_sleeping_agent(id, Default::default());
        self.set_agent_b(id, Default::default());
        self.set_volume(id, Self::CELL_VOLUME);
        self.set_temperature(id, Default::default()); //in Kelvin
        self.set_last_share(id, Default::default());
        self.set_oxygen_archived(id, Default::default());
        self.set_carbon_dioxide_archived(id, Default::default());
        self.set_nitrogen_archived(id, Default::default());
        self.set_toxins_archived(id, Default::default());
        self.set_sleeping_agent_archived(id, Default::default());
        self.set_agent_b_archived(id, Default::default());
        self.set_temperature_archived(id, Default::default());
        self.set_fuel_burnt(id, Default::default());
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_is_initialized(&self, id: usize) -> bool {
        *self.is_initialized.get_unchecked(id)
    }

    #[inline(always)]
    pub unsafe fn set_is_initialized(&mut self, id: usize, value: bool) {
        *self.is_initialized.get_unchecked_mut(id) = value;
    }

    #[must_use]
    #[inline(always)]
    fn compare_condition(self_value: f32, sample_value: f32) -> bool {
        ((self_value - sample_value).abs() > MINIMUM_AIR_TO_SUSPEND)
            && ((self_value < (1.0 - MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_value)
                || (self_value > (1.0 + MINIMUM_AIR_RATIO_TO_SUSPEND) * sample_value))
    }

    #[must_use]
    #[inline(always)]
    fn check_turf_condition(value_01: f32, value_02: f32) -> bool {
        (value_01 > MINIMUM_AIR_TO_SUSPEND) && (value_01 >= value_02 * MINIMUM_AIR_RATIO_TO_SUSPEND)
    }
}
