use std::rc::Rc;

use leptos::*;

use klick_app_components::forms::render_field_sets;
use klick_boundary::FormData;

use crate::{
    forms::ListOfMissingFields,
    pages::tool::{CalculationOutcome, PageSection},
    sankey::Sankey,
};

mod field_sets;

pub use self::field_sets::field_sets;

#[component]
pub fn DataCollection(
    form_data: RwSignal<FormData>,
    current_section: RwSignal<PageSection>,
    outcome: Signal<CalculationOutcome>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    // -----   ----- //
    //     Form      //
    // -----   ----- //

    let field_sets = field_sets(form_data);
    let (field_views, missing_fields, labels) =
        render_field_sets(field_sets, accessibility_always_show_option);
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
          <ListOfMissingFields
              missing_fields
          />
        })
    };

    view! {
      <div>
        { field_views }
        <Show when = move || !missing_fields.get().is_empty()>
          <p>
            "Bitte ergänzen Sie folgende Werte, damit die Gesamtemissionen Ihrer Kläranlage,
            anhand verschiedener Szenarien, berechnet werden können:"
          </p>
        </Show>
        { list_of_missing_fields }
        <h4 class="my-8 text-lg font-bold">
        { move || outcome.with(|out|out.output.as_ref().map(|out|{
              klick_presenter::create_sankey_chart_header(
                &form_data.with(Clone::clone), // TODO: avoid clone
                out.emission_factors.clone(),
                out.calculation_methods,
                klick_presenter::Formatting::Text,
              )
            }))
        }
        </h4>
        { move || outcome.with(|outcome|outcome.output.clone()).map(|outcome|{
            let data = (outcome.co2_equivalents, outcome.emission_factors);
            view!{ <Sankey data /> }
          })
        }
        <Show when = move || outcome.with(|outcome|outcome.output.is_some())>
          <button
            class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
            on:click = move |_| { current_section.set(PageSection::Sensitivity); }
          >
            "zur Sensitivität"
          </button>
        </Show>
      </div>
    }
}
