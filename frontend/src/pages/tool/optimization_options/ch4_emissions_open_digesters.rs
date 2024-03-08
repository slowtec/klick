use leptos::*;

use super::Card;
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
    show_sludge_bags_controls: RwSignal<bool>,
    show_sludge_storage_containers_controls: RwSignal<bool>,
    sludge_bags_are_open_recommendation: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open_recommendation: RwSignal<Option<bool>>,
) -> impl IntoView {
    let field_set = field_set1();
    let (signals1, form1, _required_fields) = render_field_sets(vec![field_set]);
    let field_set = field_set2();
    let (signals2, form2, _required_fields) = render_field_sets(vec![field_set]);
    create_effect(move |_| {
        let field_signal1 = signals1
            .get(&ProfileValueId::from(SewageSludgeTreatmentId::SludgeBagsRecommended).into());

        match field_signal1.and_then(FieldSignal::get_bool) {
            Some(v) => sludge_bags_are_open_recommendation.set(Some(v)),
            None => sludge_bags_are_open_recommendation.set(None),
        }
        let field_signal2 = signals2.get(
            &ProfileValueId::from(SewageSludgeTreatmentId::SludgeStorageContainersRecommended)
                .into(),
        );
        match field_signal2.and_then(FieldSignal::get_bool) {
            Some(v) => sludge_storage_containers_are_open_recommendation.set(Some(v)),
            None => sludge_storage_containers_are_open_recommendation.set(None),
        }
    });
    view! {
      <div class = move || { if show_sludge_bags_controls.get() || show_sludge_storage_containers_controls.get() { None } else { Some("hidden") } }>
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
                    true => "".to_string(),
              };
              let show_sludge_storage_containers_controls_class = match show_sludge_storage_containers_controls.get() {
                  false => "hidden".to_string(),
                  true => "".to_string(),
              };
              output.get().map(|out|
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {}", show_sludge_bags_controls_class) }>"CH₄ Schlupf Schlammtaschen"</dt>
                    <dd class={ format!("text-lg py-1 px-3 {}", show_sludge_bags_controls_class) }>
                      { format!("{:.1}", f64::from(out.co2_equivalents.ch4_sludge_bags)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
                    <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {}", show_sludge_storage_containers_controls_class) }>"CH₄ Schlupf Schlammlagerung"</dt>
                    <dd class={ format!("text-lg py-1 px-3 {}", show_sludge_storage_containers_controls_class) }>
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
      </div>
    }
}

fn field_set1() -> FieldSet {
    let id1 = ProfileValueId::from(SewageSludgeTreatmentId::SludgeBagsRecommended).into();
    let custom_factor_field1 = Field {
        id: id1,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };
    let fields = vec![custom_factor_field1];
    FieldSet {
        title: None,
        fields,
    }
}

fn field_set2() -> FieldSet {
    let id2 =
        ProfileValueId::from(SewageSludgeTreatmentId::SludgeStorageContainersRecommended).into();
    let custom_factor_field2 = Field {
        id: id2,
        description: None,
        required: false,
        field_type: FieldType::Bool {
            initial_value: None,
        },
    };
    let fields = vec![custom_factor_field2];
    FieldSet {
        title: None,
        fields,
    }
}
