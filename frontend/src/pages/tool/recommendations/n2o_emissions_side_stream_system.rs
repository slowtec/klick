use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_presenter::*;

use crate::pages::tool::{CalculationOutcome, Card};

pub fn options(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
) -> impl IntoView {
    let field_set = field_set(form_data.write_only(), input_data);
    let (form1, _, _) = render_field_sets(vec![field_set]);

    view! {
      <div class = move || { if show_side_stream_controls.get() { None } else { Some("hidden") } } >
       <Card title = "Lachgasemissionen bei der Prozesswasserbehandlung" bg_color="bg-yellow">
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
              outcome.with(|out|out.recommendation.output.as_ref().map(|out|{
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"N₂O Prozesswasserbehandlung"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.n2o_side_stream)).replace('.',",") }
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

fn field_set(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> FieldSet {
    let custom_factor_field = Field {
        label: ScenarioFieldId::N2OSideStreamCoverIsOpen.label(), // TODO: Rename ID
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
            on_change: Callback::new(move |v| {
                form_data.update(|d| {
                    d.optimization_scenario
                        .side_stream_treatment
                        .side_stream_cover_is_closed = Some(v);
                });
            }),
            input: Signal::derive(move || {
                input_data
                    .with(|d| {
                        d.optimization_scenario
                            .side_stream_treatment
                            .side_stream_cover_is_closed
                    })
                    .unwrap_or(false)
            }),
        },
    };
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}
