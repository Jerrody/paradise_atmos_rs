use byondapi::value::ByondValue;

pub struct Turf {
    pub oxygen: f32,
    pub carbon_dioxide: f32,
    pub nitrogen: f32,
    pub toxins: f32,
    pub sleeping_agent: f32,
    pub agent_b: f32,
    pub temperature: f32,
}

impl Turf {
    const OXYGEN: &'static str = "oxygen";
    const CARBON_DIOXIDE: &'static str = "carbon_dioxide";
    const NITROGEN: &'static str = "nitrogen";
    const TOXINS: &'static str = "toxins";
    const SLEEPING_AGENT: &'static str = "sleeping_agent";
    const AGENT_B: &'static str = "agent_b";
    const TEMPERATURE: &'static str = "temperature";

    #[inline(always)]
    #[must_use]
    pub unsafe fn new(turf: ByondValue) -> Self {
        Self {
            oxygen: turf
                .read_var(Self::OXYGEN)
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked(),
            carbon_dioxide: turf
                .read_var(Self::CARBON_DIOXIDE)
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked(),
            nitrogen: turf
                .read_var(Self::NITROGEN)
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked(),
            toxins: turf
                .read_var(Self::TOXINS)
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked(),
            sleeping_agent: turf
                .read_var(Self::SLEEPING_AGENT)
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked(),
            agent_b: turf
                .read_var(Self::AGENT_B)
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked(),
            temperature: turf
                .read_var(Self::TEMPERATURE)
                .unwrap_unchecked()
                .get_number()
                .unwrap_unchecked(),
        }
    }
}
