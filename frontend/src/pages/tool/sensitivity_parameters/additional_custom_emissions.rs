use leptos::*;

use crate::pages::tool::{CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL};

use klick_boundary::FormData;
use klick_codemirror::CodeMirror;
use klick_domain::{
    self as domain, output_value::required, InputValueId as Id, OutputValueId as Out, Value,
};

#[component]
pub fn AdditionalCustomEmissions(
    form_data: RwSignal<FormData>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    custom_emissions_message: RwSignal<String>,
) -> impl IntoView {
    // FIXME
    // How to create a Copy to Clipboard Button:
    // https://github.com/leptos-rs/leptos/discussions/2399
    let helper_node_names = get_all_internal_nodes_names();

    let input_signal: Signal<Option<String>> = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::AdditionalCustomEmissions)
                .cloned()
                .map(Value::as_text_unchecked)
        })
    });

    let errors = RwSignal::new(vec![]);

    view! {
      <Card
        id = "sensitivity-misc-emissions"
        title = "Weitere benutzerdefinierte Emissionen"
        bg_color = "bg-blue"
        accessibility_always_show_option
      >
        <p class="my-2">
          "Sankey-Diagramm Erweiterung mit Ihren Werten."
        </p>
        <CodeMirror
          input = input_signal
          on_change = Callback::new(move |value: Option<String>| {
              form_data.update(|d|{
                match value {
                    Some(v) => {
                      d.insert(Id::AdditionalCustomEmissions, Value::text(v));
                    }
                    None => {
                      d.remove(&Id::AdditionalCustomEmissions);
                    }
                }
              });
          })
          errors = errors.into()
        />
        <Show when = move || sensitivity_outcome.with(|out|out.output.is_some())>
          <p class="mt-2 text-sm" style="color: red">
            { custom_emissions_message }
          </p>
        </Show>
        <InfoBox text = "Syntax" accessibility_always_show_option>
          <Cite source = "" url = DWA_MERKBLATT_URL>
            <pre>
              "\"ID\" \"ID\"\n"
              "\"ID\" NUM \"ID\""
            </pre>
          </Cite>
        </InfoBox>
        <InfoBox text = "Namensliste von Sankey-Knoten" accessibility_always_show_option>
          <Cite source = "" url = DWA_MERKBLATT_URL>
            { helper_node_names }
          </Cite>
        </InfoBox>
        <InfoBox text = "Beispiel" accessibility_always_show_option>
          <Cite source = "" url = DWA_MERKBLATT_URL>
            <pre>
              "\"H₂ Generator\" 1223,2 \"TotalEmissions\"\n"
              "\"Kettensäge\" 400 \"Fällmittel\"\n"
              "\"Eisen(II)-sulfat\" 200 \"Fällmittel\"\n"
              "\"Fällmittel\" \"OperatingMaterials\"\n"
              "\"Abfalldeponie\" 23 \"Ch4Emissions\""
            </pre>
          </Cite>
        </InfoBox>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
        { move ||
          sensitivity_outcome.with(|outcome|
            outcome.output.as_ref().map(|out|{
              view! {
                <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Benutzerdefinierte Emissionen"</dt>
                  <dd class="text-lg py-1 px-3">
                    { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::AdditionalCustomEmissions, out).unwrap()), 2) }
                    <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                  </dd>
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                  <dd class="text-lg py-1 px-3">
                    { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::TotalEmissions, out).unwrap()), 2) }
                    <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                  </dd>
                </dl>
              }
            })
          )
        }
        </div>
      </Card>
    }
}

fn get_all_internal_nodes_names() -> String {
    domain::get_all_internal_nodes()
        .iter()
        .fold(String::new(), |mut acc, entry| {
            let sep = if acc.len() == 0 { "" } else { ", " };
            let n = format!("{}\"{:?}\"", sep, entry);
            acc.push_str(&n);
            acc
        })
}
