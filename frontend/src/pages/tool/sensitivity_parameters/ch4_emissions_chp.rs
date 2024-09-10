use leptos::*;
use leptos_fluent::*;
use num_traits::{FromPrimitive, ToPrimitive};

use klick_app_charts::BarChartRadioInput;
use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{
    output_value::required,
    units::{Ch4ChpEmissionFactorCalcMethod, Tons},
    InputValueId as Id, OutputValueId as Out, Value,
};
use klick_presenter::{Lng, ValueLabel};

use crate::{
    current_lang,
    pages::tool::{
        fields::create_field, CalculationOutcome, Card, Cite, InfoBox, DWA_MERKBLATT_URL,
    },
};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn CH4EmissionsCHP(
    form_data: RwSignal<FormData>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    lang: Lng,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let selected_scenario = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::SensitivityCH4ChpCalculationMethod)
                .cloned()
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
        sensitivity_outcome.with(|out| {
            out.sensitivity_ch4_chp_calculations
                .as_ref()
                .map(|out| !out.is_empty() && out.iter().any(|(_, tons, _)| *tons > Tons::zero()))
        }) == Some(true)
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_set = field_set(form_data.write_only(), form_data.read_only().into());
    let (chp_view, _, _) = render_field_sets(
        vec![field_set],
        accessibility_always_show_option,
        current_lang(),
    ); // FIXME not sure if this works

    // -----   ----- //
    //   Callbacks   //
    // -----   ----- //

    let on_bar_chart_input_changed = move |idx| {
        let Some(method) = Ch4ChpEmissionFactorCalcMethod::from_u64(idx) else {
            log::warn!("Invalid index {idx} for selection of calc method");
            return;
        };
        form_data.update(|d| {
            d.insert(
                Id::SensitivityCH4ChpCalculationMethod,
                Value::ch4_chp_emission_factor_calc_method(method),
            );
        });
    };

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let bar_chart_view = move || {
        sensitivity_outcome.with(|out| {
            out.sensitivity_ch4_chp_calculations.as_ref().map(|out| {
                let data = out
                    .iter()
                    .map(|(szenario, value, factor)| {
                        klick_app_charts::BarChartRadioInputArguments {
                            label: Some(szenario.label(lang)),
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
                    aria_label = Some(move_tr!("sensitivity-ch4-chp-aria").get())
                    lang = lang
                    on_change = on_bar_chart_input_changed
                  />
                }
            })
        })
    };

    view! {
      <div class = move ||{ if show_ch4_chp.get() { None } else { Some("hidden") } } >
        <Card id = "sensitivity-ch4-chp" title = move_tr!("sensitivity-ch4-chp").get() bg_color="bg-blue" accessibility_always_show_option>
          <InfoBox text = move_tr!("sensitivity-ch4-chp-infobox-1-text").get() accessibility_always_show_option>
            <Cite source = move_tr!("sensitivity-ch4-chp-infobox-1-cite-source").get() url = DWA_MERKBLATT_URL>
              { move_tr!("sensitivity-ch4-chp-infobox-1-cite") }
            </Cite>
          </InfoBox>

          <p>
            <div inner_html={ move_tr!("sensitivity-ch4-chp-p-1") }></div>
          </p>

          <div class="my-4 ml-4">

            { bar_chart_view }

            <p>
            { move_tr!("sensitivity-ch4-chp-scenario") }

            " \"" { move ||
                selected_scenario.get().as_ref().map(|id|id.label(current_lang().get()))
            }
            "\" "
            { move_tr!("sensitivity-ch4-chp-scenario-2") }
            </p>

            <InfoBox text = move_tr!("sensitivity-ch4-chp-infobox-2-text").get() accessibility_always_show_option>
            <Cite source = move_tr!("sensitivity-ch4-chp-infobox-2-cite-source").get() url = DWA_MERKBLATT_URL>
              { move_tr!("sensitivity-ch4-chp-infobox-2-cite") }
            </Cite>
          </InfoBox>

          { chp_view }

            <div class="border-t pt-3 mt-4 border-gray-900/10">
              { move ||
                sensitivity_outcome.with(|outcome|
                  outcome.output.as_ref().map(|out|{
                    view! {
                      <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                        <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::Ch4CombinedHeatAndPowerPlant.label(lang) }</dt>
                        <dd class="text-lg py-1 px-3">
                          { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::Ch4CombinedHeatAndPowerPlant, out).unwrap()), 2) }
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
          </div>
        </Card>
      </div>
    }
}

fn field_set(form_data: WriteSignal<FormData>, input_data: Signal<FormData>) -> FieldSet {
    let custom_factor_field =
        create_field(form_data, input_data, Id::SensitivityCH4ChpCustomFactor);
    let fields = vec![custom_factor_field];
    FieldSet {
        title: None,
        fields,
        draw_border: false,
    }
}
