use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_presenter::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, SewageSludgeTreatmentId, SideStreamTreatmentId, ValueLabel,
};

#[allow(clippy::too_many_lines)]
pub fn field_sets(
    form_data: WriteSignal<FormData>,
    input_data: ReadSignal<FormData>,
) -> Vec<FieldSet> {
    vec![
    FieldSet {
        title: None,
        fields: vec![
            Field {
                label: "Projekt Name",
                description: Some(
                    "In diesem Feld können Sie einen Namen für Ihr Projekt hinterlegen. In der <b>angemeldeten</b> Version,
                    dient der Projektname der Speicherung Ihrer Eingaben/Ergebnisse unter dem Reiter „Projekte“.

                    Wenn Sie sich <b>nicht angemeldet</b> haben, wird der Projektname ausschließlich nur auf Ihrer Festplatte
                    gespeichert und in Ihrem lokalen Browser verarbeitet. Weitere Informationen zur Datenverarbeitung
                    finden Sie in den <b>FAQ</b>."
                ),
                required: false,
                field_type: FieldType::Text {
                    initial_value: None,
                    placeholder: Some("Projektname".to_string()),
                    max_len: None,
                    on_change: Callback::new(move |v|{
                        form_data.update(|d|d.project_title = v);
                    }),
                    input: Signal::derive(move||input_data.with(|d|d.project_title.clone()))
                },
            },
        ],
    },
    FieldSet {
        title: Some("Angaben zur Kläranlage"),
        fields: vec![
            Field {
                label: ProfileValueId::PlantName.label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.plant_name = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d
                        .plant_profile.plant_name.clone())
                    })
                },
            },
            Field {
                label: ProfileValueId::PopulationEquivalent.label(),
                description: Some(
                    "Ausbaugröße Ihrer Kläranlage in Einwohnerwerten (EW) als Summe der angeschlossenen Einwohner (E) und der gewerblichen Einwohnergleichwerte (EGW).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Angeschlossene Einwohner".to_string(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.population_equivalent = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d
                        .plant_profile .population_equivalent)
                    })
                },
            },
            Field {
                label: ProfileValueId::Wastewater.label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.wastewater = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.wastewater) })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Zulauf-Parameter (Jahresmittelwerte)"),
        fields: vec![
            Field {
                label: ProfileValueId::from(AnnualAverageInfluentId::ChemicalOxygenDemand).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.influent_average.chemical_oxygen_demand = v);
                    })
                    , input: Signal::derive(move||
                        input_data.with(|d|d.plant_profile.influent_average.chemical_oxygen_demand)
                    )
                },
            },
            Field {
                label: ProfileValueId::from(AnnualAverageInfluentId::Nitrogen).label(),
                description: Some(
                    "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Zulauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gesamtstickstoff".to_string(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.influent_average.total_nitrogen = v);
                    })
                    , input: Signal::derive(move||
                      input_data.with(|d|d.plant_profile.influent_average.total_nitrogen)
                    )
                },
            },
            Field {
                label: ProfileValueId::from(AnnualAverageInfluentId::TotalOrganicCarbohydrates).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.influent_average.total_organic_carbohydrates = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.influent_average.total_organic_carbohydrates)
                    })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Ablauf-Parameter (Jahresmittelwerte)"),
        fields: vec![
            Field {
                label: ProfileValueId::from(AnnualAverageEffluentId::ChemicalOxygenDemand).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.effluent_average.chemical_oxygen_demand = v);
                    })
                    , input: Signal::derive(move|| input_data.with(|d|d .plant_profile.effluent_average.chemical_oxygen_demand))
                },
            },
            Field {
                label: ProfileValueId::from(AnnualAverageEffluentId::Nitrogen).label(),
                description: Some(
                    "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gesamtstickstoff".to_string(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.effluent_average.total_nitrogen = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile                   .effluent_average.total_nitrogen)
                    })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Energiebedarf"),
        fields: vec![
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::TotalPowerConsumption).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.total_power_consumption = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.              plant_profile     .energy_consumption.total_power_consumption)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::OnSitePowerGeneration).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.on_site_power_generation = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.energy_consumption.on_site_power_generation)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::EmissionFactorElectricityMix).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.emission_factor_electricity_mix = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.energy_consumption.emission_factor_electricity_mix)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::GasSupply).label(),
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
                    unit: "m³/a",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.gas_supply = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.energy_consumption.gas_supply)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::PurchaseOfBiogas).label(),
                description: Some(
                    "Falls Ihre Kläranlage Biogas von extern bezieht, dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.purchase_of_biogas = Some(v));
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.energy_consumption.purchase_of_biogas.unwrap_or_default())
                    })
                },
            },
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::HeatingOil).label(),
                description: Some(
                    "Menge an Heizöl (z.B. für die Beheizung von Gebäuden) in Litern (L) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Heizöl von extern bezogen wird, dieses Feld bitte freilassen."
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.heating_oil = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.energy_consumption.heating_oil)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::SewageGasProduced).label(),
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
                    unit: "m³/a",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.sewage_gas_produced = v);
                    })
                    , input: Signal::derive(move||
                        input_data.with(|d|d.plant_profile.energy_consumption.sewage_gas_produced)
                    )
                },
            },
            Field {
                label: ProfileValueId::from(EnergyConsumptionId::MethaneFraction).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.methane_fraction = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.energy_consumption.methane_fraction)
                    })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Klärschlammbehandlung"),
        fields: vec![
            Field {
                label: ProfileValueId::from(SewageSludgeTreatmentId::DigesterCount).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.sewage_sludge_treatment.digester_count = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.sewage_sludge_treatment.digester_count)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(SewageSludgeTreatmentId::SludgeBags).label(),
                description: Some(
                    "Falls die Schlammtaschen des Faulturms / der Faultürme Ihrer Kläranlage geschlossen sind und nicht zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.sewage_sludge_treatment.sludge_bags_are_closed = Some(v));
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.sewage_sludge_treatment.sludge_bags_are_closed.unwrap_or_default())
                    })
                },
            },
            Field {
                label: ProfileValueId::from(SewageSludgeTreatmentId::SludgeStorageContainers).label(),
                description: Some(
                    "Falls die Schlammstapelbehälter Ihrer Kläranlage dicht abgedeckt sind, dann dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.sewage_sludge_treatment.sludge_storage_containers_are_closed = Some(v));
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.sewage_sludge_treatment.sludge_storage_containers_are_closed.unwrap_or_default())
                    })
                },
            },
            Field {
                label: ProfileValueId::from(SewageSludgeTreatmentId::SewageSludgeForDisposal).label(),
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.sewage_sludge_treatment.sewage_sludge_for_disposal = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.sewage_sludge_treatment.sewage_sludge_for_disposal)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(SewageSludgeTreatmentId::TransportDistance).label(),
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
                            2000.0,
                        ),
                    },
                    unit: "km",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.sewage_sludge_treatment.transport_distance = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.sewage_sludge_treatment.transport_distance)
                    })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Prozesswasserbehandlung"),
        fields: vec![
            Field {
                label: ProfileValueId::from(SideStreamTreatmentId::TotalNitrogen).label(),
                description: Some(
                    "Falls auf Ihrer Kläranlage eine Prozesswasserbehandlung vorhanden ist, dann geben Sie bitte deren jährliche
                    Gesamtsticksoffmenge in Tonnen [t/a] ein. Falls nicht lassen Sie das Feld bitte offen oder tragen eine 0 ein. ",
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
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.side_stream_treatment.total_nitrogen = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.side_stream_treatment.total_nitrogen)
                    })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Eingesetzte Betriebsstoffe"),
        fields: vec![
            Field {
                label: ProfileValueId::from(OperatingMaterialId::FeCl3).label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisen(III)-chlorid (FeCl3) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Lösung".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            500_000.0,
                        ),
                    },
                    unit: "t",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.operating_materials.fecl3 = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.operating_materials.fecl3)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(OperatingMaterialId::FeClSO4).label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisenchloridsulfat (FeClSO4) in Tonnen (t).",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Lösung".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            100_000.0,
                        ),
                    },
                    unit: "t",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.operating_materials.feclso4 = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.operating_materials.feclso4)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(OperatingMaterialId::CaOH2).label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Kalkhydrat (Ca(OH)2) in Tonnen (t).",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Branntkalk".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            500_000.0,
                        ),
                    },
                    unit: "t",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.operating_materials.caoh2 = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.operating_materials.caoh2)
                    })
                },
            },
            Field {
                label: ProfileValueId::from(OperatingMaterialId::SyntheticPolymers).label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an synthetischen Polymeren in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Polymere".to_string(),
                    ),
                    limits: MinMax {
                        min: None,
                        max: Some(
                            50000.0,
                        ),
                    },
                    unit: "t",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.operating_materials.synthetic_polymers = v);
                    })
                    , input: Signal::derive(move||{
                        input_data.with(|d|d.plant_profile.operating_materials.synthetic_polymers)
                    })
                },
            },
        ],
    },
]
}
