use std::rc::Rc;

use leptos::*;
use leptos_fluent::*;

use klick_presenter::Lng;

use klick_app_components::forms::render_field_sets;
use klick_boundary::FormData;

use crate::{
    current_lang,
    forms::ListOfMissingFields,
    pages::tool::{CalculationOutcome, PageSection},
    sankey::Sankey,
};

mod field_sets;

pub use self::field_sets::field_sets;

#[component]
pub fn PlantProfile(
    form_data: RwSignal<FormData>,
    current_section: RwSignal<PageSection>,
    profile_outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    lang: Lng,
) -> impl IntoView {
    // -----   ----- //
    //     Form      //
    // -----   ----- //

    let field_sets = field_sets(form_data, lang);
    let (field_views, missing_fields, labels) =
        render_field_sets(field_sets, accessibility_always_show_option, current_lang());
    let labels = Rc::new(labels);

    // -----   ----- //
    //     View      //
    // -----   ----- //

    let list_of_missing_fields = move || {
        let fields = missing_fields.get();
        if fields.is_empty() {
            return None;
        }
        let missing_fields = fields
            .iter()
            .map(|id| {
                let label = labels[id];
                (*id, label)
            })
            .collect::<Vec<_>>();

        Some(view! {
          <ListOfMissingFields missing_fields />
        })
    };

    view! {
      <div>
        { field_views }
        <Show when = move || !missing_fields.get().is_empty()>
          <p>
            {move_tr!("datacollection_missing_fields").get()}
          </p>
        </Show>
        { list_of_missing_fields }
        <h4 class="my-8 text-lg font-bold">
        { move || profile_outcome.with(|outcome|outcome.output.as_ref().map(|out|{
              klick_presenter::create_sankey_chart_header(
                &outcome.input,
                out.clone(),
                klick_presenter::Formatting::Text,
                current_lang().get(),
              )
            }))
        }
        </h4>
        { move || profile_outcome.with(|out| out.output.clone().zip(out.graph.clone()).map(|(data, graph)|{
            let lang = current_lang().get();
            view!{ <Sankey data graph lang/> }
          }))
        }
        <Show when = move || profile_outcome.with(|outcome|outcome.output.is_some())>
          <button
            class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
            on:click = move |_| { current_section.set(PageSection::Sensitivity); }
          >
            { move_tr!("to-the-sensitivity") }
          </button>
        </Show>
      </div>
    }
}
