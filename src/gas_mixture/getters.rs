impl super::Mixture {
    #[inline(always)]
    #[must_use]
    pub unsafe fn get_oxygen(&self, id: usize) -> f32 {
        *self.oxygen.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_carbon_dioxide(&self, id: usize) -> f32 {
        *self.carbon_dioxide.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_nitrogen(&self, id: usize) -> f32 {
        *self.nitrogen.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_toxins(&self, id: usize) -> f32 {
        *self.toxins.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_sleeping_agent(&self, id: usize) -> f32 {
        *self.sleeping_agent.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_agent_b(&self, id: usize) -> f32 {
        *self.agent_b.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_volume(&self, id: usize) -> f32 {
        *self.volume.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_temperature(&self, id: usize) -> f32 {
        *self.temperature.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_last_share(&self, id: usize) -> f32 {
        *self.last_share.get_unchecked(id)
    }

    #[inline(always)]
    pub unsafe fn get_fuel_burnt(&self, id: usize) -> f32 {
        *self.fuel_burnt.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_oxygen_archived(&self, id: usize) -> f32 {
        *self.oxygen_archived.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_carbon_dioxide_archived(&self, id: usize) -> f32 {
        *self.carbon_dioxide_archived.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_nitrogen_archived(&self, id: usize) -> f32 {
        *self.nitrogen_archived.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_toxins_archived(&self, id: usize) -> f32 {
        *self.toxins_archived.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_sleeping_agent_archived(&self, id: usize) -> f32 {
        *self.sleeping_agent_archived.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_agent_b_archived(&self, id: usize) -> f32 {
        *self.agent_b_archived.get_unchecked(id)
    }

    #[inline(always)]
    #[must_use]
    pub unsafe fn get_temperature_archived(&self, id: usize) -> f32 {
        *self.temperature_archived.get_unchecked(id)
    }
}
