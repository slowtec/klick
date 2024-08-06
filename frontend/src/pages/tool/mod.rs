use gloo_file::{Blob, File, ObjectUrl};
use leptos::*;

use klick_app_components::message::*;
use klick_boundary::{
    calculate, export_to_vec_pretty, import_from_slice, CalculationOutcome, Data, FormData,
    Project, ProjectId, SavedProject,
};
use klick_domain::{InputValueId as Id, Value};
use klick_presenter as presenter;

use crate::{api::AuthorizedApi, SECTION_ID_TOOL_HOME};

mod breadcrumbs;
mod example_data;
mod fields;
mod form_data_overview;
mod plant_profile;
mod project_menu;
mod recommendations;
mod sensitivity_parameters;
mod widgets;

use self::{
    breadcrumbs::Breadcrumbs, plant_profile::DataCollection, project_menu::ProjectMenu,
    recommendations::Recommendations, sensitivity_parameters::SensitivityParameters, widgets::*,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PageSection {
    #[default]
    DataCollection,
    Sensitivity,
    Recommendation,
}

impl PageSection {
    pub const fn section_id(self) -> &'static str {
        match self {
            PageSection::DataCollection => SECTION_ID_TOOL_HOME,
            PageSection::Sensitivity => "data-sensitivity",
            PageSection::Recommendation => "data-recommendations",
        }
    }
}

const BREADCRUMPS_ENTRIES: &[(&str, PageSection)] = &[
    ("Datenerfassung", PageSection::DataCollection),
    ("Sensitivität", PageSection::Sensitivity),
    ("Handlungsempfehlungen", PageSection::Recommendation),
];

const DEFAULT_UNNAMED_PROJECT_TITLE: &str = "Unbenannt";

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn Tool(
    api: Signal<Option<AuthorizedApi>>,
    current_project: RwSignal<Option<Project>>,
    current_section: RwSignal<PageSection>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let form_data = RwSignal::new(FormData::default());
    let is_logged_in = Signal::derive(move || api.get().is_some());
    let save_result_message = RwSignal::new(None);

    let outcome = create_memo(move |_| calculate(form_data.get()));

    let show_side_stream_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.get(&Id::SideStreamTreatmentTotalNitrogen)
                .map(Value::as_tons_unchecked)
                .is_some_and(|v| f64::from(v) > 0.0)
        })
    });

    // TODO: allow to export at any time
    let show_csv_export = Signal::derive(move || outcome.with(|out| out.profile.output.is_some()));

    // -----   ----- //
    //    Actions    //
    // -----   ----- //

    let upload_action = create_action({
        move |file: &File| {
            let file = file.clone();
            async move {
                let bytes = match gloo_file::futures::read_as_bytes(&file).await {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        log::warn!("Unable to upload data: {err}");
                        return;
                    }
                };
                let project = match import_from_slice(&bytes) {
                    Ok(project) => project,
                    Err(err) => {
                        log::warn!("Unable to import data: {err}");
                        return;
                    }
                };
                let data = match project {
                    Project::Saved(d) => d.data,
                    Project::Unsaved(d) => d,
                };
                form_data.set(data);
            }
        }
    });

    let load_action = create_action({
        move |id: &ProjectId| {
            let id = *id;
            async move {
                let Some(api) = api.get() else {
                    log::warn!("No authorized API");
                    return;
                };
                match api.read_project(&id).await {
                    Ok(p) => {
                        current_project.set(Some(p.into()));
                    }
                    Err(err) => {
                        log::warn!("Unable to read project: {err}");
                    }
                }
            }
        }
    });

    let save_action = create_action({
        move |project: &Project| {
            let project = project.clone();
            async move {
                let Some(api) = api.get() else {
                    log::warn!("No authorized API");
                    return;
                };
                let result_msg = match project {
                    Project::Saved(mut p) => {
                        let name = p.data.get(&Id::ProjectName).map(Value::as_text_unchecked);
                        if name.is_none() || name.as_deref() == Some("") {
                            p.data.set(
                                Id::ProjectName,
                                Some(Value::text(DEFAULT_UNNAMED_PROJECT_TITLE)),
                            );
                        }
                        api.update_project(&p)
                            .await
                            .map(|()| {
                                current_project.set(Some(Project::Saved(p)));
                                "Das Projekt wurde gespeichert."
                            })
                            .map_err(|err| {
                                log::warn!("Unable to update project: {err}");
                                "Das Projekt konnte leider nicht gespeichert werden."
                            })
                    }
                    Project::Unsaved(mut p) => {
                        let name = p.get(&Id::ProjectName).map(Value::as_text_unchecked);
                        if name.is_none() || name.as_deref() == Some("") {
                            p.set(
                                Id::ProjectName,
                                Some(Value::text(DEFAULT_UNNAMED_PROJECT_TITLE)),
                            );
                        }
                        api.create_project(&p)
                            .await
                            .map(|new_id| {
                                load_action.dispatch(new_id);
                                "Das Projekt wurde neu angelegt."
                            })
                            .map_err(|err| {
                                log::warn!("Unable to create project: {err}");
                                "Das Projekt konnte leider nicht gespeichert werden."
                            })
                    }
                };
                save_result_message.set(Some(result_msg));
            }
        }
    });

    // -----   ----- //
    //   Callbacks   //
    // -----   ----- //

    let clear_form_data = {
        move |()| {
            form_data.set(FormData::default());
            current_project.set(None);
        }
    };

    let load_example_values = {
        move |()| {
            form_data.set(example_data::example_form_data());
        }
    };

    let download = {
        move |()| -> ObjectUrl {
            let data = form_data.get();
            let project = Project::from(data);
            let data = Data { project };
            let json_bytes = export_to_vec_pretty(&data);

            let blob = Blob::new_with_options(&*json_bytes, Some("application/json"));

            ObjectUrl::from(blob)
        }
    };

    let save_project = {
        move |()| {
            let project_data = form_data.get();
            let project = match current_project.get() {
                Some(Project::Saved(p)) => {
                    let SavedProject {
                        id,
                        created_at,
                        modified_at,
                        ..
                    } = p;
                    let updated = SavedProject {
                        id,
                        created_at,
                        modified_at,
                        data: project_data,
                    };
                    Project::from(updated)
                }
                Some(Project::Unsaved(_)) | None => Project::from(project_data),
            };
            save_action.dispatch(project);
        }
    };

    let export_csv = {
        move |()| -> Option<ObjectUrl> {
            let csv = presenter::calculation_outcome_as_csv(&outcome.get());
            let blob = Blob::new_with_options(csv.as_bytes(), Some("text/csv"));
            Some(ObjectUrl::from(blob))
        }
    };

    // -----   ----- //
    //    Effects    //
    // -----   ----- //

    create_effect(move |_| {
        let s = current_section.get();
        let id = s.section_id();
        scroll_to_element_by_id(id);
    });

    create_effect(move |_| {
        let Some(p) = current_project.get() else {
            return;
        };
        form_data.set(p.form_data().clone());
    });

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let section_view = move || match current_section.get() {
        PageSection::DataCollection => view! {
            <DataCollection
              form_data
              current_section
              outcome = outcome.into()
              accessibility_always_show_option
            />
        }
        .into_view(),
        PageSection::Sensitivity => view! {
            <SensitivityParameters
              form_data
              current_section
              outcome = outcome.into()
              show_side_stream_controls
              accessibility_always_show_option
            />
        }
        .into_view(),
        PageSection::Recommendation => view! {
            <Recommendations
              form_data
              outcome = outcome.into()
              show_side_stream_controls
              current_section
              accessibility_always_show_option
            />
        }
        .into_view(),
    };

    view! {
      <div class="space-y-10" >
        <div class="flex center-items justify-between">
          <Breadcrumbs
            entries = { BREADCRUMPS_ENTRIES }
            current = current_section
          />
          <ProjectMenu
            logged_in = is_logged_in
            clear = clear_form_data
            load = load_example_values
            save = save_project
            download
            export_csv
            upload_action
            show_csv_export
          />
          { move || save_result_message.get().map(|res| match res {
            Ok(msg) => view!{ <SuccessMessage message = msg /> }.into_view(),
            Err(msg) => view!{ <ErrorMessage message = msg /> }.into_view()
            })
          }
        </div>
        { section_view }
      </div>
    }
}

fn scroll_to_element_by_id(element_id: &str) {
    let document = window().document().expect("HTML document");
    if let Some(element) = document.get_element_by_id(element_id) {
        element.scroll_into_view();
    }
}

#[component]
pub fn DataCollectionEnforcementHelper(current_section: RwSignal<PageSection>) -> impl IntoView {
    view! {
        <div class="my-8 border-b border-gray-200 pb-5" >
        <p>
          "Bitte ergänzen Sie im Eingabeformular die fehlenden Werte, damit die Emissionen berechnet und visualisiert werden können."
        </p>
      </div>
      <button
       class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
       on:click = move |_| current_section.set(PageSection::DataCollection)
      >
        "zu der Datenerfassung"
      </button>
    }
}
