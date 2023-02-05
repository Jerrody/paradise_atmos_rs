use super::Mixture;

impl Mixture {
    #[inline(always)]
    pub unsafe fn sub_oxygen(&mut self, id: usize, value: f32) {
        *self.oxygen.get_unchecked_mut(id) -= value;
    }

    #[inline(always)]
    pub unsafe fn sub_carbon_dioxide(&mut self, id: usize, value: f32) {
        *self.carbon_dioxide.get_unchecked_mut(id) -= value;
    }

    #[inline(always)]
    pub unsafe fn sub_nitrogen(&mut self, id: usize, value: f32) {
        *self.nitrogen.get_unchecked_mut(id) -= value;
    }

    #[inline(always)]
    pub unsafe fn sub_toxins(&mut self, id: usize, value: f32) {
        *self.toxins.get_unchecked_mut(id) -= value;
    }

    #[inline(always)]
    pub unsafe fn sub_sleeping_agent(&mut self, id: usize, value: f32) {
        *self.sleeping_agent.get_unchecked_mut(id) -= value;
    }

    #[inline(always)]
    pub unsafe fn sub_agent_b(&mut self, id: usize, value: f32) {
        *self.agent_b.get_unchecked_mut(id) -= value;
    }

    #[inline(always)]
    pub unsafe fn sub_temperature(&mut self, id: usize, value: f32) {
        *self.temperature.get_unchecked_mut(id) -= value;
    }
}
