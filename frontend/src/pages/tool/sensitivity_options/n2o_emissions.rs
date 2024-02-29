use klick_app_charts::BarChartRadioInput;
use leptos::*;

use klick_domain as domain;

use crate::{
    forms::{render_field_sets, FieldType, MinMax},
    pages::tool::{
        field_sets::{Field, FieldSet},
        fields::{FieldId, ScenarioFieldId},
        FieldSignal,
    },
};

use super::{Card, ScenarioHint};

const N2O_DEFAULT_CUSTOM_FACTOR: f64 = 3.0;

pub fn options(
    // incoming signals
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    barchart_arguments_radio_inputs: ReadSignal<Vec<klick_app_charts::BarChartRadioInputArguments>>,
    selected_scenario_name_n2o: RwSignal<String>,
    // outgoing signals
    selected_scenario_n2o: RwSignal<Option<u64>>,
    custom_factor_n2o: RwSignal<Option<f64>>,
) -> impl IntoView {
    let field_set = field_set();
    let (signals, chp_view, _required_fields) = render_field_sets(vec![field_set]);
    let custom_factor = signals
        .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    create_effect(move |_| {
        if let Some(custom_factor) = custom_factor.get() {
            custom_factor_n2o.set(Some(custom_factor));
        } else {
            custom_factor_n2o.set(Some(N2O_DEFAULT_CUSTOM_FACTOR));
        }
    });
    view! {
      <div class =move || {
        if 1 > 0 {
          None
        } else {
          Some("hidden")
        }
      }>
      <Card title = "Lachgasemissionen">
        <div class="my-4 ml-4">
        <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">Auswahl des Auswertungsszenarios für Lachgasemissionen</h3>
        { move || {
            view! {
              <BarChartRadioInput
                width = 900.0
                height = 300.0
                data  = barchart_arguments_radio_inputs.get()
                selected_bar = selected_scenario_n2o
                emission_factor_label = Some("N₂O EF")
              />
            }
          }
        }
        { chp_view }
        <p>
          "Es ist das Szenario \"" { move || selected_scenario_name_n2o.get() } "\" ausgewählt in t CO₂ Äquivalente/Jahr.
          Durch Anklicken kann ein anderes Szenario ausgewählt werden."
        </p>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          <ScenarioHint output = output.into() />
          { move || {
              output.get().map(|out|
                view! {
                   <p>
                   </p>
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
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
        </div>
      </Card>
      </div>
    }
}

fn field_set() -> FieldSet {
    let id = FieldId::Scenario(ScenarioFieldId::N2oCustomFactor);
    let custom_factor_field = Field {
        id,
        description: Some(
            "Über dieses Eingabefeld können Sie (z.B. anhand einer eigenen Abschätzung
            oder einer Messkampagne) einen Wert für den EF N₂O eintragen.

            <br>Ohne Angabe gibt es keine Auswahl zum Benuzterdefinierten Szenario in der Auswahl zum Szenario
            für die Lachgasemissionen. Nach Eingabe erscheint eine weitere Auswahlmöglichkeit (Benutzerdefiniert),
            diese muss manuell ausgewählt werden, um den eingegebenen Wert zu verwenden.",
        ),
        required: false,
        field_type: FieldType::Float {
            initial_value: None,
            placeholder: Some(
                N2O_DEFAULT_CUSTOM_FACTOR.to_string(),
            ),
            limits: MinMax {
                min: Some(
                    0.0,
                ),
                max: Some(
                    100.0,
                ),
            },
            unit: "%",
        },
    };
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}
