use std::collections::HashMap;

use crate::{v1, v2, v3, v4, v5, v6, v7, v8, v9};

const V1_OPERATING_MATERIALS_DIVISOR: f64 = 1_000.0;

pub fn from_v1(data: v1::Import) -> v2::Import {
    let v1::Import {
        input:
            v1::InputData {
                plant_name,
                population_values,
                waste_water,
                inflow_averages,
                effluent_averages,
                energy_consumption,
                sewage_sludge_treatment,
                operating_materials,
            },
        scenario,
    } = data;

    let v2::OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    } = operating_materials;

    let v1::EnergyConsumption {
        sewage_gas_produced,
        methane_level,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        in_house_power_generation,
        emission_factor_electricity_mix,
    } = energy_consumption;

    let methane_fraction = methane_level;
    let on_site_power_generation = in_house_power_generation;

    let energy_consumption = v2::EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
    };

    let map_value = |v| v / V1_OPERATING_MATERIALS_DIVISOR;
    let fecl3 = fecl3.map(map_value);
    let feclso4 = feclso4.map(map_value);
    let caoh2 = caoh2.map(map_value);
    let synthetic_polymers = synthetic_polymers.map(map_value);

    let operating_materials = v2::OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    };

    let population_equivalent = population_values;
    let wastewater = waste_water;
    let effluent_average = effluent_averages;
    let influent_average = inflow_averages;

    let input = v2::InputData {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    };

    v2::Import { input, scenario }
}

pub fn from_v2(data: v2::Import) -> v3::Import {
    let v2::Import {
        input:
            v2::InputData {
                plant_name,
                population_equivalent,
                wastewater,
                influent_average,
                effluent_average,
                energy_consumption,
                sewage_sludge_treatment,
                operating_materials,
            },
        scenario: v2::Scenario {
            n2o_emission_factor,
        },
    } = data;

    let scenario = v3::Scenario {
        n2o_emission_factor,
        ch4_chp_emission_factor: None,
    };

    v3::Import {
        input: v3::InputData {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        },
        scenario,
    }
}

pub fn from_v3(data: v3::Import) -> v4::Import {
    let v3::Import {
        input:
            v3::InputData {
                plant_name,
                population_equivalent,
                wastewater,
                influent_average,
                effluent_average,
                energy_consumption,
                sewage_sludge_treatment,
                operating_materials,
            },
        scenario:
            v3::Scenario {
                n2o_emission_factor,
                ch4_chp_emission_factor,
            },
    } = data;

    let v3::N2oEmissionFactorScenario {
        calculation_method,
        custom_factor,
    } = n2o_emission_factor;

    let calculation_method = calculation_method.into();

    let n2o_emission_factor = v4::N2oEmissionFactorScenario {
        calculation_method,
        custom_factor,
    };

    let scenario = v4::Scenario {
        n2o_emission_factor,
        ch4_chp_emission_factor,
    };

    v4::Import {
        input: v3::InputData {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        },
        scenario,
    }
}

impl From<v3::N2oEmissionFactorCalcMethod> for v4::N2oEmissionFactorCalcMethod {
    fn from(from: v3::N2oEmissionFactorCalcMethod) -> Self {
        use v3::N2oEmissionFactorCalcMethod as M3;
        use v4::N2oEmissionFactorCalcMethod as M4;

        match from {
            M3::ExtrapolatedParravicini => M4::TuWien2016,
            M3::Optimistic => M4::Optimistic,
            M3::Pesimistic => M4::Pesimistic,
            M3::Ipcc2019 => M4::Ipcc2019,
            M3::CustomFactor => M4::CustomFactor,
        }
    }
}

pub fn from_v4(data: v4::Import) -> v5::Data {
    let v4::Import { input, scenario } = data;

    let plant_profile = input;
    let optimization_scenario = scenario;

    let unsaved_project = v5::ProjectData {
        title: None,
        plant_profile,
        optimization_scenario,
    };

    let project = v5::Project::from(unsaved_project);

    v5::Data { project }
}

impl From<v5::ProjectData> for v6::ProjectData {
    fn from(from: v5::ProjectData) -> Self {
        let v5::ProjectData {
            title,
            plant_profile,
            optimization_scenario,
        } = from;

        let v5::PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        } = plant_profile;

        let v5::SewageSludgeTreatment {
            open_sludge_bags,
            open_sludge_storage_containers,
            sewage_sludge_for_disposal,
            transport_distance,
        } = sewage_sludge_treatment;

        let sewage_sludge_treatment = v6::SewageSludgeTreatment {
            sludge_bags_are_open: open_sludge_bags,
            sludge_storage_containers_are_open: open_sludge_storage_containers,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count: None,
        };

        let plant_profile = v6::PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        };

        Self {
            title,
            plant_profile,
            optimization_scenario,
        }
    }
}

pub fn from_v5(data: v5::Data) -> v6::Data {
    let v5::Data { project } = data;

    let project = match project {
        v5::Project::Saved(saved_project) => {
            let v5::SavedProject {
                id,
                created_at,
                modified_at,
                data,
            } = saved_project;
            let data = data.into();
            let saved = v6::SavedProject {
                id,
                created_at,
                modified_at,
                data,
            };
            v6::Project::Saved(saved)
        }
        v5::Project::Unsaved(unsaved_project) => v6::Project::Unsaved(unsaved_project.into()),
    };

    v6::Data { project }
}

impl From<v6::ProjectData> for v7::ProjectData {
    fn from(from: v6::ProjectData) -> Self {
        let v6::ProjectData {
            title,
            plant_profile,
            optimization_scenario,
        } = from;

        let v6::PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        } = plant_profile;

        let v6::SewageSludgeTreatment {
            sludge_bags_are_open,
            sludge_storage_containers_are_open,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        } = sewage_sludge_treatment;

        let sewage_sludge_treatment = v7::SewageSludgeTreatment {
            sludge_bags_are_open,
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open,
            custom_sludge_storage_containers_factor: None,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        };

        let plant_profile = v7::PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        };

        Self {
            title,
            plant_profile,
            optimization_scenario,
        }
    }
}

pub fn from_v6(data: v6::Data) -> v7::Data {
    let v6::Data { project } = data;

    let project = match project {
        v6::Project::Saved(saved_project) => {
            let v6::SavedProject {
                id,
                created_at,
                modified_at,
                data,
            } = saved_project;
            let data = data.into();
            let saved = v7::SavedProject {
                id,
                created_at,
                modified_at,
                data,
            };
            v7::Project::Saved(saved)
        }
        v6::Project::Unsaved(unsaved_project) => v7::Project::Unsaved(unsaved_project.into()),
    };

    v7::Data { project }
}

impl From<v7::ProjectData> for v8::JsonFormData {
    fn from(from: v7::ProjectData) -> Self {
        let v7::ProjectData {
            title: project_title,
            plant_profile,
            optimization_scenario,
        } = from;

        let v7::PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        } = plant_profile;

        let v7::EnergyConsumption {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
        } = energy_consumption;

        let energy_consumption = v8::EnergyConsumption {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil: None,
        };

        let v7::AnnualAverage {
            nitrogen: total_nitrogen,
            chemical_oxygen_demand,
            phosphorus: _,
        } = influent_average;

        let influent_average = v8::AnnualAverageInfluent {
            total_nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates: None,
        };

        let v7::AnnualAverage {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus: _,
        } = effluent_average;

        let effluent_average = v8::AnnualAverageEffluent {
            total_nitrogen: nitrogen,
            chemical_oxygen_demand,
        };

        let v7::SewageSludgeTreatment {
            sludge_bags_are_open,
            custom_sludge_bags_factor,
            sludge_storage_containers_are_open,
            custom_sludge_storage_containers_factor,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        } = sewage_sludge_treatment;

        let sludge_bags_are_closed = sludge_bags_are_open.map(|x| !x);
        let sludge_storage_containers_are_closed = sludge_storage_containers_are_open.map(|x| !x);

        let sewage_sludge_treatment = v8::SewageSludgeTreatment {
            digester_count,
            sludge_bags_are_closed,
            sludge_storage_containers_are_closed,
            sewage_sludge_for_disposal,
            transport_distance,
        };

        let side_stream_treatment = v8::SideStreamTreatment {
            total_nitrogen: None,
        };

        let plant_profile = v8::PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            side_stream_treatment,
            operating_materials,
        };

        let v7::OptimizationScenario {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        } = optimization_scenario;

        let v7::N2oEmissionFactorScenario {
            calculation_method,
            custom_factor: custom_emission_factor,
        } = n2o_emission_factor;
        let calculation_method = Some(calculation_method);
        let n2o_emissions = v8::N2OEmissionsSensitivity {
            calculation_method,
            custom_emission_factor,
            side_stream_emission_factor: None,
        };

        let (calculation_method, custom_emission_factor) = match ch4_chp_emission_factor {
            Some(v7::CH4ChpEmissionFactorScenario {
                calculation_method,
                custom_factor,
            }) => (Some(calculation_method), custom_factor),
            None => (None, None),
        };
        let ch4_chp_emissions = v8::CH4ChpEmissionsSensitivity {
            calculation_method,
            custom_emission_factor,
        };

        let ch4_sewage_sludge_emissions = v8::SewageSludgeTreatmentEmissionsSensitivity {
            emission_factor_sludge_bags: custom_sludge_bags_factor,
            emission_factor_sludge_storage_containers: custom_sludge_storage_containers_factor,
        };
        let co2_fossil_emissions = v8::FossilEmissonsSensitivity {
            emission_factor: None,
        };

        let sensitivity_parameters = v8::SensitivityParameters {
            n2o_emissions,
            ch4_chp_emissions,
            ch4_sewage_sludge_emissions,
            co2_fossil_emissions,
        };
        let sewage_sludge_treatment = v8::SewageSludgeTreatmentScenario {
            sludge_bags_are_closed: None,
            sludge_storage_containers_are_closed: None,
        };

        let energy_emissions = v8::EnergyEmissionScenario {
            process_energy_savings: None,
            fossil_energy_savings: None,
            photovoltaic_energy_expansion: None,
            estimated_self_photovoltaic_usage: None,
            wind_energy_expansion: None,
            estimated_self_wind_energy_usage: None,
            water_energy_expansion: None,
            estimated_self_water_energy_usage: None,
            district_heating: None,
        };

        let side_stream_treatment = v8::SideStreamTreatmentScenario {
            side_stream_cover_is_closed: None,
        };

        let optimization_scenario = v8::OptimizationScenario {
            sewage_sludge_treatment,
            energy_emissions,
            side_stream_treatment,
        };

        Self {
            project_title,
            plant_profile,
            sensitivity_parameters,
            optimization_scenario,
        }
    }
}

pub fn from_v7(data: v7::Data) -> v8::Data {
    let v7::Data { project } = data;
    let project = match project {
        v7::Project::Saved(saved_project) => {
            let v7::SavedProject {
                id,
                created_at,
                modified_at,
                data,
            } = saved_project;
            let data = data.into();
            let saved = v8::SavedProject {
                id,
                created_at,
                modified_at,
                data,
            };
            v8::Project::Saved(saved)
        }
        v7::Project::Unsaved(unsaved_project) => v8::Project::Unsaved(unsaved_project.into()),
    };
    v8::Data { project }
}

pub fn from_v8(data: v8::Data) -> v9::Project {
    let v8::Data { project } = data;
    match project {
        v8::Project::Unsaved(form_data) => {
            let form_data = from_v8_form_data(form_data);
            v9::UnsavedProject { form_data }.into()
        }
        v8::Project::Saved(saved_project) => {
            let v8::SavedProject {
                id,
                created_at,
                modified_at,
                data,
            } = saved_project;
            let form_data = from_v8_form_data(data);
            v9::SavedProject {
                id,
                created_at,
                modified_at,
                form_data,
            }
            .into()
        }
    }
}

fn from_v8_form_data(data: v8::JsonFormData) -> v9::JsonFormData {
    let form_data = v8::FormData::from(data)
        .into_iter()
        .map(|(id, value)| {
            let id = v9::InputValueId::from(id);
            let value = id.value_to_json(value).unwrap();
            (id, value)
        })
        .collect::<HashMap<_, _>>();
    v9::JsonFormData::from(form_data)
}
