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
    outcome: Signal<Option<CalculationOutcome>>,
) -> impl IntoView {
    // -----   ----- //
    //     Form      //
    // -----   ----- //

    let field_sets = field_sets(form_data);
    let (field_views, missing_fields, labels) = render_field_sets(field_sets);
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
              before_focus = move || {
                  current_section.set(PageSection::DataCollection);
              }
          />
        })
    };

    view! {
      <div>
        { field_views }
        { list_of_missing_fields }
        <Show when = move || !missing_fields.get().is_empty()>
          <p>
            "Bitte ergänzen Sie folgende Werte, damit die Gesamtemissionen Ihrer Kläranlage,
            anhand verschiedener Szenarien, berechnet werden können:"
          </p>
        </Show>
        <h4 class="my-8 text-lg font-bold">
        { move || outcome.with(|out|out.as_ref().map(|out|{
              let out = &out.profile.output;
              klick_presenter::create_sankey_chart_header(
                &form_data.with(|d| d.plant_profile.clone()),
                out.emission_factors,
                out.calculation_methods,
              )
            }))
        }
        </h4>
        { move || outcome.get().map(|outcome|{
          let outcome = outcome.profile.output;
          let data = (outcome.co2_equivalents, outcome.emission_factors);
          view!{ <Sankey data /> }
        })}
        <Show when = move || outcome.get().is_some()>
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
