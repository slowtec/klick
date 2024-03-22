use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_presenter::*;

use crate::pages::tool::{CalculationOutcome, Card};

pub fn options(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<Option<CalculationOutcome>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let show_sludge_bags_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.plant_profile
                .sewage_sludge_treatment
                .sludge_bags_are_closed
                != Some(true)
        })
    });

    let show_sludge_storage_containers_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.plant_profile
                .sewage_sludge_treatment
                .sludge_storage_containers_are_closed
                != Some(true)
        })
    });

    let show_dialog = Signal::derive(move || {
        let digester_count = form_data.with(|d| {
            d.plant_profile
                .sewage_sludge_treatment
                .digester_count
                .unwrap_or(0)
        });
        let sewage_gas_produced = form_data.with(|d| {
            d.plant_profile
                .energy_consumption
                .sewage_gas_produced
                .unwrap_or(0.0)
        });
        (show_sludge_bags_controls.get() || show_sludge_storage_containers_controls.get())
            && (sewage_gas_produced > 0.0 || digester_count > 0)
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_set = field_set1(form_data.write_only(), input_data);
    let (form1, _, _) = render_field_sets(vec![field_set]);

    let field_set = field_set2(form_data.write_only(), input_data);
    let (form2, _, _) = render_field_sets(vec![field_set]);

    // -----   ----- //
    //     View      //
    // -----   ----- //

    view! {
      <div class = move || { if show_dialog.get() { None } else { Some("hidden") } }>
      <Card title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung" bg_color="bg-yellow">
        <p>
          "Das Schließen von Schlammtaschen an Faultürmen und der Schlammlager wirkt sich durch die Eindämmung von Methanschlupfen positiv auf die Klimabilanz von Kläranlagen aus. Dies können Sie über die nachfolgenden Checkboxen bilanzieren."
        </p>
          <div class = move || { if show_sludge_bags_controls.get() { None } else { Some("hidden") } }>
          {form1}
          </div>
          <div class = move || { if show_sludge_storage_containers_controls.get() { None } else { Some("hidden") } }>
          {form2}
          </div>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          { move || {
              let show_sludge_bags_controls_class = match show_sludge_bags_controls.get() {
                    false => "hidden".to_string(),
                    true => String::new(),
              };
              let show_sludge_storage_containers_controls_class = match show_sludge_storage_containers_controls.get() {
                  false => "hidden".to_string(),
                  true => String::new(),
              };
              outcome.with(|out|out.as_ref().map(|out|{
                let out = &out.recommendation.output;
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_bags_controls_class}") }>"CH₄ Schlupf Schlammtaschen"</dt>
                    <dd class={ format!("text-lg py-1 px-3 {show_sludge_bags_controls_class}") }>
                      { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_bags)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_storage_containers_controls_class}") }>"CH₄ Schlupf Schlammlagerung"</dt>
                    <dd class={ format!("text-lg py-1 px-3 {show_sludge_storage_containers_controls_class}") }>
                      { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_storage_containers)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.total_emissions)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                  </dl>
                }
              }))
            }
          }
        </div>
      </Card>
      </div>
    }
}

fn field_set1(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> FieldSet {
    let custom_factor_field1 = Field {
        label: SewageSludgeTreatmentId::SludgeBagsRecommended.label(),
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
            on_change: Callback::new(move |v| {
                form_data.update(|d| {
                    d.optimization_scenario
                        .sewage_sludge_treatment
                        .sludge_bags_are_closed = Some(v);
                });
            }),
            input: Signal::derive(move || {
                input_data.with(|d| {
                    d.optimization_scenario
                        .sewage_sludge_treatment
                        .sludge_bags_are_closed
                        .unwrap_or(false)
                })
            }),
        },
    };
    let fields = vec![custom_factor_field1];
    FieldSet {
        title: None,
        fields,
    }
}

fn field_set2(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> FieldSet {
    let custom_factor_field2 = Field {
        label: SewageSludgeTreatmentId::SludgeStorageContainersRecommended.label(),
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
            on_change: Callback::new(move |v| {
                form_data.update(|d| {
                    d.optimization_scenario
                        .sewage_sludge_treatment
                        .sludge_storage_containers_are_closed = Some(v);
                });
            }),
            input: Signal::derive(move || {
                input_data.with(|d| {
                    d.optimization_scenario
                        .sewage_sludge_treatment
                        .sludge_storage_containers_are_closed
                        .unwrap_or(false)
                })
            }),
        },
    };
    let fields = vec![custom_factor_field2];
    FieldSet {
        title: None,
        fields,
    }
}
