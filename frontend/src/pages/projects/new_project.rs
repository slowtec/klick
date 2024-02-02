use leptos::*;
use strum::AsRefStr;

use klick_boundary::{ProjectData, ProjectId};
use klick_presenter::ValueLabel;

use crate::{api::AuthorizedApi, forms};

use super::{DISABLED_BUTTON_CLASS, ENABLED_BUTTON_CLASS};

#[component]
pub fn NewProject(
    api: Signal<AuthorizedApi>,
    on_cancel: Callback<(), ()>,
    on_success: Callback<ProjectId, ()>,
) -> impl IntoView {
    let field = Field {
        id: Id::Name,
        description: None,
        required: true,
        field_type: forms::FieldType::Text {
            initial_value: None,
            placeholder: Some("Projektname"),
            max_len: None,
        },
    };

    let error = RwSignal::new(None);
    let wait_for_response = RwSignal::new(false);
    let field_id = forms::form_field_id(&field.id);

    let (field_signal, field_view) = forms::render_field(field, field_id);

    let create_project = create_action(move |_: &()| {
        let title = field_signal.get_text().expect("Project name");
        let api = api.clone();
        let mut project = ProjectData::default();
        project.title = Some(title.clone());
        async move {
            wait_for_response.set(true);
            let result = api.get().create_project(&project).await;
            wait_for_response.set(false);
            match result {
                Ok(new_id) => {
                    error.set(None);
                    on_success.call(new_id);
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
        let Some(txt) = field_signal.get_text() else {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr)]
enum Id {
    Name,
}

impl ValueLabel for Id {
    fn label(&self) -> &str {
        match self {
            Self::Name => "Projektname",
        }
    }
}

type Field = forms::Field<Id>;
