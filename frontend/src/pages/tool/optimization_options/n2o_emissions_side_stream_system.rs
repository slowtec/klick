use super::Card;
use leptos::*;
use klick_domain as domain;

use crate::{
    forms::{render_field_sets, FieldType},
    pages::tool::{
        field_sets::{Field, FieldSet},
        fields::{FieldId, ScenarioFieldId},
        FieldSignal,
    },
};

pub fn options(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    n2o_side_stream_cover_is_open: RwSignal<Option<bool>>
) -> impl IntoView {
    let field_set = field_set();
    let (signals1, form1, _required_fields) = render_field_sets(vec![field_set]);
    create_effect(move |_| {
        let field_signal = signals1.get(&FieldId::Scenario(ScenarioFieldId::N2OSideStreamCoverIsOpen));
        match field_signal.and_then(FieldSignal::get_bool) {
            Some(v) => n2o_side_stream_cover_is_open.set(Some(!v)),
            None => n2o_side_stream_cover_is_open.set(None),
        }
    });
    view! {
      <Card title = "Lachgasemissionen von Nebenstromanlagen" bg_color="bg-yellow">
        <p class="my-2">
          "Da es sich bei den Nebenstromanlagen um relativ kleine Becken handelt, können die
          Lachgasemissionen hier durch Abdeckung und Abluftbehandlung (Oxidation) beseitigt werden."
        </p>
        { form1 }
        <p class="my-2">
        "Im Sinne der Nachhaltigkeit und der Kreislaufschließung kann anstelle der Nebenstromanlage eine Stickstoffrückgewinnungsanlage errichtet werden."
        </p>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          { move || {
              output.get().map(|out|
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"N₂O Nebenstromanlage"</dt>
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
              )
            }
          }
        </div>
      </Card>
    }
}

fn field_set() -> FieldSet {
    let id = FieldId::Scenario(ScenarioFieldId::N2OSideStreamCoverIsOpen);
    let custom_factor_field = Field {
        id,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}
