use leptos::*;
use leptos_fluent::*;

use crate::pages::tool::{CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL};

use klick_boundary::FormData;
use klick_codemirror::CodeMirror;
use klick_domain::{
    self as domain, output_value::required, InputValueId as In, OutputValueId as Out, Value,
};
use klick_presenter::{Lng, ValueLabel};

#[component]
pub fn AdditionalCustomEmissions(
    form_data: RwSignal<FormData>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    custom_emissions_message: RwSignal<String>,
    lang: Lng,
) -> impl IntoView {
    // FIXME
    // How to create a Copy to Clipboard Button:
    // https://github.com/leptos-rs/leptos/discussions/2399
    let helper_node_names = get_all_internal_nodes_names();

    let input_signal: Signal<Option<String>> = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&In::SensitivityAdditionalCustomEmissions)
                .cloned()
                .map(Value::as_text_unchecked)
        })
    });

    let errors = RwSignal::new(vec![]);

    let namelist = match lang {
        Lng::De => "Namensliste von Sankey-Knoten",
        Lng::En => "List of Sankey-Node names",
    };

    let example = match lang {
        Lng::De => "Beispiel",
        Lng::En => "Example",
    };

    view! {
      <Card
        id = "sensitivity-misc-emissions"
        title = move_tr!("sensitivity-custom-emissions").get()
        bg_color = "bg-blue"
        accessibility_always_show_option
      >
        <p class="my-2">
        { move_tr!("sensitivity-custom-emissions-description") }
        </p>
        <CodeMirror
          input = input_signal
          on_change = Callback::new(move |value: Option<String>| {
              form_data.update(|d|{
                match value {
                    Some(v) => {
                      d.insert(In::SensitivityAdditionalCustomEmissions, Value::text(v));
                    }
                    None => {
                      d.remove(&In::SensitivityAdditionalCustomEmissions);
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
        <InfoBox text = "Syntax".to_string() accessibility_always_show_option>
          <Cite source = "".to_string() url = DWA_MERKBLATT_URL>
            <pre>
              "\"ID\" \"ID\"\n"
              "\"ID\" NUM \"ID\""
            </pre>
          </Cite>
        </InfoBox>
        <InfoBox text = namelist.to_string() accessibility_always_show_option>
          <Cite source = "".to_string() url = DWA_MERKBLATT_URL>
            { helper_node_names }
          </Cite>
        </InfoBox>
        <InfoBox text = example.to_string() accessibility_always_show_option>
          <Cite source = "".to_string() url = DWA_MERKBLATT_URL>
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
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::AdditionalCustomEmissions.label(lang) }</dt>
                  <dd class="text-lg py-1 px-3">
                    { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::AdditionalCustomEmissions, out).unwrap()), 2) }
                    <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                  </dd>
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::TotalEmissions.label(lang) }</dt>
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
            let sep = if acc.is_empty() { "" } else { ", " };
            let n = format!("{sep}\"{entry:?}\"");
            acc.push_str(&n);
            acc
        })
}
