use serde::Deserialize;

pub use crate::v7::OperatingMaterials;

#[derive(Deserialize, Default)]
pub struct PlantProfile {
    pub(crate) plant_name: Option<String>,
    pub(crate) population_equivalent: Option<f64>,
    pub(crate) wastewater: Option<f64>,
    pub(crate) influent_average: AnnualAverageInfluent,
    pub(crate) effluent_average: AnnualAverageEffluent,
    pub(crate) energy_consumption: EnergyConsumption,
    pub(crate) sewage_sludge_treatment: SewageSludgeTreatment,
    pub(crate) side_stream_treatment: SideStreamTreatment,
    pub(crate) operating_materials: OperatingMaterials,
}

#[derive(Deserialize, Default)]
pub struct AnnualAverageInfluent {
    pub chemical_oxygen_demand: Option<f64>,
    pub total_nitrogen: Option<f64>,
    pub total_organic_carbohydrates: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct AnnualAverageEffluent {
    pub chemical_oxygen_demand: Option<f64>,
    pub total_nitrogen: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct EnergyConsumption {
    pub total_power_consumption: Option<f64>,
    pub on_site_power_generation: Option<f64>,
    pub emission_factor_electricity_mix: Option<f64>,
    pub gas_supply: Option<f64>,
    pub purchase_of_biogas: Option<bool>,
    pub heating_oil: Option<f64>,
    pub sewage_gas_produced: Option<f64>,
    pub methane_fraction: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct SewageSludgeTreatment {
    pub digester_count: Option<u64>,
    pub sludge_bags_are_closed: Option<bool>,
    pub sludge_storage_containers_are_closed: Option<bool>,
    pub sewage_sludge_for_disposal: Option<f64>,
    pub transport_distance: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct SideStreamTreatment {
    pub total_nitrogen: Option<f64>,
}
