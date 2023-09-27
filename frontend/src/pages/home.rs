use leptos::*;

use crate::forms;

fn example_data() -> klick_boundary::FormData {
    klick_boundary::FormData {
        name: "Lingen".to_string(),
        ew: "120000".to_string(),
        flow: "5000000".to_string(),
        csb_zu: "1045".to_string(),
        tkn_zu: "122".to_string(),
        p_zu: "9,9".to_string(),
        csb_ab: "129".to_string(),
        tkn_ab: "11,76".to_string(),
        p_ab: "0,4".to_string(),
        klaergas: "1260000".to_string(),
        methangehalt: "23".to_string(),
        gas_zusatz: "1300000".to_string(),
        biogas: Some("no".to_string()),
        strombedarf: "2683259".to_string(),
        eigenstrom: "2250897".to_string(),
        ef_strommix: "468".to_string(),
        schlammtaschen: Some("yes".to_string()),
        schlammstapel: Some("yes".to_string()),
        klaerschlamm_enstorgung: "3687,6".to_string(),
        klaerschlamm_transport: "47".to_string(),
        betriebsstoffe_fe3: "0".to_string(),
        betriebsstoffe_feso4: "326000".to_string(),
        betriebsstoffe_kalk: "326260".to_string(),
        betriebsstoffe_poly: "23620".to_string(),
        n2o_szenario: "3".to_string(),
    }
}

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
    let klick_boundary::FormData {
        name,
        ew,
        flow,
        csb_zu,
        tkn_zu,
        p_zu,
        csb_ab,
        tkn_ab,
        p_ab,
        klaergas,
        methangehalt,
        gas_zusatz,
        biogas,
        strombedarf,
        eigenstrom,
        ef_strommix,
        schlammtaschen,
        schlammstapel,
        klaerschlamm_enstorgung,
        klaerschlamm_transport,
        betriebsstoffe_fe3,
        betriebsstoffe_feso4,
        betriebsstoffe_kalk,
        betriebsstoffe_poly,
        n2o_szenario,
    } = example_data();

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
                    initial_value: Some(ew.parse().unwrap()),
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
                    initial_value: Some(flow.parse().unwrap()),
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
                    initial_value: Some(csb_zu.parse().unwrap()),
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
                    initial_value: Some(tkn_zu.parse().unwrap()),
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
                    initial_value: Some(p_zu.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(csb_ab.parse().unwrap()),
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
                    initial_value: Some(tkn_ab.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(p_ab.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(klaergas.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(methangehalt.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(gas_zusatz.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(biogas.as_deref() == Some("yes")),
                },
                // Strombedarf gesamt
                Field::Float {
                    base: FieldBase {
                        id: Id::Strombedarf,
                        label: "Strombedarf gesamt",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(strombedarf.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(eigenstrom.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(ef_strommix.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(schlammtaschen.as_deref() == Some("yes")),
                },
                Field::Bool {
                    base: FieldBase {
                        label: "Offene Schlammstapelbehälter",
                        id: Id::Schlammstapel,
                        description: Some("ja/nein"),
                        required: false,
                    },
                    initial_value: Some(schlammstapel.as_deref() == Some("yes")),
                },
                Field::Float {
                    base: FieldBase {
                        id: Id::KlaerschlammEnstorgung,
                        label: "Kläraschlamm zur Entsorgung",
                        description: None,
                        required: true,
                    },
                    initial_value: Some(klaerschlamm_enstorgung.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(klaerschlamm_transport.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(betriebsstoffe_fe3.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(betriebsstoffe_feso4.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(betriebsstoffe_kalk.replace(',', ".").parse().unwrap()),
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
                    initial_value: Some(betriebsstoffe_poly.replace(',', ".").parse().unwrap()),
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
                initial_value: Some(n2o_szenario.parse().unwrap()),
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
    debug_assert!({
        let mut ids = signals.iter().map(|(id, _)| id).collect::<Vec<_>>();
        let n = ids.len();
        ids.sort();
        ids.dedup();
        n == ids.len()
    });

    view! {

    <form action="/submit" method="post" class="space-y-12">
      { set_views }

      <div class="mt-6 flex items-center justify-end gap-x-6">
        <input
          type="submit"
          class="rounded-md bg-indigo-600 px-3 py-2 text-lg font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          value = "Start" />
      </div>

    </form>
    }
}
