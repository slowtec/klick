use leptos::*;
use num_traits::{FromPrimitive, ToPrimitive};

use klick_app_charts::BarChartRadioInput;
use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{
    units::{Ch4ChpEmissionFactorCalcMethod, Tons},
    InputValueId as Id, Value,
};
use klick_presenter::ValueLabel;

use crate::pages::tool::{
    fields::create_field, CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL,
};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn CH4EmissionsCHP(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show: Option<RwSignal<bool>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let selected_scenario = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::SensitivityCH4ChpCalculationMethod)
                .map(Value::as_ch4_chp_emission_factor_calc_method_unchecked)
        })
    });
    let selected_scenario_index = Signal::derive(move || {
        selected_scenario
            .get()
            .as_ref()
            .and_then(ToPrimitive::to_u64)
    });
    let show_ch4_chp = Signal::derive(move || {
        outcome.with(|out| {
            out.sensitivity_ch4_chp_calculations
                .as_ref()
                .map(|out| !out.is_empty() && out.iter().any(|(_, tons, _)| *tons > Tons::zero()))
        }) == Some(true)
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_set = field_set(form_data.write_only(), input_data);
    let (chp_view, _, _) = render_field_sets(vec![field_set], accessibility_always_show);

    // -----   ----- //
    //   Callbacks   //
    // -----   ----- //

    let on_bar_chart_input_changed = move |idx| {
        let Some(method) = Ch4ChpEmissionFactorCalcMethod::from_u64(idx) else {
            log::warn!("Invalid index {idx} for selection of calc method");
            return;
        };
        form_data.update(|d| {
            d.set(
                Id::SensitivityCH4ChpCalculationMethod,
                Some(Value::ch4_chp_emission_factor_calc_method(method)),
            );
        });
    };

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let bar_chart_view = move || {
        outcome.with(|out| {
            out.sensitivity_ch4_chp_calculations.as_ref().map(|out| {
                let data = out
                    .iter()
                    .map(|(szenario, value, factor)| {
                        klick_app_charts::BarChartRadioInputArguments {
                            label: Some(szenario.label()),
                            value: (*value).into(),
                            emission_factor: f64::from(*factor),
                        }
                    })
                    .collect();
                view! {
                  <BarChartRadioInput
                    width = 900.0
                    height = 300.0
                    data
                    selected_bar = selected_scenario_index
                    emission_factor_label = Some("CH₄ EF")
                    aria_label = Some("Ein Balkendiagramm welches verschiedene Szenarien zur Berechnung von Methanemissionen grafisch aufzeigt und gleichzeitig zur Auswahl eines dieser Szenarien verwendet wird.".to_string())
                    on_change = on_bar_chart_input_changed
                  />
                }
            })
        })
    };

    view! {
      <div class = move ||{ if show_ch4_chp.get() { None } else { Some("hidden") } } >
        <Card id = "sensitivity-ch4-chp" title = "Methanemissionen aus Blockheizkraftwerken (BHKW)" bg_color="bg-blue" accessibility_always_show>
          <InfoBox text = "BHKW weisen je nach Modell und Alter unterschiedliche Methanschlupfe auf" accessibility_always_show>
            <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 25)" url = DWA_MERKBLATT_URL>
              "Auch bei der Gasverwertung entstehen prozessbedingte Methan-Emissionen:
              BHKW-Motoren arbeiten nach dem Vier-Takt-Prinzip.
              Dabei sind zum Gasaustausch für eine kurze Zeit beim Übergang vom vierten (Ausstoßen)
              in den ersten (Ansaugen) Takt sowohl das Einlass- als auch das Auslassventil (teilweise) geöffnet.
              Durch diese Überschneidung können unter Umständen geringe Mengen unverbrannten Faulgases in den Abgasstrom gelangen.
              Ottomotoren haben dabei einen Methanschlupf im Bereich von 1 % bis 2 %
              Zündstrahlmotoren (sind für Faulgas nicht relevant) liegen höher in der Größenordnung von 2 % bis 3 %.
              Mikrogasturbinen (typische Leistungsklasse von 30 kW bis 65 kW) können dagegen einen
              Methanschlupf < 1 % erreichen (STMWI 2016)."
            </Cite>
          </InfoBox>

          <p>
            "Mit der folgenden Auswahl bzw. Eingabe eines eigenen Emissionsfaktors (EF) für das BHKW Ihrer Kläranlage
            kann Ihre Klimabilanz bezüglich der Methanemissionen verfeinert abgeschätzt werden:"
          </p>

          <div class="my-4 ml-4">

            { bar_chart_view }

            { chp_view }

            <p>
            "Es ist das Szenario \"" { move ||
                selected_scenario.get().as_ref().map(ValueLabel::label)
            }
            "\" ausgewählt in t CO₂ Äquivalente/Jahr.
            Durch Anklicken kann ein anderes Szenario ausgewählt werden."
            </p>

            <InfoBox text = "Zusatzinformation zum Methanschlupf:" accessibility_always_show>
              <Cite source = "Auszug aus dem DWA-Merkblatt 230-1 (2022, S. 25)" url = DWA_MERKBLATT_URL>
                "Die Gaszusammensetzung, Brennraumtemperatur (Gasfeuchte), Brennraumgestaltung und Betriebsweise beeinflussen die Verbrennungsvorgänge.
                Bei hohen Sauerstoffkonzentrationen (Magerbetrieb), welche für die Reduktion der NOₓ,-Bildung bei hohen Temperaturen notwendig sind,
                steigt der Methanschlupf.
                Neben der Betriebsweise hat auch die Aggregateleistung einen Einfluss auf den Methan-schlupf.
                So hat sich bei Messungen im Betrieb gezeigt, dass unter Volllast in der Regel weniger Methan über das Abgas emittiert wird
                als bei Teillastbetrieb.
                Bei Mikrogasturbinen ist dieser Effekt sehr stark ausgeprägt
                und kann zu einem Anstieg bis auf > 5 % im 60-%-Teillastbetrieb führen (STMWI 2016)."
              </Cite>
            </InfoBox>

            <div class="border-t pt-3 mt-4 border-gray-900/10">
              { move ||
                outcome.with(|outcome|
                  outcome.sensitivity.output.as_ref().map(|out|{
                    view! {
                      <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                        <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Methanemissionen aus Blockheizkraftwerken (BHKW)"</dt>
                        <dd class="text-lg py-1 px-3">
                          { format!("{:.1}", f64::from(out.co2_equivalents.ch4_combined_heat_and_power_plant)).replace('.',",") }
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
      </div>
    }
}

fn field_set(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> FieldSet {
    let custom_factor_field =
        create_field(form_data, input_data, Id::SensitivityCH4ChpCustomFactor);
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}
