use leptos::*;
use num_traits::{FromPrimitive, ToPrimitive};

use klick_app_charts::BarChartRadioInput;
use klick_app_components::forms::*;
use klick_boundary::{self as boundary, FormData};
use klick_presenter::{Lng, ValueLabel};

use crate::pages::tool::{
    default_values::CH4_DEFAULT_CUSTOM_FACTOR, CalculationOutcome, Card, Cite, InfoBox,
    DWA_MERKBLATT_URL,
};

#[component]
pub fn CH4EmissionsCHP(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<Option<CalculationOutcome>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let selected_scenario_chp_number = RwSignal::new(Option::<u64>::None);
    let selected_scenario_chp = Signal::derive(move || {
        selected_scenario_chp_number
            .with(|n| n.and_then(boundary::CH4ChpEmissionFactorCalcMethod::from_u64))
    });
    let show_ch4_chp = Signal::derive(move || {
        outcome.with(|out| {
            out.as_ref()
                .map(|out| !out.sensitivity_ch4_chp_calculations.is_empty())
        }) == Some(true)
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_set = field_set(form_data.write_only(), input_data);
    let (chp_view, _, _) = render_field_sets(vec![field_set]);

    // -----   ----- //
    //    Effects    //
    // -----   ----- //

    create_effect(move |_| {
        let method = selected_scenario_chp_number
            .get()
            .and_then(boundary::CH4ChpEmissionFactorCalcMethod::from_u64);
        form_data.update(|d| {
            d.sensitivity_parameters
                .ch4_chp_emissions
                .calculation_method = method
        });
    });

    create_effect(move |_| {
        let m = input_data
            .with(|d| {
                d.sensitivity_parameters
                    .ch4_chp_emissions
                    .calculation_method
            })
            .as_ref()
            .and_then(ToPrimitive::to_u64);
        selected_scenario_chp_number.set(m);
    });

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let bar_chart_view = move || {
        outcome.with(|out| {
            out.as_ref().map(|out| {
                let data = out
                    .sensitivity_ch4_chp_calculations
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
                    selected_bar = selected_scenario_chp_number
                    emission_factor_label = Some("CH₄ EF")
                  />
                }
            })
        })
    };

    view! {
      <div class = move ||{ if show_ch4_chp.get() { None } else { Some("hidden") } } >
        <Card title = "Methanemissionen aus Blockheizkraftwerken (BHKW)" bg_color="bg-blue">
          <InfoBox text = "BHKW weisen je nach Modell und Alter unterschiedliche Methanschlupfe auf">
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
            "Es ist das Szenario \"" { move || selected_scenario_chp.get().as_ref().map(ValueLabel::label) } "\" ausgewählt in t CO₂ Äquivalente/Jahr.
            Durch Anklicken kann ein anderes Szenario ausgewählt werden."
            </p>

            <InfoBox text = "Zusatzinformation zum Methanschlupf:">
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
                  outcome.as_ref().map(|out|{
                    view! {
                      <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                        <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Methanemissionen aus Blockheizkraftwerken (BHKW)"</dt>
                        <dd class="text-lg py-1 px-3">
                          { format!("{:.1}", f64::from(out.sensitivity.co2_equivalents.ch4_combined_heat_and_power_plant)).replace('.',",") }
                          <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                        </dd>
                        <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Gesamtemissionen"</dt>
                        <dd class="text-lg py-1 px-3">
                          { format!("{:.1}", f64::from(out.sensitivity.co2_equivalents.total_emissions)).replace('.',",") }
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
    let custom_factor_field = Field {
        label: "",
        description: Some("Über dieses Eingabefeld können Sie (z.B. basierend auf einer eigenen Abschätzung oder einer Messkampagne) einen Wert für den EF CH₄ eintragen."),
        required: false,
        field_type: FieldType::Float {
            initial_value: None,
            placeholder: Some(Lng::De.format_number(f64::from(CH4_DEFAULT_CUSTOM_FACTOR))),
            limits: MinMax {
                min: Some(0.0),
                max: Some(100.0),
            },
            unit: "%",
            on_change: Callback::new(move |v| {
                form_data.update(|d| {
                    d.sensitivity_parameters
                        .ch4_chp_emissions
                        .custom_emission_factor = v
                });
            }),
            input: Signal::derive(move || {
                input_data.with(|d| {
                    d.sensitivity_parameters
                        .ch4_chp_emissions
                        .custom_emission_factor
                })
            }),
        },
    };
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
    }
}
