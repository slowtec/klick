use leptos::wasm_bindgen::JsCast;
use leptos::*;

use klick_app_components::forms::*;

#[component]
pub fn ListOfMissingFields(missing_fields: Vec<(FieldId, Signal<String>)>) -> impl IntoView {
    view! {
      <ul class="ml-5 my-4 list-disc list-inside">
        <For
          each = move || missing_fields.clone()
          key = |(id,_)| *id
          let:e
        >
          <li>
            <a
              class = "cursor-pointer"
              on:click=move |_| {
                let field_id = &e.0;
                let element_id = format!("#{field_id}");
                let query = document().query_selector(&element_id);
                match query {
                  Ok(query_result) => {
                    match query_result {
                      Some(query_element) => {
                        let element: web_sys::HtmlInputElement = query_element.unchecked_into();
                        // uses might have to click the list link twice because if they are in input editing the on:blur event needs to change the html first and
                        // this seems to interfere with this focus event
                        let _ = element.focus();
                      },
                      None => {
                        log::error!("Element to focus on not found in DOM tree.");
                      }
                    }
                  },
                  Err(_) => {
                    log::error!("Query selector failed, so element to focus on 'not found' in DOM tree.");
                  }
                }
              }
            >
              { e.1 }
            </a>
          </li>
        </For>
      </ul>
    }
}
