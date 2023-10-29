use super::Mixture;

macro_rules! get_methods {
    ($($method:ident, $field:ident);+ $(;)?) => {
        impl Mixture {
            $(
                #[inline(always)]
                #[must_use]
                pub unsafe fn $method(&self, id: usize) -> f32 {
                    *self.$field.get_unchecked(id)
                }
            )+
        }
    }
}

get_methods! {
    get_oxygen, oxygen;
    get_carbon_dioxide, carbon_dioxide;
    get_nitrogen, nitrogen;
    get_toxins, toxins;
    get_sleeping_agent, sleeping_agent;
    get_agent_b, agent_b;
    get_volume, volume;
    get_temperature, temperature;
    get_last_share, last_share;
    get_fuel_burnt, fuel_burnt;
    get_oxygen_archived, oxygen_archived;
    get_carbon_dioxide_archived, carbon_dioxide_archived;
    get_nitrogen_archived, nitrogen_archived;
    get_toxins_archived, toxins_archived;
    get_sleeping_agent_archived, sleeping_agent_archived;
    get_agent_b_archived, agent_b_archived;
    get_temperature_archived, temperature_archived;
}
