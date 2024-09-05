use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{InputValueId as Id, Value};

use crate::pages::tool::fields::create_field;

#[allow(clippy::too_many_lines)]
pub fn field_sets(form_data: RwSignal<FormData>) -> Vec<FieldSet> {
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
        let title = Some("Angaben zur Kläranlage");
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
            title: Some("Zulauf-Parameter (Jahresmittelwerte)"),
            fields: [
                Id::InfluentChemicalOxygenDemand,
                Id::InfluentNitrogen,
                Id::InfluentTotalOrganicCarbohydrates,
            ]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
            draw_border
        },
    FieldSet {
        title: Some("Ablauf-Parameter (Jahresmittelwerte)"),
        fields: [ Id::EffluentChemicalOxygenDemand, Id::EffluentNitrogen]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
        draw_border
    },
    FieldSet {
        title: Some("Energiebedarf"),
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
        title: Some("Klärschlammbehandlung"),
        fields: vec![
            create_field(write, read, Id::SludgeTreatmentDigesterCount),
            Field {
                label: RwSignal::new("Schlammtaschen sind geschlossen".to_string()).into(), // TODO: Invert label of Id::SludgeTreatmentBagsAreOpen.label(),
                description: Some(
                    "Falls die Schlammtaschen des Faulturms / der Faultürme Ihrer Kläranlage geschlossen sind und nicht zur Umgebungsluft offen sind, dann dieses Feld bitte anklicken.".to_string(),
                ), // TODO: move to locales and use a key here
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                    on_change: Callback::new(move|v: bool|{
                        form_data.update(|d|{d.insert(Id::SludgeTreatmentBagsAreOpen, Value::bool(!v));});
                    }),
                    input: Signal::derive(move||{
                        form_data.with(|d|d.get(&Id::SludgeTreatmentBagsAreOpen)
                          .cloned()
                          .map(Value::as_bool_unchecked)
                          .is_some_and(|v|!v)
                        )
                    })
                },
            },
            Field {
                label: RwSignal::new("Schlammlagerung ist geschlossen".to_string()).into(), // TODO: Invert label of Id::SludgeTreatmentStorageContainersAreOpen.label(),
                description: Some(
                    "Falls die Schlammstapelbehälter Ihrer Kläranlage dicht abgedeckt sind, dann dieses Feld bitte anklicken.".to_string(),
                ),  // TODO: move to locales and use a key here 
                required: false,
                field_type: FieldType::Bool {
                    initial_value: None,
                    on_change: Callback::new(move|v: bool|{
                        form_data.update(|d|{d.insert(Id::SludgeTreatmentStorageContainersAreOpen, Value::bool(!v)); });
                    }),
                    input: Signal::derive(move||{
                        form_data.with(|d|d.get(&Id::SludgeTreatmentStorageContainersAreOpen)
                          .cloned()
                          .map(Value::as_bool_unchecked)
                          .is_some_and(|v|!v)
                        )
                    })
                },
            },
            create_field(write, read, Id::SludgeTreatmentDisposal),
            create_field(write, read, Id::SludgeTreatmentTransportDistance),
        ],
        draw_border,
    },
    FieldSet {
        title: Some("Prozesswasserbehandlung"),
        fields: vec![create_field(write, read, Id::SideStreamTreatmentTotalNitrogen)],
        draw_border,
    },
    FieldSet {
        title: Some("Eingesetzte Betriebsstoffe"),
        fields: [
            Id::OperatingMaterialFeCl3,
            Id::OperatingMaterialFeClSO4,
            Id::OperatingMaterialCaOH2,
            Id::OperatingMaterialSyntheticPolymers,
        ]
            .into_iter()
            .map(|id| create_field(write, read, id))
            .collect(),
            draw_border
        },
  ].to_vec()
}
