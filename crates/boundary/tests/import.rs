use klick_boundary::{
    import_from_str, EnergyConsumption, ImportError, N2oEmissionFactorCalcMethod, PlantProfile,
    Project, ProjectData, SewageSludgeTreatment, CURRENT_VERSION,
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
    let Project::Unsaved(ProjectData {
        title: _,
        plant_profile,
        optimization_scenario,
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
    } = plant_profile;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
    } = energy_consumption;

    let SewageSludgeTreatment {
        sludge_bags_are_open,
        sludge_bags_are_open_recommendation,
        custom_sludge_bags_factor,
        sludge_storage_containers_are_open,
        sludge_storage_containers_are_open_recommendation,
        custom_sludge_storage_containers_factor,
        sewage_sludge_for_disposal,
        transport_distance,
        digester_count,
    } = sewage_sludge_treatment;

    assert_eq!(plant_name.as_deref(), Some("Example Plant"));
    assert_eq!(population_equivalent, Some(120_000.0));
    assert_eq!(wastewater, Some(5_000_000.0));

    assert_eq!(influent_average.nitrogen, Some(122.0));
    assert_eq!(influent_average.chemical_oxygen_demand, Some(333.0));
    assert_eq!(influent_average.phosphorus, Some(555.0));

    assert_eq!(effluent_average.nitrogen, Some(11.76));
    assert_eq!(effluent_average.chemical_oxygen_demand, Some(129.0));
    assert_eq!(effluent_average.phosphorus, Some(10.0));

    assert_eq!(sewage_gas_produced, Some(1_260_000.0));
    assert_eq!(methane_fraction, Some(62.0));
    assert_eq!(on_site_power_generation, Some(2_250_897.0));
    assert_eq!(emission_factor_electricity_mix, Some(468.0));
    assert_eq!(gas_supply, Some(500.0));
    assert_eq!(purchase_of_biogas, Some(true));
    assert_eq!(total_power_consumption, Some(2_683_259.0));

    assert_eq!(sludge_bags_are_open, Some(true));
    assert_eq!(custom_sludge_bags_factor, None);
    assert_eq!(sludge_storage_containers_are_open, Some(true));
    assert_eq!(custom_sludge_storage_containers_factor, None);
    assert_eq!(sewage_sludge_for_disposal, Some(3687.6));
    assert_eq!(transport_distance, Some(47.0));
    assert_eq!(digester_count, None);

    assert_eq!(operating_materials.fecl3, Some(12.345));
    assert_eq!(operating_materials.feclso4, Some(326.0));
    assert_eq!(operating_materials.caoh2, Some(326.26));
    assert_eq!(operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(
        optimization_scenario.n2o_emission_factor.custom_factor,
        Some(1.5)
    );
    assert_eq!(
        optimization_scenario.n2o_emission_factor.calculation_method,
        N2oEmissionFactorCalcMethod::CustomFactor
    );
    assert_eq!(optimization_scenario.ch4_chp_emission_factor, None);
}

#[test]
fn import_v2() {
    let json = include_str!("example_data_v2.json");

    let Project::Unsaved(ProjectData {
        title: _,
        plant_profile,
        optimization_scenario,
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
    } = plant_profile;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply,
        purchase_of_biogas,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
    } = energy_consumption;

    let SewageSludgeTreatment {
        sludge_bags_are_open,
        custom_sludge_bags_factor,
        sludge_storage_containers_are_open,
        custom_sludge_storage_containers_factor,
        sewage_sludge_for_disposal,
        transport_distance,
        digester_count,
    } = sewage_sludge_treatment;

    assert_eq!(plant_name.as_deref(), Some("Example Plant"));
    assert_eq!(population_equivalent, Some(120_000.0));
    assert_eq!(wastewater, Some(5_000_000.0));

    assert_eq!(influent_average.nitrogen, Some(122.0));
    assert_eq!(influent_average.chemical_oxygen_demand, Some(333.0));
    assert_eq!(influent_average.phosphorus, Some(555.0));

    assert_eq!(effluent_average.nitrogen, Some(11.76));
    assert_eq!(effluent_average.chemical_oxygen_demand, Some(129.0));
    assert_eq!(effluent_average.phosphorus, Some(10.0));

    assert_eq!(sewage_gas_produced, Some(1_260_000.0));
    assert_eq!(methane_fraction, Some(62.0));
    assert_eq!(on_site_power_generation, Some(2_250_897.0));
    assert_eq!(emission_factor_electricity_mix, Some(468.0));
    assert_eq!(gas_supply, Some(500.0));
    assert_eq!(purchase_of_biogas, Some(true));
    assert_eq!(total_power_consumption, Some(2_683_259.0));

    assert_eq!(sludge_bags_are_open, Some(true));
    assert_eq!(custom_sludge_bags_factor, None);
    assert_eq!(sludge_storage_containers_are_open, Some(true));
    assert_eq!(custom_sludge_storage_containers_factor, None);
    assert_eq!(sewage_sludge_for_disposal, Some(3687.6));
    assert_eq!(transport_distance, Some(47.0));
    assert_eq!(digester_count, None);

    assert_eq!(operating_materials.fecl3, Some(12.345));
    assert_eq!(operating_materials.feclso4, Some(326.0));
    assert_eq!(operating_materials.caoh2, Some(326.26));
    assert_eq!(operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(
        optimization_scenario.n2o_emission_factor.custom_factor,
        Some(1.5)
    );
    assert_eq!(
        optimization_scenario.n2o_emission_factor.calculation_method,
        N2oEmissionFactorCalcMethod::CustomFactor
    );

    assert_eq!(optimization_scenario.ch4_chp_emission_factor, None);
}

#[test]
fn import_v3() {
    let json = include_str!("example_data_v3.json");

    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    assert_eq!(
        project
            .optimization_scenario
            .n2o_emission_factor
            .calculation_method,
        N2oEmissionFactorCalcMethod::TuWien2016
    );

    assert_eq!(project.optimization_scenario.ch4_chp_emission_factor, None);
}

#[test]
fn import_v5() {
    let json = include_str!("example_data_v5.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    assert_eq!(
        project
            .optimization_scenario
            .n2o_emission_factor
            .calculation_method,
        N2oEmissionFactorCalcMethod::Ipcc2019
    );

    assert_eq!(project.optimization_scenario.ch4_chp_emission_factor, None);
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_bags_are_open,
        Some(true)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_storage_containers_are_open,
        Some(true)
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
            .optimization_scenario
            .n2o_emission_factor
            .calculation_method,
        N2oEmissionFactorCalcMethod::Ipcc2019
    );

    assert_eq!(project.optimization_scenario.ch4_chp_emission_factor, None);
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_bags_are_open,
        Some(true)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_storage_containers_are_open,
        Some(true)
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
            .optimization_scenario
            .n2o_emission_factor
            .calculation_method,
        N2oEmissionFactorCalcMethod::Ipcc2019
    );

    assert_eq!(project.optimization_scenario.ch4_chp_emission_factor, None);
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .custom_sludge_bags_factor,
        Some(1.12)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .custom_sludge_storage_containers_factor,
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
            .optimization_scenario
            .n2o_emission_factor
            .calculation_method,
        N2oEmissionFactorCalcMethod::Ipcc2019
    );

    assert_eq!(influent_average.total_organic_carbohydrates, Some(101.99));
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .total_nitrogen,
        Some(101.4)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_bags_are_open,
        Some(true)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_bags_are_open_recommendation,
        Some(true)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_storage_containers_are_open,
        Some(true)
    );
    assert_eq!(
        project
            .plant_profile
            .sewage_sludge_treatment
            .sludge_storage_containers_are_open_recommendation,
        Some(true)
    );
}
