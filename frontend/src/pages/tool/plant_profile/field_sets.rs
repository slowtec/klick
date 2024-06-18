use leptos::*;

use klick_app_components::forms::{self, *};
use klick_boundary::FormData;
use klick_domain::{InputValueId as Id, Value};
use klick_presenter::ValueLabel;

use crate::pages::tool::fields::{create_field, create_field_type};

#[allow(clippy::too_many_lines)]
pub fn field_sets(form_data: RwSignal<FormData>) -> Vec<FieldSet> {
    let read = form_data.read_only();
    let write = form_data.write_only();

    let field_set_project_name = FieldSet {
        title: None,
        fields: [Id::ProjectName]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
    };

    let field_set_basics = {
        let title = Some("Angaben zur Kläranlage");
        let fields = [Id::PlantName, Id::PopulationEquivalent, Id::Wastewater]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect();
        FieldSet { title, fields }
    };

    [
      field_set_project_name,
      field_set_basics,
      FieldSet {
            title: Some("Zulauf-Parameter (Jahresmittelwerte)"),
            fields: [
                Id::InfluentChemicalOxygenDemand,
                Id::InfluentNitrogen,
                Id::InfluentTotalOrganicCarbohydrates,
            ]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
      },
    FieldSet {
        title: Some("Ablauf-Parameter (Jahresmittelwerte)"),
        fields: vec![
            {
                let id = Id::EffluentChemicalOxygenDemand;
                let placeholder = Some(
                    "CSB".to_string(),
                );
                let field_type = create_field_type(
                      write,
                      read,
                      id,
                      placeholder,
                );
            Field {
                label: id.label(),
                description: Some(
                    "Der Jahresmittelwert des chemischen Sauerstoffbedarf (CSB) des Abwassers im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L).",
                ),
                required: true,
                field_type,
            }},
            {
                  let id = Id::EffluentNitrogen;
                  let placeholder= Some(
                      "Gesamtstickstoff".to_string(),
                  );
                  let field_type = create_field_type(
                        write,
                        read,
                        id,
                        placeholder,
                  );
              Field {
                  label: id.label(),
                  description: Some(
                      "Der Gesamtstickstoff-Gehalt des Abwassers (TN) im Ablauf Ihrer Kläranlage in Milligramm (mg) pro Liter (L) als Jahresmittelwert.",
                  ),
                  required: true,
                  field_type,
              }
            },
        ],
    },
    FieldSet {
        title: Some("Energiebedarf"),
        fields: vec![
            {
                let id = Id::TotalPowerConsumption;
                let placeholder = Some(
                    "Gesamtstrombedarf".to_string(),
                );
                let field_type= create_field_type(
                    write,
                    read,
                    id,
                    placeholder,
                );
                let description = Some(
                    "Der Gesamt-Strombedarf Ihrer Kläranlage in Kilowattstunden (kWh) pro Jahr (a).",
                );
                Field {
                    label: id.label(),
                    description,
                    required: true,
                    field_type,
                }
            },
            {
                let id = Id::OnSitePowerGeneration;
                let placeholder = Some(
                    "Eigenstrom".to_string(),
                );
                let field_type= create_field_type(
                    write,
                    read,
                    id,
                    placeholder,
                );
                let description = Some(
                    "Anteil der Eigenstromerzeugung in Kilowattstunden (kWh) pro Jahr (a). Falls kein Eigenstrom erzeugt wird, dieses Feld bitte freilassen.",
                );
                Field {
                    label: id.label(),
                    description,
                    required: false,
                    field_type,
                }
            },
            {
                let id = Id::EmissionFactorElectricityMix;
                let placeholder = Some(
                    "485".to_string(),
                );
                let description = Some(
                    "Angabe des Emissionsfaktors des von extern bezogenen Strommixes in Gramm (g) CO₂ pro Kilowattstunde (kWh). Falls dieser Wert nicht verfügbar ist, bitte den Referenzwert stehen lassen.",
                );
                let field_type= create_field_type(
                    write,
                    read,
                    id,
                    placeholder,
                );
                Field {
                    label: id.label(),
                    description,
                    required: true,
                    field_type,
                }
            },
            Field {
                label: Id::GasSupply.label(),
                description: Some(
                    "Menge an Gas (Erdgas/Biogas) in Kubikmeter (m³) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Gas von extern bezogen wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Gasbezug".to_string(),
                    ),
                    limits: forms::MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "m³/a",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.gas_supply = v);
                    })
                    , input: Signal::derive(move||{
                        form_data.with(|d|d.plant_profile.energy_consumption.gas_supply)
                    })
                },
            },
            {
              let id = Id::PurchaseOfBiogas;
              Field {
                  label: id.label(),
                  description: Some(
                      "Falls Ihre Kläranlage Biogas von extern bezieht, dieses Feld bitte anklicken.",
                  ),
                  required: false,
                  field_type: create_field_type(
                      write,
                      read,
                      id,
                      None,
                  ),
              }
            },
            Field {
                label: Id::HeatingOil.label(),
                description: Some(
                    "Menge an Heizöl (z.B. für die Beheizung von Gebäuden) in Litern (L) pro Jahr (a) die von einem externen Versorger bezogen werden. Falls an Ihrer Kläranlage kein Heizöl von extern bezogen wird, dieses Feld bitte freilassen."
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Heizölbezug".to_string(),
                    ),
                    limits: forms::MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "L/a",
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.energy_consumption.heating_oil = v);
                    })
                    , input: Signal::derive(move||{
                        form_data.with(|d|d.plant_profile.energy_consumption.heating_oil)
                    })
                },
            },
            Field {
                label: Id::SewageGasProduced.label(),
                description: Some(
                    "Das an Ihrer Kläranlage erzeugte Klärgas in Kubikmeter (m³) pro Jahr (a). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Klärgas".to_string(),
                    ),
                    limits: forms::MinMax {
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
                        form_data.with(|d|d.plant_profile.energy_consumption.sewage_gas_produced)
                    )
                },
            },
            Field {
                label: Id::MethaneFraction.label(),
                description: Some(
                    "Der Methangehalt des an Ihrer Kläranlage erzeugten Klärgases in Prozent (%). Falls an Ihrer Kläranlage kein Klärgas erzeugt wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "62".to_string(),
                    ),
                    limits: forms::MinMax {
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
                        form_data.with(|d|d.plant_profile.energy_consumption.methane_fraction)
                    })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Klärschlammbehandlung"),
        fields: vec![
            {
              let id = Id::SludgeTreatmentDigesterCount;
             let placeholder = Some(
                 "Anzahl Faultürme".to_string(),
             );
             let field_type = create_field_type(
                   write,
                   read,
                   id,
                   placeholder,
             );
            Field {
                label: id.label(),
                description: Some(
                    "Falls auf Ihrer Kläranlage eine Faulung vorhanden ist, dann geben Sie bitte die Anzahl der Faultürme ein. Falls nicht lassen Sie das Feld bitte offen oder tragen eine 0 ein.",
                ),
                required: false,
                field_type,
              }
            },
            Field {
                label: "Schlammtaschen sind geschlossen", // TODO: Invert label of Id::SludgeTreatmentBagsAreOpen.label(),
                description: Some(
                    "Falls die Schlammtaschen des Faulturms / der Faultürme Ihrer Kläranlage geschlossen sind und nicht zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.sewage_sludge_treatment.sludge_bags_are_closed = Some(v));
                    }),
                    input: Signal::derive(move||{
                        form_data.with(|d|d.plant_profile.sewage_sludge_treatment.sludge_bags_are_closed.unwrap_or_default())
                    })
                },
            },
            Field {
                label: "Schlammlagerung ist geschlossen", // TODO: Invert label of Id::SludgeTreatmentStorageContainersAreOpen.label(),
                description: Some(
                    "Falls die Schlammstapelbehälter Ihrer Kläranlage dicht abgedeckt sind, dann dieses Feld bitte anklicken.",
                ),
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                    on_change: Callback::new(move|v|{
                        form_data.update(|d|d.plant_profile.sewage_sludge_treatment.sludge_storage_containers_are_closed = Some(v));
                    }),
                    input: Signal::derive(move||{
                        form_data.with(|d|d.plant_profile.sewage_sludge_treatment.sludge_storage_containers_are_closed.unwrap_or_default())
                    })
                },
            },
            Field {
                label: Id::SludgeTreatmentDisposal.label(),
                description: Some(
                    "Angabe der Menge an Klärschlamm in Tonnen (t) die zur Entsorgung anfallen.",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Masse entwässert".to_string(),
                    ),
                    limits: forms::MinMax {
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
                        form_data.with(|d|d.plant_profile.sewage_sludge_treatment.sewage_sludge_for_disposal)
                    })
                },
            },
            Field {
                label: Id::SludgeTreatmentTransportDistance.label(),
                description: Some(
                    "Entfernung von Ihrer Kläranlage zum Entsorgungsort des Klärschlamms in Kilometer (km). Die Angabe ist unabhängig von der Entsorgungsart (z.B. Verbrennung) oder der Transportform (z.B. entwässert/trocken). Falls der Klärschlamm auf Ihrer Kläranlage entsorgt wird, dieses Feld bitte freilassen.",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Entfernung".to_string(),
                    ),
                    limits: forms::MinMax {
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
                        form_data.with(|d|d.plant_profile.sewage_sludge_treatment.transport_distance)
                    })
                },
            },
        ],
    },
    FieldSet {
        title: Some("Prozesswasserbehandlung"),
        fields: vec![
            {
                let id = Id::SideStreamTreatmentTotalNitrogen;
                let on_change = Callback::new(move|v: Option<_>|{
                    form_data.update(|d|d.set(id,v.map(Value::tons)));
                });
                let input = Signal::derive(move||{
                    form_data.with(|d|d.get(&id).map(Value::as_tons_unchecked).map(f64::from))
                });
                Field {
                    label: id.label(),
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
                        limits: forms::MinMax {
                            min: None,
                            max: None,
                        },
                        unit: "t/a",
                        on_change,
                        input,
                    }
                }
            }
        ],
    },
    FieldSet {
        title: Some("Eingesetzte Betriebsstoffe"),
        fields: vec![{
            let id = Id::OperatingMaterialFeCl3;
            let on_change = Callback::new(move|v: Option<_>|{
                form_data.update(|d|d.set(id,v.map(Value::tons)));
            });
            let input = Signal::derive(move||{
                form_data.with(|d|d.get(&id).map(Value::as_tons_unchecked).map(f64::from))
            });
            Field {
                label: id.label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisen(III)-chlorid (FeCl3) in Tonnen (t).",
                ),
                required: true,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Lösung".to_string(),
                    ),
                    limits: forms::MinMax {
                        min: None,
                        max: Some(
                            500_000.0,
                        ),
                    },
                    unit: "t",
                    on_change,
                    input,
                },
            }},
            {
              let id = Id::OperatingMaterialFeClSO4;
              let on_change = Callback::new(move|v: Option<_>|{
                  form_data.update(|d|d.set(id,v.map(Value::tons)));
              });
              let input = Signal::derive(move||{
                  form_data.with(|d|d.get(&id).map(Value::as_tons_unchecked).map(f64::from))
              });
              Field {
                label: id.label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Eisenchloridsulfat (FeClSO4) in Tonnen (t).",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Lösung".to_string(),
                    ),
                    limits: forms::MinMax {
                        min: None,
                        max: Some(
                            100_000.0,
                        ),
                    },
                    unit: "t",
                    on_change,
                    input
                },
            }},
            {
              let id = Id::OperatingMaterialCaOH2;
              let on_change = Callback::new(move|v: Option<_>|{
                  form_data.update(|d|d.set(id,v.map(Value::tons)));
              });
              let input = Signal::derive(move||{
                  form_data.with(|d|d.get(&id).map(Value::as_tons_unchecked).map(f64::from))
              });
              Field {
                label: id.label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an Kalkhydrat (Ca(OH)2) in Tonnen (t).",
                ),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some(
                        "Branntkalk".to_string(),
                    ),
                    limits: forms::MinMax {
                        min: None,
                        max: Some(
                            500_000.0,
                        ),
                    },
                    unit: "t",
                    on_change,
                    input,
                },
            }},
            {
              let id = Id::OperatingMaterialSyntheticPolymers;
              let placeholder = Some(
                  "Polymere".to_string(),
              );
              let field_type = create_field_type(
                  write,
                  read,
                  id,
                  placeholder,
              );

              Field {
                label: id.label(),
                description: Some(
                    "Angabe der pro Jahr (a) eingesetzten Menge an synthetischen Polymeren in Tonnen (t).",
                ),
                required: true,
                field_type,
            }},
        ],
    },
  ].to_vec()
}
