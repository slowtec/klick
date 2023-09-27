use charming::{
    component::Title,
    element::{Emphasis, EmphasisFocus},
    series::Sankey,
    Chart,
};
use leptos::*;

use crate::forms::{self, FieldSignal};

type Field = forms::Field<Id>;
type FieldBase = forms::FieldBase<Id>;
type FieldSet = forms::FieldSet<Id>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Id {
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
}

impl Id {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Ew => "ew",
            Self::Flow => "flow",
            Self::CsbZu => "csb_zu",
            Self::TknZu => "tkn_zu",
            Self::PZu => "p_zu",
            Self::CsbAb => "csb_ab",
            Self::TknAb => "tkn_ab",
            Self::PAb => "p_ab",
            Self::Klaergas => "klaergas",
            Self::Methangehalt => "methangehalt",
            Self::GasZusatz => "gas_zusatz",
            Self::Biogas => "biogas",
            Self::Strombedarf => "strombedarf",
            Self::Eigenstrom => "eigenstrom",
            Self::EfStrommix => "ef_strommix",
            Self::Schlammtaschen => "schlammtaschen",
            Self::Schlammstapel => "schlammstapel",
            Self::KlaerschlammEnstorgung => "klaerschlamm_enstorgung",
            Self::KlaerschlammTransport => "klaerschlamm_transport",
            Self::BetriebsstoffeFe3 => "betriebsstoffe_fe3",
            Self::BetriebsstoffeFeso4 => "betriebsstoffe_feso4",
            Self::BetriebsstoffeKalk => "betriebsstoffe_kalk",
            Self::BetriebsstoffePoly => "betriebsstoffe_poly",
            Self::N2oSzenario => "n2o_szenario",
        }
    }
}

impl From<Id> for &'static str {
    fn from(from: Id) -> Self {
        from.as_str()
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let name = "Lingen".to_string();
    let ew = 120000.0;
    let flow = 5000000.0;
    let csb_zu = 1045.0;
    let tkn_zu = 122.0;
    let p_zu = 9.9;
    let csb_ab = 129.0;
    let tkn_ab = 11.76;
    let p_ab = 0.4;
    let klaergas = 1260000.0;
    let methangehalt = 23.0;
    let gas_zusatz = 1300000.0;
    let biogas = false;
    let strombedarf = 2683259.0;
    let eigenstrom = 2250897.0;
    let ef_strommix = 468.0;
    let schlammtaschen = true;
    let schlammstapel = true;
    let klaerschlamm_enstorgung = 3687.6;
    let klaerschlamm_transport = 47.0;
    let betriebsstoffe_fe3 = 0.0;
    let betriebsstoffe_feso4 = 326000.0;
    let betriebsstoffe_kalk = 326260.0;
    let betriebsstoffe_poly = 23620.0;
    let n2o_szenario = 3;

    let field_sets = vec![
        // Allgemeine Infos zur Kläranlage
        FieldSet {
            title: "Angaben zur Kläranlage",
            fields: vec![
                // Name der Kläranlage
                Field::Text {
                    base: FieldBase {
                        id: Id::Name,
                        label: "Name oder Ort",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(name),
                    max_len: None,
                    placeholder: Some("Name der Kläranlage"),
                },
                Field::Float {
                    base: FieldBase {
                        id: Id::Ew,
                        label: "Ausbaugröße",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(ew),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Ausbaugröße [EW]"),
                    unit: "EW",
                },
                Field::Float {
                    base: FieldBase {
                        id: Id::Flow,
                        label: "Abwassermenge",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(flow),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Abwassermenge"),
                    unit: "m³/a",
                },
            ],
        },
        // Zulauf-Parameter
        FieldSet {
            title: "Zulauf-Parameter (Jahresmittelwerte)",
            fields: vec![
                // CSB
                Field::Float {
                    base: FieldBase {
                        id: Id::CsbZu,
                        label: "Chemischer Sauerstoffbedarf",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(csb_zu),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("CSB"),
                    unit: "mg/L",
                },
                // TKN
                Field::Float {
                    base: FieldBase {
                        id: Id::TknZu,
                        label: "Gesamtstickstoff",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(tkn_zu),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("TKN"),
                    unit: "mg/L",
                },
                // P
                Field::Float {
                    base: FieldBase {
                        id: Id::PZu,
                        label: "Phosphor",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(p_zu),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("P"),
                    unit: "mg/L",
                },
            ],
        },
        // Ablauf-Parameter
        FieldSet {
            title: "Ablauf-Parameter (Jahresmittelwerte)",
            fields: vec![
                // CSB
                Field::Float {
                    base: FieldBase {
                        id: Id::CsbAb,
                        label: "Chemischer Sauerstoffbedarf",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(csb_ab),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("CSB"),
                    unit: "mg/L",
                },
                // TKN
                Field::Float {
                    base: FieldBase {
                        id: Id::TknAb,
                        label: "Gesamtstickstoff",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(tkn_ab),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("TKN"),
                    unit: "mg/L",
                },
                // P
                Field::Float {
                    base: FieldBase {
                        id: Id::PAb,
                        label: "Phosphor",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(p_ab),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("P"),
                    unit: "mg/L",
                },
            ],
        },
        // Energiebedarf
        FieldSet {
            title: "Energiebedarf",
            fields: vec![
                // Klärgas erzeugt
                Field::Float {
                    base: FieldBase {
                        id: Id::Klaergas,
                        label: "Erzeugtes Klärgas",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(klaergas),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Klärgas"),
                    unit: "m³",
                },
                Field::Float {
                    base: FieldBase {
                        id: Id::Methangehalt,
                        label: "Methangehalt",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(methangehalt),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("65"),
                    unit: "%",
                },
                // Erdgas zugekauft
                Field::Float {
                    base: FieldBase {
                        id: Id::GasZusatz,
                        label: "Gasbezug (Versorger)",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(gas_zusatz),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Gasbezug"),
                    unit: "kWh/a",
                },
                // Biogas ja/nein
                Field::Bool {
                    base: FieldBase {
                        label: "Bezug von Biogas",
                        id: Id::Biogas,
                        description: Some("ja/nein"),
                        required: false,
                    },
                    initial_value: Some(biogas),
                },
                // Strombedarf gesamt
                Field::Float {
                    base: FieldBase {
                        id: Id::Strombedarf,
                        label: "Strombedarf gesamt",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(strombedarf),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Gesamtstrombedarf"),
                    unit: "kWh/a",
                },
                Field::Float {
                    base: FieldBase {
                        id: Id::Eigenstrom,
                        label: "Eigenstromerzeugung",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(eigenstrom),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Eigenstrom"),
                    unit: "kWh/a",
                },
                // Emissionsfaktor Strom-Mix
                Field::Float {
                    base: FieldBase {
                        id: Id::EfStrommix,
                        label: "Emissionsfaktor Strommix (Versorger)",
                        description: None,
                        required: true,
                    },
                    // defaultValue = 485
                    initial_value: Some(ef_strommix),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("485"),
                    unit: "g CO₂/kWh",
                },
            ],
        },
        // Klärschlammbehandlung
        FieldSet {
            title: "Klärschlammbehandlung",
            fields: vec![
                Field::Bool {
                    base: FieldBase {
                        label: "Offene Schlammtaschen",
                        id: Id::Schlammtaschen,
                        description: Some("ja/nein"),
                        required: false,
                    },
                    initial_value: Some(schlammtaschen),
                },
                Field::Bool {
                    base: FieldBase {
                        label: "Offene Schlammstapelbehälter",
                        id: Id::Schlammstapel,
                        description: Some("ja/nein"),
                        required: false,
                    },
                    initial_value: Some(schlammstapel),
                },
                Field::Float {
                    base: FieldBase {
                        id: Id::KlaerschlammEnstorgung,
                        label: "Kläraschlamm zur Entsorgung",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(klaerschlamm_enstorgung),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Masse entwässert"),
                    unit: "t",
                },
                Field::Float {
                    base: FieldBase {
                        id: Id::KlaerschlammTransport,
                        label: "Transportdistanz",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(klaerschlamm_transport),
                    min_value: None,
                    max_value: None,
                    placeholder: Some("Entfernung"),
                    unit: "km",
                },
            ],
        },
        // Betriebsstoffe
        FieldSet {
            title: "Eingesetzte Betriebsstoffe",
            fields: vec![
                // Eisen(III)Chlorid
                Field::Float {
                    base: FieldBase {
                        label: "Eisen(III)-chlorid-Lösung",
                        description: None,
                        id: Id::BetriebsstoffeFe3,
                        required: true,
                    },
                    unit: "kg",
                    placeholder: Some("kg Lösung"),
                    initial_value: Some(betriebsstoffe_fe3),
                    min_value: None,
                    max_value: None,
                },
                // Eisen(III)Chlorid
                Field::Float {
                    base: FieldBase {
                        label: "Eisenchloridsulfat-Lösung",
                        description: None,
                        id: Id::BetriebsstoffeFeso4,
                        required: true,
                    },
                    unit: "kg",
                    placeholder: Some("kg Lösung"),
                    initial_value: Some(betriebsstoffe_feso4),
                    min_value: None,
                    max_value: None,
                },
                // Kalkhydrat
                Field::Float {
                    base: FieldBase {
                        label: "Kalkhydrat",
                        description: None,
                        id: Id::BetriebsstoffeKalk,
                        required: true,
                    },
                    unit: "kg",
                    placeholder: Some("kg Branntkalk"),
                    initial_value: Some(betriebsstoffe_kalk),
                    min_value: None,
                    max_value: None,
                },
                // Polymere
                Field::Float {
                    base: FieldBase {
                        label: "Synthetische Polymere",
                        description: None,
                        id: Id::BetriebsstoffePoly,
                        required: true,
                    },
                    placeholder: Some("kg Polymere"),
                    unit: "kg",
                    initial_value: Some(betriebsstoffe_poly),
                    min_value: None,
                    max_value: None,
                },
            ],
        },
        // Szenario
        FieldSet {
            title: "Schätzung des N₂O-Emissionsfaktors",
            fields: vec![Field::Selection {
                base: FieldBase {
                    description: None,
                    label: "Szenario",
                    id: Id::N2oSzenario,
                    required: true,
                },
                initial_value: Some(n2o_szenario),
                options: vec![
                    forms::SelectOption {
                        value: 0,
                        label: "Extrapoliert nach Parravicini et al. 2016",
                    },
                    forms::SelectOption {
                        value: 1,
                        label: "Optimistisch",
                    },
                    forms::SelectOption {
                        value: 2,
                        label: "Pessimistisch",
                    },
                    forms::SelectOption {
                        value: 3,
                        label: "Nach IPCC 2019",
                    },
                ],
            }],
        },
    ];

    let (signals, set_views) = forms::render_field_sets(field_sets);

    let name_ka = *signals.get(&Id::Name).unwrap();
    let ew = *signals.get(&Id::Ew).unwrap();

    let render = create_action(move |output_data: &klick_application::OutputData| {
        let output_data = output_data.clone();

        async move {
            let klick_application::OutputData {
                co2eq_n2o_anlage,
                co2eq_n2o_gewaesser,
                co2eq_ch4_klaerprozes,
                co2eq_ch4_schlammstapel,
                co2eq_ch4_schlammtasche,
                co2eq_ch4_gewaesser,
                co2eq_ch4_bhkw,
                co2eq_betriebsstoffe_fe3,
                co2eq_betriebsstoffe_feso4,
                co2eq_betriebsstoffe_kalk,
                co2eq_betriebsstoffe_poly,
                co2eq_strommix,
                co2eq_betriebsstoffe,
                co2eq_klaerschlamm_transport,
                direkte_emissionen_co2_eq,
                indirekte_emissionen_co2_eq,
                weitere_indirekte_emissionen_co2_eq,
                emissionen_co2_eq,
            } = output_data;

            let dir_em = "Direkte Emissionen";
            let indir_em = "Indirekte Emissionen";
            let wei_indir_em = "Weitere Indirekte Emissionen";
            let nu = "Nutzung";
            let em = "Emission";

            let streams: Vec<(_, _, _)> = vec![
                ("N<sub>2</sub>O Anlage", dir_em, co2eq_n2o_anlage),
                ("N<sub>2</sub>O Gewaesser", dir_em, co2eq_n2o_gewaesser),
                ("CH<sub>4</sub> Klärprozess", dir_em, co2eq_ch4_klaerprozes),
                (
                    "CH<sub>4</sub> Schlupf Schlammstapel",
                    dir_em,
                    co2eq_ch4_schlammstapel,
                ),
                (
                    "CH<sub>4</sub> Schlupf Schlammtasche",
                    dir_em,
                    co2eq_ch4_schlammtasche,
                ),
                ("CH<sub>4</sub> Gewaesser", dir_em, co2eq_ch4_gewaesser),
                ("CH<sub>4</sub> BHKW", dir_em, co2eq_ch4_bhkw),
                (
                    "Eisen(III)-chlorid-Lösung",
                    "Betriebsstoffe",
                    co2eq_betriebsstoffe_fe3,
                ),
                (
                    "Eisenchloridsulfat-Lösung",
                    "Betriebsstoffe",
                    co2eq_betriebsstoffe_feso4,
                ),
                ("Kalkhydrat", "Betriebsstoffe", co2eq_betriebsstoffe_kalk),
                (
                    "Synthetische Polymere",
                    "Betriebsstoffe",
                    co2eq_betriebsstoffe_poly,
                ),
                ("Strommix", indir_em, co2eq_strommix),
                ("Betriebsstoffe", wei_indir_em, co2eq_betriebsstoffe),
                (
                    "Klaerschlamm Transport",
                    wei_indir_em,
                    co2eq_klaerschlamm_transport,
                ),
                (dir_em, nu, direkte_emissionen_co2_eq),
                (indir_em, nu, indirekte_emissionen_co2_eq),
                (wei_indir_em, nu, weitere_indirekte_emissionen_co2_eq),
                (nu, em, emissionen_co2_eq),
            ];

            let mut labels: Vec<_> = vec![];

            for (src, target, _) in &streams {
                labels.push(src.to_string());
                labels.push(target.to_string());
            }

            let einheit = "t co2-eq/Jahr"; // Ebenfalls in Anführungszeichen, Einheitliche - Gesamt KA oder Bezug auf EW
            let name_ka = name_ka
                .get_text()
                .unwrap_or_else(|| "Kläranlage".to_string());
            let ew = ew.get_float().unwrap_or_default();
            let title = format!("{name_ka} ({ew} EW)<br />Treibhausgasemissionen [{einheit}]");

            labels.sort();
            labels.dedup();
            let sankey_data: Vec<_> = labels;
            let sankey_links: Vec<(_, _, f64)> = streams;

            let chart = Chart::new().title(Title::new().text(title)).series(
                Sankey::new()
                    .emphasis(Emphasis::new().focus(EmphasisFocus::Adjacency))
                    .data(sankey_data)
                    .links(sankey_links),
            );
            log::debug!("Render Sankey chart");
            let renderer = charming::WasmRenderer::new(1200, 800);
            renderer.render("chart", &chart).unwrap();
        }
    });

    let s = signals.clone();
    create_effect(move |_| {
        let Some(ew) = s.get(&Id::Ew).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(abwasser) = s.get(&Id::Flow).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(n_ges_zu) = s.get(&Id::TknZu).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(csb_ab) = s.get(&Id::CsbAb).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(n_ges_ab) = s.get(&Id::TknAb).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(klaergas_gesamt) = s.get(&Id::Klaergas).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(methangehalt) = s.get(&Id::Methangehalt).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(strombedarf) = s.get(&Id::Strombedarf).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(energie_eigen) = s.get(&Id::Eigenstrom).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(ef_co2_strommix) = s.get(&Id::EfStrommix).and_then(FieldSignal::get_float) else {
            return;
        };
        let Some(schlammtaschen) = s.get(&Id::Schlammtaschen).and_then(FieldSignal::get_bool)
        else {
            return;
        };
        let Some(schlammstapel) = s.get(&Id::Schlammstapel).and_then(FieldSignal::get_bool) else {
            return;
        };
        let Some(klaerschlamm_transport_km) = s
            .get(&Id::KlaerschlammTransport)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(klaerschlamm_entsorgung_m) = s
            .get(&Id::KlaerschlammEnstorgung)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_fe3) = s
            .get(&Id::BetriebsstoffeFe3)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_feso4) = s
            .get(&Id::BetriebsstoffeFeso4)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_kalk) = s
            .get(&Id::BetriebsstoffeKalk)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(betriebsstoffe_poly) = s
            .get(&Id::BetriebsstoffePoly)
            .and_then(FieldSignal::get_float)
        else {
            return;
        };
        let Some(n2o_szenario) = s.get(&Id::N2oSzenario).and_then(FieldSignal::get_selection)
        else {
            return;
        };

        let input_data = klick_application::InputData {
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
    });

    view! {
      <div class="space-y-12">
        { set_views }
      </div>
      <div id="chart" class="mt-8"></div>
    }
}
