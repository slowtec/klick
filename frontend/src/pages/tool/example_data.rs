use std::collections::HashMap;

use klick_application::{InputData, N2OSzenario, ValueId};
use leptos::*;

use crate::{forms::FieldSignal, pages::tool::util};

pub fn load_example_field_signal_values(signals: &HashMap<ValueId, FieldSignal>) {
    let InputData {
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
        n2o_szenario,
    } = example_input_data();

    let name = "Muster Klärwerk".to_string();

    signals
        .get(&ValueId::Name)
        .and_then(FieldSignal::get_text_signal)
        .unwrap()
        .set(Some(name));
    signals
        .get(&ValueId::Ew)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(ew));
    signals
        .get(&ValueId::Flow)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(abwasser));
    signals
        .get(&ValueId::TknZu)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(n_ges_zu));
    signals
        .get(&ValueId::CsbAb)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(csb_ab));
    signals
        .get(&ValueId::TknAb)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(n_ges_ab));
    signals
        .get(&ValueId::Klaergas)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(klaergas_gesamt));
    signals
        .get(&ValueId::Methangehalt)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(methangehalt));
    signals
        .get(&ValueId::Strombedarf)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(strombedarf));
    signals
        .get(&ValueId::Eigenstrom)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(energie_eigen));
    signals
        .get(&ValueId::EfStrommix)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(ef_co2_strommix));
    signals
        .get(&ValueId::Schlammtaschen)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(schlammtaschen);
    signals
        .get(&ValueId::Schlammstapel)
        .and_then(FieldSignal::get_bool_signal)
        .unwrap()
        .set(schlammstapel);
    signals
        .get(&ValueId::KlaerschlammTransport)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(klaerschlamm_transport_km));
    signals
        .get(&ValueId::KlaerschlammEnstorgung)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(klaerschlamm_entsorgung_m));
    signals
        .get(&ValueId::BetriebsstoffeFe3)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_fe3));
    signals
        .get(&ValueId::BetriebsstoffeFeso4)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_feso4));
    signals
        .get(&ValueId::BetriebsstoffeKalk)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_kalk));
    signals
        .get(&ValueId::BetriebsstoffePoly)
        .and_then(FieldSignal::get_float_signal)
        .unwrap()
        .set(Some(betriebsstoffe_poly));
    signals
        .get(&ValueId::N2oSzenario)
        .and_then(FieldSignal::get_selection_signal)
        .unwrap()
        .set(Some(util::n2o_szenario_to_usize(n2o_szenario)));
}

fn example_input_data() -> InputData {
    let ew = 120_000.0;
    let abwasser = 5_000_000.0;
    // TODO: let csb_zu = 1045.0;
    let n_ges_zu = 122.0;
    // TODO: let p_zu = 9.9;
    let csb_ab = 129.0;
    let n_ges_ab = 11.76;
    // TODO: let p_ab = 0.4;
    let klaergas_gesamt = 1_260_000.0;
    let methangehalt = 23.0;
    // TODO: let gas_zusatz = 1_300_000.0;
    // TODO: let biogas = false;
    let strombedarf = 2_683_259.0;
    let energie_eigen = 2_250_897.0;
    let ef_co2_strommix = 468.0;
    let schlammtaschen = true;
    let schlammstapel = true;
    let klaerschlamm_entsorgung_m = 3687.6;
    let klaerschlamm_transport_km = 47.0;
    let betriebsstoffe_fe3 = 0.0;
    let betriebsstoffe_feso4 = 326_000.0;
    let betriebsstoffe_kalk = 326_260.0;
    let betriebsstoffe_poly = 23_620.0;
    let n2o_szenario = N2OSzenario::Ipcc2019;

    InputData {
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
        n2o_szenario,
    }
}
