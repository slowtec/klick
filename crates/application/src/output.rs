use klick_domain::{CO2Equivalents, EmissionFactors};

#[derive(Debug, Clone)]
pub struct Output {
    pub co2_equivalents: CO2Equivalents,
    pub emission_factors: EmissionFactors,
}
