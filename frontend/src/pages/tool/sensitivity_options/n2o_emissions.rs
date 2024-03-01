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
const N2O_DEFAULT_SECONDARY_POWER_FACTOR: f64 = 2.0;

pub fn options(
    // incoming signals
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    barchart_arguments_radio_inputs: ReadSignal<Vec<klick_app_charts::BarChartRadioInputArguments>>,
    selected_scenario_name_n2o: RwSignal<String>,
    // outgoing signals
    selected_scenario_n2o: RwSignal<Option<u64>>,
    custom_factor_n2o: RwSignal<Option<f64>>,
    // custom_secondary_factor_n2o: RwSignal<Option<f64>>,// FIXME placeholder
) -> impl IntoView {
    let custom_secondary_factor_n2o: RwSignal<Option<f64>> = RwSignal::new(None); // FIXME placeholder
    let field_set1 = field_set1();
    let (signals1, form1, _required_fields) = render_field_sets(vec![field_set1]);
    let custom_factor1 = signals1
        .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    let field_set2 = field_set2();
    let (signals2, form2, _required_fields) = render_field_sets(vec![field_set2]);
    let custom_factor2 = signals2
        .get(&FieldId::Scenario(ScenarioFieldId::N2oSecondaryPowerFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    create_effect(move |_| {
        if let Some(custom_factor1) = custom_factor1.get() {
            custom_factor_n2o.set(Some(custom_factor1));
        } else {
            custom_factor_n2o.set(Some(N2O_DEFAULT_CUSTOM_FACTOR));
        }
        if let Some(custom_factor2) = custom_factor2.get() {
            custom_secondary_factor_n2o.set(Some(custom_factor2));
        } else {
            custom_secondary_factor_n2o.set(Some(N2O_DEFAULT_SECONDARY_POWER_FACTOR));
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
      <Card title = "Lachgasemissionen" bg_color="bg-blue">
        <div class="my-4 ml-4">
        <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">Lachgasemissionen bei der biologischen Reinigungsstufe</h3>
        <p class="my-2">
        "Lachgasemissionen tragen wesentlich zum gesamten Treibhausgaspotenzial von Kläranlagen bei.
        Die erste Abschätzung dieses Potenzials bei der Datenerhebung erfolgt mit einem Emissionsfaktor
        für Lachgas (N2O-EF) von XX % nach Parravicini et al. (2016, TU Wien)."
        </p>
        <p class="my-2">
        "Da das Auftreten von N₂O-Emissionen in der Regel anlagenspezifisch ist, bietet das KlicK-Tool weitere
        Auswertungsszenarien für Lachgasemissionen an. Diese sind im folgenden Balkendiagramm dargestellt,
        einschließlich der daraus resultierenden Lachgasemissionen [als CO2-Äquivalente]."
        </p>
        <p class="my-2">
        "Durch Anklicken der einzelnen Balken im Diagramm wird das jeweilige Szenario für \
        die untenstehende Gesamtbilanz (im Sankey-Diagramm) verwendet."
        </p>
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
        <p>
          "Es ist das Szenario \"" { move || selected_scenario_name_n2o.get() } "\" ausgewählt in t CO₂ Äquivalente/Jahr.
          Durch Anklicken kann ein anderes Szenario ausgewählt werden."
        </p>
        <p class="my-2">
        "Zusätzlich können Sie (z.B. aufgrund einer eigenen Abschätzung oder einer Messkampagne) einen
        benutzerdefinierten Wert für den N₂O- EF eingeben und bilanzieren. Der EF-Faktor erscheint im
        Balkendiagramm und kann anschließend ebenfalls ausgewählt werden."
        </p>
        { form1 }

        <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">Lachgasemissionen aus Nebenstromanlagen</h3>
        <p class="my-2">
        "Nebenstrombehandlungen in Kläranlagen können mit erheblichen zusätzlichen Lachgasemissionen verbunden sein.
        Vasilaki et al. (2019) geben in ihrer Metastudie einen Lachgas-EF von 1,7-5,1 % des Gesamtstickstoffs im Nebenstrom an."
        </p>
        <p class="my-2">
        "Durch die Eingabe der jährlich behandelten Stickstoffmenge des Nebenstromprozesses [t/a] können Sie den
        resultierenden Anteil an den Treibhausgasemissionen [CO2-Äq./a] abschätzen."
        </p>
        <p class="my-2">
        "Den hierfür verwendeten N2O-EF können Sie über das Eingabefeld „N2O-EF Nebenstrom“ unten frei wählen oder
        leer lassen, um mit einem mittleren EF von 2 % (nach Vasilaki et al. 2019) zu rechnen."
        </p>
        { form2 }
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

fn field_set1() -> FieldSet {
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

fn field_set2() -> FieldSet {
    let id = FieldId::Scenario(ScenarioFieldId::N2oSecondaryPowerFactor);
    let custom_factor_field = Field {
        id,
        description: Some(
            "Über dieses Eingabefeld können Sie (z.B. anhand einer eigenen Abschätzung
            oder einer Messkampagne) einen Wert für den EF Nebenstromanlagen eintragen."
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