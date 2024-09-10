use leptos::*;
use leptos_fluent::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{output_value::required, InputValueId as Id, OutputValueId as Out, Value};

use crate::pages::tool::{CalculationOutcome, Card};
use klick_presenter::{Lng, ValueLabel};

#[allow(clippy::too_many_lines)] // TODO
pub fn options(
    form_data: RwSignal<FormData>,
    input_data: Signal<FormData>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    lang: Lng,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let show_sludge_bags_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::ProfileSludgeTreatmentBagsAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                != Some(false)
        })
    });

    let show_sludge_storage_containers_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::ProfileSludgeTreatmentStorageContainersAreOpen)
                .cloned()
                .map(Value::as_bool_unchecked)
                != Some(false)
        })
    });

    let show_dialog = Signal::derive(move || {
        let digester_count = form_data.with(|d| {
            d.get(&Id::ProfileSludgeTreatmentDigesterCount)
                .cloned()
                .map(Value::as_count_unchecked)
                .map(u64::from)
                .unwrap_or_default()
        });
        let sewage_gas_produced = form_data.with(|d| {
            d.get(&Id::ProfileSewageGasProduced)
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
    let (form1, _, _) = render_field_sets(
        vec![field_set],
        accessibility_always_show_option,
        crate::current_lang(),
    );

    let field_set = field_set2(form_data.write_only(), input_data);
    let (form2, _, _) = render_field_sets(
        vec![field_set],
        accessibility_always_show_option,
        crate::current_lang(),
    );

    // -----   ----- //
    //     View      //
    // -----   ----- //
    view! {
      <div class = move || { if show_dialog.get() { None } else { Some("hidden") } }>
      <Card id="recommendation-ch4-open-digesters" title = move_tr!("recommendation-methan-emissions").get() bg_color="bg-yellow" accessibility_always_show_option>
        <p>
          <div inner_html={ move_tr!("recommendation-ch4-open-digesters-p-1") }></div>
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
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_bags_controls_class}") }>{ Out::Ch4SludgeBags.label(lang) }</dt>
                    <dd class={ format!("text-lg py-1 px-3 {show_sludge_bags_controls_class}") }>
                      { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::Ch4SludgeBags, out).unwrap()), 2) }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_sludge_storage_containers_controls_class}") }>{ Out::Ch4SludgeStorageContainers.label(lang) }</dt>
                    <dd class={ format!("text-lg py-1 px-3 {show_sludge_storage_containers_controls_class}") }>
                      { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::Ch4SludgeStorageContainers, out).unwrap()), 2) }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::TotalEmissions.label(lang) }</dt>
                    <dd class="text-lg py-1 px-3">
                      { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::TotalEmissions, out).unwrap()), 2) }
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

fn field_set1(form_data: WriteSignal<FormData>, input_data: Signal<FormData>) -> FieldSet {
    let id = Id::RecommendationSludgeBagsAreOpen;
    let custom_factor_field1 = Field {
        label: RwSignal::new(move_tr!("recommendation-ch4-open-closing-sludge-bags").get()).into(),
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

fn field_set2(form_data: WriteSignal<FormData>, input_data: Signal<FormData>) -> FieldSet {
    let id = Id::RecommendationSludgeStorageContainersAreOpen;
    let custom_factor_field2 = Field {
        label: RwSignal::new(move_tr!("recommendation-ch4-open-closing-sludge-storage").get())
            .into(),
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
