use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{output_value::required, InputValueId as Id, OutputValueId as Out, Value};
use klick_presenter::{Lng, ValueLabel};

use crate::{
    label_signal,
    pages::tool::{CalculationOutcome, Card},
};

pub fn options(
    form_data: RwSignal<FormData>,
    input_data: Signal<FormData>,
    outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    lang: Lng,
) -> impl IntoView {
    let field_set = field_set(form_data.write_only(), input_data);
    let (form1, _, _) = render_field_sets(
        vec![field_set],
        accessibility_always_show_option,
        crate::current_lang(),
    );

    view! {
      <div class = move || { if show_side_stream_controls.get() { None } else { Some("hidden") } } >
       <Card id = "recommendation-n2o-side-stream" title = "Lachgasemissionen bei der Prozesswasserbehandlung" bg_color="bg-yellow" accessibility_always_show_option>
        <p class="my-2">
          "Da es sich bei Prozesswasserbehandlungsanlagen um relativ kleine Becken handelt, können die
          Lachgasemissionen hier durch Abdeckung und Abluftbehandlung (Oxidation) beseitigt werden."
        </p>
        { form1 }
        <p class="my-2">
        "Im Sinne der Nachhaltigkeit und der Kreislaufschließung kann eine Stickstoffrückgewinnungsanlage integriert werden."
        </p>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          { move || {
              outcome.with(|out|out.output.as_ref().map(|out|{
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::N2oSideStream.label(lang) }</dt>
                    <dd class="text-lg py-1 px-3">
                      { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::N2oSideStream, out).unwrap()), 2) }
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

fn field_set(form_data: WriteSignal<FormData>, input_data: Signal<FormData>) -> FieldSet {
    let id = Id::ScenarioN2OSideStreamCoverIsOpen;
    let custom_factor_field = Field {
        label: label_signal(id),
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
                input_data
                    .with(|d| {
                        d.get(&id)
                            .cloned()
                            .map(Value::as_bool_unchecked)
                            .map(|v| !v)
                    })
                    .unwrap_or(false)
            }),
        },
    };
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
        draw_border: false,
    }
}
