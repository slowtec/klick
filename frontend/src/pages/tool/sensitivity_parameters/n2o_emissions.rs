use leptos::*;
use leptos_fluent::*;
use num_traits::{FromPrimitive, ToPrimitive};

use klick_app_charts::BarChartRadioInput;
use klick_app_components::forms::*;
use klick_boundary::{CalculationOutcome, FormData};
use klick_domain::{
    output_value::*, units::N2oEmissionFactorCalcMethod, InputValueId as In, OutputValueId as Out,
    Value,
};
use klick_presenter::{Lng, ValueLabel};

use crate::pages::tool::{fields::create_field, Card};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn N2OEmissionsSensitivity(
    form_data: RwSignal<FormData>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    lang: Lng,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let selected_scenario = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&In::SensitivityN2OCalculationMethod)
                .cloned()
                .map(Value::as_n2o_emission_factor_calc_method_unchecked)
        })
    });
    let selected_scenario_index = Signal::derive(move || {
        selected_scenario
            .get()
            .as_ref()
            .and_then(ToPrimitive::to_u64)
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let n2o_custom_factor_field_set = n2o_custom_factor(form_data);
    let (n2o_custom_factor_view, _, _) = render_field_sets(
        n2o_custom_factor_field_set,
        accessibility_always_show_option,
        crate::current_lang(),
    );

    let side_stream_factor_field_set = side_stream_factor(form_data);
    let (side_stream_factor_view, _, _) = render_field_sets(
        side_stream_factor_field_set,
        accessibility_always_show_option,
        crate::current_lang(),
    );

    // -----   ----- //
    //   Callbacks   //
    // -----   ----- //

    let on_bar_chart_input_changed = move |idx| {
        let Some(method) = N2oEmissionFactorCalcMethod::from_u64(idx) else {
            log::warn!("Invalid index {idx} for selection of calc method");
            return;
        };
        form_data.update(|d| {
            d.insert(
                In::SensitivityN2OCalculationMethod,
                Value::n2o_emission_factor_calc_method(method),
            );
        });
    };

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let bar_chart_view = move || {
        sensitivity_outcome.with(|outcome| {
            outcome.sensitivity_n2o_calculations.as_ref().map(|out| {
                let data = out
                    .iter()
                    .map(
                        |(szenario, outcome)| klick_app_charts::BarChartRadioInputArguments {
                            label: Some(szenario.label(lang)),
                            value: required!(Out::N2oPlant, outcome).unwrap().into(),
                            emission_factor: f64::from(required!(Out::N2oCalculatedEmissionFactor, outcome).unwrap()),
                        },
                    )
                    .collect();
                view! {
                  <BarChartRadioInput
                    width = 900.0
                    height = 300.0
                    data
                    selected_bar = selected_scenario_index
                    emission_factor_label = Some("N₂O EF")
                    aria_label = Some("Ein Balkendiagramm welches verschiedene Szenarien zur Berechnung von Lachgasemissionen grafisch aufzeigt und gleichzeitig zur Auswahl eines dieser Szenarien verwendet wird.".to_string())
                    lang = lang
                    on_change = on_bar_chart_input_changed
                  />
                }
            })
        })
    };

    view! {
      <Card id = "sensitivity-n2o" title = move_tr!("sensitivity-n2o").get() bg_color="bg-blue" accessibility_always_show_option>
        <div class="my-4 ml-4">

          <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">{move_tr!("n2o_emissions-h3-1").get()}</h3>

          <p class="my-2">
            <div inner_html={ move_tr!("n2o_emissions-p-1") }></div>
          </p>

          <p class="my-2">
            <div inner_html={ move_tr!("n2o_emissions-p-2") }></div>
          </p>

          <p class="my-2">
            <div inner_html={ move_tr!("n2o_emissions-p-3") }></div>
          </p>

          { bar_chart_view }

          <p>
            { move_tr!("n2o_emissions-p-4-1") }
            " \"" { move || selected_scenario.get().as_ref().map(|id|id.label(lang)) } "\" "
             { move_tr!("n2o_emissions-p-4-2") }
          </p>

          <p class="my-2">
          <div inner_html={ move_tr!("n2o_emissions-p-5") }></div>
          </p>

          { n2o_custom_factor_view }

          <div class = move || { if show_side_stream_controls.get() { None } else { Some("hidden") } } >

            <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">{move_tr!("n2o_emissions-h3-2").get()}</h3>

            <p class="my-2">
              <div inner_html={ move_tr!("n2o_emissions-p-6") }></div>
            </p>

            <p class="my-2">
              <div inner_html={ move_tr!("n2o_emissions-p-7") }></div>
            </p>

            <p class="my-2">
              <div inner_html={ move_tr!("n2o_emissions-p-8") }></div>
            </p>

            { side_stream_factor_view }

          </div>

          <div class="border-t pt-3 mt-4 border-gray-900/10">
            { move ||
              sensitivity_outcome.with(|outcome|
                outcome.output.as_ref().map(|out|{
                  let show_side_stream_controls_class = if show_side_stream_controls.get() { String::new() } else { "hidden".to_string() };
                  view! {
                    <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                      <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ Out::N2oPlant.label(lang) }</dt>
                      <dd class="text-lg py-1 px-3">
                        { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::N2oPlant, out).unwrap()), 2) }
                        <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                      </dd>
                      <dt class={ format!("text-lg font-semibold text-right px-3 py-1 text-gray-500 {show_side_stream_controls_class}") }>{ Out::N2oSideStream.label(lang) }</dt>
                      <dd class={ format!("text-lg py-1 px-3 {show_side_stream_controls_class}") }>
                        { crate::current_lang().get().format_number_with_fixed_precision(f64::from(required!(Out::N2oSideStream, out).unwrap()), 2) }
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
    }
}

fn n2o_custom_factor(form_data: RwSignal<FormData>) -> Vec<FieldSet> {
    let id = In::SensitivityN2OCustomFactor;
    let custom_factor_field = create_field(form_data.write_only(), form_data.into(), id);
    let fields = vec![custom_factor_field];
    vec![FieldSet {
        title: None,
        fields,
        draw_border: false,
    }]
}

fn side_stream_factor(form_data: RwSignal<FormData>) -> Vec<FieldSet> {
    let id = In::SensitivityN2OSideStreamFactor;
    let custom_factor_field = create_field(form_data.write_only(), form_data.into(), id);
    let fields = vec![custom_factor_field];
    vec![FieldSet {
        title: None,
        fields,
        draw_border: false,
    }]
}
