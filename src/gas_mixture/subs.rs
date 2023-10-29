use super::Mixture;

macro_rules! sub_methods {
    ($($method:ident, $field:ident);+ $(;)?) => {
        impl Mixture {
            $(
                #[inline(always)]
                pub unsafe fn $method(&mut self, id: usize, value: f32) {
                    *self.$field.get_unchecked_mut(id) -= value;
                }
            )+
        }
    }
}

sub_methods! {
    sub_oxygen, oxygen;
    sub_carbon_dioxide, carbon_dioxide;
    sub_nitrogen, nitrogen;
    sub_toxins, toxins;
    sub_sleeping_agent, sleeping_agent;
    sub_agent_b, agent_b;
    sub_temperature, temperature;
}
