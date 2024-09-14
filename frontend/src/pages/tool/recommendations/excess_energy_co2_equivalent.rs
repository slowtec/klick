use leptos::*;
use leptos_fluent::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{
    required_output_value_id as required, units::*, InputValueId as Id, OutputValueId as Out,
};

use crate::pages::tool::{fields::create_field, CalculationOutcome, Card};
use klick_presenter::{Lng, ValueLabel};

#[allow(clippy::too_many_lines)] // TODO
pub fn options(
    form_data: RwSignal<FormData>,
    input_data: Signal<FormData>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let lang = crate::current_lang();

    let excess_energy_co2_equivalent = Signal::derive(move || {
        outcome.with(|out| {
            out.output
                .as_ref()
                .map(|out| required!(Out::ExcessEnergyCo2Equivalent, out).unwrap())
        })
    });

    let electricity_mix_savings = Signal::derive(move || {
        outcome.with(|out| {
            out.output.as_ref().map(|out| {
                // TOOD: move this to calculation module
                let eq = &out;
                (required!(Out::TotalEmissions, eq).unwrap()
                    - required!(Out::ExcessEnergyCo2Equivalent, eq).unwrap())
                    * Factor::new(-1.0)
            })
        })
    });

    let electricity_mix = Signal::derive(move || {
        outcome.with(|out| {
            out.output
                .as_ref()
                .map(|out| required!(Out::ElectricityMix, out).unwrap())
        })
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_sets = field_sets(form_data.write_only(), input_data, lang.get());
    let (view, _, _) = render_field_sets(field_sets, accessibility_always_show_option, lang);

    // -----   ----- //
    //     View      //
    // -----   ----- //

    let lang = crate::current_lang().get();

    view! {
      <Card id="recommenation-excess-energy" title = move_tr!("recommenation-excess-energy").get()  bg_color="bg-yellow" accessibility_always_show_option>
        <p>
          <div inner_html={ move_tr!("recommenation-excess-energy-p-1") }></div>
        </p>
        { view }
          <Show
            when= move || excess_energy_co2_equivalent.with(|v| *v > Some(Tons::zero()))
          >
            <p>
            { move_tr!("recommenation-excess-energy-p-2-1") }
            " "
            {
              electricity_mix_savings.with(|d|
                d.map(|v| lang.format_number_with_fixed_precision(f64::from(v), 0))
              )
            }
            " "
            { move_tr!("recommenation-excess-energy-p-2-2") }
            </p>
          </Show>
          <Show
            when= move || excess_energy_co2_equivalent.with(|v| *v <= Some(Tons::zero())) && electricity_mix.with(|v| *v > Some(Tons::zero()))
          >
            <p>
            { move_tr!("recommenation-excess-energy-p-3-1") }
            " "
            {
              electricity_mix.with(|d|
                d.map(|v| lang.format_number_with_fixed_precision(f64::from(v), 0))
              )
            }
            " "
            { move_tr!("recommenation-excess-energy-p-3-2") }
            </p>
          </Show>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
        { move || outcome.with(|out|out.output.clone().map(|out|{
            let list = [
              (Out::ProcessEnergySavings),
              (Out::FossilEnergySavings),
              (Out::PhotovoltaicExpansionSavings),
              (Out::WindExpansionSavings),
              (Out::WaterExpansionSavings),
              (Out::DistrictHeatingSavings),
            ]
            .into_iter()
            .filter_map(|id| {
                let value = out.get(&id.into()).cloned().and_then(Value::as_tons).unwrap();
                if value > Tons::zero() {
                   Some((format!("{} {}", move_tr!("co2-savings").get(), id.label(lang)), value))
                } else {
                   None
                }
            })
            .map(|(label, value)| view! {
                <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">{ label }</dt>
                <dd class="text-lg py-1 px-3">
                    { crate::current_lang().get().format_number_with_fixed_precision(value, 2) }
                    <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
            })
            .collect::<Vec<_>>();

            view! {
              <dl class="mx-3 my-2 grid grid-cols-2 text-sm"> { list } </dl>
            }
          }))
        }
        </div>
      </Card>
    }
}

fn field_sets(
    form_data: WriteSignal<FormData>,
    input_data: Signal<FormData>,
    lang: Lng,
) -> Vec<FieldSet> {
    let draw_border = false;

    vec![
        FieldSet {
            title: match lang {
                Lng::De => Some("Prozesse und fossile Energieträger"),
                Lng::En => Some("Processes and fossil fuels"),
            },
            fields: vec![
                create_field(form_data, input_data, Id::RecommendationProcessEnergySaving),
                create_field(form_data, input_data, Id::RecommendationFossilEnergySaving),
            ],
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Photovoltaik"),
                Lng::En => Some("Photovoltaics"),
            },
            fields: vec![
                create_field(
                    form_data,
                    input_data,
                    Id::RecommendationPhotovoltaicEnergyExpansion,
                ),
                create_field(
                    form_data,
                    input_data,
                    Id::RecommendationEstimatedSelfPhotovolaticUsage,
                ),
            ],
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Windkraft"),
                Lng::En => Some("Windpower"),
            },
            fields: vec![
                create_field(form_data, input_data, Id::RecommendationWindEnergyExpansion),
                create_field(
                    form_data,
                    input_data,
                    Id::RecommendationEstimatedSelfWindEnergyUsage,
                ),
            ],
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Wasserkraft"),
                Lng::En => Some("Hydropower"),
            },
            fields: vec![
                create_field(
                    form_data,
                    input_data,
                    Id::RecommendationWaterEnergyExpansion,
                ),
                create_field(
                    form_data,
                    input_data,
                    Id::RecommendationEstimatedSelfWaterEnergyUsage,
                ),
            ],
            draw_border,
        },
        FieldSet {
            title: match lang {
                Lng::De => Some("Abwärmenutzung"),
                Lng::En => Some("Waste heat utilization"),
            },
            fields: vec![create_field(
                form_data,
                input_data,
                Id::RecommendationDistrictHeating,
            )],
            draw_border,
        },
    ]
}
