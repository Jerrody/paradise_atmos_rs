use super::Mixture;

macro_rules! set_methods {
    ($($method:ident, $field:ident);+ $(;)?) => {
        impl Mixture {
            $(
                #[inline(always)]
                pub unsafe fn $method(&mut self, id: usize, value: f32) {
                    *self.$field.get_unchecked_mut(id) = value;
                }
            )+
        }
    }
}

set_methods! {
    set_oxygen, oxygen;
    set_carbon_dioxide, carbon_dioxide;
    set_nitrogen, nitrogen;
    set_toxins, toxins;
    set_sleeping_agent, sleeping_agent;
    set_agent_b, agent_b;
    set_volume, volume;
    set_temperature, temperature;
    set_last_share, last_share;
    set_fuel_burnt, fuel_burnt;
    set_oxygen_archived, oxygen_archived;
    set_carbon_dioxide_archived, carbon_dioxide_archived;
    set_nitrogen_archived, nitrogen_archived;
    set_toxins_archived, toxins_archived;
    set_sleeping_agent_archived, sleeping_agent_archived;
    set_agent_b_archived, agent_b_archived;
    set_temperature_archived, temperature_archived;
}
