use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::{units::*, InputValueId as Id};
use klick_presenter::*;

use crate::pages::tool::{fields::create_field, CalculationOutcome, Card};

#[allow(clippy::too_many_lines)] // TODO
pub fn options(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let excess_energy_co2_equivalent = Signal::derive(move || {
        outcome.with(|out| {
            out.recommendation
                .output
                .as_ref()
                .map(|out| out.co2_equivalents.excess_energy_co2_equivalent)
        })
    });

    let electricity_mix_savings = Signal::derive(move || {
        outcome.with(|out| {
            out.recommendation.output.as_ref().map(|out| {
                // TOOD: move this to calculation module
                let eq = &out.co2_equivalents;
                (eq.total_emissions - eq.excess_energy_co2_equivalent) * Factor::new(-1.0)
            })
        })
    });

    let electricity_mix = Signal::derive(move || {
        outcome.with(|out| {
            out.recommendation
                .output
                .as_ref()
                .map(|out| out.co2_equivalents.electricity_mix)
        })
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_sets = field_sets(form_data.write_only(), input_data);
    let (view, _, _) = render_field_sets(field_sets, accessibility_always_show_option);

    // -----   ----- //
    //     View      //
    // -----   ----- //

    let lng = Lng::De;

    view! {
      <Card id="recommenation-excess-energy" title ="Energiebedingte Emissionen" bg_color="bg-yellow" accessibility_always_show_option>
        <p>
        <b>"Energiesparmaßnahmen"</b>" und "<b>"Erneuerbare Energien"</b>" können maßgeblich zur Minderung indirekter Emissionen und
             zur Energieautarkie beitragen. Um die positiven Auswirkungen eines Zubaus der erneuerbaren Energien:
             Photovoltaik (PV), Wind-, Wasserkraft und/oder Abwärmenutzung aufzuzeigen, können nachfolgend verschiedene
             Szenarien bilanziert werden. Wenn Sie die jeweilige Technologie nicht bilanzieren wollen können Sie
             das jeweilige Feld freilassen."
        </p>
        { view }
          <Show
            when= move || excess_energy_co2_equivalent.with(|v| *v > Some(Tons::zero()))
          >
            <p>
            " Ihre Kläranlage ist energieneutral. Die Kläranlage spart "
            {
              electricity_mix_savings.with(|d|
                d.map(|v| lng.format_number_with_fixed_precision(f64::from(v), 0))
              )
            }
            " t CO2-Äq./a ein."
            </p>
          </Show>
          <Show
            when= move || excess_energy_co2_equivalent.with(|v| *v <= Some(Tons::zero())) && electricity_mix.with(|v| *v > Some(Tons::zero()))
          >
            <p>
            "Ihre Kläranlage benötigt weiterhin externen Strom (Versorger), wodurch "
            {
              electricity_mix.with(|d|
                d.map(|v| lng.format_number_with_fixed_precision(f64::from(v), 0))
              )
            }
            " t CO₂-Äq./a energiebedingte Emissionen entstehen."
            </p>
          </Show>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
        { move || outcome.with(|out|out.recommendation.output.clone().map(|out|{
            view! {
              <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                <Show when= move || (f64::from(out.co2_equivalents.process_energy_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Energieeinsparung bei Prozessen"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.process_energy_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.fossil_energy_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung bei Fossilen Energiequellen"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.fossil_energy_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.photovoltaic_expansion_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Photovoltaik"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.photovoltaic_expansion_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.wind_expansion_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Windkraft"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.wind_expansion_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.water_expansion_savings) > 0.0) >
                <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Wasserkraft"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.water_expansion_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.district_heating_savings) > 0.0) >
                <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Abwärmenutzung"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.district_heating_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
              </dl>
            }
          }))
        }
        </div>
      </Card>
    }
}

fn field_sets(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> Vec<FieldSet> {
    vec![
        FieldSet {
            title: Some("Prozesse und fossile Energieträger"),
            fields: vec![
                create_field(form_data, input_data, Id::ScenarioProcessEnergySaving),
                create_field(form_data, input_data, Id::ScenarioFossilEnergySaving),
            ],
        },
        FieldSet {
            title: Some("Photovoltaik"),
            fields: vec![
                create_field(
                    form_data,
                    input_data,
                    Id::ScenarioPhotovoltaicEnergyExpansion,
                ),
                create_field(
                    form_data,
                    input_data,
                    Id::ScenarioEstimatedSelfPhotovolaticUsage,
                ),
            ],
        },
        FieldSet {
            title: Some("Windkraft"),
            fields: vec![
                create_field(form_data, input_data, Id::ScenarioWindEnergyExpansion),
                create_field(
                    form_data,
                    input_data,
                    Id::ScenarioEstimatedSelfWindEnergyUsage,
                ),
            ],
        },
        FieldSet {
            title: Some("Wasserkraft"),
            fields: vec![
                create_field(form_data, input_data, Id::ScenarioWaterEnergyExpansion),
                create_field(
                    form_data,
                    input_data,
                    Id::ScenarioEstimatedSelfWaterEnergyUsage,
                ),
            ],
        },
        FieldSet {
            title: Some("Abwärmenutzung"),
            fields: vec![create_field(
                form_data,
                input_data,
                Id::ScenarioDistrictHeating,
            )],
        },
    ]
}
