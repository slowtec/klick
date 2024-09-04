use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{output_value::required, InputValueId as Id, OutputValueId as Out, Value};

use crate::pages::tool::{CalculationOutcome, Card};

#[allow(clippy::too_many_lines)] // TODO
pub fn options(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let show_sludge_bags_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::SludgeTreatmentBagsAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                != Some(false)
        })
    });

    let show_sludge_storage_containers_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::SludgeTreatmentStorageContainersAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                != Some(false)
        })
    });

    let show_dialog = Signal::derive(move || {
        let digester_count = form_data.with(|d| {
            d.get(&Id::SludgeTreatmentDigesterCount)
                .cloned()
                .map(Value::as_count_unchecked)
                .map(u64::from)
                .unwrap_or_default()
        });
        let sewage_gas_produced = form_data.with(|d| {
            d.get(&Id::SewageGasProduced)
                .cloned()
                .map(Value::as_qubicmeters_unchecked)
                .map(f64::from)
                .unwrap_or_default()
        });
        (show_sludge_bags_controls.get() || show_sludge_storage_containers_controls.get())
            && (sewage_gas_produced > 0.0 || digester_count > 0)
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_set = field_set1(form_data.write_only(), input_data);
    let (form1, _, _) = render_field_sets(vec![field_set], accessibility_always_show_option);

    let field_set = field_set2(form_data.write_only(), input_data);
    let (form2, _, _) = render_field_sets(vec![field_set], accessibility_always_show_option);

    // -----   ----- //
    //     View      //
    // -----   ----- //

    view! {
      <div class = move || { if show_dialog.get() { None } else { Some("hidden") } }>
      <Card id="recommendation-ch4-open-digesters" title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung" bg_color="bg-yellow" accessibility_always_show_option>
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
              let show_sludge_bags_controls_class = if show_sludge_bags_controls.get() { String::new() } else { "hidden".to_string() };
              let show_sludge_storage_containers_controls_class = if show_sludge_storage_containers_controls.get() { String::new() } else { "hidden".to_string() };
              outcome.with(|out|out.output.as_ref().map(|out|{
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_bags_controls_class}") }>"CH₄ Schlupf Schlammtaschen"</dt>
                    <dd class={ format!("text-lg py-1 px-3 {show_sludge_bags_controls_class}") }>
                      { format!("{:.1}", f64::from(required!(Out::Ch4SludgeBags, out).unwrap())).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_storage_containers_controls_class}") }>"CH₄ Schlupf Schlammlagerung"</dt>
                    <dd class={ format!("text-lg py-1 px-3 {show_sludge_storage_containers_controls_class}") }>
                      { format!("{:.1}", f64::from(required!(Out::Ch4SludgeStorageContainers, out).unwrap())).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(required!(Out::TotalEmissions, out).unwrap())).replace('.',",") }
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
    let id = Id::ScenarioSludgeBagsAreOpen;
    let custom_factor_field1 = Field {
        label: RwSignal::new("Schließen der Schlammtaschen".to_string()).into(),
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
            on_change: Callback::new(move |v: bool| {
                form_data.update(|d| {
                    d.insert(id, Value::bool(!v));
                });
            }),
            input: Signal::derive(move || {
                input_data.with(|d| {
                    d.get(&id)
                        .cloned()
                        .map(Value::as_bool_unchecked)
                        .is_some_and(|v| !v)
                })
            }),
        },
    };
    let fields = vec![custom_factor_field1];
    FieldSet {
        title: None,
        fields,

        draw_border: false,
    }
}

fn field_set2(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> FieldSet {
    let id = Id::ScenarioSludgeStorageContainersAreOpen;
    let custom_factor_field2 = Field {
        label: RwSignal::new("Schließen der Schlammlagerung".to_string()).into(),
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
            on_change: Callback::new(move |v: bool| {
                form_data.update(|d| {
                    d.insert(id, Value::bool(!v));
                });
            }),
            input: Signal::derive(move || {
                input_data.with(|d| {
                    d.get(&id)
                        .cloned()
                        .map(Value::as_bool_unchecked)
                        .is_some_and(|v| !v)
                })
            }),
        },
    };
    let fields = vec![custom_factor_field2];
    FieldSet {
        title: None,
        fields,

        draw_border: false,
    }
}
