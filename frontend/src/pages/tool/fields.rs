use std::collections::HashMap;

use derive_more::From;
use leptos::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_boundary::{
    AnnualAverageInfluent, AnnualAverageEffluent, EnergyConsumption, N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario,
    OperatingMaterials, OptimizationScenario, PlantProfile, Project, ProjectData, SavedProject,
    SewageSludgeTreatment, SideStreamTreatment
};
use klick_presenter::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, SewageSludgeTreatmentId, ValueLabel, SideStreamTreatmentId,
};

use crate::forms::{self, format_f64_into_de_string, FieldSignal, MissingField};

pub type RequiredField = forms::RequiredField<FieldId>;
pub type FieldSet = forms::FieldSet<FieldId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize, From)]
pub enum FieldId {
    ProjectName,
    Profile(ProfileValueId),
    Scenario(ScenarioFieldId),
    Ch4EmissionsOpenDigesters(
        crate::pages::tool::sensitivity_options::ch4_emissions_open_digesters::Id, // FIXME remove this and make it a global definition in here
    ),
}

impl ValueLabel for FieldId {
    fn label(&self) -> &str {
        match self {
            Self::ProjectName => "Projektname",
            Self::Profile(id) => id.label(),
            Self::Scenario(id) => id.label(),
            Self::Ch4EmissionsOpenDigesters(id) => id.label(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum ScenarioFieldId {
    N2OCustomFactor,
    N2OSideStreamFactor,
    N2OSideStreamCoverIsOpen,
    CH4ChpCalculationMethod,
    CH4ChpCustomFactor,
    CO2FossilCustomFactor,
}

impl ValueLabel for ScenarioFieldId {
    fn label(&self) -> &str {
        match self {
            Self::N2OCustomFactor => "N₂O-EF Benutzerdefiniert",
            Self::N2OSideStreamFactor => "N₂O-EF Nebenstrom",
            Self::N2OSideStreamCoverIsOpen => "Abdeckung mit Abluftbehandlung Nebenstromanlage",
            Self::CH4ChpCalculationMethod => "BHKW Emmisionsfaktor",
            Self::CH4ChpCustomFactor => "BHKW CH₄-EF benutzerdefiniert",
            Self::CO2FossilCustomFactor => "CO₂-EF (fossil)",
        }
    }
}

pub fn read_all_project_fields(signals: &HashMap<FieldId, FieldSignal>) -> ProjectData {
    let (plant_profile, _) = read_input_fields(&signals, &vec![]);
    let optimization_scenario = read_scenario_fields(&signals);
    let title = read_title(&signals);
    ProjectData {
        title,
        plant_profile,
        optimization_scenario,
    }
}

pub fn read_title(s: &HashMap<FieldId, FieldSignal>) -> Option<String> {
    s.get(&FieldId::ProjectName).and_then(FieldSignal::get_text)
}

pub fn read_input_fields(
    s: &HashMap<FieldId, FieldSignal>,
    required_fields: &Vec<RequiredField>,
) -> (PlantProfile, Vec<MissingField<FieldId>>) {
    let missing_fields = required_fields
        .iter()
        .filter_map(|field| {
            let Some(field_signal) = s.get(&field.id) else {
                return None;
            };
            let is_missing = match field_signal {
                FieldSignal::Float { output, .. } => output.get().is_none(),
                FieldSignal::UnsignedInteger { output, .. } => output.get().is_none(),
                FieldSignal::Text(signal) => signal.get().is_none(),
                FieldSignal::Bool(signal) => !signal.get(),
                FieldSignal::Selection(signal) => signal.get().is_none(),
            };
            if is_missing {
                Some(MissingField::new(field.id, field.field_id.clone()))
            } else {
                None
            }
        })
        .collect();

    let plant_name = s
        .get(&ProfileValueId::PlantName.into())
        .and_then(FieldSignal::get_text);
    let population_equivalent = s
        .get(&ProfileValueId::PopulationEquivalent.into())
        .and_then(FieldSignal::get_float);
    let wastewater = s
        .get(&ProfileValueId::Wastewater.into())
        .and_then(FieldSignal::get_float);

    let influent_average = AnnualAverageInfluent {
        nitrogen: s
            .get(&ProfileValueId::from(AnnualAverageInfluentId::Nitrogen).into())
            .and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s
            .get(&ProfileValueId::from(AnnualAverageInfluentId::ChemicalOxygenDemand).into())
            .and_then(FieldSignal::get_float),
        total_organic_carbohydrates: s
            .get(&ProfileValueId::from(AnnualAverageInfluentId::TotalOrganicCarbohydrates).into())
            .and_then(FieldSignal::get_float),
    };

    let effluent_average = AnnualAverageEffluent {
        nitrogen: s
            .get(&ProfileValueId::from(AnnualAverageEffluentId::Nitrogen).into())
            .and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s
            .get(&ProfileValueId::from(AnnualAverageEffluentId::ChemicalOxygenDemand).into())
            .and_then(FieldSignal::get_float),
    };

    let energy_consumption = EnergyConsumption {
        sewage_gas_produced: s
            .get(&ProfileValueId::from(EnergyConsumptionId::SewageGasProduced).into())
            .and_then(FieldSignal::get_float),
        methane_fraction: s
            .get(&ProfileValueId::from(EnergyConsumptionId::MethaneFraction).into())
            .and_then(FieldSignal::get_float),
        gas_supply: s
            .get(&ProfileValueId::from(EnergyConsumptionId::GasSupply).into())
            .and_then(FieldSignal::get_float),
        purchase_of_biogas: s
            .get(&ProfileValueId::from(EnergyConsumptionId::PurchaseOfBiogas).into())
            .and_then(FieldSignal::get_bool),
        total_power_consumption: s
            .get(&ProfileValueId::from(EnergyConsumptionId::TotalPowerConsumption).into())
            .and_then(FieldSignal::get_float),
        on_site_power_generation: s
            .get(&ProfileValueId::from(EnergyConsumptionId::OnSitePowerGeneration).into())
            .and_then(FieldSignal::get_float),
        emission_factor_electricity_mix: s
            .get(&ProfileValueId::from(EnergyConsumptionId::EmissionFactorElectricityMix).into())
            .and_then(FieldSignal::get_float),
        heating_oil: s
            .get(&ProfileValueId::from(EnergyConsumptionId::HeatingOil).into())
            .and_then(FieldSignal::get_float),
    };

    let sewage_sludge_treatment = SewageSludgeTreatment {
        sludge_bags_are_open: Some(true),
        sludge_bags_are_open_recommendation: Some(true),
        custom_sludge_bags_factor: None, // FIXME no value parsing here?
        sludge_storage_containers_are_open: Some(true),
        sludge_storage_containers_are_open_recommendation: Some(true),
        custom_sludge_storage_containers_factor: None,
        sewage_sludge_for_disposal: s
            .get(&ProfileValueId::from(SewageSludgeTreatmentId::SewageSludgeForDisposal).into())
            .and_then(FieldSignal::get_float),
        transport_distance: s
            .get(&ProfileValueId::from(SewageSludgeTreatmentId::TransportDistance).into())
            .and_then(FieldSignal::get_float),
        digester_count: s
            .get(&ProfileValueId::from(SewageSludgeTreatmentId::DigesterCount).into())
            .and_then(FieldSignal::get_unsigned_integer),
    };

    let side_stream_treatment = SideStreamTreatment {
        total_nitrogen: s
            .get(&ProfileValueId::from(SideStreamTreatmentId::TotalNitrogen).into())
            .and_then(FieldSignal::get_float),
    };

    let operating_materials = OperatingMaterials {
        fecl3: s
            .get(&ProfileValueId::from(OperatingMaterialId::FeCl3).into())
            .and_then(FieldSignal::get_float),
        feclso4: s
            .get(&ProfileValueId::from(OperatingMaterialId::FeClSO4).into())
            .and_then(FieldSignal::get_float),
        caoh2: s
            .get(&ProfileValueId::from(OperatingMaterialId::CaOH2).into())
            .and_then(FieldSignal::get_float),
        synthetic_polymers: s
            .get(&ProfileValueId::from(OperatingMaterialId::SyntheticPolymers).into())
            .and_then(FieldSignal::get_float),
    };

    (
        PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            side_stream_treatment,
            operating_materials,
        },
        missing_fields,
    )
}

pub fn read_scenario_fields(s: &HashMap<FieldId, FieldSignal>) -> OptimizationScenario {
    let custom_factor = s
        .get(&FieldId::Scenario(ScenarioFieldId::N2OCustomFactor))
        .and_then(FieldSignal::get_float);

    let calculation_method = N2oEmissionFactorCalcMethod::Ipcc2019; // TODO: read from signal
    let n2o_emission_factor = N2oEmissionFactorScenario {
        calculation_method,
        custom_factor,
    };

    let _custom_factor = s
        .get(&FieldId::Scenario(ScenarioFieldId::CH4ChpCustomFactor))
        .and_then(FieldSignal::get_float);

    // TODO:
    let ch4_chp_emission_factor = None;

    OptimizationScenario {
        n2o_emission_factor,
        ch4_chp_emission_factor,
    }
}

fn float_to_sting_option(f: Option<f64>) -> Option<String> {
    f.map(format_f64_into_de_string)
}

fn unsigned_integer_to_sting_option(f: Option<u64>) -> Option<String> {
    f.map(|x| x.to_string())
}

#[allow(clippy::too_many_lines)]
pub fn load_project_fields(signals: &HashMap<FieldId, FieldSignal>, project: Project) {
    let (title, plant_profile, optimization_scenario) = match project {
        Project::Unsaved(ProjectData {
            title,
            plant_profile,
            optimization_scenario,
        }) => (title, plant_profile, optimization_scenario),
        Project::Saved(SavedProject {
            id: _,
            created_at: _,
            modified_at: _,
            data:
                ProjectData {
                    title,
                    plant_profile,
                    optimization_scenario,
                },
        }) => (title, plant_profile, optimization_scenario),
    };

    let PlantProfile {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        side_stream_treatment,
        operating_materials,
    } = plant_profile;

    let OptimizationScenario {
        n2o_emission_factor: _,
        ch4_chp_emission_factor: _,
    } = optimization_scenario;

    let AnnualAverageInfluent {
        nitrogen: nitrogen_influent,
        chemical_oxygen_demand: chemical_oxygen_demand_influent,
        total_organic_carbohydrates,
    } = influent_average;

    let AnnualAverageEffluent {
        nitrogen: nitrogen_effluent,
        chemical_oxygen_demand: chemical_oxygen_demand_effluent,
    } = effluent_average;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
        heating_oil,
    } = energy_consumption;

    let SewageSludgeTreatment {
        sludge_bags_are_open: _,
        sludge_bags_are_open_recommendation: _,
        custom_sludge_bags_factor: _,
        sludge_storage_containers_are_open: _,
        sludge_storage_containers_are_open_recommendation: _,
        custom_sludge_storage_containers_factor: _,
        sewage_sludge_for_disposal,
        transport_distance,
        digester_count,
    } = sewage_sludge_treatment;

    let SideStreamTreatment {
        total_nitrogen,
    } = side_stream_treatment;

    let OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    } = operating_materials;

    signals
        .get(&FieldId::ProjectName)
        .and_then(FieldSignal::get_text_signal)
        .unwrap()
        .set(title);
    signals
        .get(&ProfileValueId::PlantName.into())
        .and_then(FieldSignal::get_text_signal)
        .unwrap()
        .set(plant_name);
    signals
        .get(&ProfileValueId::PopulationEquivalent.into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(population_equivalent));
    signals
        .get(&ProfileValueId::Wastewater.into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(wastewater));
    signals
        .get(&ProfileValueId::from(AnnualAverageInfluentId::Nitrogen).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(nitrogen_influent));
    signals
        .get(&ProfileValueId::from(AnnualAverageInfluentId::ChemicalOxygenDemand).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(chemical_oxygen_demand_influent));
    signals
        .get(&ProfileValueId::from(AnnualAverageInfluentId::TotalOrganicCarbohydrates).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(total_organic_carbohydrates));
    signals
        .get(&ProfileValueId::from(AnnualAverageEffluentId::Nitrogen).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(nitrogen_effluent));
    signals
        .get(&ProfileValueId::from(AnnualAverageEffluentId::ChemicalOxygenDemand).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(chemical_oxygen_demand_effluent));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::SewageGasProduced).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(sewage_gas_produced));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::MethaneFraction).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(methane_fraction));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::TotalPowerConsumption).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(total_power_consumption));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::OnSitePowerGeneration).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(on_site_power_generation));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::EmissionFactorElectricityMix).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(emission_factor_electricity_mix));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::HeatingOil).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(heating_oil));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::PurchaseOfBiogas).into())
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(purchase_of_biogas == Some(true));
    signals
        .get(&ProfileValueId::from(EnergyConsumptionId::GasSupply).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(gas_supply));
    signals
        .get(&ProfileValueId::from(SewageSludgeTreatmentId::TransportDistance).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(transport_distance));
    signals
        .get(&ProfileValueId::from(SewageSludgeTreatmentId::SewageSludgeForDisposal).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(sewage_sludge_for_disposal));
    signals
        .get(&ProfileValueId::from(SewageSludgeTreatmentId::DigesterCount).into())
        .and_then(FieldSignal::get_unsigned_integer_signal)
        .unwrap()
        .set(unsigned_integer_to_sting_option(digester_count));
    signals
        .get(&ProfileValueId::from(OperatingMaterialId::FeCl3).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(fecl3));
    signals
        .get(&ProfileValueId::from(OperatingMaterialId::FeClSO4).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(feclso4));
    signals
        .get(&ProfileValueId::from(OperatingMaterialId::CaOH2).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(caoh2));
    signals
        .get(&ProfileValueId::from(OperatingMaterialId::SyntheticPolymers).into())
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(synthetic_polymers));
}
