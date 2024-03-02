use chrono::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use gloo_file::{Blob, File, ObjectUrl};
use leptos::*;

use klick_app_charts::BarChart;
use klick_app_components::message::{ErrorMessage, SuccessMessage};
use klick_boundary::{
    export_to_vec_pretty, import_from_slice, Data, N2oEmissionFactorCalcMethod, Project, ProjectId,
    SavedProject,
};
use klick_domain as domain;
use klick_presenter::ProfileValueId;
use klick_presenter::{self as presenter, Lng};

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
mod project_menu;

pub mod widgets;

pub mod optimization_options;

pub mod sensitivity_options;

use self::{
    breadcrumbs::Breadcrumbs,
    field_sets::field_sets,
    fields::{load_project_fields, read_input_fields, FieldId, FieldSet},
    input_data_list::InputDataList,
    optimization_options::OptimizationOptions,
    project_menu::ProjectMenu,
    sensitivity_options::SensitivityOptions,
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
            PageSection::DataCollection => "data-collection",
            PageSection::Sensitivity => "data-sensitivity",
            PageSection::Recommendation => "data-recommendations",
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
    let missing_fields = RwSignal::new(Vec::<MissingField<_>>::new());
    let custom_sludge_bags_factor = RwSignal::new(Option::<f64>::None);
    let custom_sludge_storage_containers_factor = RwSignal::new(Option::<f64>::None);
    let co2_fossil_custom_factor: RwSignal<Option<f64>> = RwSignal::new(Option::<f64>::None);
    let input_data = RwSignal::new(Option::<domain::EmissionInfluencingValues>::None);
    let input_data_validation_error = RwSignal::new(false); // FIXME rename

    let input_data_optimizationOptions_model =
        RwSignal::new(Option::<domain::EmissionInfluencingValues>::None);
    let sankey_data =
        RwSignal::new(Option::<(domain::CO2Equivalents, domain::EmissionFactors)>::None);
    let sankey_header = RwSignal::new(String::new());
    let selected_scenario_chp = RwSignal::new(Option::<u64>::Some(0));
    let selected_scenario_n2o = RwSignal::new(Option::<u64>::Some(0));
    let selected_scenario_name_chp = RwSignal::new(String::new());
    let selected_scenario_name_n2o = RwSignal::new(String::new());
    let barchart_arguments_radio_inputs: RwSignal<
        Vec<klick_app_charts::BarChartRadioInputArguments>,
    > = RwSignal::new(vec![]);
    let barchart_arguments_radio_inputs_bhkw: RwSignal<
        Vec<klick_app_charts::BarChartRadioInputArguments>,
    > = RwSignal::new(vec![]);

    let current_section = RwSignal::new(Option::<PageSection>::Some(PageSection::DataCollection));

    let nitrogen_io_warning = RwSignal::new(Option::<String>::None);
    let chemical_oxygen_io_warning = RwSignal::new(Option::<String>::None);
    let phosphorus_io_warning = RwSignal::new(Option::<String>::None);
    let is_logged_in = Signal::derive(move || api.get().is_some());

    let save_result_message = RwSignal::new(None);
    let show_handlungsempfehlungen: RwSignal<bool> = RwSignal::new(false);
    let output_optimization_options_model =
        RwSignal::new(Option::<domain::EmissionsCalculationOutcome>::None);
    let sankey_data_optimization_options_model =
        RwSignal::new(Option::<domain::EmissionsCalculationOutcome>::None);
    let sankey_header_optimization_options_model = RwSignal::new(String::new());
    let barchart_arguments: RwSignal<Vec<klick_app_charts::BarChartArguments>> =
        RwSignal::new(vec![]);
    let custom_factor_bhkw: RwSignal<Option<f64>> = Some(3.0 as f64).into();
    let custom_factor_n2o: RwSignal<Option<f64>> = Some(3.0 as f64).into();
    let n2o_emission_factor_method =
        RwSignal::new(Option::<domain::N2oEmissionFactorCalcMethod>::None);
    let n2o_side_stream_cover_is_open: RwSignal<Option<bool>> = None.into();
    let sludge_bags_are_open: RwSignal<Option<bool>> = RwSignal::new(None);
    let sludge_storage_containers_are_open: RwSignal<Option<bool>> = RwSignal::new(None);

    let s = Rc::clone(&signals);

    // -----   ----- //
    //    Effects    //
    // -----   ----- //

    create_effect(move |_| {
        let (mut data, filtered_required_fields) = read_input_fields(&s, &required_fields);
        if data.energy_consumption.sewage_gas_produced.is_none() {
            data.energy_consumption.sewage_gas_produced = Some(0.0);
        }
        if data.energy_consumption.methane_fraction.is_none() {
            data.energy_consumption.methane_fraction = Some(0.0);
        }
        if data.energy_consumption.on_site_power_generation.is_none() {
            data.energy_consumption.on_site_power_generation = Some(0.0);
        }
        if data.sewage_sludge_treatment.transport_distance.is_none() {
            data.sewage_sludge_treatment.transport_distance = Some(0.0);
        }
        missing_fields.set(filtered_required_fields);
        input_data.set(data.try_into().ok());
    });

    create_effect(move |_| {
        let Some(n) = selected_scenario_n2o.get() else {
            n2o_emission_factor_method.set(None);
            return;
        };

        let f = match n {
            0 => domain::N2oEmissionFactorCalcMethod::TuWien2016,
            1 => domain::N2oEmissionFactorCalcMethod::Optimistic,
            2 => domain::N2oEmissionFactorCalcMethod::Pesimistic,
            3 => domain::N2oEmissionFactorCalcMethod::Ipcc2019,
            4 => domain::N2oEmissionFactorCalcMethod::Custom(domain::units::Factor::new(
                custom_factor_n2o.get().unwrap_or_default() / 100.0,
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
            // FIXME check also required fields
            sankey_header.update(String::clear);
            show_handlungsempfehlungen.set(false);
            barchart_arguments_radio_inputs.update(Vec::clear);
            sankey_data.set(None);
            nitrogen_io_warning.set(None);
            chemical_oxygen_io_warning.set(None);
            phosphorus_io_warning.set(None);
            return;
        };
        if input_data.effluent_average.nitrogen > input_data.influent_average.nitrogen {
            nitrogen_io_warning.set(Some(format!(
                "Ablauf Gesamtstickstoff {} größer als dessen Zulauf {}!",
                Lng::De.format_number(input_data.effluent_average.nitrogen),
                Lng::De.format_number(input_data.influent_average.nitrogen)
            )));
            input_data_validation_error.set(true);
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
        //         input_data_validation_error.set(true);
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
        //             input_data_validation_error.set(true);
        //         } else {
        //             phosphorus_io_warning.set(None);
        //         }
        //     }
        // }

        if input_data_validation_error.get() {
            sankey_data.set(None);
        }

        let ch4_chp_calc_method = match selected_scenario_chp.get() {
            Some(0) => Some(domain::CH4ChpEmissionFactorCalcMethod::MicroGasTurbines),
            Some(1) => Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine),
            Some(2) => Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine),
            Some(3) => match custom_factor_bhkw.get() {
                Some(f) => Some(domain::CH4ChpEmissionFactorCalcMethod::Custom(
                    domain::units::Factor::new(f / 100.0),
                )),
                None => None,
            },
            _ => Some(domain::CH4ChpEmissionFactorCalcMethod::MicroGasTurbines),
        };
        let n2o_calculations = domain::calculate_all_n2o_emission_factor_scenarios(
            &input_data,
            Some(domain::units::Factor::new(
                custom_factor_n2o.get().unwrap_or_default() / 100.0,
            )),
            ch4_chp_calc_method,
        );

        let szenario_calculations = if input_data_validation_error.get() {
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

        if let Some(i) = selected_scenario_n2o.get() {
            if let Some((method, output_data)) = szenario_calculations.get(i as usize) {
                let szenario_name = label_of_n2o_emission_factor_calc_method(&method);
                selected_scenario_name_n2o.set(szenario_name.to_string().clone());
                let ef =
                    Lng::De.format_number_with_precision(f64::from(output_data.1.n2o) * 100.0, 2);
                let title = format!(
                    "{name_ka} ({ew} EW) / Treibhausgasemissionen [{einheit}] - Szenario {szenario_name} (N₂O EF={ef}%)"
                );
                sankey_header.set(title);
                sankey_data.set(Some(output_data.clone()));
            }
        }

        barchart_arguments_radio_inputs.set(
            szenario_calculations
                .iter()
                .map(|(szenario, (co2_equivalents, emission_factors))| {
                    klick_app_charts::BarChartRadioInputArguments {
                        label: Some(label_of_n2o_emission_factor_calc_method(szenario)),
                        value: co2_equivalents.total_emissions.into(),
                        emission_factor: f64::from(emission_factors.n2o),
                    }
                })
                .collect(),
        );
        if !input_data_validation_error.get() {
            log::info!("computing final output data");
            let ch4_chp_emission_factor: Option<domain::CH4ChpEmissionFactorCalcMethod> =
                match selected_scenario_chp.get() {
                    Some(0) => Some(domain::CH4ChpEmissionFactorCalcMethod::MicroGasTurbines),
                    Some(1) => Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine),
                    Some(2) => Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine),
                    Some(3) => match custom_factor_bhkw.get() {
                        Some(f) => Some(domain::CH4ChpEmissionFactorCalcMethod::Custom(
                            domain::units::Factor::new(f / 100.0),
                        )),
                        None => None,
                    },
                    _ => Some(domain::CH4ChpEmissionFactorCalcMethod::MicroGasTurbines),
                };
            // TODO: move to presenter layer
            selected_scenario_name_chp.set(
                selected_scenario_chp
                    .get()
                    .map(|x| match x {
                        0 => "Mikrogasturbinen",
                        1 => "Ottomotor",
                        2 => "Zündstrahlmotor",
                        3 => "Benutzerdefiniert",
                        _ => "",
                    })
                    .unwrap_or("")
                    .to_string(),
            );
            let scenario = domain::EmissionFactorCalculationMethods {
                n2o: n2o_emission_factor_method
                    .get()
                    .unwrap_or(domain::N2oEmissionFactorCalcMethod::Ipcc2019),
                ch4: ch4_chp_emission_factor,
            };
            let mut input_data = input_data.clone();
            input_data.sewage_sludge_treatment.sludge_bags_are_open =
                sludge_bags_are_open.get().unwrap_or(true);
            input_data
                .sewage_sludge_treatment
                .sludge_storage_containers_are_open =
                sludge_storage_containers_are_open.get().unwrap_or(true);
            input_data.sewage_sludge_treatment.custom_sludge_bags_factor =
                custom_sludge_bags_factor.get();
            input_data
                .sewage_sludge_treatment
                .custom_sludge_storage_containers_factor =
                custom_sludge_storage_containers_factor.get();
            input_data_optimizationOptions_model.set(Some(input_data.clone()));
            let output = domain::calculate_emissions(input_data.clone(), scenario);
            output_optimization_options_model.set(Some(output.clone()));
            sankey_data_optimization_options_model.set(Some(output.clone()));
            sankey_header_optimization_options_model.set(format!(
                "{name_ka} ({ew} EW) / Treibhausgasemissionen [{einheit}] - Szenario {szenario_name}",
                name_ka = name_ka,
                ew = Lng::De.format_number(ew),
                einheit = einheit,
                szenario_name = selected_scenario_name_chp.get()
            ));
            log::info!("computing barchart_arguments_radio_inputs_bhkw");
            // TODO: move to presenter layer
            barchart_arguments_radio_inputs_bhkw.set(
                (vec![
                    (0, "Mikrogasturbinen"),
                    (1, "Ottomotor"),
                    (2, "Zündstrahlmotor"),
                    (3, "Benutzerdefiniert"),
                ])
                .iter()
                .filter_map(|i| {
                    let ch4_chp_emission_factor: Option<domain::CH4ChpEmissionFactorCalcMethod> =
                        match i.0 {
                            0 => Some(domain::CH4ChpEmissionFactorCalcMethod::MicroGasTurbines),
                            1 => Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine),
                            2 => Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine),
                            3 => match custom_factor_bhkw.get() {
                                Some(f) => Some(domain::CH4ChpEmissionFactorCalcMethod::Custom(
                                    domain::units::Factor::new(f / 100.0),
                                )),
                                None => None,
                            },
                            _ => Some(domain::CH4ChpEmissionFactorCalcMethod::MicroGasTurbines),
                        };
                    let scenario = domain::EmissionFactorCalculationMethods {
                        n2o: n2o_emission_factor_method
                            .get()
                            .unwrap_or(domain::N2oEmissionFactorCalcMethod::Ipcc2019),
                        ch4: ch4_chp_emission_factor,
                    };
                    let mut input_data = input_data.clone();
                    input_data.sewage_sludge_treatment.sludge_bags_are_open =
                        sludge_bags_are_open.get().unwrap_or(true);
                    input_data
                        .sewage_sludge_treatment
                        .sludge_storage_containers_are_open =
                        sludge_storage_containers_are_open.get().unwrap_or(true);
                    let output = domain::calculate_emissions(input_data, scenario);
                    if f64::from(output.co2_equivalents.ch4_combined_heat_and_power_plant) < 0.1 {
                        return None;
                    }
                    Some(klick_app_charts::BarChartRadioInputArguments {
                        label: Some(i.1),
                        value: f64::from(output.co2_equivalents.ch4_combined_heat_and_power_plant),
                        emission_factor: f64::from(output.emission_factors.ch4),
                    })
                })
                .collect(),
            );

            let old = szenario_calculations[selected_scenario_n2o.get().unwrap_or(0) as usize]
                .clone()
                .1
                 .0;
            let new = output.co2_equivalents;

            let mut comp = vec![];
            let sludgy = f64::from(new.ch4_sludge_bags) - f64::from(old.ch4_sludge_bags);
            comp.push(klick_app_charts::BarChartArguments {
                label: "CH₄ Schlupf Schlammtasche",
                value: sludgy,
                percentage: Some(sludgy / f64::from(new.total_emissions) * 100.0),
            });
            let schlammy = f64::from(new.ch4_sludge_storage_containers)
                - f64::from(old.ch4_sludge_storage_containers);
            comp.push(klick_app_charts::BarChartArguments {
                label: "CH₄ Schlupf Schlammlagerung",
                value: schlammy,
                percentage: Some(schlammy / f64::from(new.total_emissions) * 100.0),
            });
            let bhkwy = f64::from(new.ch4_combined_heat_and_power_plant)
                - f64::from(old.ch4_combined_heat_and_power_plant);
            comp.push(klick_app_charts::BarChartArguments {
                label: "CH₄ BHKW",
                value: bhkwy,
                percentage: Some(bhkwy / f64::from(new.total_emissions) * 100.0),
            });
            let excessy = -1.0 * f64::from(new.excess_energy_co2_equivalent);
            comp.push(klick_app_charts::BarChartArguments {
                label: "Strombedarf",
                value: excessy,
                percentage: Some(excessy / f64::from(new.total_emissions) * 100.0),
            });
            let emissionsy = f64::from(new.total_emissions)
                - f64::from(old.total_emissions)
                - f64::from(new.excess_energy_co2_equivalent);
            comp.push(klick_app_charts::BarChartArguments {
                label: "Emissionen",
                value: emissionsy,
                percentage: Some(emissionsy / f64::from(new.total_emissions) * 100.0),
            });
            barchart_arguments.set(comp);
            if missing_fields.get().len() > 0 {
                show_handlungsempfehlungen.set(false);
            } else {
                show_handlungsempfehlungen.set(true);
            }
        } else {
            log::info!("NOT computing final output data, input incomplete");
            output_optimization_options_model.set(None);
            sankey_header_optimization_options_model.set(String::new());
            sankey_data_optimization_options_model.set(None);
            barchart_arguments.set(vec![]);
            show_handlungsempfehlungen.set(false);
        }
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

    let export_csv = {
        move |_| {
            let mut s: String = "".to_string();
            s.push_str(
                format!(
                    "\n# export from klimabilanzklaeranlage.de - {}\n",
                    Utc::now()
                )
                .as_str(),
            );

            s.push_str("\n# input_data_optimizationOptions_model\n");
            if let Some(v) = input_data_optimizationOptions_model.get() {
                s.push_str(&v.to_csv());
            }
            //output_optimization_options_model
            sankey_data_optimization_options_model.get().map(
                |domain::EmissionsCalculationOutcome {
                     co2_equivalents,
                     emission_factors,
                     calculation_methods,
                 }| {
                    s.push_str("\n# sankey_data_optimization_options_model\n");
                    s.push_str(&co2_equivalents.to_csv());
                    s.push_str(&presenter::emission_factors_to_csv(&emission_factors));
                    s.push_str(&presenter::emission_factor_calculation_methods_to_csv(
                        calculation_methods,
                    ));
                },
            );

            // final result +/- values of emissions FIXME
            s.push_str("\n# final result +/- values of emissions\n");
            for b in barchart_arguments.get() {
                s.push_str(&b.label);
                s.push_str(",");
                s.push_str(&b.value.to_string());
                s.push_str("\n");
            }

            let example_file =
                File::new_with_options("", s.as_str(), Some("text/plain"), Some(Utc::now().into()));
            ObjectUrl::from(example_file)
        }
    };

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
        ("Sensitivität", PageSection::Sensitivity),
        ("Handlungsempfehlungen", PageSection::Recommendation),
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
            export_csv
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
          class = move || {
            if current_section.get() == Some(PageSection::DataCollection){
                None
            } else {
                Some("hidden")
            }
          }
        >
        <DataCollectionView
          current_section
          show_handlungsempfehlungen
          set_views
          nitrogen_io_warning
          chemical_oxygen_io_warning
          phosphorus_io_warning
          missing_fields
          input_data
          sankey_data
          sankey_header
        />
        </div>

        <div
          class = move || {
            if current_section.get() == Some(PageSection::Sensitivity) {
                None
            } else {
                Some("hidden")
            }
          }
        >
        <SensitivityView
          current_section
          show_handlungsempfehlungen
          output_optimization_options_model
          selected_scenario_n2o
          selected_scenario_chp
          custom_factor_bhkw
          barchart_arguments_radio_inputs
          barchart_arguments_radio_inputs_bhkw
          sankey_data
          sankey_header
          selected_scenario_name_n2o
          selected_scenario_name_chp
          custom_factor_n2o
          co2_fossil_custom_factor
        />
        </div>

        <div
          class = move || {
            if current_section.get() == Some(PageSection::Recommendation) {
                None
            } else {
                Some("hidden")
            }
          }
        >
        <RecommendationView
          current_section
          show_handlungsempfehlungen
          output_optimization_options_model
          sludge_bags_are_open
          sludge_storage_containers_are_open
          barchart_arguments
          custom_sludge_bags_factor
          custom_sludge_storage_containers_factor
          sankey_data_optimization_options_model
          sankey_header_optimization_options_model
          field_sets
          signals = Rc::clone(&signals)
          n2o_side_stream_cover_is_open
        />
        </div>
      </div>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn DataCollectionView(
    current_section: RwSignal<Option<PageSection>>,
    show_handlungsempfehlungen: RwSignal<bool>,
    set_views: Vec<View>,
    nitrogen_io_warning: RwSignal<Option<String>>,
    chemical_oxygen_io_warning: RwSignal<Option<String>>,
    phosphorus_io_warning: RwSignal<Option<String>>,
    missing_fields: RwSignal<Vec<MissingField<FieldId>>>,
    input_data: RwSignal<Option<domain::EmissionInfluencingValues>>,
    sankey_data: RwSignal<Option<(domain::CO2Equivalents, domain::EmissionFactors)>>,
    sankey_header: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div id = PageSection::DataCollection.section_id()>
          { set_views.clone() } // input fields for data collection
        </div>

      { move ||
          if !show_handlungsempfehlungen.get() {
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
      <div
        class = move || {
          if input_data.get().is_some() {
              None
            } else {
                Some("hidden")
            }
          }
        >
        // sankey diagram
        <h4 class="my-8 text-lg font-bold">
          { move || sankey_header.get().to_string() }
        </h4>
        { move || sankey_data.get().map(|data| view!{ <Sankey data /> }) }
      </div>
      <Show when = move || show_handlungsempfehlungen.get()>
        <button
            class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
            on:click = move |_| current_section.set(Some(PageSection::Sensitivity))
          >
             "zur Sensitivität"
        </button>
      </Show>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn SensitivityView(
    current_section: RwSignal<Option<PageSection>>,
    show_handlungsempfehlungen: RwSignal<bool>,
    output_optimization_options_model: RwSignal<Option<domain::EmissionsCalculationOutcome>>,
    selected_scenario_n2o: RwSignal<Option<u64>>,
    selected_scenario_chp: RwSignal<Option<u64>>,
    custom_factor_bhkw: RwSignal<Option<f64>>,
    barchart_arguments_radio_inputs: RwSignal<Vec<klick_app_charts::BarChartRadioInputArguments>>,
    barchart_arguments_radio_inputs_bhkw: RwSignal<
        Vec<klick_app_charts::BarChartRadioInputArguments>,
    >,
    sankey_data: RwSignal<Option<(domain::CO2Equivalents, domain::EmissionFactors)>>,
    sankey_header: RwSignal<String>,
    selected_scenario_name_n2o: RwSignal<String>,
    selected_scenario_name_chp: RwSignal<String>,
    custom_factor_n2o: RwSignal<Option<f64>>,
    co2_fossil_custom_factor: RwSignal<Option<f64>>,
) -> impl IntoView {
    view! {
        <DataCollectionEnforcementHelper
            show_handlungsempfehlungen = show_handlungsempfehlungen
            current_section = current_section
        />
        <div
          class = move || {
            if show_handlungsempfehlungen.get() {
                None
            } else {
                Some("hidden")
            }
          }
        >
        { move || {
            view! {
              <SensitivityOptions
                output = output_optimization_options_model.read_only()
                selected_scenario_n2o
                selected_scenario_chp
                custom_factor_bhkw = custom_factor_bhkw
                barchart_arguments_radio_inputs = barchart_arguments_radio_inputs.read_only()
                barchart_arguments_radio_inputs_bhkw = barchart_arguments_radio_inputs_bhkw.read_only()
                selected_scenario_name_n2o
                selected_scenario_name_chp
                custom_factor_n2o
                co2_fossil_custom_factor
              />
            }
          }
        }
        <div
        class = move || {
          if show_handlungsempfehlungen.get() {
              None
            } else {
                Some("hidden")
            }
          }
        >
        // sankey diagram
        <h4 class="my-8 text-lg font-bold">
          { move || sankey_header.get().to_string() }
        </h4>
        { move || sankey_data.get().map(|data| view!{ <Sankey data /> }) }
        </div>
        </div>
        <Show when = move || show_handlungsempfehlungen.get()>
        <button
            class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
            on:click = move |_| current_section.set(Some(PageSection::Recommendation))
          >
             "zur den Handlungsempfehlungen"
        </button>
        </Show>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn RecommendationView(
    current_section: RwSignal<Option<PageSection>>,
    show_handlungsempfehlungen: RwSignal<bool>,
    output_optimization_options_model: RwSignal<Option<domain::EmissionsCalculationOutcome>>,
    sludge_bags_are_open: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open: RwSignal<Option<bool>>,
    barchart_arguments: RwSignal<Vec<klick_app_charts::BarChartArguments>>,
    custom_sludge_bags_factor: RwSignal<Option<f64>>,
    custom_sludge_storage_containers_factor: RwSignal<Option<f64>>,
    sankey_data_optimization_options_model: RwSignal<Option<domain::EmissionsCalculationOutcome>>,
    sankey_header_optimization_options_model: RwSignal<String>,
    field_sets: Vec<FieldSet>,
    signals: Rc<HashMap<FieldId, FieldSignal>>,
    n2o_side_stream_cover_is_open: RwSignal<Option<bool>>,
) -> impl IntoView {
    view! {
        <DataCollectionEnforcementHelper
            show_handlungsempfehlungen = show_handlungsempfehlungen
            current_section = current_section
        />

        <div
          class = move || {
            if show_handlungsempfehlungen.get() {
                None
            } else {
                Some("hidden")
            }
          }
        >
        <Show when= move || show_handlungsempfehlungen.get()>
        <div>
          <InputDataList // FIXME refactor name, also compute arguments top level
            field_sets = { &field_sets }
            signals = { &signals }
          />
        </div>
        </Show>
        <div class="my-8 border-b border-gray-200 pb-5" >
          <h3 class="text-xl font-semibold leading-6 text-gray-900">
            "Minderungsmaßnahmen für THG-Emissionen an Kläranlagen"
          </h3>
        <p class="mt-2 max-w-4xl text-lg text-gray-500">
          "Die vorgestellten Handlungsempfehlungen stellen eine erste Auswahl an
          möglichen Minderungsmaßnahmen für Treibhausgasemissionen (THG) an Kläranlagen
          dar. Diese sollen Ihnen wichtige Mehrwerte bieten, um die Klimaauswirkungen Ihrer
          Kläranlage zu minimieren und deren Wettbewerbsfähigkeit langfristig zu sichern."
        </p>
          <p class="mt-2 max-w-4xl text-lg text-gray-500">
            "THG treten an mehreren Prozessschritten auf. Die
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
          <p class="mt-2 max-w-4xl text-lg text-gray-500">
            <b>
            "Unter nachfolgenden „aufklappbaren“ Abschnitten erhalten Sie weiterführende
            Informationen zu einzelnen Maßnahmen und können gegebenenfalls Ihre Klimabilanz
            neu berechnen lassen/verbessern."
            </b>
          </p>
        </div>
        <div>
          { move || {
              view! {
                <OptimizationOptions
                  output = output_optimization_options_model.read_only()
                  sludge_bags_are_open
                  sludge_storage_containers_are_open
                  custom_sludge_bags_factor
                  custom_sludge_storage_containers_factor
                  n2o_side_stream_cover_is_open
                />
              }
            }
          }
          </div>
          <div
            class = move || {
              if sankey_data_optimization_options_model.get().is_some() {
                  None
              } else {
                  Some("hidden")
              }
            }
          >
            <div>
              <h4 class="my-8 text-lg font-bold">
                { move || sankey_header_optimization_options_model.get().to_string() }
              </h4>
              { move ||
               sankey_data_optimization_options_model.get().map(|domain::EmissionsCalculationOutcome{co2_equivalents, emission_factors, ..}| {
                 let data = (co2_equivalents, emission_factors);
                 view!{ <Sankey data /> }
               } )
              }
            </div>
          </div>
          <div
            class = move || {
              if barchart_arguments.get().iter().any(|x| f64::abs(x.value) > 0.1) {
                  None
              } else {
                  Some("hidden")
              }
            }
          >
            <div class="mx-auto p-8" >
              <h3 class="text-xl font-semibold leading-6 text-gray-900">
                "Änderungen durch Optionen der Handlungsmaßnahmen"
              </h3>
              <p class="mt-2 max-w-4xl text-lg text-gray-500">
                "Die folgende Grafik zeigt die Änderungen der Treibhausgasemissionen [t CO₂ Äquivalente/Jahr] bzw. % der Gesamtemissionen durch die ausgewählten Handlungsmaßnahmen."
              </p>
              { move || {
                      let barchart_arguments_filtered: Vec<klick_app_charts::BarChartArguments> = barchart_arguments.get()
                        .iter()
                        .filter_map(|x| {
                            if f64::abs(x.value) > 0.1 {
                                Some(x.clone())
                            } else {
                                None
                            }
                        }).collect();
                      view! {
                      <BarChart
                          width = 1100.0
                          height = 400.0
                          data=barchart_arguments_filtered
                      />
                      }
                  }
              }
            </div>
          </div>
        </div>
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn DataCollectionEnforcementHelper(
    show_handlungsempfehlungen: RwSignal<bool>,
    current_section: RwSignal<Option<PageSection>>,
) -> impl IntoView {
    view! {
      <Show when = move || !show_handlungsempfehlungen.get()>
        <div class="my-8 border-b border-gray-200 pb-5" >
        <p>
        "Bitte ergänzen Sie im Eingabeformular die fehlenden Werte, damit die Emissionen berechnet und visualisiert werden können."
        </p>
        </div>
        <button
         class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
         on:click = move |_| current_section.set(Some(PageSection::DataCollection))
        >
        "zu der Datenerfassung"
        </button>
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
