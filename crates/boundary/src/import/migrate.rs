use crate::{v1, v2, v3, v4, v5, v6, v7};

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
            sludge_bags_are_open: sludge_bags_are_open,
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open: sludge_storage_containers_are_open,
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
