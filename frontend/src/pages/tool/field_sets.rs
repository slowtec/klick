use crate::forms::{self, FieldType, MinMax};

use super::fields::{FieldId, ScenarioFieldId};
type Id = FieldId;

pub type FieldSet = forms::FieldSet<Id>;
pub type Field = forms::Field<Id>;

#[allow(clippy::too_many_lines)]
pub fn field_sets() -> Vec<FieldSet> {
    vec![
    FieldSet {
        title: None,
        fields: vec![
            Field {
                id: Id::ProjectName,
                label: "Projektname",
                description: None,
                required: false,
                field_type: FieldType::Text {
                    initial_value: None,
                    placeholder: Some("Projektname"),
                    max_len: None,
                },
            },
        ]
    },
    FieldSet {
        title: Some("Angaben zur Kläranlage"),
        fields: vec![
            Field {
                id: Id::Name,
                label: "Name oder Ort",
                description: Some(
                    "Die Angabe des Namens und/oder Orts sind freiwillig. Alternativ kann für das Feld ein Platzhalter eingetragen werden. Sämtliche Eintragungen können nur von Ihnen (nicht der UTBW) eingesehen oder gespeichert werden.",
                ),
                required: false,
                field_type: FieldType::Text {
                    initial_value: None,
                    placeholder: Some(
                        "Name der Kläranlage",
                    ),
                    max_len: None,
                },
            },
            Field {
                id: Id::Ew,
                label: "Ausbaugröße",
                description: Some(
                    "Ausbaugröße Ihrer Kläranlage in Einwohnerwerten (EW) als Summe der angeschlossenen Einwohner (E) und der gewerblichen Einwohnergleichwerte (EGW).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Ausbaugröße [EW]",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            5_000_000.0,
                        ),
                    },
                    unit: "EW",
                },
            },
            Field {
                id: Id::Flow,
                label: "Abwassermenge",
                description: Some(
                    "Die jährliche (a) Abwassermenge in Kubikmeter (m³) im Zulauf Ihrer Kläranlage.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Abwassermenge",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            1_000_000_000.0,
                        ),
                    },
                    unit: "m³/a",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Zulauf-Parameter (Jahresmittelwerte)"),
        fields: vec![
            Field {
                id: Id::CsbZu,
                label: "Chemischer Sauerstoffbedarf",
                description: Some(
                    "Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "CSB",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            5000.0,
                        ),
                    },
                    unit: "mg/L",
                },
            },
            Field {
                id: Id::TknZu,
                label: "Gesamtstickstoff",
                description: Some(
                    "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "TKN",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            5000.0,
                        ),
                    },
                    unit: "mg/L",
                },
            },
            Field {
                id: Id::PZu,
                label: "Phosphor",
                description: Some(
                    "Der Gesamt-Phosphor-Gehalt des Abwassers (Pges) im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "P",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            1000.0,
                        ),
                    },
                    unit: "mg/L",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Ablauf-Parameter (Jahresmittelwerte)"),
        fields: vec![
            Field {
                id: Id::CsbAb,
                label: "Chemischer Sauerstoffbedarf",
                description: Some(
                    "Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "CSB",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            1000.0,
                        ),
                    },
                    unit: "mg/L",
                },
            },
            Field {
                id: Id::TknAb,
                label: "Gesamtstickstoff",
                description: Some(
                    "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "TKN",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            1000.0,
                        ),
                    },
                    unit: "mg/L",
                },
            },
            Field {
                id: Id::PAb,
                label: "Phosphor",
                description: Some(
                    "Der Gesamt-Phosphor-Gehalt des Abwassers (Pges) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "P",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            50.0,
                        ),
                    },
                    unit: "mg/L",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Energiebedarf"),
        fields: vec![
            Field {
                id: Id::Klaergas,
                label: "Erzeugtes Klärgas",
                description: Some(
                    "Das an Ihrer Kläranlage erzeugte Klärgas in Kubikmeter (m³) pro Jahr (a). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Klärgas",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            100_000_000.0,
                        ),
                    },
                    unit: "m³",
                },
            },
            Field {
                id: Id::Methangehalt,
                label: "Methangehalt",
                description: Some(
                    "Der Methangehalt des an Ihrer Kläranlage erzeugten Klärgases in Prozent (%). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "62",
                    ),
                    limits: MinMax {
                        min: Some(
                            20.0,
                        ),
                        max: Some(
                            90.0,
                        ),
                    },
                    unit: "%",
                },
            },
            Field {
                id: Id::GasZusatz,
                label: "Gasbezug (Versorger)",
                description: Some(
                    "Menge an Gas (Erdgas/Biogas) in Kilowattstunden (kWh) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Gas von extern bezogen wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gasbezug",
                    ),
                    limits: MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "kWh/a",
                },
            },
            Field {
                id: Id::Biogas,
                label: "Bezug von Biogas",
                description: Some(
                    "Falls Ihre Kläranlage Biogas von extern bezieht, dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                },
            },
            Field {
                id: Id::Strombedarf,
                label: "Strombedarf gesamt",
                description: Some(
                    "Der Gesamt-Strombedarf Ihrer Kläranlage in Kilowattstunden (kWh) pro Jahr (a).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gesamtstrombedarf",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            1_000_000_000.0,
                        ),
                    },
                    unit: "kWh/a",
                },
            },
            Field {
                id: Id::Eigenstrom,
                label: "Eigenstromerzeugung",
                description: Some(
                    "Anteil der Eigenstromerzeugung in Kilowattstunden (kWh) pro Jahr (a). Falls kein Eigenstrom erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Eigenstrom",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            50_000_000.0,
                        ),
                    },
                    unit: "kWh/a",
                },
            },
            Field {
                id: Id::EfStrommix,
                label: "Emissionsfaktor Strommix (Versorger)",
                description: Some(
                    "Angabe des Emissionsfaktors des von extern bezogenen Strommixes in Gramm (g) CO₂ pro Kilowattstunde (kWh). Falls dieser Wert nicht verfügbar ist, bitte den Referenzwert stehen lassen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "485",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            2500.0,
                        ),
                    },
                    unit: "g CO₂/kWh",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Klärschlammbehandlung"),
        fields: vec![
            Field {
                id: Id::Schlammtaschen,
                label: "Offene Schlammtaschen",
                description: Some(
                    "Falls die Schlammtaschen des Faulturms nicht geschlossen sind, sondern zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                },
            },
            Field {
                id: Id::KlaerschlammEnstorgung,
                label: "Klärschlamm zur Entsorgung",
                description: Some(
                    "Angabe der Menge an Klärschlamm in Tonnen (t) die zur Entsorgung anfallen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Masse entwässert",
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            500_000.0,
                        ),
                    },
                    unit: "t",
                },
            },
            Field {
                id: Id::KlaerschlammTransport,
                label: "Transportdistanz",
                description: Some(
                    "Entfernung von Ihrer Kläranlage zum Entsorgungsort des Klärschlamms in Kilometer (km). Die Angabe ist unabhängig von der Entsorgungsart (z.B. Verbrennung) oder der Transportform (z.B. entwässert/trocken). Falls der Klärschlamm auf Ihrer Kläranlage entsorgt wird, dieses Feld bitte freilassen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Entfernung",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            500.0,
                        ),
                    },
                    unit: "km",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Eingesetzte Betriebsstoffe"),
        fields: vec![
            Field {
                id: Id::BetriebsstoffeFe3,
                label: "Eisen(III)-chlorid-Lösung",
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisen(III)-chlorid (FeCl3) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Lösung",
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            500_000.0,
                        ),
                    },
                    unit: "t",
                },
            },
            Field {
                id: Id::BetriebsstoffeFeso4,
                label: "Eisenchloridsulfat-Lösung",
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisenchloridsulfat (FeClSO4) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Lösung",
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            100_000.0,
                        ),
                    },
                    unit: "t",
                },
            },
            Field {
                id: Id::BetriebsstoffeKalk,
                label: "Kalkhydrat",
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Kalkhydrat (Ca(OH)2) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Branntkalk",
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            500_000.0,
                        ),
                    },
                    unit: "t",
                },
            },
            Field {
                id: Id::BetriebsstoffePoly,
                label: "Synthetische Polymere",
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an synthetischen Polymeren in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Polymere",
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            50000.0,
                        ),
                    },
                    unit: "t",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Szenario Benutzerdefiniert"),
        fields: vec![
            Field {
                id: Id::Scenario(ScenarioFieldId::N2oCustomFactor),
                label: "N₂O-Emissionsfaktor",
                description: Some(
                    "N₂O-Emissionsfaktor für das Szenario Benutzerdefiniert.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "% des TN(Zulauf)",
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            100.0,
                        ),
                    },
                    unit: "%",
                },
            },
        ],
    },
]
}
