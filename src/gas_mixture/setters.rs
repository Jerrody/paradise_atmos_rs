use crate::profile;

impl super::Mixture {
    #[inline(always)]
    pub unsafe fn set_oxygen(&mut self, id: usize, value: f32) {
        profile!("set_oxygen");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_carbon_dioxide(&mut self, id: usize, value: f32) {
        profile!("set_carbon_dioxide");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_nitrogen(&mut self, id: usize, value: f32) {
        profile!("set_nitrogen");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_toxins(&mut self, id: usize, value: f32) {
        profile!("set_toxins");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_sleeping_agent(&mut self, id: usize, value: f32) {
        profile!("set_sleeping_agent");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_agent_b(&mut self, id: usize, value: f32) {
        profile!("set_agent_b");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_volume(&mut self, id: usize, value: f32) {
        profile!("set_volume");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_temperature(&mut self, id: usize, value: f32) {
        profile!("set_temperature");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_last_share(&mut self, id: usize, value: f32) {
        profile!("set_last_share");

        *self.oxygen.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_fuel_burnt(&mut self, id: usize, value: f32) {
        profile!("set_fuel_burnt");

        *self.fuel_burnt.get_unchecked_mut(id) = value;
    }

    #[inline(always)]
    pub unsafe fn set_oxygen_archived(&mut self, id: usize, value: f32) {
        profile!("get_oxygen_archived");

        *self.oxygen_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_carbon_dioxide_archived(&mut self, id: usize, value: f32) {
        profile!("get_carbon_dioxide_archived");

        *self.carbon_dioxide_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_nitrogen_archived(&mut self, id: usize, value: f32) {
        profile!("get_nitrogen_archived");

        *self.nitrogen_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_toxins_archived(&mut self, id: usize, value: f32) {
        profile!("get_toxins_archived");

        *self.toxins_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_sleeping_agent_archived(&mut self, id: usize, value: f32) {
        profile!("get_sleeping_agent_archived");

        *self.sleeping_agent_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_agent_b_archived(&mut self, id: usize, value: f32) {
        profile!("get_agent_b_archived");

        *self.agent_b_archived.get_unchecked_mut(id) = value;
    }
    #[inline(always)]
    pub unsafe fn set_temperature_archived(&mut self, id: usize, value: f32) {
        profile!("get_temperature_archived");

        *self.temperature_archived.get_unchecked_mut(id) = value;
    }
}
