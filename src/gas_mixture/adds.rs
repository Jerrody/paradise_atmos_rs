use super::Mixture;

macro_rules! add_methods {
    ($($method:ident, $field:ident);+ $(;)?) => {
        impl Mixture {
            $(
                #[inline(always)]
                pub unsafe fn $method(&mut self, id: usize, value: f32) {
                    *self.$field.get_unchecked_mut(id) += value;
                }
            )+
        }
    }
}

add_methods! {
    add_oxygen, oxygen;
    add_carbon_dioxide, carbon_dioxide;
    add_nitrogen, nitrogen;
    add_toxins, toxins;
    add_sleeping_agent, sleeping_agent;
    add_agent_b, agent_b;
    add_temperature, temperature;
    add_fuel_burnt, fuel_burnt;
}
