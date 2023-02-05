use super::Mixture;

impl Mixture {
    #[inline(always)]
    pub unsafe fn add_oxygen(&mut self, id: usize, value: f32) {
        *self.oxygen.get_unchecked_mut(id) += value;
    }

    #[inline(always)]
    pub unsafe fn add_carbon_dioxide(&mut self, id: usize, value: f32) {
        *self.carbon_dioxide.get_unchecked_mut(id) += value;
    }

    #[inline(always)]
    pub unsafe fn add_nitrogen(&mut self, id: usize, value: f32) {
        *self.nitrogen.get_unchecked_mut(id) += value;
    }

    #[inline(always)]
    pub unsafe fn add_toxins(&mut self, id: usize, value: f32) {
        *self.toxins.get_unchecked_mut(id) += value;
    }

    #[inline(always)]
    pub unsafe fn add_sleeping_agent(&mut self, id: usize, value: f32) {
        *self.sleeping_agent.get_unchecked_mut(id) += value;
    }

    #[inline(always)]
    pub unsafe fn add_agent_b(&mut self, id: usize, value: f32) {
        *self.agent_b.get_unchecked_mut(id) += value;
    }

    #[inline(always)]
    pub unsafe fn add_temperature(&mut self, id: usize, value: f32) {
        *self.temperature.get_unchecked_mut(id) += value;
    }

    #[inline(always)]
    pub unsafe fn add_fuel_burnt(&mut self, id: usize, value: f32) {
        *self.fuel_burnt.get_unchecked_mut(id) += value;
    }
}
