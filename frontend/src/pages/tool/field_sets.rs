use klick_presenter::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, SewageSludgeTreatmentId, SideStreamTreatmentId,
};

use crate::forms::{self, FieldType, MinMax};

use super::fields::FieldId;
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
                description: Some(
                    "In diesem Feld können Sie einen Namen für Ihr Projekt hinterlegen. <br>

                    Falls Sie sich <b>angemeldet</b> haben,
                    wird der Projektname zur Speicherung Ihrer Eingabewerte und Ergebnisse unter dem Reiter Projekte verwendet.
                    Diese Daten werden unverschluesselt auf einem Server hinterlegt, Dritte können diese Daten allerdings
                    nicht einsehen und die UTBW wird diese Daten nicht weitergeben oder weiterverarbeiten.<br>

                    Wenn Sie sich <b>nicht angemeldet</b> haben, können Sie das Tool natürlich dennoch in vollem Umfang nutzen.
                    Ihre Daten inkl. des Projektnamens werden dabei ausschließlich lokal auf Ihrer Festplatte gespeichert
                    sowie nur in Ihrem Browser verarbeitet."
                ),
                required: false,
                field_type: FieldType::Text {
                    initial_value: None,
                    placeholder: Some("Projektname".to_string()),
                    max_len: None,
                },
            },
        ]
    },
    FieldSet {
        title: Some("Angaben zur Kläranlage"),
        fields: vec![
            Field {
                id: ProfileValueId::PlantName.into(),
                description: Some(
                    "Die Angabe des Namens und/oder Orts sind freiwillig. Alternativ kann für das Feld ein Platzhalter eingetragen werden. Sämtliche Eintragungen können nur von Ihnen (nicht der UTBW) eingesehen oder gespeichert werden.",
                ),
                required: false,
                field_type: FieldType::Text {
                    initial_value: None,
                    placeholder: Some(
                        "Name der Kläranlage".to_string(),
                    ),
                    max_len: None,
                },
            },
            Field {
                id: ProfileValueId::PopulationEquivalent.into(),
                description: Some(
                    "Ausbaugröße Ihrer Kläranlage in Einwohnerwerten (EW) als Summe der angeschlossenen Einwohner (E) und der gewerblichen Einwohnergleichwerte (EGW).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Ausbaugröße [EW]".to_string(),
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
                id: ProfileValueId::Wastewater.into(),
                description: Some(
                    "Die jährliche (a) Abwassermenge in Kubikmeter (m³) im Zulauf Ihrer Kläranlage.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Abwassermenge".to_string(),
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
                id: ProfileValueId::from(AnnualAverageInfluentId::ChemicalOxygenDemand).into(),
                description: Some(
                    "Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "CSB".to_string(),
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
                id: ProfileValueId::from(AnnualAverageInfluentId::Nitrogen).into(),
                description: Some(
                    "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "TKN".to_string(),
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
                id: ProfileValueId::from(AnnualAverageInfluentId::TotalOrganicCarbohydrates).into(),
                description: Some(
                    "Der Jahresmittelwert des Gesamten organischen Kohlenstoffs (Total Organic Carbon, TOC)
                    des Abwassers im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).<br>
                    Wenn Sie keinen Wert für den TOC haben dann dieses Feld bitte freilassen
                    (Anm.: für die Berechnung der fossilen CO₂-Emissionen wird in diesem Fall der CSB verwendet). ",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "TOC".to_string(),
                    ),
                    limits: MinMax {
                        min: Some(
                            0.0,
                        ),
                        max: Some(
                            2000.0,
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
                id: ProfileValueId::from(AnnualAverageEffluentId::ChemicalOxygenDemand).into(),
                description: Some(
                    "Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "CSB".to_string(),
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
                id: ProfileValueId::from(AnnualAverageEffluentId::Nitrogen).into(),
                description: Some(
                    "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "TKN".to_string(),
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
        title: Some("Energiebedarf"),
        fields: vec![
            Field {
                id: ProfileValueId::from(EnergyConsumptionId::SewageGasProduced).into(),
                description: Some(
                    "Das an Ihrer Kläranlage erzeugte Klärgas in Kubikmeter (m³) pro Jahr (a). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Klärgas".to_string(),
                    ),
                    limits: MinMax {
                        min: Some(0.0),
                        max: Some(
                            100_000_000.0,
                        ),
                    },
                    unit: "m³",
                },
            },
            Field {
                id: ProfileValueId::from(EnergyConsumptionId::MethaneFraction).into(),
                description: Some(
                    "Der Methangehalt des an Ihrer Kläranlage erzeugten Klärgases in Prozent (%). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "62".to_string(),
                    ),
                    limits: MinMax {
                        min: Some(0.0),
                        max: Some(
                            90.0,
                        ),
                    },
                    unit: "%",
                },
            },
            Field {
                id: ProfileValueId::from(EnergyConsumptionId::GasSupply).into(),
                description: Some(
                    "Menge an Gas (Erdgas/Biogas) in Kilowattstunden (kWh) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Gas von extern bezogen wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gasbezug".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "kWh/a",
                },
            },
            Field {
                id: ProfileValueId::from(EnergyConsumptionId::PurchaseOfBiogas).into(),
                description: Some(
                    "Falls Ihre Kläranlage Biogas von extern bezieht, dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                },
            },
            Field {
                id: ProfileValueId::from(EnergyConsumptionId::TotalPowerConsumption).into(),
                description: Some(
                    "Der Gesamt-Strombedarf Ihrer Kläranlage in Kilowattstunden (kWh) pro Jahr (a).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gesamtstrombedarf".to_string(),
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
                id: ProfileValueId::from(EnergyConsumptionId::OnSitePowerGeneration).into(),
                description: Some(
                    "Anteil der Eigenstromerzeugung in Kilowattstunden (kWh) pro Jahr (a). Falls kein Eigenstrom erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Eigenstrom".to_string(),
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
                id: ProfileValueId::from(EnergyConsumptionId::EmissionFactorElectricityMix).into(),
                description: Some(
                    "Angabe des Emissionsfaktors des von extern bezogenen Strommixes in Gramm (g) CO₂ pro Kilowattstunde (kWh). Falls dieser Wert nicht verfügbar ist, bitte den Referenzwert stehen lassen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "485".to_string(),
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
            Field {
                id: ProfileValueId::from(EnergyConsumptionId::HeatingOil).into(),
                description: Some(
                    "boak",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Heizölbezug".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "L/a",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Klärschlammbehandlung"),
        fields: vec![
            Field {
                id: ProfileValueId::from(SewageSludgeTreatmentId::DigesterCount).into(),
                description: Some(
                    "Falls auf Ihrer Kläranlage eine Faulung vorhanden ist, dann geben Sie bitte die Anzahl der Faultürme ein. Falls nicht lassen Sie das Feld bitte offen oder tragen eine 0 ein.",
                ),
                required: false,
                field_type: FieldType::UnsignedInteger {
                    initial_value: None,
                    placeholder: Some(
                        "Anzahl Faultürme".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            9,
                        ),
                    },
                    unit: "Türme",
                },
            },
            Field {
                id: ProfileValueId::from(SewageSludgeTreatmentId::SludgeBags).into(),
                description: Some(
                    "Falls die Schlammtaschen des Faulturms nicht geschlossen sind, sondern zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                },
            },
            Field {
                id: ProfileValueId::from(SewageSludgeTreatmentId::SludgeStorageContainers).into(),
                description: Some(
                    "Falls die Schlammstapelbehälter nicht geschlossen sind, sondern zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                },
            },
            Field {
                id: ProfileValueId::from(SewageSludgeTreatmentId::SewageSludgeForDisposal).into(),
                description: Some(
                    "Angabe der Menge an Klärschlamm in Tonnen (t) die zur Entsorgung anfallen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Masse entwässert".to_string(),
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
                id: ProfileValueId::from(SewageSludgeTreatmentId::TransportDistance).into(),
                description: Some(
                    "Entfernung von Ihrer Kläranlage zum Entsorgungsort des Klärschlamms in Kilometer (km). Die Angabe ist unabhängig von der Entsorgungsart (z.B. Verbrennung) oder der Transportform (z.B. entwässert/trocken). Falls der Klärschlamm auf Ihrer Kläranlage entsorgt wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Entfernung".to_string(),
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
        title: Some("Nebenstrombehandlung"),
        fields: vec![
            Field {
                id: ProfileValueId::from(SideStreamTreatmentId::TotalNitrogen).into(),
                description: Some(
                    "boak",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gesamtstickstoff".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "t/a",
                },
            },
        ],
    },
    FieldSet {
        title: Some("Eingesetzte Betriebsstoffe"),
        fields: vec![
            Field {
                id: ProfileValueId::from(OperatingMaterialId::FeCl3).into(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisen(III)-chlorid (FeCl3) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Lösung".to_string(),
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
                id: ProfileValueId::from(OperatingMaterialId::FeClSO4).into(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisenchloridsulfat (FeClSO4) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Lösung".to_string(),
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
                id: ProfileValueId::from(OperatingMaterialId::CaOH2).into(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Kalkhydrat (Ca(OH)2) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Branntkalk".to_string(),
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
                id: ProfileValueId::from(OperatingMaterialId::SyntheticPolymers).into(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an synthetischen Polymeren in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "t Polymere".to_string(),
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
]
}
