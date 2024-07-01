use derive_more::From;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use klick_domain::{self as domain, Value};

pub use crate::v7::ProjectId;

mod optimization_scenario;
mod plant_profile;
mod sensitivity_parameters;

pub use self::{optimization_scenario::*, plant_profile::*, sensitivity_parameters::*};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct Data {
    pub project: Project,
}

#[derive(Serialize, Deserialize, From)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
#[serde(untagged)]
pub enum Project {
    Saved(SavedProject),
    Unsaved(FormData),
}

impl Project {
    #[must_use]
    pub const fn form_data(&self) -> &FormData {
        match self {
            Self::Saved(SavedProject { data, .. }) | Self::Unsaved(data) => data,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct SavedProject {
    pub id: ProjectId,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    #[serde(flatten)]
    pub data: FormData,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct FormData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) project_title: Option<String>,
    // First page in the tool frontend
    pub(crate) plant_profile: PlantProfile,
    // Second page in the tool frontend
    pub(crate) sensitivity_parameters: SensitivityParameters,
    // Third page in the tool frontend
    pub(crate) optimization_scenario: OptimizationScenario,
}

// NOTE:
// In the future, we want to use a HashMap,
// which is why we are first implementing manual access via variable IDs.
impl FormData {
    // TODO: return reference: Option<&Value>
    pub fn get(&self, id: &domain::InputValueId) -> Option<Value> {
        use domain::{InputValueId as Id, Value as V};

        let value = match id {
            Id::ProjectName => self.project_title.clone().map(Value::Text),
            Id::PlantName => self.plant_profile.plant_name.clone().map(Value::Text),
            Id::PopulationEquivalent => self
                .plant_profile
                .population_equivalent
                .map(|v| v as u64)
                .map(V::count),
            Id::Wastewater => self.plant_profile.wastewater.map(Value::qubicmeters),
            Id::InfluentNitrogen => self
                .plant_profile
                .influent_average
                .total_nitrogen
                .map(V::milligrams_per_liter),
            Id::InfluentChemicalOxygenDemand => self
                .plant_profile
                .influent_average
                .chemical_oxygen_demand
                .map(V::milligrams_per_liter),
            Id::InfluentTotalOrganicCarbohydrates => self
                .plant_profile
                .influent_average
                .total_organic_carbohydrates
                .map(V::milligrams_per_liter),
            Id::EffluentNitrogen => self
                .plant_profile
                .effluent_average
                .total_nitrogen
                .map(V::milligrams_per_liter),
            Id::EffluentChemicalOxygenDemand => self
                .plant_profile
                .effluent_average
                .chemical_oxygen_demand
                .map(V::milligrams_per_liter),
            Id::SideStreamTreatmentTotalNitrogen => self
                .plant_profile
                .side_stream_treatment
                .total_nitrogen
                .map(V::tons),
            Id::OperatingMaterialFeCl3 => self.plant_profile.operating_materials.fecl3.map(V::tons),
            Id::OperatingMaterialFeClSO4 => {
                self.plant_profile.operating_materials.feclso4.map(V::tons)
            }
            Id::OperatingMaterialCaOH2 => self.plant_profile.operating_materials.caoh2.map(V::tons),
            Id::OperatingMaterialSyntheticPolymers => self
                .plant_profile
                .operating_materials
                .synthetic_polymers
                .map(V::tons),
            Id::SensitivityN2OCalculationMethod => self
                .sensitivity_parameters
                .n2o_emissions
                .calculation_method
                .map(Into::into)
                .map(V::n2o_emission_factor_calc_method),
            Id::SensitivityN2OCustomFactor => self
                .sensitivity_parameters
                .n2o_emissions
                .custom_emission_factor
                .map(V::percent),
            Id::SensitivityN2OSideStreamFactor => self
                .sensitivity_parameters
                .n2o_emissions
                .side_stream_emission_factor
                .map(V::percent),
            Id::SensitivityCH4ChpCalculationMethod => self
                .sensitivity_parameters
                .ch4_chp_emissions
                .calculation_method
                .map(Into::into)
                .map(V::ch4_chp_emission_factor_calc_method),
            Id::SensitivityCH4ChpCustomFactor => self
                .sensitivity_parameters
                .ch4_chp_emissions
                .custom_emission_factor
                .map(V::percent),
            Id::SensitivityCO2FossilCustomFactor => self
                .sensitivity_parameters
                .co2_fossil_emissions
                .emission_factor
                .map(V::percent),
            Id::SensitivitySludgeBagsCustomFactor => self
                .sensitivity_parameters
                .ch4_sewage_sludge_emissions
                .emission_factor_sludge_bags
                .map(V::qubicmeters_per_hour),
            Id::SensitivitySludgeStorageCustomFactor => self
                .sensitivity_parameters
                .ch4_sewage_sludge_emissions
                .emission_factor_sludge_storage_containers
                .map(V::percent),
            Id::SludgeTreatmentBagsAreOpen => self
                .plant_profile
                .sewage_sludge_treatment
                .sludge_bags_are_closed
                .map(|v| !v) // closed => open
                .map(V::bool),
            Id::SludgeTreatmentStorageContainersAreOpen => self
                .plant_profile
                .sewage_sludge_treatment
                .sludge_storage_containers_are_closed
                .map(|v| !v) // closed => open
                .map(V::bool),
            Id::SludgeTreatmentDisposal => self
                .plant_profile
                .sewage_sludge_treatment
                .sewage_sludge_for_disposal
                .map(V::tons),
            Id::SludgeTreatmentTransportDistance => self
                .plant_profile
                .sewage_sludge_treatment
                .transport_distance
                .map(V::kilometers),
            Id::SludgeTreatmentDigesterCount => self
                .plant_profile
                .sewage_sludge_treatment
                .digester_count
                .map(V::count),
            Id::SewageGasProduced => self
                .plant_profile
                .energy_consumption
                .sewage_gas_produced
                .map(V::qubicmeters),
            Id::MethaneFraction => self
                .plant_profile
                .energy_consumption
                .methane_fraction
                .map(V::percent),
            Id::GasSupply => self
                .plant_profile
                .energy_consumption
                .gas_supply
                .map(V::qubicmeters),
            Id::PurchaseOfBiogas => self
                .plant_profile
                .energy_consumption
                .purchase_of_biogas
                .map(V::bool),
            Id::TotalPowerConsumption => self
                .plant_profile
                .energy_consumption
                .total_power_consumption
                .map(V::kilowatthours),
            Id::OnSitePowerGeneration => self
                .plant_profile
                .energy_consumption
                .on_site_power_generation
                .map(V::kilowatthours),
            Id::EmissionFactorElectricityMix => self
                .plant_profile
                .energy_consumption
                .emission_factor_electricity_mix
                .map(V::grams_per_kilowatthour),
            Id::HeatingOil => self
                .plant_profile
                .energy_consumption
                .heating_oil
                .map(V::liters),
            Id::ScenarioSludgeBagsAreOpen => self
                .optimization_scenario
                .sewage_sludge_treatment
                .sludge_bags_are_closed
                .map(|v| !v) // closed => open
                .map(V::bool),
            Id::ScenarioSludgeStorageContainersAreOpen => self
                .optimization_scenario
                .sewage_sludge_treatment
                .sludge_storage_containers_are_closed
                .map(|v| !v) // closed => open
                .map(V::bool),
            Id::ScenarioN2OSideStreamFactor => self
                .sensitivity_parameters
                .n2o_emissions
                .side_stream_emission_factor
                .map(V::factor),
            Id::ScenarioN2OSideStreamCoverIsOpen => self
                .optimization_scenario
                .side_stream_treatment
                .side_stream_cover_is_closed
                .map(|v| !v)
                .map(V::bool),
            Id::ScenarioProcessEnergySaving => self
                .optimization_scenario
                .energy_emissions
                .process_energy_savings
                .map(V::percent),
            Id::ScenarioFossilEnergySaving => self
                .optimization_scenario
                .energy_emissions
                .fossil_energy_savings
                .map(V::percent),
            Id::ScenarioPhotovoltaicEnergyExpansion => self
                .optimization_scenario
                .energy_emissions
                .photovoltaic_energy_expansion
                .map(V::kilowatthours),
            Id::ScenarioEstimatedSelfPhotovolaticUsage => self
                .optimization_scenario
                .energy_emissions
                .estimated_self_photovoltaic_usage
                .map(V::percent),
            Id::ScenarioWindEnergyExpansion => self
                .optimization_scenario
                .energy_emissions
                .wind_energy_expansion
                .map(V::kilowatthours),
            Id::ScenarioEstimatedSelfWindEnergyUsage => self
                .optimization_scenario
                .energy_emissions
                .estimated_self_wind_energy_usage
                .map(V::percent),
            Id::ScenarioWaterEnergyExpansion => self
                .optimization_scenario
                .energy_emissions
                .water_energy_expansion
                .map(V::kilowatthours),
            Id::ScenarioEstimatedSelfWaterEnergyUsage => self
                .optimization_scenario
                .energy_emissions
                .estimated_self_water_energy_usage
                .map(V::percent),
            Id::ScenarioDistrictHeating => self
                .optimization_scenario
                .energy_emissions
                .district_heating
                .map(V::kilowatthours),
        };
        debug_assert!(value
            .as_ref()
            .map(|v| v.value_type() == domain::value_spec(id).value_type())
            .unwrap_or(true));
        value
    }

    // TODO:
    // Use value spec to validate the input
    // and return a result with
    // enum Error {
    //    InvalidType,
    //    OutOfRange,
    //    Immutable, // e.g. constants
    // }
    pub fn set(&mut self, id: domain::InputValueId, value: Option<Value>) {
        use domain::{InputValueId as Id, Value as V};

        log::debug!("Set {id:?}: {value:?}");
        debug_assert!(value
            .as_ref()
            .map(|v| v.value_type() == domain::value_spec(&id).value_type())
            .unwrap_or(true));

        match id {
            Id::ProjectName => {
                self.project_title = value.map(Value::as_text_unchecked);
            }
            Id::PlantName => {
                self.plant_profile.plant_name = value.map(Value::as_text_unchecked);
            }
            Id::PopulationEquivalent => {
                self.plant_profile.population_equivalent = value
                    .map(V::as_count_unchecked)
                    .map(|v| u64::from(v) as f64);
            }
            Id::Wastewater => {
                self.plant_profile.wastewater =
                    value.map(V::as_qubicmeters_unchecked).map(Into::into);
            }
            Id::InfluentNitrogen => {
                self.plant_profile.influent_average.total_nitrogen = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::InfluentChemicalOxygenDemand => {
                self.plant_profile.influent_average.chemical_oxygen_demand = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::InfluentTotalOrganicCarbohydrates => {
                self.plant_profile
                    .influent_average
                    .total_organic_carbohydrates = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::EffluentNitrogen => {
                self.plant_profile.effluent_average.total_nitrogen = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::EffluentChemicalOxygenDemand => {
                self.plant_profile.effluent_average.chemical_oxygen_demand = value
                    .map(V::as_milligrams_per_liter_unchecked)
                    .map(Into::into);
            }
            Id::SideStreamTreatmentTotalNitrogen => {
                self.plant_profile.side_stream_treatment.total_nitrogen =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::OperatingMaterialFeCl3 => {
                self.plant_profile.operating_materials.fecl3 =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::OperatingMaterialFeClSO4 => {
                self.plant_profile.operating_materials.feclso4 =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::OperatingMaterialCaOH2 => {
                self.plant_profile.operating_materials.caoh2 =
                    value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::OperatingMaterialSyntheticPolymers => {
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
            Id::SludgeTreatmentBagsAreOpen => {
                let closed = value.map(V::as_bool_unchecked).map(|v| !v); // open => closed
                self.plant_profile
                    .sewage_sludge_treatment
                    .sludge_bags_are_closed = closed;
            }
            Id::SludgeTreatmentStorageContainersAreOpen => {
                let closed = value.map(V::as_bool_unchecked).map(|v| !v); // open => closed
                self.plant_profile
                    .sewage_sludge_treatment
                    .sludge_storage_containers_are_closed = closed;
            }
            Id::SludgeTreatmentDisposal => {
                self.plant_profile
                    .sewage_sludge_treatment
                    .sewage_sludge_for_disposal = value.map(V::as_tons_unchecked).map(Into::into);
            }
            Id::SludgeTreatmentTransportDistance => {
                self.plant_profile
                    .sewage_sludge_treatment
                    .transport_distance = value.map(V::as_kilometers_unchecked).map(Into::into);
            }
            Id::SludgeTreatmentDigesterCount => {
                self.plant_profile.sewage_sludge_treatment.digester_count =
                    value.map(V::as_count_unchecked).map(Into::into);
            }
            Id::SewageGasProduced => {
                self.plant_profile.energy_consumption.sewage_gas_produced =
                    value.map(V::as_qubicmeters_unchecked).map(Into::into);
            }
            Id::MethaneFraction => {
                self.plant_profile.energy_consumption.methane_fraction =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::GasSupply => {
                self.plant_profile.energy_consumption.gas_supply =
                    value.map(V::as_qubicmeters_unchecked).map(Into::into);
            }
            Id::PurchaseOfBiogas => {
                self.plant_profile.energy_consumption.purchase_of_biogas =
                    value.map(V::as_bool_unchecked);
            }
            Id::TotalPowerConsumption => {
                self.plant_profile
                    .energy_consumption
                    .total_power_consumption =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::OnSitePowerGeneration => {
                self.plant_profile
                    .energy_consumption
                    .on_site_power_generation =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::EmissionFactorElectricityMix => {
                self.plant_profile
                    .energy_consumption
                    .emission_factor_electricity_mix = value
                    .map(V::as_grams_per_kilowatthour_unchecked)
                    .map(Into::into);
            }
            Id::HeatingOil => {
                self.plant_profile.energy_consumption.heating_oil =
                    value.map(V::as_liters_unchecked).map(Into::into);
            }
            Id::ScenarioSludgeBagsAreOpen => {
                self.optimization_scenario
                    .sewage_sludge_treatment
                    .sludge_bags_are_closed = value.map(V::as_bool_unchecked).map(|v| !v);
                // open => closed
            }
            Id::ScenarioSludgeStorageContainersAreOpen => {
                self.optimization_scenario
                    .sewage_sludge_treatment
                    .sludge_storage_containers_are_closed =
                    value.map(V::as_bool_unchecked).map(|v| !v); // open => closed
            }
            Id::ScenarioN2OSideStreamFactor => {
                self.sensitivity_parameters
                    .n2o_emissions
                    .side_stream_emission_factor =
                    value.map(V::as_factor_unchecked).map(Into::into);
            }
            Id::ScenarioN2OSideStreamCoverIsOpen => {
                self.optimization_scenario
                    .side_stream_treatment
                    .side_stream_cover_is_closed = value.map(V::as_bool_unchecked).map(|v| !v);
                // open => closed
            }
            Id::ScenarioProcessEnergySaving => {
                self.optimization_scenario
                    .energy_emissions
                    .process_energy_savings = value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::ScenarioFossilEnergySaving => {
                self.optimization_scenario
                    .energy_emissions
                    .fossil_energy_savings = value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::ScenarioPhotovoltaicEnergyExpansion => {
                self.optimization_scenario
                    .energy_emissions
                    .photovoltaic_energy_expansion =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::ScenarioEstimatedSelfPhotovolaticUsage => {
                self.optimization_scenario
                    .energy_emissions
                    .estimated_self_photovoltaic_usage =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::ScenarioWindEnergyExpansion => {
                self.optimization_scenario
                    .energy_emissions
                    .wind_energy_expansion =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::ScenarioEstimatedSelfWindEnergyUsage => {
                self.optimization_scenario
                    .energy_emissions
                    .estimated_self_wind_energy_usage =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::ScenarioWaterEnergyExpansion => {
                self.optimization_scenario
                    .energy_emissions
                    .water_energy_expansion =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
            Id::ScenarioEstimatedSelfWaterEnergyUsage => {
                self.optimization_scenario
                    .energy_emissions
                    .estimated_self_water_energy_usage =
                    value.map(V::as_percent_unchecked).map(Into::into);
            }
            Id::ScenarioDistrictHeating => {
                self.optimization_scenario.energy_emissions.district_heating =
                    value.map(V::as_kilowatthours_unchecked).map(Into::into);
            }
        }
    }
}
