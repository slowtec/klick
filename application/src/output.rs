use crate::{Factor, Tons};

#[derive(Debug, Clone)]
pub struct Output {
    pub co2_equivalents: CO2Equivalents,
    pub n2o_emission_factor: Factor,
}

#[derive(Debug, Clone)]
pub struct CO2Equivalents {
    pub n2o_plant: Tons,
    pub n2o_water: Tons,
    pub n2o_emissions: Tons,
    pub ch4_sewage_treatment: Tons,
    pub ch4_sludge_storage_containers: Tons,
    pub ch4_sludge_bags: Tons,
    pub ch4_water: Tons,
    pub ch4_combined_heat_and_power_plant: Tons,
    pub ch4_emissions: Tons,
    pub fecl3: Tons,
    pub feclso4: Tons,
    pub caoh2: Tons,
    pub synthetic_polymers: Tons,
    pub electricity_mix: Tons,
    pub operating_materials: Tons,
    pub sewage_sludge_transport: Tons,
    pub emissions: Tons,
    pub direct_emissions: Tons,
    pub indirect_emissions: Tons,
    pub other_indirect_emissions: Tons,
}
