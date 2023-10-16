use klick_application::{N2OSzenario, ValueId};

use crate::forms::{self, FieldType};

use super::util;

type Field = forms::Field<ValueId>;
type FieldSet = forms::FieldSet<ValueId>;

pub fn field_sets() -> Vec<FieldSet> {
    vec![
        FieldSet {
            title: "Angaben zur Kläranlage",
            fields: vec![
                Field {
                    id: ValueId::Name,
                    label: "Name oder Ort",
                    description: None,
                    required: true,
                    field_type: FieldType::Text {
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
                    },
                },
                Field {
                    id: ValueId::Strombedarf,
                    label: "Strombedarf gesamt",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
                    },
                },
                Field {
                    label: "Offene Schlammstapelbehälter",
                    id: ValueId::Schlammstapel,
                    description: Some("ja/nein"),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: None,
                    },
                },
                Field {
                    id: ValueId::KlaerschlammEnstorgung,
                    label: "Kläraschlamm zur Entsorgung",
                    description: None,
                    required: true,
                    field_type: FieldType::Float {
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                        initial_value: None,
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
                    initial_value: None,
                    options: vec![
                        forms::SelectOption {
                            value: util::n2o_szenario_to_usize(
                                N2OSzenario::ExtrapolatedParravicini,
                            ),
                            label: "Extrapoliert nach Parravicini et al. 2016",
                        },
                        forms::SelectOption {
                            value: util::n2o_szenario_to_usize(N2OSzenario::Optimistic),
                            label: "Optimistisch",
                        },
                        forms::SelectOption {
                            value: util::n2o_szenario_to_usize(N2OSzenario::Pesimistic),
                            label: "Pessimistisch",
                        },
                        forms::SelectOption {
                            value: util::n2o_szenario_to_usize(N2OSzenario::Ipcc2019),
                            label: "Nach IPCC 2019",
                        },
                    ],
                },
            }],
        },
    ]
}
