use gloo_file::{Blob, File, ObjectUrl};
use leptos::*;

use klick_app_components::message::*;
use klick_boundary::{
    export_to_vec_pretty, import_from_slice, Data, FormData, Project, ProjectId, SavedProject,
};

use crate::{api::AuthorizedApi, Page, SECTION_ID_TOOL_HOME};

mod breadcrumbs;
mod calculation;
mod default_values;
mod example_data;
mod form_data_overview;
mod plant_profile;
mod project_menu;
mod recommendations;
mod sensitivity_parameters;
mod widgets;

use self::{
    breadcrumbs::Breadcrumbs,
    calculation::{calculate, CalculationOutcome},
    plant_profile::DataCollection,
    project_menu::ProjectMenu,
    recommendations::Recommendations,
    sensitivity_parameters::SensitivityParameters,
    widgets::*,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PageSection {
    #[default]
    DataCollection,
    Sensitivity,
    Recommendation,
}

impl PageSection {
    const fn section_id(self) -> &'static str {
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

#[component]
pub fn Tool(
    api: Signal<Option<AuthorizedApi>>,
    current_project: RwSignal<Option<Project>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let example_data = RwSignal::new(FormData::default());
    let form_data = RwSignal::new(FormData::default());
    let current_section = RwSignal::new(PageSection::DataCollection);
    let is_logged_in = Signal::derive(move || api.get().is_some());
    let save_result_message = RwSignal::new(None);

    let outcome = create_memo(move |_| calculate(form_data.get()));

    let show_side_stream_controls = Signal::derive(move || {
        form_data.with(|d| {
            d.plant_profile
                .side_stream_treatment
                .total_nitrogen
                .map(|v| v > 0.0)
                .unwrap_or(false)
        })
    });

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
                example_data.set(data);
            }
        }
    });

    let load_action = create_action({
        let api = api.clone();
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
        let api = api.clone();
        move |project: &Project| {
            let project = project.clone();
            async move {
                let Some(api) = api.get() else {
                    log::warn!("No authorized API");
                    return;
                };
                let result_msg = match project {
                    Project::Saved(mut p) => {
                        if p.data.project_title.is_none()
                            || p.data.project_title.as_deref() == Some("")
                        {
                            p.data.project_title = Some(DEFAULT_UNNAMED_PROJECT_TITLE.to_string());
                        }
                        api.update_project(&p)
                            .await
                            .map(|_| {
                                current_project.set(Some(Project::Saved(p)));
                                "Das Projekt wurde gespeichert."
                            })
                            .map_err(|err| {
                                log::warn!("Unable to update project: {err}");
                                "Das Projekt konnte leider nicht gespeichert werden."
                            })
                    }
                    Project::Unsaved(mut p) => {
                        if p.project_title.is_none() || p.project_title.as_deref() == Some("") {
                            p.project_title = Some(DEFAULT_UNNAMED_PROJECT_TITLE.to_string());
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
        move |_| {
            example_data.set(FormData::default());
        }
    };

    let load_example_values = {
        move |_| {
            example_data.set(example_data::example_form_data());
        }
    };

    let download = {
        move |_| -> ObjectUrl {
            let data = form_data.get();
            let project = Project::from(data);
            let data = Data { project };
            let json_bytes = export_to_vec_pretty(&data);

            let blob = Blob::new_with_options(&*json_bytes, Some("application/json"));

            ObjectUrl::from(blob)
        }
    };

    let save_project = {
        move |_| {
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
        move |_| -> ObjectUrl {
            // TODO
            todo!()
        }
    };

    // -----   ----- //
    //    Effects    //
    // -----   ----- //

    // TODO: Use router:
    // e.g. /tool/plant-profile/ instead of /tool#plant-profile
    create_effect(move |_| {
        let s = current_section.get();
        let id = s.section_id();
        let path = Page::Tool.path();
        let href = format!("{path}#{id}");
        window().location().set_href(&href).unwrap();
    });

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let section_view = move || match current_section.get() {
        PageSection::DataCollection => view! {
            <DataCollection
              form_data
              input_data = example_data.read_only()
              current_section
              outcome = outcome.into()
            />
        }
        .into_view(),
        PageSection::Sensitivity => view! {
            <SensitivityParameters
              form_data
              input_data = example_data.read_only()
              current_section
              outcome = outcome.into()
              show_side_stream_controls
            />
        }
        .into_view(),
        PageSection::Recommendation => view! {
            <Recommendations
              form_data
              input_data = example_data.read_only()
              outcome = outcome.into()
              show_side_stream_controls
            />
        }
        .into_view(),
    };

    view! {
      <div class="space-y-10" id = move || current_section.get().section_id() >
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
