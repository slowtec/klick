use std::collections::HashMap;

use derive_more::From;
use serde::Deserialize;
use time::OffsetDateTime;

use klick_domain::{self as domain, Value};

pub use crate::v7::ProjectId;

mod optimization_scenario;
mod plant_profile;
mod sensitivity_parameters;

pub use self::{optimization_scenario::*, plant_profile::*, sensitivity_parameters::*};

#[derive(Deserialize)]
pub struct Data {
    pub project: Project,
}

#[derive(Deserialize, From)]
#[serde(untagged)]
pub enum Project {
    Saved(SavedProject),
    Unsaved(JsonFormData),
}

impl From<FormData> for Project {
    fn from(from: FormData) -> Self {
        Self::Unsaved(from.into())
    }
}

#[derive(Deserialize)]
pub struct SavedProject {
    pub id: ProjectId,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    #[serde(flatten)]
    pub data: JsonFormData,
}

pub type FormData = HashMap<domain::InputValueId, Value>;

impl From<JsonFormData> for HashMap<domain::InputValueId, Value> {
    fn from(from: JsonFormData) -> Self {
        use domain::{InputValueId as Id, Value as V};

        let values = [
            (Id::ProjectName, from.project_title.map(Value::Text)),
            (
                Id::ProfilePlantName,
                from.plant_profile.plant_name.map(Value::Text),
            ),
            (
                Id::ProfilePopulationEquivalent,
                from.plant_profile
                    .population_equivalent
                    .map(|v| v as u64)
                    .map(V::count),
            ),
            (
                Id::ProfileWastewater,
                from.plant_profile.wastewater.map(Value::qubicmeters),
            ),
            (
                Id::ProfileInfluentNitrogen,
                from.plant_profile
                    .influent_average
                    .total_nitrogen
                    .map(V::milligrams_per_liter),
            ),
            (
                Id::ProfileInfluentChemicalOxygenDemand,
                from.plant_profile
                    .influent_average
                    .chemical_oxygen_demand
                    .map(V::milligrams_per_liter),
            ),
            (
                Id::ProfileInfluentTotalOrganicCarbohydrates,
                from.plant_profile
                    .influent_average
                    .total_organic_carbohydrates
                    .map(V::milligrams_per_liter),
            ),
            (
                Id::ProfileEffluentNitrogen,
                from.plant_profile
                    .effluent_average
                    .total_nitrogen
                    .map(V::milligrams_per_liter),
            ),
            (
                Id::ProfileEffluentChemicalOxygenDemand,
                from.plant_profile
                    .effluent_average
                    .chemical_oxygen_demand
                    .map(V::milligrams_per_liter),
            ),
            (
                Id::ProfileSideStreamTreatmentTotalNitrogen,
                from.plant_profile
                    .side_stream_treatment
                    .total_nitrogen
                    .map(V::tons),
            ),
            (
                Id::ProfileOperatingMaterialFeCl3,
                from.plant_profile.operating_materials.fecl3.map(V::tons),
            ),
            (
                Id::ProfileOperatingMaterialFeClSO4,
                from.plant_profile.operating_materials.feclso4.map(V::tons),
            ),
            (
                Id::ProfileOperatingMaterialCaOH2,
                from.plant_profile.operating_materials.caoh2.map(V::tons),
            ),
            (
                Id::ProfileOperatingMaterialSyntheticPolymers,
                from.plant_profile
                    .operating_materials
                    .synthetic_polymers
                    .map(V::tons),
            ),
            (
                Id::SensitivityN2OCalculationMethod,
                from.sensitivity_parameters
                    .n2o_emissions
                    .calculation_method
                    .map(Into::into)
                    .map(V::n2o_emission_factor_calc_method),
            ),
            (
                Id::SensitivityN2OCustomFactor,
                from.sensitivity_parameters
                    .n2o_emissions
                    .custom_emission_factor
                    .map(V::percent),
            ),
            (
                Id::SensitivityN2OSideStreamFactor,
                from.sensitivity_parameters
                    .n2o_emissions
                    .side_stream_emission_factor
                    .map(V::percent),
            ),
            (
                Id::SensitivityCH4ChpCalculationMethod,
                from.sensitivity_parameters
                    .ch4_chp_emissions
                    .calculation_method
                    .map(Into::into)
                    .map(V::ch4_chp_emission_factor_calc_method),
            ),
            (
                Id::SensitivityCH4ChpCustomFactor,
                from.sensitivity_parameters
                    .ch4_chp_emissions
                    .custom_emission_factor
                    .map(V::percent),
            ),
            (
                Id::SensitivityCO2FossilCustomFactor,
                from.sensitivity_parameters
                    .co2_fossil_emissions
                    .emission_factor
                    .map(V::percent),
            ),
            (
                Id::SensitivitySludgeBagsCustomFactor,
                from.sensitivity_parameters
                    .ch4_sewage_sludge_emissions
                    .emission_factor_sludge_bags
                    .map(V::qubicmeters_per_hour),
            ),
            (
                Id::SensitivitySludgeStorageCustomFactor,
                from.sensitivity_parameters
                    .ch4_sewage_sludge_emissions
                    .emission_factor_sludge_storage_containers
                    .map(V::percent),
            ),
            (
                Id::ProfileSludgeTreatmentBagsAreOpen,
                from.plant_profile
                    .sewage_sludge_treatment
                    .sludge_bags_are_closed
                    .map(|v| !v) // closed => open
                    .map(V::bool),
            ),
            (
                Id::ProfileSludgeTreatmentStorageContainersAreOpen,
                from.plant_profile
                    .sewage_sludge_treatment
                    .sludge_storage_containers_are_closed
                    .map(|v| !v) // closed => open
                    .map(V::bool),
            ),
            (
                Id::ProfileSludgeTreatmentDisposal,
                from.plant_profile
                    .sewage_sludge_treatment
                    .sewage_sludge_for_disposal
                    .map(V::tons),
            ),
            (
                Id::ProfileSludgeTreatmentTransportDistance,
                from.plant_profile
                    .sewage_sludge_treatment
                    .transport_distance
                    .map(V::kilometers),
            ),
            (
                Id::ProfileSludgeTreatmentDigesterCount,
                from.plant_profile
                    .sewage_sludge_treatment
                    .digester_count
                    .map(V::count),
            ),
            (
                Id::ProfileSewageGasProduced,
                from.plant_profile
                    .energy_consumption
                    .sewage_gas_produced
                    .map(V::qubicmeters),
            ),
            (
                Id::ProfileMethaneFraction,
                from.plant_profile
                    .energy_consumption
                    .methane_fraction
                    .map(V::percent),
            ),
            (
                Id::ProfileGasSupply,
                from.plant_profile
                    .energy_consumption
                    .gas_supply
                    .map(V::qubicmeters),
            ),
            (
                Id::ProfilePurchaseOfBiogas,
                from.plant_profile
                    .energy_consumption
                    .purchase_of_biogas
                    .map(V::bool),
            ),
            (
                Id::ProfileTotalPowerConsumption,
                from.plant_profile
                    .energy_consumption
                    .total_power_consumption
                    .map(V::kilowatthours),
            ),
            (
                Id::ProfileOnSitePowerGeneration,
                from.plant_profile
                    .energy_consumption
                    .on_site_power_generation
                    .map(V::kilowatthours),
            ),
            (
                Id::ProfileEmissionFactorElectricityMix,
                from.plant_profile
                    .energy_consumption
                    .emission_factor_electricity_mix
                    .map(V::grams_per_kilowatthour),
            ),
            (
                Id::ProfileHeatingOil,
                from.plant_profile
                    .energy_consumption
                    .heating_oil
                    .map(V::liters),
            ),
            (
                Id::RecommendationSludgeBagsAreOpen,
                from.optimization_scenario
                    .sewage_sludge_treatment
                    .sludge_bags_are_closed
                    .map(|v| !v) // closed => open
                    .map(V::bool),
            ),
            (
                Id::RecommendationSludgeStorageContainersAreOpen,
                from.optimization_scenario
                    .sewage_sludge_treatment
                    .sludge_storage_containers_are_closed
                    .map(|v| !v) // closed => open
                    .map(V::bool),
            ),
            (
                Id::RecommendationN2OSideStreamCoverIsOpen,
                from.optimization_scenario
                    .side_stream_treatment
                    .side_stream_cover_is_closed
                    .map(|v| !v)
                    .map(V::bool),
            ),
            (
                Id::RecommendationProcessEnergySaving,
                from.optimization_scenario
                    .energy_emissions
                    .process_energy_savings
                    .map(V::percent),
            ),
            (
                Id::RecommendationFossilEnergySaving,
                from.optimization_scenario
                    .energy_emissions
                    .fossil_energy_savings
                    .map(V::percent),
            ),
            (
                Id::RecommendationPhotovoltaicEnergyExpansion,
                from.optimization_scenario
                    .energy_emissions
                    .photovoltaic_energy_expansion
                    .map(V::kilowatthours),
            ),
            (
                Id::RecommendationEstimatedSelfPhotovolaticUsage,
                from.optimization_scenario
                    .energy_emissions
                    .estimated_self_photovoltaic_usage
                    .map(V::percent),
            ),
            (
                Id::RecommendationWindEnergyExpansion,
                from.optimization_scenario
                    .energy_emissions
                    .wind_energy_expansion
                    .map(V::kilowatthours),
            ),
            (
                Id::RecommendationEstimatedSelfWindEnergyUsage,
                from.optimization_scenario
                    .energy_emissions
                    .estimated_self_wind_energy_usage
                    .map(V::percent),
            ),
            (
                Id::RecommendationWaterEnergyExpansion,
                from.optimization_scenario
                    .energy_emissions
                    .water_energy_expansion
                    .map(V::kilowatthours),
            ),
            (
                Id::RecommendationEstimatedSelfWaterEnergyUsage,
                from.optimization_scenario
                    .energy_emissions
                    .estimated_self_water_energy_usage
                    .map(V::percent),
            ),
            (
                Id::RecommendationDistrictHeating,
                from.optimization_scenario
                    .energy_emissions
                    .district_heating
                    .map(V::kilowatthours),
            ),
        ];
        values
            .into_iter()
            .filter_map(|(id, value)| value.map(|v| (id, v)))
            .collect()
    }
}

impl From<HashMap<domain::InputValueId, Value>> for JsonFormData {
    fn from(from: HashMap<domain::InputValueId, Value>) -> Self {
        let mut data = JsonFormData::default();
        for (id, value) in from {
            data.set(id, Some(value));
        }
        data
    }
}

#[derive(Deserialize, Default)]
pub struct JsonFormData {
    pub(crate) project_title: Option<String>,
    // First page in the tool frontend
    pub(crate) plant_profile: PlantProfile,
    // Second page in the tool frontend
    pub(crate) sensitivity_parameters: SensitivityParameters,
    // Third page in the tool frontend
    pub(crate) optimization_scenario: OptimizationScenario,
}

impl JsonFormData {
    fn set(&mut self, id: domain::InputValueId, value: Option<Value>) {
        use domain::{InputValueId as Id, Value as V};

        debug_assert!(value
            .as_ref()
            .map_or(true, |v| v.value_type() == id.value_type()));

        match id {
            Id::ProjectName => {
                self.project_title = value.map(Value::as_text_unchecked);
            }
            Id::ProfilePlantName => {
                self.plant_profile.plant_name = value.map(Value::as_text_unchecked);
            }
            Id::ProfilePopulationEquivalent => {
                self.plant_profile.population_equivalent = value
                    .map(V::as_count_unchecked)
                    .map(|v| u64::from(v) as f64);
            }
            Id::ProfileWastewater => {
                self.plant_profile.wastewater =
                    value.map(V::as_qubicmeters_unchecked).map(Into::into);
            }
            Id::ProfileInfluentNitrogen => {
                self.plant_profile.influent_average.total_nitrogen = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::ProfileInfluentChemicalOxygenDemand => {
                self.plant_profile.influent_average.chemical_oxygen_demand = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::ProfileInfluentTotalOrganicCarbohydrates => {
                self.plant_profile
                    .influent_average
                    .total_organic_carbohydrates = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::ProfileEffluentNitrogen => {
                self.plant_profile.effluent_average.total_nitrogen = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::ProfileEffluentChemicalOxygenDemand => {
                self.plant_profile.effluent_average.chemical_oxygen_demand = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::ProfileSideStreamTreatmentTotalNitrogen => {
                self.plant_profile.side_stream_treatment.total_nitrogen =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::ProfileOperatingMaterialFeCl3 => {
                self.plant_profile.operating_materials.fecl3 =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::ProfileOperatingMaterialFeClSO4 => {
                self.plant_profile.operating_materials.feclso4 =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::ProfileOperatingMaterialCaOH2 => {
                self.plant_profile.operating_materials.caoh2 =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::ProfileOperatingMaterialSyntheticPolymers => {
                self.plant_profile.operating_materials.synthetic_polymers =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::SensitivityN2OCalculationMethod => {
                self.sensitivity_parameters.n2o_emissions.calculation_method = value
                    .map(V::as_n2o_emission_factor_calc_method_unchecked)
                    .map(Into::into);
            }
            Id::SensitivityN2OCustomFactor => {
                self.sensitivity_parameters
                    .n2o_emissions
                    .custom_emission_factor = value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::SensitivityN2OSideStreamFactor => {
                self.sensitivity_parameters
                    .n2o_emissions
                    .side_stream_emission_factor =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::SensitivityCH4ChpCalculationMethod => {
                self.sensitivity_parameters
                    .ch4_chp_emissions
                    .calculation_method = value
                    .map(V::as_ch4_chp_emission_factor_calc_method_unchecked)
                    .map(Into::into);
            }
            Id::SensitivityCH4ChpCustomFactor => {
                self.sensitivity_parameters
                    .ch4_chp_emissions
                    .custom_emission_factor = value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::SensitivityCO2FossilCustomFactor => {
                self.sensitivity_parameters
                    .co2_fossil_emissions
                    .emission_factor = value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::SensitivitySludgeBagsCustomFactor => {
                self.sensitivity_parameters
                    .ch4_sewage_sludge_emissions
                    .emission_factor_sludge_bags = value
                    .map(V::as_qubicmeters_per_hour_unchecked)
                    .map(Into::into);
            }
            Id::SensitivitySludgeStorageCustomFactor => {
                self.sensitivity_parameters
                    .ch4_sewage_sludge_emissions
                    .emission_factor_sludge_storage_containers =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::ProfileSludgeTreatmentBagsAreOpen => {
                let closed = value.map(V::as_bool_unchecked).map(|v| !v); // open => closed
                self.plant_profile
                    .sewage_sludge_treatment
                    .sludge_bags_are_closed = closed;
            }
            Id::ProfileSludgeTreatmentStorageContainersAreOpen => {
                let closed = value.map(V::as_bool_unchecked).map(|v| !v); // open => closed
                self.plant_profile
                    .sewage_sludge_treatment
                    .sludge_storage_containers_are_closed = closed;
            }
            Id::ProfileSludgeTreatmentDisposal => {
                self.plant_profile
                    .sewage_sludge_treatment
                    .sewage_sludge_for_disposal = value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::ProfileSludgeTreatmentTransportDistance => {
                self.plant_profile
                    .sewage_sludge_treatment
                    .transport_distance = value.map(V::as_kilometers_unchecked).map(Into::into);
            }
            Id::ProfileSludgeTreatmentDigesterCount => {
                self.plant_profile.sewage_sludge_treatment.digester_count =
                    value.map(V::as_count_unchecked).map(Into::into);
            }
            Id::ProfileSewageGasProduced => {
                self.plant_profile.energy_consumption.sewage_gas_produced =
                    value.map(V::as_qubicmeters_unchecked).map(Into::into);
            }
            Id::ProfileMethaneFraction => {
                self.plant_profile.energy_consumption.methane_fraction =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::ProfileGasSupply => {
                self.plant_profile.energy_consumption.gas_supply =
                    value.map(V::as_qubicmeters_unchecked).map(Into::into);
            }
            Id::ProfilePurchaseOfBiogas => {
                self.plant_profile.energy_consumption.purchase_of_biogas =
                    value.map(V::as_bool_unchecked);
            }
            Id::ProfileTotalPowerConsumption => {
                self.plant_profile
                    .energy_consumption
                    .total_power_consumption =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::ProfileOnSitePowerGeneration => {
                self.plant_profile
                    .energy_consumption
                    .on_site_power_generation =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::ProfileEmissionFactorElectricityMix => {
                self.plant_profile
                    .energy_consumption
                    .emission_factor_electricity_mix = value
                    .map(V::as_grams_per_kilowatthour_unchecked)
                    .map(Into::into);
            }
            Id::ProfileHeatingOil => {
                self.plant_profile.energy_consumption.heating_oil =
                    value.map(V::as_liters_unchecked).map(Into::into);
            }
            Id::RecommendationSludgeBagsAreOpen => {
                self.optimization_scenario
                    .sewage_sludge_treatment
                    .sludge_bags_are_closed = value.map(V::as_bool_unchecked).map(|v| !v);
                // open => closed
            }
            Id::RecommendationSludgeStorageContainersAreOpen => {
                self.optimization_scenario
                    .sewage_sludge_treatment
                    .sludge_storage_containers_are_closed =
                    value.map(V::as_bool_unchecked).map(|v| !v); // open => closed
            }
            Id::RecommendationN2OSideStreamCoverIsOpen => {
                self.optimization_scenario
                    .side_stream_treatment
                    .side_stream_cover_is_closed = value.map(V::as_bool_unchecked).map(|v| !v);
                // open => closed
            }
            Id::RecommendationProcessEnergySaving => {
                self.optimization_scenario
                    .energy_emissions
                    .process_energy_savings = value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::RecommendationFossilEnergySaving => {
                self.optimization_scenario
                    .energy_emissions
                    .fossil_energy_savings = value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::RecommendationPhotovoltaicEnergyExpansion => {
                self.optimization_scenario
                    .energy_emissions
                    .photovoltaic_energy_expansion =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::RecommendationEstimatedSelfPhotovolaticUsage => {
                self.optimization_scenario
                    .energy_emissions
                    .estimated_self_photovoltaic_usage =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::RecommendationWindEnergyExpansion => {
                self.optimization_scenario
                    .energy_emissions
                    .wind_energy_expansion =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::RecommendationEstimatedSelfWindEnergyUsage => {
                self.optimization_scenario
                    .energy_emissions
                    .estimated_self_wind_energy_usage =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::RecommendationWaterEnergyExpansion => {
                self.optimization_scenario
                    .energy_emissions
                    .water_energy_expansion =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::RecommendationEstimatedSelfWaterEnergyUsage => {
                self.optimization_scenario
                    .energy_emissions
                    .estimated_self_water_energy_usage =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::RecommendationDistrictHeating => {
                self.optimization_scenario.energy_emissions.district_heating =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::SensitivityAdditionalCustomEmissions => {
                // FIXME
            }
        }
    }
}
