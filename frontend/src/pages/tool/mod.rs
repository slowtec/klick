use std::collections::HashMap;

use gloo_file::{Blob, File, ObjectUrl};
use leptos::*;

use klick_app_components::message::*;
use klick_boundary::{
    calculate, export_to_vec_pretty, import_from_slice, CalculationOutcome, Data, FormData,
    JsonFormData, Project, ProjectId, SavedProject,
};
use klick_domain::{
    get_all_internal_nodes, input_value::optional as optional_in, units::Tons, Id,
    InputValueId as In, Value,
};
use klick_presenter as presenter;

use crate::{
    api::AuthorizedApi,
    pages::tool::parser::{self as custom_emission_parser, CustomEmission},
    SECTION_ID_TOOL_HOME,
};

mod breadcrumbs;
mod example_data;
mod fields;
mod form_data_overview;
mod parser;
mod plant_profile;
mod project_menu;
mod recommendations;
mod sensitivity_parameters;
mod widgets;

use self::{
    breadcrumbs::Breadcrumbs, plant_profile::PlantProfile, project_menu::ProjectMenu,
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

    let profile_form_data = RwSignal::new(FormData::default());
    let sensitivity_form_data = RwSignal::new(FormData::default());
    let recommendation_form_data = RwSignal::new(FormData::default());

    let load_form_data = move |data: HashMap<_, _>| {
        profile_form_data.set(data.clone());
        sensitivity_form_data.set(data.clone());
        recommendation_form_data.set(data);
    };

    let form_data: Memo<HashMap<In, Value>> = Memo::new(move |_| {
        let profile = profile_form_data.get().clone();
        let sensitivity = sensitivity_form_data.get().clone();
        let recommendation = recommendation_form_data.get().clone();
        profile
            .into_iter()
            .chain(sensitivity.into_iter())
            .chain(recommendation.into_iter())
            .collect()
    });

    let is_logged_in = Memo::new(move |_| api.get().is_some());
    let save_result_message = RwSignal::new(None);

    let custom_emissions_message = RwSignal::new(String::new());
    let custom_edges = RwSignal::new(vec![]);
    let custom_values = RwSignal::<Vec<(Id, Value)>>::new(vec![]);
    let custom_leafs = RwSignal::<Vec<Id>>::new(vec![]);

    let clear_custom_values_and_edges = move || {
        custom_values.update(|values| values.clear());
        custom_edges.update(|edges| edges.clear());
    };

    fn try_id_lookup(id: String) -> Id {
        let id_lookup: HashMap<String, Id> = get_all_internal_nodes()
            .iter()
            .map(|&id| (format!("{:?}", id), id.into()))
            .collect::<HashMap<_, _>>();
        if let Some(r) = id_lookup.get(&id.to_string()) {
            r.clone()
        } else {
            id.to_string().into()
        }
    }

    let profile_outcome = Memo::new(move |_| {
        let values = profile_form_data
            .get()
            .into_iter()
            .map(|(id, value)| (Id::from(id), value))
            .collect();

        let custom_leafs = vec![];
        let custom_edges = None;

        calculate(&values, custom_edges, custom_leafs)
    });

    let sensitivity_outcome = Memo::new(move |_| {
        let profile = profile_form_data.get().clone();
        let sensitivity = sensitivity_form_data.get().clone();

        let custom_values = custom_values
            .get()
            .into_iter()
            .map(|(id, value)| (Id::from(id), value));

        let values = profile
            .into_iter()
            .chain(sensitivity.into_iter())
            .map(|(id, value)| (Id::from(id), value))
            .chain(custom_values)
            .collect();

        let edges = custom_edges.get();
        let leafs = custom_leafs.get();
        let custom_edges = if edges.is_empty() {
            None
        } else {
            Some(&*edges)
        };

        calculate(&values, custom_edges, leafs)
    });

    let recommendation_outcome = Memo::new(move |_| {
        let profile = profile_form_data.get().clone();
        let sensitivity = sensitivity_form_data.get().clone();
        let recommendation = recommendation_form_data.get().clone();

        let custom_values = custom_values
            .get()
            .into_iter()
            .map(|(id, value)| (Id::from(id), value));

        let values = profile
            .into_iter()
            .chain(sensitivity.into_iter())
            .chain(recommendation.into_iter())
            .map(|(id, value)| (Id::from(id), value))
            .chain(custom_values)
            .collect();

        let edges = custom_edges.get();
        let leafs = custom_leafs.get();
        let custom_edges = if edges.is_empty() {
            None
        } else {
            Some(&*edges)
        };

        calculate(&values, custom_edges, leafs)
    });

    let show_side_stream_controls = Memo::new(move |_| {
        form_data.with(|d| {
            optional_in!(In::SideStreamTreatmentTotalNitrogen, d).is_some_and(|v| v > Tons::zero())
        })
    });

    // TODO: allow to export at any time
    let show_csv_export =
        Signal::derive(move || recommendation_outcome.with(|out| out.output.is_some()));

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
                load_form_data(FormData::from(data));
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
                        let mut data = FormData::from(p.data);
                        let name = data
                            .get(&In::ProjectName)
                            .cloned()
                            .map(Value::as_text_unchecked);
                        if name.is_none() || name.as_deref() == Some("") {
                            data.insert(
                                In::ProjectName,
                                Value::text(DEFAULT_UNNAMED_PROJECT_TITLE),
                            );
                        }
                        p.data = data.into();
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
                    Project::Unsaved(p) => {
                        let mut p = FormData::from(p);
                        let name = p
                            .get(&In::ProjectName)
                            .cloned()
                            .map(Value::as_text_unchecked);
                        if name.is_none() || name.as_deref() == Some("") {
                            p.insert(In::ProjectName, Value::text(DEFAULT_UNNAMED_PROJECT_TITLE));
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
            load_form_data(FormData::default());
            current_project.set(None);
        }
    };

    let load_example_values = {
        move |()| {
            load_form_data(example_data::example_form_data());
        }
    };

    let download = {
        move |()| -> ObjectUrl {
            let data = JsonFormData::from(form_data.get());
            let project = Project::from(data);
            let data = Data { project };
            let json_bytes = export_to_vec_pretty(&data);

            let blob = Blob::new_with_options(&*json_bytes, Some("application/json"));

            ObjectUrl::from(blob)
        }
    };

    let save_project = {
        move |()| {
            let project_data = JsonFormData::from(form_data.get());
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
            let lang = crate::current_lang().get();
            let csv = presenter::calculation_outcome_as_csv(&recommendation_outcome.get(), lang);
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
        let data = p.form_data().clone().into();
        load_form_data(data);
    });

    create_effect(move |_| {
        let additional_custom_emissions_string = form_data.with(|values| {
            values
                .get(&In::AdditionalCustomEmissions)
                .cloned()
                .map(Value::as_text_unchecked)
        });

        let Some(input) = additional_custom_emissions_string else {
            custom_emissions_message.set("".to_string());
            clear_custom_values_and_edges();
            return;
        };
        let res = custom_emission_parser::parse_emission(
            input.as_str(),
            custom_emission_parser::NumberFormat::DE,
        );
        let Ok(r) = res.map_err(|err| {
            custom_emissions_message.set(err.to_string());
            clear_custom_values_and_edges();
        }) else {
            return;
        };

        let mut custom_edges_vec: Vec<(Id, Id)> = vec![];
        let mut custom_values_vec: Vec<(Id, Value)> = vec![];
        let mut custom_leafs_vec: Vec<Id> = vec![];

        r.iter().for_each(|e: &CustomEmission| match &e {
            CustomEmission::EdgeDefined(edge) => {
                custom_edges_vec.push((
                    edge.source.to_string().into(),
                    try_id_lookup(edge.target.to_string()),
                ));
                custom_leafs_vec.push(edge.source.to_string().into());
                custom_values_vec.push((edge.source.to_string().into(), Value::tons(edge.value)));
            }
            CustomEmission::EdgeUndefined(edge) => {
                custom_edges_vec.push((
                    edge.source.clone().into(),
                    try_id_lookup(edge.target.to_string()),
                ));
            }
        });
        let all_internal_nodes_names: Vec<String> = get_all_internal_nodes()
            .iter()
            .map(|x| format!("{:?}", x).to_string())
            .collect();

        match custom_emission_parser::check_graph(r, all_internal_nodes_names) {
            Ok(_) => {
                custom_emissions_message.set("".to_string());
                custom_values.set(custom_values_vec);
                custom_edges.set(custom_edges_vec);
                custom_leafs.set(custom_leafs_vec);
            }
            Err(e) => {
                custom_emissions_message.set(e.to_string());
                clear_custom_values_and_edges();
            }
        }
    });

    // -----   ----- //
    //     Views     //
    // -----   ----- //

    let section_view = move || match current_section.get() {
        PageSection::DataCollection => view! {
            <PlantProfile
              form_data = profile_form_data
              current_section
              outcome = profile_outcome.into()
              accessibility_always_show_option
            />
        }
        .into_view(),
        PageSection::Sensitivity => view! {
            <SensitivityParameters
              form_data = sensitivity_form_data
              current_section
              outcome = sensitivity_outcome.into()
              profile_outcome = profile_outcome.into()
              show_side_stream_controls = show_side_stream_controls.into()
              accessibility_always_show_option
              custom_emissions_message
            />
        }
        .into_view(),
        PageSection::Recommendation => view! {
            <Recommendations
              form_data = recommendation_form_data
              outcome = recommendation_outcome.into()
              sensitivity_outcome = sensitivity_outcome.into()
              show_side_stream_controls = show_side_stream_controls.into()
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
            logged_in = is_logged_in.into()
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
