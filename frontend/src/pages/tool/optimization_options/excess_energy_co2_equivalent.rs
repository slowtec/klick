use leptos::*;

use klick_application as app;
use klick_domain as domain;
use klick_format_numbers::Lng;

use super::Card;

pub fn options(
    input_data: Signal<Option<domain::PlantProfile>>,
    n2o_emission_factor_method: Signal<Option<domain::N2oEmissionFactorCalcMethod>>,
) -> impl IntoView {
    let excess_energy_co2_equivalent = RwSignal::new(0.0);
    let emissions = RwSignal::new(0.0);
    let indirect_emissions = RwSignal::new(0.0);

    create_effect(move |_| {
        let Some(input_data) = input_data.get() else {
            log::debug!("No input data");
            excess_energy_co2_equivalent.set(0.0);
            emissions.set(0.0);
            indirect_emissions.set(0.0);
            return;
        };

        let n2o_emission_factor = n2o_emission_factor_method
            .get()
            .unwrap_or(domain::N2oEmissionFactorCalcMethod::Ipcc2019);

        let scenario = domain::OptimizationScenario {
            n2o_emission_factor,
            ch4_chp_emission_factor: None,
        };

        let output_data = app::calculate_emissions(&input_data, scenario);
        excess_energy_co2_equivalent.set(
            output_data
                .clone()
                .co2_equivalents
                .excess_energy_co2_equivalent
                .into(),
        );
        emissions.set(output_data.clone().co2_equivalents.emissions.into());
        indirect_emissions.set(
            output_data
                .clone()
                .co2_equivalents
                .indirect_emissions
                .into(),
        );
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
