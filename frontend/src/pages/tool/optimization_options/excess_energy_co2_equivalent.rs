use leptos::*;

use klick_domain as domain;
use klick_presenter::Lng;

use super::Card;

pub fn options(
    output: ReadSignal<
        Option<(
            domain::CO2Equivalents,
            domain::EmissionFactors,
            domain::EmissionFactorCalculationMethods,
        )>,
    >,
) -> impl IntoView {
    let excess_energy_co2_equivalent = RwSignal::new(0.0);
    let emissions = RwSignal::new(0.0);
    let indirect_emissions = RwSignal::new(0.0);

    create_effect(move |_| match output.get() {
        None => {
            excess_energy_co2_equivalent.set(0.0);
            emissions.set(0.0);
            indirect_emissions.set(0.0);
        }
        Some((co2_equivalents, _emission_factors, _calc_methods)) => {
            excess_energy_co2_equivalent.set(co2_equivalents.excess_energy_co2_equivalent.into());
            emissions.set(co2_equivalents.emissions.into());
            indirect_emissions.set(co2_equivalents.indirect_emissions.into());
        }
    });
    view! {
      <Card title ="Strombilanz">
        <p>
          <Show when= move || (excess_energy_co2_equivalent.get() > 0.0)>
          "Das Klärwerk selbst produziert mehr Strom, als es verbraucht: Dies entspricht einer Einsparung von: "
          { Lng::De.format_number_with_thousands_seperator(excess_energy_co2_equivalent.get()) } " t CO₂ Äquivalente/Jahr, was theoretisch von den Gesammtemissionen von "
          { Lng::De.format_number_with_thousands_seperator(emissions.get()) } " t CO₂ Äquivalente/Jahr noch abgezogen werden kann."
          </Show>
          <Show when= move || (excess_energy_co2_equivalent.get() <= 0.0)>
          "Das Klärwerk selbst benötigt externen Strom in Höhe von "
          { Lng::De.format_number_with_thousands_seperator(indirect_emissions.get()) } " t CO₂ Äquivalente/Jahr."
          </Show>
        </p>
      </Card>
    }
}
