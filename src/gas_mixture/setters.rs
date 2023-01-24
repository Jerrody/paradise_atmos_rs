impl super::Mixture {
    #[inline(always)]
    pub unsafe fn set_oxygen(&mut self, id: usize, value: f32) {
        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_carbon_dioxide(&mut self, id: usize, value: f32) {
        *self.carbon_dioxide.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_nitrogen(&mut self, id: usize, value: f32) {
        *self.nitrogen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_toxins(&mut self, id: usize, value: f32) {
        *self.toxins.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_sleeping_agent(&mut self, id: usize, value: f32) {
        *self.sleeping_agent.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_agent_b(&mut self, id: usize, value: f32) {
        *self.agent_b.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_volume(&mut self, id: usize, value: f32) {
        *self.volume.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_temperature(&mut self, id: usize, value: f32) {
        *self.temperature.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_last_share(&mut self, id: usize, value: f32) {
        *self.last_share.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_fuel_burnt(&mut self, id: usize, value: f32) {
        *self.fuel_burnt.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_oxygen_archived(&mut self, id: usize, value: f32) {
        *self.oxygen_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_carbon_dioxide_archived(&mut self, id: usize, value: f32) {
        *self.carbon_dioxide_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_nitrogen_archived(&mut self, id: usize, value: f32) {
        *self.nitrogen_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_toxins_archived(&mut self, id: usize, value: f32) {
        *self.toxins_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_sleeping_agent_archived(&mut self, id: usize, value: f32) {
        *self.sleeping_agent_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_agent_b_archived(&mut self, id: usize, value: f32) {
        *self.agent_b_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_temperature_archived(&mut self, id: usize, value: f32) {
        *self.temperature_archived.get_unchecked_mut(id) = value;
    }
}
