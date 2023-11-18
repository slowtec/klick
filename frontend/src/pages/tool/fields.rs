use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_boundary::{
    AnnualAverages, EnergyConsumption, InputData, N2oEmissionFactorCalcMethod,
    N2oEmissionFactorScenario, OperatingMaterials, Scenario, SewageSludgeTreatment,
};

use crate::forms::{self, FieldSignal};

type FieldSet = forms::FieldSet<FieldId>;

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
    N2oSzenario,
    CustomN2oScenarioSupport,
    CustomN2oScenarioValue,
}

const JSON_FIELD_SETS: &[u8] = include_bytes!("form_field_sets.json");

pub fn field_sets() -> Vec<FieldSet> {
    serde_json::from_slice(JSON_FIELD_SETS).unwrap()
}

pub fn read_input_fields(s: &HashMap<FieldId, FieldSignal>) -> InputData {
    let plant_name = s.get(&FieldId::Name).and_then(FieldSignal::get_text);
    let population_values = s.get(&FieldId::Ew).and_then(FieldSignal::get_float);
    let waste_water = s.get(&FieldId::Flow).and_then(FieldSignal::get_float);

    let inflow_averages = AnnualAverages {
        nitrogen: s.get(&FieldId::TknZu).and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s.get(&FieldId::CsbZu).and_then(FieldSignal::get_float),
        phosphorus: s.get(&FieldId::PZu).and_then(FieldSignal::get_float),
    };
    let effluent_averages = AnnualAverages {
        nitrogen: s.get(&FieldId::TknAb).and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s.get(&FieldId::CsbAb).and_then(FieldSignal::get_float),
        phosphorus: s.get(&FieldId::PAb).and_then(FieldSignal::get_float),
    };

    let energy_consumption = EnergyConsumption {
        sewage_gas_produced: s.get(&FieldId::Klaergas).and_then(FieldSignal::get_float),
        methane_level: s
            .get(&FieldId::Methangehalt)
            .and_then(FieldSignal::get_float),
        gas_supply: s.get(&FieldId::GasZusatz).and_then(FieldSignal::get_float),
        purchase_of_biogas: s.get(&FieldId::Biogas).and_then(FieldSignal::get_bool),
        total_power_consumption: s
            .get(&FieldId::Strombedarf)
            .and_then(FieldSignal::get_float),
        in_house_power_generation: s.get(&FieldId::Eigenstrom).and_then(FieldSignal::get_float),
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

    InputData {
        plant_name,
        population_values,
        waste_water,
        inflow_averages,
        effluent_averages,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    }
}

pub fn read_scenario_fields(s: &HashMap<FieldId, FieldSignal>) -> Scenario {
    let custom_factor = s
        .get(&FieldId::CustomN2oScenarioValue)
        .and_then(FieldSignal::get_float);
    let calculation_method = N2oEmissionFactorCalcMethod::Ipcc2019; // TODO: read from signal
    Scenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            calculation_method,
            custom_factor,
        },
    }
}
