use leptos::*;

use super::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};
use klick_domain as domain;


pub fn options(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,

) -> impl IntoView {
    // let custom_factor_field = Field {
    //     id: FieldId::Scenario(ScenarioFieldId::SludgeBagsCustomFactor),
    //     description: Some("Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen."),
    //     required: false,
    //     field_type: FieldType::Float {
    //         initial_value: None,
    //         placeholder: Some(Lng::De.format_number(f64::from(klick_domain::constants::EMISSION_FACTOR_SLUDGE_BAGS))),
    //         limits: MinMax {
    //             min: Some(0.0),
    //             max: Some(100.0),
    //         },
    //         unit: "m³(CH₄)/h",
    //     },
    // };
    //
    // let field_set = FieldSet {
    //     title: None,
    //     fields: vec![custom_factor_field],
    // };
    //
    // let (signals1, fields_view1, _required_fields) = render_field_sets(vec![field_set]);

    view! {
      <Card title = "Methanemissionen aus offenen Faultürmen und bei der Schlammlagerung" bg_color="bg-yellow">
        <p>
          "Das Schließen von Schlammtaschen an Faultürmen und der Schlammlager wirkt sich durch die Eindämmung von Methanschlupfen positiv auf die Klimabilanz von Kläranlagen aus. Dies können Sie über die nachfolgenden Checkboxen bilanzieren."

        </p>
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
//
//
// let field_signal = signals1.get(&Id::SludgeBags.into());
// match field_signal.and_then(FieldSignal::get_bool) {
// Some(v) => sludge_bags_are_open.set(Some(!v)),
// None => sludge_bags_are_open.set(None),
// }
//
// let field_signal = signals2.get(&Id::SludgeStorageContainers.into());
// match field_signal.and_then(FieldSignal::get_bool) {
// Some(v) => sludge_storage_containers_are_open.set(Some(!v)),
// None => sludge_storage_containers_are_open.set(None),
// }