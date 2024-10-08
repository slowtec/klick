use std::collections::HashMap;

use crate::wasm_bindgen::closure::Closure;
use js_sys::wasm_bindgen::JsCast;
use leptos::*;
use leptos_fluent::*;
use leptos_hotkeys::use_hotkeys;

use klick_app_components::{
    forms::{dom_node_id, render_field},
    icons,
};
use klick_domain::{InputValueId as In, Value, ValueId as Id, ValueType};
use klick_presenter::{
    plant_profile_as_table, sensitivity_parameters_as_table, Formatting, Lng, TableRow,
};

use crate::{
    pages::tool::{fields::create_field, CalculationOutcome},
    Modal,
};

#[component]
pub fn FormDataOverview(
    form_data: RwSignal<HashMap<In, Value>>,
    outcome: Signal<CalculationOutcome>,
    lang: Signal<Lng>,
) -> impl IntoView {
    let input = Signal::derive(move || outcome.with(|out| out.input.clone()));

    let show_modal = RwSignal::new(Option::<In>::None);
    let formatting = Formatting::Text;
    let profile_table = move || {
        let table = {
            let lang = lang.get();
            let input = input.get();
            let mut profile = plant_profile_as_table(&input, formatting, lang);
            let mut sensitivity = sensitivity_parameters_as_table(&input, formatting, lang);
            profile.sections.append(&mut sensitivity.sections);
            profile
        };

        table
            .sections
            .into_iter()
            .map(|section| {
                let values: Vec<_> = section.rows.into_iter().map(| TableRow { id, label, value, unit }|{
                  view! {
                    <dt class="font-semibold text-right px-3 py-1 text-gray-500 flex items-center justify-end">{ label }</dt>
                    <dd class="py-1 px-3 flex items-center">
                      {
                        match id {
                          Id::Custom(_) => None,
                          Id::In(id) if matches!(id.value_type(), ValueType::Enum(_)) => {
                              None
                          }
                          _ => Some(view!{
                            <button
                              type = "button"
                              class="form-data-edit-button"
                              on:click = move |_| {
                                  let Id::In(id) = id else {
                                    return;
                                  };
                                  show_modal.set(Some(id));
                              }
                            >
                              <icons::Pencil />
                            </button>
                          })
                        }
                      }
                      <span class="ml-2 mr-2">{ value.unwrap_or_else(||"-".to_string()) }</span>
                      <span class="text-gray-400">{ unit }</span>
                    </dd>
                  }}
                ).collect();
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

    view! {
      <ul class="grid grid-cols-3">
        { profile_table }
      </ul>
      {
        move || show_modal.get().map(|input_id| view! {
          <ModalInput
            id = input_id
            form_data
            lang
            close = Callback::new(move |_| { show_modal.set(None); })
          />
        })
      }
    }
}

#[component]
fn ModalInput(
    id: In,
    form_data: RwSignal<HashMap<In, Value>>,
    lang: Signal<Lng>,
    close: Callback<(), ()>,
) -> impl IntoView {
    let accessibility_always_show_option: Option<RwSignal<bool>> = Some(RwSignal::new(true));
    let read = form_data.into();
    let write = form_data.write_only();
    let field = create_field(write, read, id);
    let dom_node_id = dom_node_id();
    let missing_fields = RwSignal::new(Default::default());

    let view = render_field(
        field,
        dom_node_id,
        missing_fields,
        lang,
        accessibility_always_show_option,
    );

    use_hotkeys!(("Escape") => move |()| {
      log::info!("exit");
      close.call(());
    });

    let node_ref: NodeRef<leptos::html::Div> = NodeRef::default();
    node_ref.on_load(move |div| {
        let _ = div.on_mount(move |div| {
            let id: String = format!("#{}", dom_node_id);
            let query = document().query_selector(&id);
            match query {
                Ok(query_result) => match query_result {
                    Some(query_element) => {
                        let element: web_sys::HtmlInputElement = query_element.unchecked_into();
                        let _ = element.focus();
                        let a = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                            move |event: web_sys::KeyboardEvent| {
                                if event.key() == "Enter" {
                                    close.call(());
                                }
                            },
                        );
                        let _ = element.add_event_listener_with_callback(
                            "keydown",
                            a.as_ref().unchecked_ref(),
                        );
                        a.forget();
                    }
                    None => {
                        log::error!("Element to focus on not found in DOM tree.");
                    }
                },
                Err(_) => {
                    log::error!(
                        "Query selector failed, so element to focus on 'not found' in DOM tree."
                    );
                }
            }
            let _ = div.focus();
        });
    });

    view! {
      <Modal>
        { view }
        <div class="mt-5 sm:mt-6">
          <button
            on:click = move |_| {
                close.call(());
            }

            type="button"
            class="inline-flex w-full justify-center rounded-md bg-gray-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          >
            { move_tr!("back-to-table") }
          </button>
        </div>
      </Modal>
    }
}
