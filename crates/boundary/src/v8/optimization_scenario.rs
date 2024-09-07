use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct OptimizationScenario {
    pub sewage_sludge_treatment: SewageSludgeTreatmentScenario,
    pub energy_emissions: EnergyEmissionScenario,
    pub side_stream_treatment: SideStreamTreatmentScenario,
}

#[derive(Deserialize, Default)]
pub struct SewageSludgeTreatmentScenario {
    pub sludge_bags_are_closed: Option<bool>,
    pub sludge_storage_containers_are_closed: Option<bool>,
}

#[derive(Deserialize, Default)]
pub struct EnergyEmissionScenario {
    pub process_energy_savings: Option<f64>,
    pub fossil_energy_savings: Option<f64>,
    pub photovoltaic_energy_expansion: Option<f64>,
    pub estimated_self_photovoltaic_usage: Option<f64>,
    pub wind_energy_expansion: Option<f64>,
    pub estimated_self_wind_energy_usage: Option<f64>,
    pub water_energy_expansion: Option<f64>,
    pub estimated_self_water_energy_usage: Option<f64>,
    pub district_heating: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct SideStreamTreatmentScenario {
    pub side_stream_cover_is_closed: Option<bool>,
}
