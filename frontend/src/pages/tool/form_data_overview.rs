use leptos::*;

use crate::pages::tool::calculation::CalculationInputOutput;
use klick_domain::units::Percent;
use klick_domain::units::Ratio;
use klick_presenter::Lng;
use klick_presenter::{plant_profile_as_table, sensitivity_parameters_as_table, UnitFormatting};

#[component]
pub fn FormDataOverview(calculation_input_output: CalculationInputOutput) -> impl IntoView {
    let profile_table = {
        let i = calculation_input_output.input;
        let o = calculation_input_output.output;
        let n2o_emission_factor: String = Lng::De.format_number_with_precision(
            f64::from(o.emission_factors.n2o.convert_to::<Percent>()),
            3,
        );
        let ch4_chp_emission_factor: String =
            Lng::De.format_number(f64::from(o.emission_factors.ch4.convert_to::<Percent>()));
        let table = {
            let mut profile = plant_profile_as_table(&i.plant_profile, UnitFormatting::Text);
            let mut sensitivity = sensitivity_parameters_as_table(
                &i.sensitivity_parameters,
                UnitFormatting::Text,
                n2o_emission_factor,
                ch4_chp_emission_factor,
            );
            profile.sections.append(&mut sensitivity.sections);
            profile
        };
        table
            .sections
            .into_iter()
            .map(|section| {
                let values: Vec<_> = section.rows.into_iter().map(|(label, value, unit)|{
              view! {
                <dt class="font-semibold text-right px-3 py-1 text-gray-500">{ label }</dt>
                <dd class="py-1 px-3">
                  { value.unwrap_or_else(||"-".to_string()) }
                  <span class="ml-2 text-gray-400">{ unit }</span>
                </dd>
              }
            }).collect();

                view! {
                  <li class="px-3">
                    <div class="font-semibold text-lg border-solid border-b text-gray-400">
                      { section.title }
                    </div>
                    <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                      { values }
                    </dl>
                  </li>
                }
            })
            .collect::<Vec<_>>()
    };

    // FIXME: Add sensitivity parameters

    view! {
      <ul class="grid grid-cols-3">
        { profile_table }
      </ul>
    }
}
