use leptos::*;

use super::{Card};
use klick_domain as domain;
use klick_presenter::ProfileValueId;
use klick_presenter::SewageSludgeTreatmentId;

use crate::{
  forms::{render_field_sets, FieldType},
  pages::tool::{
    field_sets::{Field, FieldSet},
    FieldSignal,
  },
};
pub fn options(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    sludge_bags_are_open_recommendation: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open_recommendation: RwSignal<Option<bool>>,
) -> impl IntoView {
  let field_set = field_set();
  let (signals1, form1, _required_fields) = render_field_sets(vec![field_set]);
  create_effect(move |_| {
    let field_signal = signals1.get(&ProfileValueId::from(SewageSludgeTreatmentId::SludgeBagsRecommended).into());

    match field_signal.and_then(FieldSignal::get_bool) {
      Some(v) => sludge_bags_are_open_recommendation.set(Some(v)),
      None => sludge_bags_are_open_recommendation.set(None),
    }
  let field_signal = signals1.get(&ProfileValueId::from(SewageSludgeTreatmentId::SludgeStorageContainersRecommended).into());
    match field_signal.and_then(FieldSignal::get_bool) {
      Some(v) => sludge_storage_containers_are_open_recommendation.set(Some(v)),
      None => sludge_storage_containers_are_open_recommendation.set(None),
    }
  });
    view! {
      <Card title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung" bg_color="bg-yellow">
        <p>
          "Das Schließen von Schlammtaschen an Faultürmen und der Schlammlager wirkt sich durch die Eindämmung von Methanschlupfen positiv auf die Klimabilanz von Kläranlagen aus. Dies können Sie über die nachfolgenden Checkboxen bilanzieren."
        </p>
        { form1 }
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          { move || {
              output.get().map(|out|
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Schließen der Schlammtaschen"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_bags)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>

                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Schließen der Schlammlagerung"</dt>
                    <dd class="text-lg py-1 px-3">
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
              )
            }
          }
        </div>
      </Card>
    }
}

fn field_set() -> FieldSet {
  let id1 = ProfileValueId::from(SewageSludgeTreatmentId::SludgeBagsRecommended).into();
  let id2 = ProfileValueId::from(SewageSludgeTreatmentId::SludgeStorageContainersRecommended).into();
  let custom_factor_field1 = Field {
    id: id1,
    description: None,
    required: false,
    field_type: FieldType::Bool {
      initial_value: None,
    },
  };
  let custom_factor_field2 = Field {
    id: id2,
    description: None,
    required: false,
    field_type: FieldType::Bool {
      initial_value: None,
    },
  };
  let fields = vec![custom_factor_field1, custom_factor_field2];
  FieldSet {
    title: None,
    fields,
  }
}