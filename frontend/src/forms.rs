use leptos::wasm_bindgen::JsCast;
use leptos::*;

pub use klick_app_components::forms::*;
pub use klick_presenter::ValueLabel;

#[component]
pub fn HelperWidget<ID, F>(missing_fields: Vec<MissingField<ID>>, before_focus: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
    ID: Clone + ValueLabel + 'static,
{
    view! {
      <ul class="ml-5 my-4 list-disc list-inside">
        <For
          each = move || missing_fields.clone()
          key = |e| e.id.label().to_string()
          let:e
        >
          <li>
            <a
              class = "cursor-pointer"
              on:click=move |_| {
                let field_id = &e.field_id; //FIXME: rename dom_node_id
                let element_id = format!("#{field_id}");
                let element: web_sys::HtmlInputElement = document().query_selector(&element_id).unwrap().unwrap().unchecked_into();
                // uses might have to click the list link twice because if they are in input editing the on:blur event needs to change the html first and
                // this seems to interfere with this focus event
                before_focus();
                let _ = element.focus();
              }
            >
              { e.id.label().to_string() }
            </a>
          </li>
        </For>
      </ul>
    }
}
