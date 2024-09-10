use leptos::*;
use leptos_fluent::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{InputValueId as Id, Value};

use crate::pages::tool::fields::create_field;
use crate::Lng;

#[allow(clippy::too_many_lines)]
pub fn field_sets(form_data: RwSignal<FormData>, lang: Lng) -> Vec<FieldSet> {
    let read = form_data.into();
    let write = form_data.write_only();
    let draw_border = true;

    let field_set_project_name = FieldSet {
        title: None,
        fields: [Id::ProjectName]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
        draw_border,
    };

    let field_set_basics = {
        let title = match lang {
            Lng::De => Some("Angaben zur Kläranlage"),
            Lng::En => Some("Sewage treatment plant details"),
        };
        let fields = [Id::PlantName, Id::PopulationEquivalent, Id::Wastewater]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect();
        FieldSet {
            title,
            fields,
            draw_border,
        }
    };

    [
        field_set_project_name,
        field_set_basics,
        FieldSet {
            title: match lang {
                Lng::De => Some("Zulauf-Parameter (Jahresmittelwerte)"),
                Lng::En => Some("Inflow parameters (annual averages)"),
            },
            fields: [
                Id::InfluentChemicalOxygenDemand,
                Id::InfluentNitrogen,
                Id::InfluentTotalOrganicCarbohydrates,
            ]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Ablauf-Parameter (Jahresmittelwerte)"),
                Lng::En => Some("Outflow parameters (annual averages)"),
            },
            fields: [Id::EffluentChemicalOxygenDemand, Id::EffluentNitrogen]
                .into_iter()
                .map(|id| create_field(write, read, id))
                .collect(),
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Energiebedarf"),
                Lng::En => Some("Energy requirements"),
            },
            fields: [
                Id::TotalPowerConsumption,
                Id::OnSitePowerGeneration,
                Id::EmissionFactorElectricityMix,
                Id::GasSupply,
                Id::PurchaseOfBiogas,
                Id::HeatingOil,
                Id::SewageGasProduced,
                Id::MethaneFraction,
            ]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Klärschlammbehandlung"),
                Lng::En => Some("Sewage sludge treatment"),
            },
            fields: vec![
                create_field(write, read, Id::SludgeTreatmentDigesterCount),
                Field {
                    label: RwSignal::new(move_tr!("sludge-bags-are-closed").get()).into(), // TODO: Invert label of Id::SludgeTreatmentBagsAreOpen.label(),
                    description: Some(move_tr!("sludge-bags-are-closed-info").get()),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: None,
                        on_change: Callback::new(move |v: bool| {
                            form_data.update(|d| {
                                d.insert(Id::SludgeTreatmentBagsAreOpen, Value::bool(!v));
                            });
                        }),
                        input: Signal::derive(move || {
                            form_data.with(|d| {
                                d.get(&Id::SludgeTreatmentBagsAreOpen)
                                    .cloned()
                                    .map(Value::as_bool_unchecked)
                                    .is_some_and(|v| !v)
                            })
                        }),
                    },
                },
                Field {
                    label: RwSignal::new(move_tr!("sludge-storage-is-closed").get()).into(), // TODO: Invert label of Id::SludgeTreatmentStorageContainersAreOpen.label(),
                    description: Some(move_tr!("sludge-storage-is-closed-info").get()),
                    required: false,
                    field_type: FieldType::Bool {
                        initial_value: None,
                        on_change: Callback::new(move |v: bool| {
                            form_data.update(|d| {
                                d.insert(
                                    Id::SludgeTreatmentStorageContainersAreOpen,
                                    Value::bool(!v),
                                );
                            });
                        }),
                        input: Signal::derive(move || {
                            form_data.with(|d| {
                                d.get(&Id::SludgeTreatmentStorageContainersAreOpen)
                                    .cloned()
                                    .map(Value::as_bool_unchecked)
                                    .is_some_and(|v| !v)
                            })
                        }),
                    },
                },
                create_field(write, read, Id::SludgeTreatmentDisposal),
                create_field(write, read, Id::SludgeTreatmentTransportDistance),
            ],
            draw_border,
        },
        FieldSet {
            // title: Some(move_tr!("used_operating_materials").into()),
            title: match lang {
                Lng::De => Some("Prozesswasserbehandlung"),
                Lng::En => Some("Process water treatment"),
            },
            fields: vec![create_field(
                write,
                read,
                Id::SideStreamTreatmentTotalNitrogen,
            )],
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Eingesetzte Betriebsstoffe"),
                Lng::En => Some("Operating materials used"),
            },
            fields: [
                Id::OperatingMaterialFeCl3,
                Id::OperatingMaterialFeClSO4,
                Id::OperatingMaterialCaOH2,
                Id::OperatingMaterialSyntheticPolymers,
            ]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
            draw_border,
        },
    ]
    .to_vec()
}
