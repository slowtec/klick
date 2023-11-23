use crate::Factor;

#[derive(Debug, Clone)]
pub struct Output {
    pub co2_equivalents: CO2Equivalents,
    pub n2o_emission_factor: Factor,
}

#[derive(Debug, Clone)]
pub struct CO2Equivalents {
    pub n2o_plant: f64,
    pub n2o_water: f64,
    pub n2o_emissions: f64,
    pub ch4_sewage_treatment: f64,
    pub ch4_sludge_storage_containers: f64,
    pub ch4_sludge_bags: f64,
    pub ch4_water: f64,
    pub ch4_combined_heat_and_power_plant: f64,
    pub ch4_emissions: f64,
    pub fecl3: f64,
    pub feclso4: f64,
    pub caoh2: f64,
    pub synthetic_polymers: f64,
    pub electricity_mix: f64,
    pub operating_materials: f64,
    pub sewage_sludge_transport: f64,
    pub emissions: f64,
    pub direct_emissions: f64,
    pub indirect_emissions: f64,
    pub other_indirect_emissions: f64,
}
