use klick_boundary::*;

pub fn example_form_data() -> FormData {
    let plant_profile = PlantProfile {
        plant_name: Some("Muster Klärwerk".to_string()),
        population_equivalent: Some(50_000.0),
        wastewater: Some(2_135_250.0),
        influent_average: AnnualAverageInfluent {
            total_nitrogen: Some(94.0),
            chemical_oxygen_demand: Some(1_020.0),
            total_organic_carbohydrates: Some(382.5),
        },
        effluent_average: AnnualAverageEffluent {
            total_nitrogen: Some(15.77),
            chemical_oxygen_demand: Some(47.18),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Some(420_000.0),
            methane_fraction: Some(62.0),
            purchase_of_biogas: Some(false),
            total_power_consumption: Some(1_665_000.0),
            on_site_power_generation: Some(810_000.0),
            emission_factor_electricity_mix: Some(420.0),
            heating_oil: None,
            gas_supply: None,
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_closed: Some(false),
            sludge_storage_containers_are_closed: Some(false),
            sewage_sludge_for_disposal: Some(3016.5),
            transport_distance: Some(150.0),
            digester_count: Some(1),
        },
        operating_materials: OperatingMaterials {
            fecl3: Some(310.5),
            feclso4: None,
            caoh2: None,
            synthetic_polymers: Some(12.0),
        },
        side_stream_treatment: SideStreamTreatment {
            total_nitrogen: None,
        },
    };
    let sensitivity_parameters = SensitivityParameters {
        n2o_emissions: N2OEmissionsSensitivity {
            calculation_method: Some(N2oEmissionFactorCalcMethod::Ipcc2019),
            custom_emission_factor: None,
            side_stream_emission_factor: None,
        },
        ch4_chp_emissions: CH4ChpEmissionsSensitivity {
            calculation_method: None,
            custom_emission_factor: None,
        },
        ch4_sewage_sludge_emissions: SewageSludgeTreatmentEmissionsSensitivity {
            emission_factor_sludge_bags: None,
            emission_factor_sludge_storage_containers: None,
        },
        co2_fossil_emissions: FossilEmissonsSensitivity {
            emission_factor: None,
        },
    };

    let optimization_scenario = OptimizationScenario {
        sewage_sludge_treatment: SewageSludgeTreatmentScenario::default(),
        energy_emissions: EnergyEmissionScenario::default(),
        side_stream_treatment: SideStreamTreatmentScenario::default(),
    };
    let project_title = None;

    FormData {
        project_title,
        plant_profile,
        sensitivity_parameters,
        optimization_scenario,
    }
}
