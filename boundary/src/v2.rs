use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub(crate) struct Import {
    pub(crate) input: InputData,
    pub(crate) scenario: Scenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct InputData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plant_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_equivalent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wastewater: Option<f64>,

    pub influent_average: AnnualAverage,

    pub effluent_average: AnnualAverage,

    pub energy_consumption: EnergyConsumption,

    pub sewage_sludge_treatment: SewageSludgeTreatment,

    pub operating_materials: OperatingMaterials,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct AnnualAverage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitrogen: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemical_oxygen_demand: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phosphorus: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct EnergyConsumption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_gas_produced: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub methane_fraction: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_supply: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_of_biogas: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_power_consumption: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_site_power_generation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_factor_electricity_mix: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct SewageSludgeTreatment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_sludge_bags: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_sludge_storage_containers: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_sludge_for_disposal: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct OperatingMaterials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecl3: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub feclso4: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caoh2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthetic_polymers: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct Scenario {
    pub n2o_emission_factor: N2oEmissionFactorScenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct N2oEmissionFactorScenario {
    pub calculation_method: N2oEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
    feature = "extra-derive",
    derive(Debug, Default, Clone, Copy, PartialEq, Eq)
)]
pub enum N2oEmissionFactorCalcMethod {
    #[cfg_attr(feature = "extra-derive", default)]
    ExtrapolatedParravicini,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    CustomFactor,
}
