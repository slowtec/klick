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
                    description: Some("Die Angabe des Namens und/oder Orts sind freiwillig. Alternativ kann für das Feld ein Platzhalter eingetragen werden. Sämtliche Eintragungen können nur von Ihnen (nicht der UTBW) eingesehen oder gespeichert werden."),
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
                    description: Some("Ausbaugröße Ihrer Kläranlage in Einwohnerwerten (EW) als Summe der angeschlossenen Einwohner (E) und der gewerblichen Einwohnergleichwerte (EGW)."),
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
                    description: Some("Die jährliche (a) Abwassermenge in Kubikmeter (m3) im Zulauf Ihrer Kläranlage."),
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
                    description: Some("Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L)."),
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
                    description: Some("Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert."),
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
                    description: Some("Der Gesamt-Phosphor-Gehalt des Abwassers (Pges) im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert."),
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
                    description: Some("Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L)."),
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
                    description: Some("Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert."),
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
                    description: Some("Der Gesamt-Phosphor-Gehalt des Abwassers (Pges) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert."),
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
                    description: Some("Das an Ihrer Kläranlage erzeugte Klärgas in Kubikmeter (m3) pro Jahr (a). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen oder Null eintragen."),
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
                    description: Some("Der Methangehalt des an Ihrer Kläranlage erzeugten Klärgases in Prozent (%). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen oder Null eintragen."),
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
                    description: Some("Menge an Gas (Erdgas/Biogas) in Kilowattstunden (kWh) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Gas von extern bezogen wird, dieses Feld bitte freilassen oder Null eintragen."),
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
                    description: Some("Falls Ihre Kläranlage Biogas von extern bezieht, dieses Feld bitte anklicken."),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: None,
                    },
                },
                Field {
                    id: ValueId::Strombedarf,
                    label: "Strombedarf gesamt",
                    description: Some("Der Gesamt-Strombedarf Ihrer Kläranlage in Kilowattstunden (kWh) pro Jahr (a)."),
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
                    description: Some("Anteil der Eigenstromerzeugung in Kilowattstunden (kWh) pro Jahr (a). Falls kein Eigenstrom erzeugt wird, dieses Feld bitte freilassen oder Null eintragen."),
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
                    description: Some("Angabe des Emissionsfaktors des von extern bezogenen Strommixes in Gramm (g) CO2 pro Kilowattstunde (kWh). Falls dieser Wert nicht verfügbar ist, bitte den Referenzwert stehen lassen."),
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
                    description: Some("Falls die Schlammtaschen des Faulturms nicht geschlossen sind, sondern zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken."),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: None,
                    },
                },
                Field {
                    label: "Offene Schlammstapelbehälter",
                    id: ValueId::Schlammstapel,
                    description: Some("Falls die Schlammstapelbehälter Ihrer Kläranlage nicht geschlossen sind, sondern offen betrieben werden, dann dieses Feld bitte anklicken."),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: None,
                    },
                },
                Field {
                    id: ValueId::KlaerschlammEnstorgung,
                    label: "Klärschlamm zur Entsorgung",
                    description: Some("Angabe der Menge an Klärschlamm in Tonnen (t) die zur Entsorgung anfallen."),
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
                    description: Some("Entfernung von Ihrer Kläranlage zum Entsorgungsort des Klärschlamms in Kilometer (km). Die Angabe ist unabhängig von der Entsorgungsart (z.B. Verbrennung) oder der Transportform (z.B. entwässert/trocken). Falls der Klärschlamm auf Ihrer Kläranlage entsorgt wird, dieses Feld bitte freilassen oder Null eintragen."),
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
                    description: Some("Angabe der pro Jahr (a) eingesetzten Menge an Eisen(III)-chlorid (FeCl3) in Tonnen (t)."),
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
                    description: Some("Angabe der pro Jahr (a) eingesetzten Menge an Eisenchloridsulfat (FeClSO4) in Tonnen (t)."),
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
                    description: Some("Angabe der pro Jahr (a) eingesetzten Menge an Kalkhydrat (Ca(OH)2) in Tonnen (t)."),
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
                    description: Some("Angabe der pro Jahr (a) eingesetzten Menge an synthetischen Polymeren in Tonnen (t)."),
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
