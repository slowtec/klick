use leptos::*;
use strum::IntoEnumIterator;
use thiserror::Error;

use klick_application::{N2OSzenario, ValueId};

use crate::{
    forms::{self, FieldSignal, FieldType},
    sankey,
};

type Field = forms::Field<ValueId>;
type FieldSet = forms::FieldSet<ValueId>;

#[component]
pub fn Home() -> impl IntoView {
    let name = "Lingen".to_string();
    let ew = 120_000.0;
    let flow = 5_000_000.0;
    let csb_zu = 1045.0;
    let tkn_zu = 122.0;
    let p_zu = 9.9;
    let csb_ab = 129.0;
    let tkn_ab = 11.76;
    let p_ab = 0.4;
    let klaergas = 1_260_000.0;
    let methangehalt = 23.0;
    let gas_zusatz = 1_300_000.0;
    let biogas = false;
    let strombedarf = 2_683_259.0;
    let eigenstrom = 2_250_897.0;
    let ef_strommix = 468.0;
    let schlammtaschen = true;
    let schlammstapel = true;
    let klaerschlamm_enstorgung = 3687.6;
    let klaerschlamm_transport = 47.0;
    let betriebsstoffe_fe3 = 0.0;
    let betriebsstoffe_feso4 = 326_000.0;
    let betriebsstoffe_kalk = 326_260.0;
    let betriebsstoffe_poly = 23_620.0;
    let n2o_szenario = 3;

    let field_sets = vec![
        FieldSet {
            title: "Angaben zur Kläranlage",
            fields: vec![
                Field {
                    id: ValueId::Name,
                    label: "Name oder Ort",
                    description: None,
                    required: true,
                    field_type: FieldType::Text {
                        initial_value: Some(name),
                        max_len: None,
                        placeholder: Some("Name der Kläranlage"),
                    },
                },
                Field {
                    id: ValueId::Ew,
                    label: "Ausbaugröße",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(ew),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Ausbaugröße [EW]"),
                        unit: "EW",
                    },
                },
                Field {
                    id: ValueId::Flow,
                    label: "Abwassermenge",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(flow),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Abwassermenge"),
                        unit: "m³/a",
                    },
                },
            ],
        },
        FieldSet {
            title: "Zulauf-Parameter (Jahresmittelwerte)",
            fields: vec![
                Field {
                    id: ValueId::CsbZu,
                    label: "Chemischer Sauerstoffbedarf",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(csb_zu),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("CSB"),
                        unit: "mg/L",
                    },
                },
                Field {
                    id: ValueId::TknZu,
                    label: "Gesamtstickstoff",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(tkn_zu),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("TKN"),
                        unit: "mg/L",
                    },
                },
                Field {
                    id: ValueId::PZu,
                    label: "Phosphor",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(p_zu),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("P"),
                        unit: "mg/L",
                    },
                },
            ],
        },
        FieldSet {
            title: "Ablauf-Parameter (Jahresmittelwerte)",
            fields: vec![
                Field {
                    id: ValueId::CsbAb,
                    label: "Chemischer Sauerstoffbedarf",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(csb_ab),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("CSB"),
                        unit: "mg/L",
                    },
                },
                Field {
                    id: ValueId::TknAb,
                    label: "Gesamtstickstoff",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(tkn_ab),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("TKN"),
                        unit: "mg/L",
                    },
                },
                Field {
                    id: ValueId::PAb,
                    label: "Phosphor",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(p_ab),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("P"),
                        unit: "mg/L",
                    },
                },
            ],
        },
        FieldSet {
            title: "Energiebedarf",
            fields: vec![
                Field {
                    id: ValueId::Klaergas,
                    label: "Erzeugtes Klärgas",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(klaergas),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Klärgas"),
                        unit: "m³",
                    },
                },
                Field {
                    id: ValueId::Methangehalt,
                    label: "Methangehalt",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(methangehalt),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("65"),
                        unit: "%",
                    },
                },
                Field {
                    id: ValueId::GasZusatz,
                    label: "Gasbezug (Versorger)",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(gas_zusatz),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Gasbezug"),
                        unit: "kWh/a",
                    },
                },
                Field {
                    label: "Bezug von Biogas",
                    id: ValueId::Biogas,
                    description: Some("ja/nein"),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: Some(biogas),
                    },
                },
                Field {
                    id: ValueId::Strombedarf,
                    label: "Strombedarf gesamt",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(strombedarf),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Gesamtstrombedarf"),
                        unit: "kWh/a",
                    },
                },
                Field {
                    id: ValueId::Eigenstrom,
                    label: "Eigenstromerzeugung",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(eigenstrom),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Eigenstrom"),
                        unit: "kWh/a",
                    },
                },
                Field {
                    id: ValueId::EfStrommix,
                    label: "Emissionsfaktor Strommix (Versorger)",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(ef_strommix),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("485"),
                        unit: "g CO₂/kWh",
                    },
                },
            ],
        },
        FieldSet {
            title: "Klärschlammbehandlung",
            fields: vec![
                Field {
                    label: "Offene Schlammtaschen",
                    id: ValueId::Schlammtaschen,
                    description: Some("ja/nein"),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: Some(schlammtaschen),
                    },
                },
                Field {
                    label: "Offene Schlammstapelbehälter",
                    id: ValueId::Schlammstapel,
                    description: Some("ja/nein"),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: Some(schlammstapel),
                    },
                },
                Field {
                    id: ValueId::KlaerschlammEnstorgung,
                    label: "Kläraschlamm zur Entsorgung",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(klaerschlamm_enstorgung),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Masse entwässert"),
                        unit: "t",
                    },
                },
                Field {
                    id: ValueId::KlaerschlammTransport,
                    label: "Transportdistanz",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: Some(klaerschlamm_transport),
                        min_value: None,
                        max_value: None,
                        placeholder: Some("Entfernung"),
                        unit: "km",
                    },
                },
            ],
        },
        FieldSet {
            title: "Eingesetzte Betriebsstoffe",
            fields: vec![
                Field {
                    label: "Eisen(III)-chlorid-Lösung",
                    description: None,
                    id: ValueId::BetriebsstoffeFe3,
                    required: true,
                    field_type: FieldType::Float {
                        unit: "kg",
                        placeholder: Some("kg Lösung"),
                        initial_value: Some(betriebsstoffe_fe3),
                        min_value: None,
                        max_value: None,
                    },
                },
                Field {
                    label: "Eisenchloridsulfat-Lösung",
                    description: None,
                    id: ValueId::BetriebsstoffeFeso4,
                    required: true,
                    field_type: FieldType::Float {
                        unit: "kg",
                        placeholder: Some("kg Lösung"),
                        initial_value: Some(betriebsstoffe_feso4),
                        min_value: None,
                        max_value: None,
                    },
                },
                Field {
                    label: "Kalkhydrat",
                    description: None,
                    id: ValueId::BetriebsstoffeKalk,
                    required: true,
                    field_type: FieldType::Float {
                        unit: "kg",
                        placeholder: Some("kg Branntkalk"),
                        initial_value: Some(betriebsstoffe_kalk),
                        min_value: None,
                        max_value: None,
                    },
                },
                Field {
                    label: "Synthetische Polymere",
                    description: None,
                    id: ValueId::BetriebsstoffePoly,
                    required: true,
                    field_type: FieldType::Float {
                        placeholder: Some("kg Polymere"),
                        unit: "kg",
                        initial_value: Some(betriebsstoffe_poly),
                        min_value: None,
                        max_value: None,
                    },
                },
            ],
        },
        FieldSet {
            title: "Schätzung des N₂O-Emissionsfaktors",
            fields: vec![Field {
                description: None,
                label: "Szenario",
                id: ValueId::N2oSzenario,
                required: true,
                field_type: FieldType::Selection {
                    initial_value: Some(n2o_szenario),
                    options: vec![
                        forms::SelectOption {
                            value: n2o_szenario_to_usize(N2OSzenario::ExtrapolatedParravicini),
                            label: "Extrapoliert nach Parravicini et al. 2016",
                        },
                        forms::SelectOption {
                            value: n2o_szenario_to_usize(N2OSzenario::Optimistic),
                            label: "Optimistisch",
                        },
                        forms::SelectOption {
                            value: n2o_szenario_to_usize(N2OSzenario::Pesimistic),
                            label: "Pessimistisch",
                        },
                        forms::SelectOption {
                            value: n2o_szenario_to_usize(N2OSzenario::Ipcc2019),
                            label: "Nach IPCC 2019",
                        },
                    ],
                },
            }],
        },
    ];

    let (signals, set_views) = forms::render_field_sets(field_sets);

    let name_ka = *signals.get(&ValueId::Name).unwrap();
    let ew = *signals.get(&ValueId::Ew).unwrap();

    let render = create_action(move |output_data: &klick_application::OutputData| {
        let output_data = output_data.clone();
        let name_ka = name_ka
            .get_text()
            .unwrap_or_else(|| "Kläranlage".to_string());
        let ew = ew.get_float().unwrap_or_default();
        async move {
            sankey::render(&name_ka, ew, output_data, "chart");
        }
    });

    let s = signals.clone();
    create_effect(move |_| {
        let Some(ew) = s.get(&ValueId::Ew).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(abwasser) = s.get(&ValueId::Flow).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(n_ges_zu) = s.get(&ValueId::TknZu).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(csb_ab) = s.get(&ValueId::CsbAb).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(n_ges_ab) = s.get(&ValueId::TknAb).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(klaergas_gesamt) = s.get(&ValueId::Klaergas).and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(methangehalt) = s
            .get(&ValueId::Methangehalt)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(strombedarf) = s
            .get(&ValueId::Strombedarf)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(energie_eigen) = s.get(&ValueId::Eigenstrom).and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(ef_co2_strommix) = s.get(&ValueId::EfStrommix).and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(schlammtaschen) = s
            .get(&ValueId::Schlammtaschen)
            .and_then(FieldSignal::get_bool)
        else {
            return;
        };
        let Some(schlammstapel) = s
            .get(&ValueId::Schlammstapel)
            .and_then(FieldSignal::get_bool)
        else {
            return;
        };
        let Some(klaerschlamm_transport_km) = s
            .get(&ValueId::KlaerschlammTransport)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(klaerschlamm_entsorgung_m) = s
            .get(&ValueId::KlaerschlammEnstorgung)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_fe3) = s
            .get(&ValueId::BetriebsstoffeFe3)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_feso4) = s
            .get(&ValueId::BetriebsstoffeFeso4)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_kalk) = s
            .get(&ValueId::BetriebsstoffeKalk)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_poly) = s
            .get(&ValueId::BetriebsstoffePoly)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(n2o_szenario) = s
            .get(&ValueId::N2oSzenario)
            .and_then(FieldSignal::get_selection)
        else {
            return;
        };

        let n2o_szenario = try_n2o_szenario_from_usize(n2o_szenario).unwrap();

        let mut input_data = klick_application::InputData {
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
        };

        log::debug!("Calculating with {input_data:#?}");
        let output_data = klick_application::calc(&input_data);
        log::debug!("Result is {output_data:#?}");
        render.dispatch(output_data);

        // Also calculate the other szenarios
        let _szenario_calculations = N2OSzenario::iter()
            .map(|szenario| {
                input_data.n2o_szenario = szenario;
                let output_data = klick_application::calc(&input_data);
                (szenario, output_data)
            })
            .collect::<Vec<_>>();

        // TODO: visualize
    });

    view! {
      <div class="space-y-12">
        { set_views }
      </div>
      <div id="chart" class="mt-8"></div>
    }
}

fn n2o_szenario_to_usize(szenario: N2OSzenario) -> usize {
    match szenario {
        N2OSzenario::ExtrapolatedParravicini => 0,
        N2OSzenario::Optimistic => 1,
        N2OSzenario::Pesimistic => 2,
        N2OSzenario::Ipcc2019 => 3,
    }
}

#[derive(Debug, Error)]
#[error("Invalid N2O szenario")]
struct InvalidN2OSzenario;

fn try_n2o_szenario_from_usize(szenario: usize) -> Result<N2OSzenario, InvalidN2OSzenario> {
    let szenario = match szenario {
        0 => N2OSzenario::ExtrapolatedParravicini,
        1 => N2OSzenario::Optimistic,
        2 => N2OSzenario::Pesimistic,
        3 => N2OSzenario::Ipcc2019,
        _ => return Err(InvalidN2OSzenario),
    };
    Ok(szenario)
}
