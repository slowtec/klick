use crate::{v1, v2, v3};

const V1_OPERATING_MATERIALS_DIVISOR: f64 = 1_000.0;

pub fn from_v1(data: v1::Import) -> v3::Import {
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

    from_v2(v2::Import { input, scenario })
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
