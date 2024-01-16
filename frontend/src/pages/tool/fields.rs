use std::collections::HashMap;

use leptos::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_boundary::{
    AnnualAverage, EnergyConsumption, N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario,
    OperatingMaterials, OptimizationScenario, PlantProfile, Project, SavedProject,
    SewageSludgeTreatment, UnsavedProject,
};

use crate::forms::{self, format_f64_into_de_string, FieldSignal, MissingField};

pub type RequiredField = forms::RequiredField<FieldId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum FieldId {
    Name,
    Ew,
    Flow,
    CsbZu,
    TknZu,
    PZu,
    CsbAb,
    TknAb,
    PAb,
    Klaergas,
    Methangehalt,
    GasZusatz,
    Biogas,
    Strombedarf,
    Eigenstrom,
    EfStrommix,
    Schlammtaschen,
    Schlammstapel,
    KlaerschlammEnstorgung,
    KlaerschlammTransport,
    BetriebsstoffeFe3,
    BetriebsstoffeFeso4,
    BetriebsstoffeKalk,
    BetriebsstoffePoly,
    Scenario(ScenarioFieldId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum ScenarioFieldId {
    N2oCustomFactor,
    CH4ChpCalculationMethod,
    CH4ChpCustomFactor,
}

pub fn read_input_fields(
    s: &HashMap<FieldId, FieldSignal>,
    required_fields: &Vec<RequiredField>,
) -> (PlantProfile, Vec<MissingField>) {
    let missing_fields: Vec<MissingField> =
        required_fields.iter().fold(vec![], |mut acc, field| {
            if field.id == FieldId::Name {
                if s.get(&field.id).and_then(FieldSignal::get_text).is_none() {
                    let x = MissingField::new(field.field_id.clone(), field.label);
                    acc.push(x);
                    acc
                } else {
                    acc
                }
            } else if s.get(&field.id).and_then(FieldSignal::get_float).is_none() {
                let x = MissingField::new(field.field_id.clone(), field.label);
                acc.push(x);
                acc
            } else {
                acc
            }
        });

    let plant_name = s.get(&FieldId::Name).and_then(FieldSignal::get_text);
    let population_equivalent = s.get(&FieldId::Ew).and_then(FieldSignal::get_float);
    let wastewater = s.get(&FieldId::Flow).and_then(FieldSignal::get_float);

    let influent_average = AnnualAverage {
        nitrogen: s.get(&FieldId::TknZu).and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s.get(&FieldId::CsbZu).and_then(FieldSignal::get_float),
        phosphorus: s.get(&FieldId::PZu).and_then(FieldSignal::get_float),
    };
    let effluent_average = AnnualAverage {
        nitrogen: s.get(&FieldId::TknAb).and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s.get(&FieldId::CsbAb).and_then(FieldSignal::get_float),
        phosphorus: s.get(&FieldId::PAb).and_then(FieldSignal::get_float),
    };

    let energy_consumption = EnergyConsumption {
        sewage_gas_produced: s.get(&FieldId::Klaergas).and_then(FieldSignal::get_float),
        methane_fraction: s
            .get(&FieldId::Methangehalt)
            .and_then(FieldSignal::get_float),
        gas_supply: s.get(&FieldId::GasZusatz).and_then(FieldSignal::get_float),
        purchase_of_biogas: s.get(&FieldId::Biogas).and_then(FieldSignal::get_bool),
        total_power_consumption: s
            .get(&FieldId::Strombedarf)
            .and_then(FieldSignal::get_float),
        on_site_power_generation: s.get(&FieldId::Eigenstrom).and_then(FieldSignal::get_float),
        emission_factor_electricity_mix: s
            .get(&FieldId::EfStrommix)
            .and_then(FieldSignal::get_float),
    };

    let sewage_sludge_treatment = SewageSludgeTreatment {
        open_sludge_bags: s
            .get(&FieldId::Schlammtaschen)
            .and_then(FieldSignal::get_bool),
        open_sludge_storage_containers: s
            .get(&FieldId::Schlammstapel)
            .and_then(FieldSignal::get_bool),
        sewage_sludge_for_disposal: s
            .get(&FieldId::KlaerschlammEnstorgung)
            .and_then(FieldSignal::get_float),
        transport_distance: s
            .get(&FieldId::KlaerschlammTransport)
            .and_then(FieldSignal::get_float),
    };

    let operating_materials = OperatingMaterials {
        fecl3: s
            .get(&FieldId::BetriebsstoffeFe3)
            .and_then(FieldSignal::get_float),
        feclso4: s
            .get(&FieldId::BetriebsstoffeFeso4)
            .and_then(FieldSignal::get_float),
        caoh2: s
            .get(&FieldId::BetriebsstoffeKalk)
            .and_then(FieldSignal::get_float),
        synthetic_polymers: s
            .get(&FieldId::BetriebsstoffePoly)
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
            operating_materials,
        },
        missing_fields,
    )
}

pub fn read_scenario_fields(s: &HashMap<FieldId, FieldSignal>) -> OptimizationScenario {
    let custom_factor = s
        .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
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

#[allow(clippy::too_many_lines)]
pub fn load_project_fields(signals: &HashMap<FieldId, FieldSignal>, project: Project) {
    let (plant_profile, optimization_scenario) = match project {
        Project::Unsaved(UnsavedProject {
            plant_profile,
            optimization_scenario,
        }) => (plant_profile, optimization_scenario),
        Project::Saved(SavedProject {
            id: _,
            title: _,
            plant_profile,
            optimization_scenario,
        }) => (plant_profile, optimization_scenario),
    };

    let PlantProfile {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    } = plant_profile;

    let OptimizationScenario {
        n2o_emission_factor,
        ch4_chp_emission_factor: _,
    } = optimization_scenario;

    let AnnualAverage {
        nitrogen: nitrogen_influent,
        chemical_oxygen_demand: chemical_oxygen_demand_influent,
        phosphorus: phosphorus_influent,
    } = influent_average;

    let AnnualAverage {
        nitrogen: nitrogen_effluent,
        chemical_oxygen_demand: chemical_oxygen_demand_effluent,
        phosphorus: phosphorus_effluent,
    } = effluent_average;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
    } = energy_consumption;

    let SewageSludgeTreatment {
        open_sludge_bags,
        open_sludge_storage_containers,
        sewage_sludge_for_disposal,
        transport_distance,
    } = sewage_sludge_treatment;

    let OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    } = operating_materials;

    signals
        .get(&FieldId::Name)
        .and_then(FieldSignal::get_text_signal)
        .unwrap()
        .set(plant_name);
    signals
        .get(&FieldId::Ew)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(population_equivalent));
    signals
        .get(&FieldId::Flow)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(wastewater));
    signals
        .get(&FieldId::TknZu)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(nitrogen_influent));
    signals
        .get(&FieldId::CsbZu)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(chemical_oxygen_demand_influent));
    signals
        .get(&FieldId::PZu)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(phosphorus_influent));

    signals
        .get(&FieldId::TknAb)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(nitrogen_effluent));
    signals
        .get(&FieldId::CsbAb)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(chemical_oxygen_demand_effluent));
    signals
        .get(&FieldId::PAb)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(phosphorus_effluent));
    signals
        .get(&FieldId::Klaergas)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(sewage_gas_produced));
    signals
        .get(&FieldId::Methangehalt)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(methane_fraction));
    signals
        .get(&FieldId::Strombedarf)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(total_power_consumption));
    signals
        .get(&FieldId::Eigenstrom)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(on_site_power_generation));
    signals
        .get(&FieldId::EfStrommix)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(emission_factor_electricity_mix));
    signals
        .get(&FieldId::Biogas)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(purchase_of_biogas == Some(true));
    signals
        .get(&FieldId::GasZusatz)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(gas_supply));
    signals
        .get(&FieldId::Schlammtaschen)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(open_sludge_bags == Some(true));
    signals
        .get(&FieldId::Schlammstapel)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(open_sludge_storage_containers == Some(true));
    signals
        .get(&FieldId::KlaerschlammTransport)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(transport_distance));
    signals
        .get(&FieldId::KlaerschlammEnstorgung)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(sewage_sludge_for_disposal));
    signals
        .get(&FieldId::BetriebsstoffeFe3)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(fecl3));
    signals
        .get(&FieldId::BetriebsstoffeFeso4)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(feclso4));
    signals
        .get(&FieldId::BetriebsstoffeKalk)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(caoh2));
    signals
        .get(&FieldId::BetriebsstoffePoly)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(synthetic_polymers));
    signals
        .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(float_to_sting_option(n2o_emission_factor.custom_factor));
}
