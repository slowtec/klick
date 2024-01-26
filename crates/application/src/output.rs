use klick_domain::{CO2Equivalents, Factor};

#[derive(Debug, Clone)]
pub struct Output {
    pub co2_equivalents: CO2Equivalents,
    pub n2o_emission_factor: Factor,
}
