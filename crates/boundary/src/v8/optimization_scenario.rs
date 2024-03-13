use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct OptimizationScenario {
    pub sewage_sludge_treatment: SewageSludgeTreatmentScenario,
    pub energy_emissions: EnergyEmissionScenario,
    pub side_stream_treatment: SideStreamTreatmentScenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct SewageSludgeTreatmentScenario {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_bags_are_closed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_storage_containers_are_closed: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct EnergyEmissionScenario {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_energy_savings: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fossil_energy_savings: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub photovoltaic_energy_expansion: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_self_photovoltaic_usage: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_energy_expansion: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_self_wind_energy_usage: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_energy_expansion: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_self_water_energy_usage: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_heating: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct SideStreamTreatmentScenario {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_stream_cover_is_closed: Option<bool>,
}
