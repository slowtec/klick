use leptos::*;
use num_traits::{FromPrimitive, ToPrimitive};

use klick_app_charts::BarChartRadioInput;
use klick_app_components::forms::*;
use klick_boundary::{self as boundary, FormData};
use klick_presenter::{Lng, ValueLabel};

use crate::pages::tool::{default_values::N2O_DEFAULT_CUSTOM_FACTOR, CalculationOutcome, Card};

#[component]
pub fn N2OEmissionsSensitivity(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<Option<CalculationOutcome>>,
    show_side_stream_controls: Signal<bool>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let selected_scenario_n2o_number = RwSignal::new(Option::<u64>::None);
    let selected_scenario_n2o = Signal::derive(move || {
        selected_scenario_n2o_number
            .with(|n| n.and_then(boundary::N2oEmissionFactorCalcMethod::from_u64))
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let n2o_custom_factor_field_set = n2o_custom_factor(form_data.write_only(), input_data);
    let (n2o_custom_factor_view, _, _) = render_field_sets(n2o_custom_factor_field_set);

    let side_stream_factor_field_set = side_stream_factor(form_data.write_only(), input_data);
    let (side_stream_factor_view, _, _) = render_field_sets(side_stream_factor_field_set);

    // -----   ----- //
    //    Effects    //
    // -----   ----- //

    create_effect(move |_| {
        let method = selected_scenario_n2o_number
            .get()
            .and_then(boundary::N2oEmissionFactorCalcMethod::from_u64);
        form_data.update(|d| d.sensitivity_parameters.n2o_emissions.calculation_method = method);
    });

    create_effect(move |_| {
        let m = input_data
            .with(|d| d.sensitivity_parameters.n2o_emissions.calculation_method)
            .as_ref()
            .and_then(ToPrimitive::to_u64);
        selected_scenario_n2o_number.set(m);
    });

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let bar_chart_view = move || {
        outcome.with(|out| {
            out.as_ref().map(|out| {
                let data = out
                    .sensitivity_n2o_calculations
                    .iter()
                    .map(
                        |(szenario, outcome)| klick_app_charts::BarChartRadioInputArguments {
                            label: Some(szenario.label()),
                            value: outcome.co2_equivalents.total_emissions.into(),
                            emission_factor: f64::from(outcome.emission_factors.n2o),
                        },
                    )
                    .collect();
                view! {
                  <BarChartRadioInput
                    width = 900.0
                    height = 300.0
                    data
                    selected_bar = selected_scenario_n2o_number
                    emission_factor_label = Some("N₂O EF")
                  />
                }
            })
        })
    };

    view! {
      <Card title = "Lachgasemissionen" bg_color="bg-blue">
        <div class="my-4 ml-4">

          <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">"Lachgasemissionen bei der biologischen Reinigungsstufe"</h3>

          <p class="my-2">
            "Lachgasemissionen tragen wesentlich zum gesamten Treibhausgaspotenzial von Kläranlagen bei.
            Die erste Abschätzung dieses Potenzials bei der Datenerhebung erfolgt mit einem Emissionsfaktor
            für Lachgas (N₂O-EF) nach Parravicini et al. (2016, TU Wien), Wert siehe erster Balken im untenstehenden Diagramm."
          </p>

          <p class="my-2">
            "Da das Auftreten von N₂O-Emissionen in der Regel anlagenspezifisch ist, bietet das KlicK-Tool weitere
            Auswertungsszenarien für Lachgasemissionen an. Diese sind im folgenden Balkendiagramm dargestellt,
            einschließlich der daraus resultierenden Lachgasemissionen [als CO₂-Äquivalente]."
          </p>

          <p class="my-2">
            "Durch Anklicken der einzelnen Balken im Diagramm wird das jeweilige Szenario für
            die untenstehende Gesamtbilanz (im Sankey-Diagramm) verwendet."
          </p>

          { bar_chart_view }

          <p>
            "Es ist das Szenario \"" { move || selected_scenario_n2o.get().as_ref().map(ValueLabel::label) } "\" ausgewählt in t CO₂ Äquivalente/Jahr.
             Durch Anklicken kann ein anderes Szenario ausgewählt werden."
          </p>

          <p class="my-2">
            "Zusätzlich können Sie (z.B. aufgrund einer eigenen Abschätzung oder einer Messkampagne) einen
            benutzerdefinierten Wert für den N₂O-EF eingeben und bilanzieren. Der EF-Faktor erscheint im
            Balkendiagramm und kann anschließend ebenfalls ausgewählt werden."
          </p>

          { n2o_custom_factor_view }

          <div class = move || { if show_side_stream_controls.get() { None } else { Some("hidden") } } >

            <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">"Lachgasemissionen bei der Prozesswasserbehandlung"</h3>

            <p class="my-2">
              "Die Prozesswasserbehandlung in Kläranlagen kann mit erheblichen zusätzlichen Lachgasemissionen verbunden sein.
              Vasilaki et al. (2019) geben in ihrer Metastudie einen Lachgas-EF von 1,7-5,1% des Gesamtstickstoffs im Nebenstrom an."
            </p>

            <p class="my-2">
              "Durch die Eingabe der jährlich behandelten Stickstoffmenge des Prozesswassers [t/a] können
              Sie den resultierenden Anteil an den Treibhausgasemissionen [CO₂-Äq./a] abschätzen."
            </p>

            <p class="my-2">
              "Den hierfür verwendeten N₂O-EF können Sie über das Eingabefeld „N₂O-EF Nebenstrom“ unten frei
              wählen oder leer lassen, um mit einem mittleren EF von 2% (nach Vasilaki et al. 2019) zu rechnen."
            </p>

            { side_stream_factor_view }

          </div>

          <div class="border-t pt-3 mt-4 border-gray-900/10">
            { move ||
              outcome.with(|outcome|
                outcome.as_ref().map(|out|{
                  let out = &out.sensitivity;
                  let show_side_stream_controls_class = match show_side_stream_controls.get() {
                      false => "hidden".to_string(),
                      true => "".to_string(),
                  };
                  view! {
                    <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                      <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"N₂O Anlage"</dt>
                      <dd class="text-lg py-1 px-3">
                        { format!("{:.1}", f64::from(out.co2_equivalents.n2o_plant)).replace('.',",") }
                        <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                      </dd>
                      <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {}", show_side_stream_controls_class) }>"N₂O Prozesswasserbehandlung"</dt>
                      <dd class={ format!("text-lg py-1 px-3 {}", show_side_stream_controls_class) }>
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
                })
              )
            }
          </div>
        </div>
      </Card>
    }
}

fn n2o_custom_factor(
    form_data: WriteSignal<FormData>,
    input_data: ReadSignal<FormData>,
) -> Vec<FieldSet> {
    let custom_factor_field = Field {
        label: "N₂O-EF Benutzerdefiniert",
        description: Some(
            "Über dieses Eingabefeld können Sie (z.B. anhand einer eigenen Abschätzung
            oder einer Messkampagne) einen Wert für den EF N₂O eintragen.

            <br>Weiter muss die Auswahlmöglichkeit (Benutzerdefiniert) manuell ausgewählt werden, um den eingegebenen Wert zu verwenden.",
        ),
        required: false,
        field_type: FieldType::Float {
            initial_value: None,
            placeholder: Some(
                Lng::De.format_number(N2O_DEFAULT_CUSTOM_FACTOR),
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
            on_change: Callback::new(move|v|{
                form_data.update(|d| d.sensitivity_parameters.n2o_emissions.custom_emission_factor = v);
            }),
            input: Signal::derive(move||
                input_data.with(|d|d.sensitivity_parameters.n2o_emissions.custom_emission_factor)
            )
        },
    };
    let fields = vec![custom_factor_field];
    vec![FieldSet {
        title: None,
        fields,
    }]
}

fn side_stream_factor(
    form_data: WriteSignal<FormData>,
    input_data: ReadSignal<FormData>,
) -> Vec<FieldSet> {
    let custom_factor_field = Field {
        label: "N₂O-EF Nebenstrom",
        description: Some(
            "Über dieses Eingabefeld können Sie (z.B. anhand einer eigenen Abschätzung oder
            einer Messkampagne) einen Wert für den EF der Prozesswasserbehandlung eintragen.",
        ),
        required: false,
        field_type: FieldType::Float {
            initial_value: None,
            placeholder: Some(Lng::De.format_number(f64::from(N2O_DEFAULT_CUSTOM_FACTOR))),
            limits: MinMax {
                min: Some(0.0),
                max: Some(100.0),
            },
            unit: "%",
            on_change: Callback::new(move |v| {
                form_data.update(|d| {
                    d.sensitivity_parameters
                        .n2o_emissions
                        .side_stream_emission_factor = v
                });
            }),
            input: Signal::derive(move || {
                input_data.with(|d| {
                    d.sensitivity_parameters
                        .n2o_emissions
                        .side_stream_emission_factor
                })
            }),
        },
    };
    let fields = vec![custom_factor_field];
    vec![FieldSet {
        title: None,
        fields,
    }]
}