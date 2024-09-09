use std::collections::HashSet;

use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::{FormData, ProjectId};
use klick_domain::{InputValueId as Id, Value};

use crate::{api::AuthorizedApi, label_signal};

use super::{DISABLED_BUTTON_CLASS, ENABLED_BUTTON_CLASS};

#[component]
pub fn NewProject(
    api: Signal<AuthorizedApi>,
    on_cancel: Callback<(), ()>,
    on_success: Callback<ProjectId, ()>,
) -> impl IntoView {
    let lang = crate::current_lang();

    let title = RwSignal::<Option<String>>::new(None);
    let on_change = Callback::new(move |txt: Option<String>| {
        title.update(|t| *t = txt);
    });

    let field = Field {
        label: label_signal(Id::ProjectName),
        description: None,
        required: true,
        field_type: FieldType::Text {
            initial_value: None,
            placeholder: Some(RwSignal::new("Projektname".to_string()).into()),
            max_len: None,
            on_change,
            input: Signal::derive(move || None),
        },
    };

    let error = RwSignal::<Option<String>>::new(None);
    let wait_for_response = RwSignal::new(false);
    let field_id = dom_node_id();
    let missing_fields = RwSignal::new(HashSet::new());

    let field_view = render_field(field, field_id, missing_fields, lang, None);

    let create_project = create_action(move |(): &()| {
        let mut project = FormData::default();
        if let Some(title) = title.get().map(Value::text) {
            project.insert(Id::ProjectName, title);
        } else {
            project.remove(&Id::ProjectName);
        }
        async move {
            wait_for_response.set(true);
            let result = api.get().create_project(&project).await;
            wait_for_response.set(false);
            match result {
                Ok(new_id) => {
                    error.set(None);
                    on_success.call(new_id.into());
                }
                Err(err) => {
                    log::warn!("Unable to create projects: {err}");
                    error.set(Some(
                        "Es ist ein Kommunikationsproblem aufgetreten.".to_string(),
                    ));
                }
            }
        }
    });

    let enable_create_button = Signal::derive(move || {
        let Some(txt) = title.get() else {
            return false;
        };
        !txt.is_empty()
    });

    view! {
      { move || error.get().map(|err_msg|view!{<p class="mb-4 text-red-700">{ err_msg }</p>}) }
      <div class="flex items-center justify-between">
        { field_view }
        <div>
        </div>
        <div class="flex items-center gap-x-6">
          <button
            on:click = move |_| on_cancel.call(())
            class="ml-3 bg-gray-100 rounded px-2 py-1 text-sm font-semibold text-black shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          >
            "abbrechen"
          </button>
          <button
            on:click = move |_| create_project.dispatch(())
            disable = move || !enable_create_button.get()
            class = move || if enable_create_button.get() { ENABLED_BUTTON_CLASS} else { DISABLED_BUTTON_CLASS  }
          >
            "erstellen"
          </button>
        </div>
      </div>
    }
}
