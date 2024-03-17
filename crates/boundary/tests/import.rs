use klick_boundary::{
    import_from_str, EnergyConsumption, FormData, ImportError, N2oEmissionFactorCalcMethod,
    PlantProfile, Project, SewageSludgeTreatment, CURRENT_VERSION,
};

#[test]
fn check_version() {
    let data = r#"{"version":0}"#;
    let err = import_from_str(data).err().unwrap();
    assert!(matches!(
        err,
        ImportError::Version {
            actual: 0,
            expected: CURRENT_VERSION
        }
    ));
}

#[test]
fn import_v1() {
    let json = include_str!("example_data_v1.json");
    let Project::Unsaved(FormData {
        project_title: _,
        plant_profile,
        sensitivity_parameters,
        optimization_scenario: _,
    }) = import_from_str(json).unwrap()
    else {
        panic!("expected unsaved project");
    };

    let PlantProfile {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
        ..
    } = plant_profile;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
        ..
    } = energy_consumption;

    let SewageSludgeTreatment {
        sludge_bags_are_closed,
        sludge_storage_containers_are_closed,
        sewage_sludge_for_disposal,
        transport_distance,
        digester_count,
    } = sewage_sludge_treatment;

    assert_eq!(plant_name.as_deref(), Some("Example Plant"));
    assert_eq!(population_equivalent, Some(120_000.0));
    assert_eq!(wastewater, Some(5_000_000.0));

    assert_eq!(influent_average.total_nitrogen, Some(122.0));
    assert_eq!(influent_average.chemical_oxygen_demand, Some(333.0));

    assert_eq!(effluent_average.total_nitrogen, Some(11.76));
    assert_eq!(effluent_average.chemical_oxygen_demand, Some(129.0));

    assert_eq!(sewage_gas_produced, Some(1_260_000.0));
    assert_eq!(methane_fraction, Some(62.0));
    assert_eq!(on_site_power_generation, Some(2_250_897.0));
    assert_eq!(emission_factor_electricity_mix, Some(468.0));
    assert_eq!(gas_supply, Some(500.0));
    assert_eq!(purchase_of_biogas, Some(true));
    assert_eq!(total_power_consumption, Some(2_683_259.0));

    assert_eq!(sludge_bags_are_closed, Some(false));
    assert_eq!(
        sensitivity_parameters
            .ch4_sewage_sludge_emissions
            .emission_factor_sludge_bags,
        None
    );
    assert_eq!(sludge_storage_containers_are_closed, Some(false));
    assert_eq!(
        sensitivity_parameters
            .ch4_sewage_sludge_emissions
            .emission_factor_sludge_storage_containers,
        None
    );
    assert_eq!(sewage_sludge_for_disposal, Some(3687.6));
    assert_eq!(transport_distance, Some(47.0));
    assert_eq!(digester_count, None);

    assert_eq!(operating_materials.fecl3, Some(12.345));
    assert_eq!(operating_materials.feclso4, Some(326.0));
    assert_eq!(operating_materials.caoh2, Some(326.26));
    assert_eq!(operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(
        sensitivity_parameters.n2o_emissions.custom_emission_factor,
        Some(1.5)
    );
    assert_eq!(
        sensitivity_parameters.n2o_emissions.calculation_method,
        Some(N2oEmissionFactorCalcMethod::CustomFactor)
    );
    assert_eq!(
        sensitivity_parameters
            .ch4_chp_emissions
            .custom_emission_factor,
        None
    );
}

#[test]
fn import_v2() {
    let json = include_str!("example_data_v2.json");

    let Project::Unsaved(FormData {
        project_title: _,
        plant_profile,
        sensitivity_parameters,
        optimization_scenario: _,
    }) = import_from_str(json).unwrap()
    else {
        panic!("expected unsaved project");
    };

    let PlantProfile {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
        ..
    } = plant_profile;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
        ..
    } = energy_consumption;

    let SewageSludgeTreatment {
        sludge_bags_are_closed,
        sludge_storage_containers_are_closed,
        sewage_sludge_for_disposal,
        transport_distance,
        digester_count,
        ..
    } = sewage_sludge_treatment;

    assert_eq!(plant_name.as_deref(), Some("Example Plant"));
    assert_eq!(population_equivalent, Some(120_000.0));
    assert_eq!(wastewater, Some(5_000_000.0));

    assert_eq!(influent_average.total_nitrogen, Some(122.0));
    assert_eq!(influent_average.chemical_oxygen_demand, Some(333.0));

    assert_eq!(effluent_average.total_nitrogen, Some(11.76));
    assert_eq!(effluent_average.chemical_oxygen_demand, Some(129.0));

    assert_eq!(sewage_gas_produced, Some(1_260_000.0));
    assert_eq!(methane_fraction, Some(62.0));
    assert_eq!(on_site_power_generation, Some(2_250_897.0));
    assert_eq!(emission_factor_electricity_mix, Some(468.0));
    assert_eq!(gas_supply, Some(500.0));
    assert_eq!(purchase_of_biogas, Some(true));
    assert_eq!(total_power_consumption, Some(2_683_259.0));

    assert_eq!(sludge_bags_are_closed, Some(false));
    assert_eq!(
        sensitivity_parameters
            .ch4_sewage_sludge_emissions
            .emission_factor_sludge_bags,
        None
    );
    assert_eq!(sludge_storage_containers_are_closed, Some(false));
    assert_eq!(
        sensitivity_parameters
            .ch4_sewage_sludge_emissions
            .emission_factor_sludge_storage_containers,
        None
    );
    assert_eq!(sewage_sludge_for_disposal, Some(3687.6));
    assert_eq!(transport_distance, Some(47.0));
    assert_eq!(digester_count, None);

    assert_eq!(operating_materials.fecl3, Some(12.345));
    assert_eq!(operating_materials.feclso4, Some(326.0));
    assert_eq!(operating_materials.caoh2, Some(326.26));
    assert_eq!(operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(
        sensitivity_parameters.n2o_emissions.custom_emission_factor,
        Some(1.5)
    );
    assert_eq!(
        sensitivity_parameters.n2o_emissions.calculation_method,
        Some(N2oEmissionFactorCalcMethod::CustomFactor)
    );

    assert_eq!(
        sensitivity_parameters
            .ch4_chp_emissions
            .custom_emission_factor,
        None
    );
}

#[test]
fn import_v3() {
    let json = include_str!("example_data_v3.json");

    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    assert_eq!(
        project
            .sensitivity_parameters
            .n2o_emissions
            .calculation_method,
        Some(N2oEmissionFactorCalcMethod::TuWien2016)
    );

    assert_eq!(
        project
            .sensitivity_parameters
            .ch4_chp_emissions
            .custom_emission_factor,
        None
    );
}

#[test]
fn import_v5() {
    let json = include_str!("example_data_v5.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    assert_eq!(
        project
            .sensitivity_parameters
            .n2o_emissions
            .calculation_method,
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );

    assert_eq!(
        project
            .sensitivity_parameters
            .ch4_chp_emissions
            .custom_emission_factor,
        None
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_bags_are_closed,
        Some(false)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_storage_containers_are_closed,
        Some(false)
    );
}

#[test]
fn import_v6() {
    let json = include_str!("example_data_v6.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    assert_eq!(
        project
            .sensitivity_parameters
            .n2o_emissions
            .calculation_method,
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );

    assert_eq!(
        project
            .sensitivity_parameters
            .ch4_chp_emissions
            .custom_emission_factor,
        None
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_bags_are_closed,
        Some(false)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_storage_containers_are_closed,
        Some(false)
    );
}

#[test]
fn import_v7() {
    let json = include_str!("example_data_v7.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    assert_eq!(
        project
            .sensitivity_parameters
            .n2o_emissions
            .calculation_method,
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );

    assert_eq!(
        project
            .sensitivity_parameters
            .ch4_chp_emissions
            .custom_emission_factor,
        None
    );
    assert_eq!(
        project
            .sensitivity_parameters
            .ch4_sewage_sludge_emissions
            .emission_factor_sludge_bags,
        Some(1.12)
    );

    assert_eq!(
        project
            .sensitivity_parameters
            .ch4_sewage_sludge_emissions
            .emission_factor_sludge_storage_containers,
        Some(1.13)
    );
}

#[test]
fn import_v8() {
    let json = include_str!("example_data_v8.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    assert_eq!(
        project
            .sensitivity_parameters
            .n2o_emissions
            .calculation_method,
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );

    assert_eq!(
        project
            .plant_profile
            .influent_average
            .total_organic_carbohydrates,
        Some(101.99)
    );

    assert_eq!(
        project.plant_profile.side_stream_treatment.total_nitrogen,
        Some(101.4)
    );

    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_bags_are_closed,
        Some(false)
    );

    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_storage_containers_are_closed,
        Some(false)
    );

    assert_eq!(
        project
            .optimization_scenario
            .sewage_sludge_treatment
            .sludge_bags_are_closed,
        Some(true)
    );

    assert_eq!(
        project
            .optimization_scenario
            .sewage_sludge_treatment
            .sludge_storage_containers_are_closed,
        Some(true)
    );
}
