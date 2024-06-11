use leptos::*;

use klick_boundary::EvaluationData;
use klick_presenter::{plant_profile_as_table, sensitivity_parameters_as_table, Formatting};

#[component]
pub fn FormDataOverview(evaluation_data: EvaluationData) -> impl IntoView {
    let profile_table = {
        let i = evaluation_data.input;
        let o = evaluation_data.output;
        let table = {
            let mut profile = plant_profile_as_table(&i, Formatting::Text);
            let mut sensitivity = sensitivity_parameters_as_table(
                &i.sensitivity_parameters,
                Formatting::Text,
                o.as_ref(),
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
