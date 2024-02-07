use std::rc::Rc;

use gloo_file::{Blob, File, ObjectUrl};
use leptos::*;

use klick_app_charts::BarChart;
use klick_app_components::message::{ErrorMessage, SuccessMessage};
use klick_application::usecases::calculate_all_n2o_emission_factor_scenarios;
use klick_boundary::{
    export_to_vec_pretty, import_from_slice, Data, N2oEmissionFactorCalcMethod, Project, ProjectId,
    SavedProject,
};
use klick_domain as domain;
use klick_presenter::Lng;
use klick_presenter::ProfileValueId;

use crate::{
    api::AuthorizedApi,
    forms::{self, FieldSignal, MissingField},
    sankey::Sankey,
    Page,
};

mod breadcrumbs;
mod example_data;
mod field_sets;
mod fields;
mod input_data_list;
mod optimization_options;
mod project_menu;

use self::{
    breadcrumbs::Breadcrumbs,
    field_sets::field_sets,
    fields::{load_project_fields, read_input_fields, FieldId, ScenarioFieldId},
    input_data_list::InputDataList,
    optimization_options::OptimizationOptions,
    project_menu::ProjectMenu,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PageSection {
    #[default]
    DataCollection,
    OptimizationOptions,
}

impl PageSection {
    const fn section_id(self) -> &'static str {
        match self {
            PageSection::DataCollection => "data-collection",
            PageSection::OptimizationOptions => "optimization-options",
        }
    }
}

const DEFAULT_UNNAMED_PROJECT_TITLE: &str = "Unbenannt";

// TODO:
// Split this component into multiple tiny components.
#[component]
#[allow(clippy::too_many_lines)]
pub fn Tool(
    api: Signal<Option<AuthorizedApi>>,
    current_project: RwSignal<Option<Project>>,
) -> impl IntoView {
    let field_sets = field_sets();

    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let (signals, set_views, required_fields) = forms::render_field_sets(field_sets.clone());
    let signals = Rc::new(signals);
    let missing_fields: RwSignal<Vec<MissingField>> = RwSignal::new(Vec::<MissingField>::new());

    let input_data = RwSignal::new(Option::<domain::EmissionInfluencingValues>::None);
    let sankey_data =
        RwSignal::new(Option::<(domain::CO2Equivalents, domain::EmissionFactors)>::None);

    let sankey_header = RwSignal::new(String::new());
    let selected_scenario = RwSignal::new(Option::<u64>::Some(0));
    let selected_scenario_name = RwSignal::new(String::new());
    let barchart_arguments: RwSignal<Vec<klick_app_charts::BarChartArguments>> =
        RwSignal::new(vec![]);

    let current_section = RwSignal::new(Option::<PageSection>::None);
    let n2o_emission_factor_method =
        RwSignal::new(Option::<domain::N2oEmissionFactorCalcMethod>::None);

    let nitrogen_io_warning = RwSignal::new(Option::<String>::None);
    let chemical_oxygen_io_warning = RwSignal::new(Option::<String>::None);
    let phosphorus_io_warning = RwSignal::new(Option::<String>::None);
    let is_logged_in = Signal::derive(move || api.get().is_some());

    let custom_factor_value: RwSignal<Option<f64>> = signals
        .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    let save_result_message = RwSignal::new(None);

    let s = Rc::clone(&signals);

    // -----   ----- //
    //    Effects    //
    // -----   ----- //

    create_effect(move |_| {
        let (data, filtered_required_fields) = read_input_fields(&s, &required_fields);
        missing_fields.set(filtered_required_fields);
        input_data.set(data.try_into().ok());
    });

    create_effect(move |_| {
        let Some(n) = selected_scenario.get() else {
            n2o_emission_factor_method.set(None);
            return;
        };

        let f = match n {
            0 => domain::N2oEmissionFactorCalcMethod::TuWien2016,
            1 => domain::N2oEmissionFactorCalcMethod::Optimistic,
            2 => domain::N2oEmissionFactorCalcMethod::Pesimistic,
            3 => domain::N2oEmissionFactorCalcMethod::Ipcc2019,
            4 => domain::N2oEmissionFactorCalcMethod::Custom(domain::units::Factor::new(
                custom_factor_value.get().unwrap_or_default() / 100.0,
            )),
            _ => {
                n2o_emission_factor_method.set(None);
                return;
            }
        };
        n2o_emission_factor_method.set(Some(f));
    });

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        let Some(project) = current_project.get() else {
            return;
        };
        let (title, id) = match &project {
            Project::Saved(p) => (p.data.title.clone(), p.id.0.to_string()),
            Project::Unsaved(data) => (data.title.clone(), "<unsaved>".to_string()),
        };
        let title = title.unwrap_or_else(|| "<unsaved>".to_string());
        log::info!("Load project '{}' (ID = {}) fields", title, id);
        load_project_fields(&s, project.into());
    });

    create_effect(move |_| {
        if let Some(s) = current_section.get() {
            let id = s.section_id();
            let path = Page::Tool.path();
            let href = format!("{path}#{id}");
            window().location().set_href(&href).unwrap();
        }
    });

    let s = Rc::clone(&signals);

    create_effect(move |_| {
        let Some(input_data) = input_data.get() else {
            sankey_header.update(String::clear);
            barchart_arguments.update(Vec::clear);
            sankey_data.set(None);
            nitrogen_io_warning.set(None);
            chemical_oxygen_io_warning.set(None);
            phosphorus_io_warning.set(None);
            return;
        };

        let custom_factor_value = s
            .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
            .and_then(FieldSignal::get_float);

        let use_custom_factor = custom_factor_value.is_some();
        if !use_custom_factor && selected_scenario.get() == Some(4) {
            selected_scenario.set(Some(0));
        }

        let mut input_data_validation_error = false;

        if input_data.effluent_average.nitrogen > input_data.influent_average.nitrogen {
            nitrogen_io_warning.set(Some(format!(
                "Ablauf Gesamtstickstoff {} größer als dessen Zulauf {}!",
                Lng::De.format_number(input_data.effluent_average.nitrogen),
                Lng::De.format_number(input_data.influent_average.nitrogen)
            )));
            input_data_validation_error = true;
        } else {
            nitrogen_io_warning.set(None);
        }

        // TODO:
        // if let Some(chemical_oxygen_demand_influent) =
        //     input_data.influent_average.chemical_oxygen_demand
        // {
        //     if input_data.effluent_average.chemical_oxygen_demand
        //         > chemical_oxygen_demand_influent
        //     {
        //         chemical_oxygen_io_warning.set(Some(format!(
        //             "Ablauf Chemischer Sauerstoffbedarf {} größer als dessen Zulauf {}!",
        //             Lng::De.format_number(input_data.effluent_average.chemical_oxygen_demand),
        //             Lng::De.format_number(chemical_oxygen_demand_influent)
        //         )));
        //         input_data_validation_error = true;
        //     } else {
        //         chemical_oxygen_io_warning.set(None);
        //     }
        // }

        // TODO:
        // if let Some(phosphorus_influent) = input_data.influent_average.phosphorus {
        //     if let Some(phosphorus_effluent) = input_data.effluent_average.phosphorus {
        //         if phosphorus_effluent > phosphorus_influent {
        //             phosphorus_io_warning.set(Some(format!(
        //                 "Ablauf Phosphor {} größer als dessen Zulauf {}!",
        //                 Lng::De.format_number(phosphorus_effluent),
        //                 Lng::De.format_number(phosphorus_influent),
        //             )));
        //             input_data_validation_error = true;
        //         } else {
        //             phosphorus_io_warning.set(None);
        //         }
        //     }
        // }

        if input_data_validation_error {
            // prevent sankey or barchart from rendering
            sankey_data.set(None);
        }

        let custom_factor = custom_factor_value
            .map(|n| n / 100.0)
            .map(domain::units::Factor::new);
        let ch4_chp_calc_method = None;
        let n2o_calculations = calculate_all_n2o_emission_factor_scenarios(
            &input_data,
            custom_factor,
            ch4_chp_calc_method,
        );

        let szenario_calculations = if input_data_validation_error {
            vec![]
        } else {
            n2o_calculations
                .into_iter()
                .map(|(method, emissions, factors)| (method.into(), (emissions, factors)))
                .collect()
        };

        let name_ka: String = s
            .get(&ProfileValueId::PlantName.into())
            .and_then(FieldSignal::get_text)
            .unwrap_or_else(|| "Kläranlage".to_string());

        let ew = s
            .get(&ProfileValueId::PopulationEquivalent.into())
            .and_then(FieldSignal::get_float)
            .unwrap_or_default();

        let einheit = "t CO₂ Äquivalente/Jahr";

        if let Some(i) = selected_scenario.get() {
            if let Some((method, output_data)) = szenario_calculations.get(i as usize) {
                let szenario_name = label_of_n2o_emission_factor_calc_method(&method);
                selected_scenario_name.set(szenario_name.to_string().clone());
                let ef =
                    Lng::De.format_number_with_precision(f64::from(output_data.1.n2o) * 100.0, 2);
                let title = format!(
                    "{name_ka} ({ew} EW) / Treibhausgasemissionen [{einheit}] - Szenario {szenario_name} (N₂O EF={ef}%)"
                );
                sankey_header.set(title);
                sankey_data.set(Some(output_data.clone()));
            }
        }

        barchart_arguments.set(
            szenario_calculations
                .iter()
                .map(|(szenario, (co2_equivalents, emission_factors))| {
                    klick_app_charts::BarChartArguments {
                        label: Some(label_of_n2o_emission_factor_calc_method(szenario)),
                        co2_data: co2_equivalents.emissions.into(),
                        n2o_factor: f64::from(emission_factors.n2o),
                    }
                })
                .collect(),
        );
    });

    // -----   ----- //
    //    Actions    //
    // -----   ----- //

    let upload_action = create_action({
        let signals = Rc::clone(&signals);
        move |file: &File| {
            let signals = Rc::clone(&signals);
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
                load_project_fields(&signals, project);
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
                        if p.data.title.is_none() || p.data.title.as_deref() == Some("") {
                            p.data.title = Some(DEFAULT_UNNAMED_PROJECT_TITLE.to_string());
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
                        if p.title.is_none() || p.title.as_deref() == Some("") {
                            p.title = Some(DEFAULT_UNNAMED_PROJECT_TITLE.to_string());
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

    let clear_signals = {
        let signals = Rc::clone(&signals);
        move |_| {
            for s in signals.values() {
                s.clear();
            }
            current_project.set(None);
        }
    };

    let load_example_values = {
        let signals = Rc::clone(&signals);
        move |_| {
            current_project.set(None);
            example_data::load_example_field_signal_values(&signals);
        }
    };

    let download = {
        let signals = Rc::clone(&signals);
        move |_| {
            let project_data = fields::read_all_project_fields(&signals);
            let project = project_data.into();
            let data = Data { project };
            let json_bytes = export_to_vec_pretty(&data);

            let blob = Blob::new_with_options(&*json_bytes, Some("application/json"));

            ObjectUrl::from(blob)
        }
    };

    let save_project = {
        let signals = Rc::clone(&signals);
        move |_| {
            let project_data = fields::read_all_project_fields(&signals);
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

    let breadcrumps_entries = vec![
        ("Datenerfassung", PageSection::DataCollection),
        (
            "Auswertung & Handlungsempfehlungen",
            PageSection::OptimizationOptions,
        ),
    ];

    view! {
      <div class="space-y-10">
        <div class="flex center-items justify-between">
          <Breadcrumbs
            entries = { breadcrumps_entries }
            current = current_section
          />
          <ProjectMenu
            logged_in = is_logged_in
            clear = clear_signals
            load = load_example_values
            save = save_project
            download
            upload_action
          />
        </div>
        { move || save_result_message.get().map(|res| match res {
            Ok(msg) => view!{ <SuccessMessage message = msg /> }.into_view(),
            Err(msg) => view!{ <ErrorMessage message = msg /> }.into_view()
            })
        }
        { move || current_project.get()
            .and_then(|p|match p {
              Project::Saved(p) => Some(p),
              Project::Unsaved(_) => None
            })
            .map(|p| view! {
              <p class="text-xs text-gray-400 !-mb-8">
                <span class="font-semibold">
                  "Projekt ID: "
                </span>
                <span>
                  { p.id.0.to_string() }
                </span>
              </p>
            }.into_view())
        }
        <div
          id = PageSection::DataCollection.section_id()
          class = move || {
            if current_section.get() == Some(PageSection::DataCollection) || current_section.get() == None {
                None
            } else {
                Some("hidden")
            }
          }
        >
         { set_views }
        </div>
        <div
          class = move || {
            if current_section.get() == Some(PageSection::DataCollection) {
                Some("hidden")
            } else {
                None
            }
          }
        >
          <InputDataList
            field_sets = { &field_sets }
            signals = { &signals }
          />
        </div>
      </div>

      { move ||
          if barchart_arguments.get().is_empty() {
              Some(view! {
                <div class="mt-5">
                  <p>"Bitte ergänzen Sie folgende Werte, damit die Gesamtemissionen Ihrer Kläranlage, anhand verschiedener Szenarien, berechnet werden können:"</p>
                    <forms::HelperWidget
                      missing_fields=missing_fields.get()
                      before_focus = move || {
                        current_section.set(Some(PageSection::DataCollection));
                      }
                    />
                  <Show when= move || nitrogen_io_warning.get().is_some()>
                    <p>
                      <ul class="ml-5 my-4 list-disc list-inside">
                        <li>{ nitrogen_io_warning.get() }</li>
                      </ul>
                    </p>
                  </Show>
                  <Show when= move || chemical_oxygen_io_warning.get().is_some()>
                    <p>
                      <ul class="ml-5 my-4 list-disc list-inside">
                        <li>{ chemical_oxygen_io_warning.get() }</li>
                      </ul>
                    </p>
                  </Show>
                  <Show when= move || phosphorus_io_warning.get().is_some()>
                    <p>
                      <ul class="ml-5 my-4 list-disc list-inside">
                        <li>{ phosphorus_io_warning.get() }</li>
                      </ul>
                    </p>
                  </Show>
                  <p>"Bei jeder Eingabe werden die Graphen automatisch neu berechnet."</p>
                </div>
              })
          } else {
              None
          }
      }

      <h3 id = PageSection::OptimizationOptions.section_id() class="my-8 text-xl font-bold">
        "Auswertung Ihrer Daten (via Barchart / Sankey-Diagramm)"
      </h3>

      <Show
        when = move || !barchart_arguments.get().is_empty()
        fallback = || view! {
          <p>
            "Bitte ergänzen Sie im Eingabeformular die fehlenden Werte, damit die Emissionen berechnet und visualisiert werden können."
          </p>
        }
      >
        // bar diagram
        <h4 class="my-8 text-lg font-bold">"Szenarien im Vergleich - Treibhausgasemissionen [t CO₂ Äquivalente/Jahr]"</h4>
        <div class="">
          <BarChart
            width = 1200.0
            height = 400.0
            data  = barchart_arguments.get()
            selected_bar = selected_scenario
          />
        </div>
        <p>
          "Es ist das Szenario \"" { selected_scenario_name.get() } "\" ausgewählt.
          Durch Anklicken kann ein anderes Szenario ausgewählt werden."
        </p>

        // sankey diagram
        <h4 class="my-8 text-lg font-bold">
          { move || sankey_header.get().to_string() }
        </h4>
        { move || sankey_data.get().map(|data| view!{ <Sankey data /> }) }
      </Show>

      <Show
        when = move || current_section.get() == Some(PageSection::OptimizationOptions)
        fallback = move || view! {
          <button
            class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
            on:click = move |_| current_section.set(Some(PageSection::OptimizationOptions))
          >
             "zu den Handlungsempfehlungen"
          </button>
        }
      >
        <div class="my-8 border-b border-gray-200 pb-5" >
          <h3 class="text-xl font-semibold leading-6 text-gray-900">
            "Minderungsmaßnahmen für THG-Emissionen an Kläranlagen"
          </h3>
          <p class="mt-2 max-w-4xl text-lg text-gray-500">
            "Die vorgestellten Handlungsempfehlungen stellen eine erste Auswahl
            an möglichen Minderungsmaßnahmen für Treibhausgasemissionen (THG) an Kläranlagen dar.
            Diese sollen Ihnen wichtige Mehrwerte bieten, um die Klimaauswirkungen Ihrer Kläranlage zu minimieren
            und deren Wettbewerbsfähigkeit langfristig zu sichern.
            THG treten an mehreren Prozessschritten auf. Die
            Minderungsmaßnahmen fokussieren sich auf Methan-
            und Lachgasemissionen sowie energiebedingte Emissionen.
            Für bestimmte Maßnahmen kann ein konkretes Minderungspotenzial
            (aus der Literatur) abgeleitet werden, für andere kann vorerst nur
            der Hinweis ausgegeben werden, ob sich die Klimabilanz dadurch
            qualitativ verbessert.
            Unter nachfolgenden „aufklappbaren“ Abschnitten erhalten Sie weiterführende
            Informationen zu einzelnen Maßnahmen und können gegebenenfalls Ihre Klimabilanz
            neu berechnen lassen/verbessern."
          </p>
        </div>
        <OptimizationOptions
          input_data = input_data.into()
          n2o_emission_factor_method = n2o_emission_factor_method.into()
        />
      </Show>
    }
}

// TODO: move to presenter layer
const fn label_of_n2o_emission_factor_calc_method(
    method: &N2oEmissionFactorCalcMethod,
) -> &'static str {
    match method {
        N2oEmissionFactorCalcMethod::TuWien2016 => "TU Wien 2016",
        N2oEmissionFactorCalcMethod::Optimistic => "Optimistisch",
        N2oEmissionFactorCalcMethod::Pesimistic => "Pessimistisch",
        N2oEmissionFactorCalcMethod::Ipcc2019 => "IPCC 2019",
        N2oEmissionFactorCalcMethod::CustomFactor => "Benutzerdefiniert",
    }
}
