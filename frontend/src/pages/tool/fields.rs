use std::collections::HashMap;

use leptos::*;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use klick_application as app;
use klick_boundary::{
    AnnualAverages, EnergyConsumption, InputData, N2oEmissionFactorCalcMethod,
    N2oEmissionFactorScenario, OperatingMaterials, Scenario, SewageSludgeTreatment,
};

use crate::forms::FieldSignal;

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

#[allow(clippy::too_many_lines)]
pub fn load_fields(
    signals: &HashMap<FieldId, FieldSignal>,
    input: InputData,
    scenario: Scenario,
) -> anyhow::Result<()> {
    let app::InputData {
        plant_name,
        ew,
        abwasser,
        n_ges_zu,
        csb_ab,
        n_ges_ab,
        klaergas_gesamt,
        methangehalt,
        strombedarf,
        energie_eigen,
        ef_co2_strommix,
        schlammtaschen,
        schlammstapel,
        klaerschlamm_transport_km,
        klaerschlamm_entsorgung_m,
        betriebsstoffe_fe3,
        betriebsstoffe_feso4,
        betriebsstoffe_kalk,
        betriebsstoffe_poly,
    } = input.try_into()?;

    let Scenario {
        n2o_emission_factor,
    } = scenario;

    signals
        .get(&FieldId::Name)
        .and_then(FieldSignal::get_text_signal)
        .unwrap()
        .set(plant_name);
    signals
        .get(&FieldId::Ew)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(ew));
    signals
        .get(&FieldId::Flow)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(abwasser));
    signals
        .get(&FieldId::TknZu)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(n_ges_zu));
    signals
        .get(&FieldId::CsbAb)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(csb_ab));
    signals
        .get(&FieldId::TknAb)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(n_ges_ab));
    signals
        .get(&FieldId::Klaergas)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(klaergas_gesamt));
    signals
        .get(&FieldId::Methangehalt)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(methangehalt));
    signals
        .get(&FieldId::Strombedarf)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(strombedarf));
    signals
        .get(&FieldId::Eigenstrom)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(energie_eigen));
    signals
        .get(&FieldId::EfStrommix)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(ef_co2_strommix));
    signals
        .get(&FieldId::Schlammtaschen)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(schlammtaschen);
    signals
        .get(&FieldId::Schlammstapel)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(schlammstapel);
    signals
        .get(&FieldId::KlaerschlammTransport)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(klaerschlamm_transport_km));
    signals
        .get(&FieldId::KlaerschlammEnstorgung)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(klaerschlamm_entsorgung_m));
    signals
        .get(&FieldId::BetriebsstoffeFe3)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_fe3));
    signals
        .get(&FieldId::BetriebsstoffeFeso4)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_feso4));
    signals
        .get(&FieldId::BetriebsstoffeKalk)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_kalk));
    signals
        .get(&FieldId::BetriebsstoffePoly)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_poly));

    signals
        .get(&FieldId::CustomN2oScenarioSupport)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(matches!(
            n2o_emission_factor.calculation_method,
            N2oEmissionFactorCalcMethod::CustomFactor
        ));

    signals
        .get(&FieldId::CustomN2oScenarioValue)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(n2o_emission_factor.custom_factor);
    Ok(())
}
