use std::collections::HashMap;

use leptos::SignalSet;

use klick_application as app;
use klick_boundary::{import_from_slice, InputData, N2oEmissionFactorCalcMethod, Scenario};

use crate::forms::FieldSignal;

use super::fields::FieldId;

#[allow(clippy::too_many_lines)]
pub fn load_example_field_signal_values(signals: &HashMap<FieldId, FieldSignal>) {
    let (input, scenario) = example_input_data();

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
    } = input.try_into().unwrap();

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
}

const EXAMPLE_DATA: &[u8] = include_bytes!("example_data.json");

fn example_input_data() -> (InputData, Scenario) {
    // TODO: let csb_zu = 1045.0;
    // TODO: let p_zu = 9.9;
    // TODO: let p_ab = 0.4;
    // TODO: let gas_zusatz = 1_300_000.0;
    // TODO: let biogas = false;
    import_from_slice(EXAMPLE_DATA).unwrap()
}
