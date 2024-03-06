use leptos::*;

use klick_domain as domain;
use klick_presenter::Lng;

use super::Card;

pub fn options(output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>) -> impl IntoView {
    let excess_energy_co2_equivalent = RwSignal::new(0.0);
    let emissions = RwSignal::new(0.0);
    let indirect_emissions = RwSignal::new(0.0);

    create_effect(move |_| match output.get() {
        None => {
            excess_energy_co2_equivalent.set(0.0);
            emissions.set(0.0);
            indirect_emissions.set(0.0);
        }
        Some(domain::EmissionsCalculationOutcome {
            co2_equivalents, ..
        }) => {
            excess_energy_co2_equivalent.set(co2_equivalents.excess_energy_co2_equivalent.into());
            emissions.set(co2_equivalents.total_emissions.into());
            indirect_emissions.set(co2_equivalents.indirect_emissions.into());
        }
    });
    view! {
      <Card title ="Strombilanz" bg_color="bg-yellow">
        <p>
          <Show
            when= move || (excess_energy_co2_equivalent.get() > 0.0)
          >
            "Das Klärwerk selbst produziert mehr Strom, als es verbraucht: Dies entspricht einer Einsparung von: "
            { Lng::De.format_number_with_thousands_seperator(excess_energy_co2_equivalent.get()) }
            " t CO₂ Äquivalente/Jahr, was theoretisch von den Gesammtemissionen von "
            { Lng::De.format_number_with_thousands_seperator(emissions.get()) }
            " t CO₂ Äquivalente/Jahr noch abgezogen werden kann."
          </Show>
          <Show
            when= move || (excess_energy_co2_equivalent.get() <= 0.0)
          >
            "Das Klärwerk selbst benötigt externen Strom in Höhe von "
            { Lng::De.format_number_with_thousands_seperator(indirect_emissions.get()) }
            " t CO₂ Äquivalente/Jahr."
          </Show>
        </p>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          { move || {
              output.get().map(|out|
                view! {
                  <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                    <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"Stommix"</dt>
                    <dd class="text-lg py-1 px-3">
                      { format!("{:.1}", f64::from(out.co2_equivalents.electricity_mix)).replace('.',",") }
                      <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                    </dd>
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
      </Card>
    }
}
