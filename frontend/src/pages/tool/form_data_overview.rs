use leptos::*;

use klick_boundary::FormData;
use klick_presenter::{plant_profile_as_table, UnitFormatting};

#[component]
pub fn FormDataOverview(form_data: ReadSignal<FormData>) -> impl IntoView {
    let profile_table = move || {
        let table =
            form_data.with(|d| plant_profile_as_table(&d.plant_profile, UnitFormatting::Text));
        table
            .sections
            .into_iter()
            .map(|section| {
                let values: Vec<_> = section.rows.into_iter().map(|(label, value, unit)|{
              view! {
                <dt class="font-semibold text-right px-3 py-1 text-gray-500">{ label }</dt>
                <dd class="py-1 px-3">
                  { value }
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
