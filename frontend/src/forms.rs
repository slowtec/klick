use leptos::wasm_bindgen::JsCast;
use leptos::*;

use klick_app_components::forms::*;

#[component]
pub fn ListOfMissingFields(missing_fields: Vec<(FieldId, &'static str)>) -> impl IntoView {
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
                // FIXME add error
                let element: web_sys::HtmlInputElement = document().query_selector(&element_id).unwrap().unwrap().unchecked_into();
                // uses might have to click the list link twice because if they are in input editing the on:blur event needs to change the html first and
                // this seems to interfere with this focus event
                let _ = element.focus();
              }
            >
              { e.1 }
            </a>
          </li>
        </For>
      </ul>
    }
}
