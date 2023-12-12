use klick_boundary::{
    import_from_str, EnergyConsumption, ImportError, InputData, N2oEmissionFactorCalcMethod,
    SewageSludgeTreatment, CURRENT_VERSION,
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
    let (input, szenario) = import_from_str(json).unwrap();

    let InputData {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    } = input;

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
        open_sludge_bags,
        open_sludge_storage_containers,
        sewage_sludge_for_disposal,
        transport_distance,
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

    assert_eq!(sewage_gas_produced, Some(1260000.0));
    assert_eq!(methane_fraction, Some(62.0));
    assert_eq!(on_site_power_generation, Some(2250897.0));
    assert_eq!(emission_factor_electricity_mix, Some(468.0));
    assert_eq!(gas_supply, Some(500.0));
    assert_eq!(purchase_of_biogas, Some(true));
    assert_eq!(total_power_consumption, Some(2683259.0));

    assert_eq!(open_sludge_bags, Some(true));
    assert_eq!(open_sludge_storage_containers, Some(true));
    assert_eq!(sewage_sludge_for_disposal, Some(3687.6));
    assert_eq!(transport_distance, Some(47.0));

    assert_eq!(operating_materials.fecl3, Some(12.345));
    assert_eq!(operating_materials.feclso4, Some(326.0));
    assert_eq!(operating_materials.caoh2, Some(326.26));
    assert_eq!(operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(szenario.n2o_emission_factor.custom_factor, Some(1.5));
    assert_eq!(
        szenario.n2o_emission_factor.calculation_method,
        N2oEmissionFactorCalcMethod::CustomFactor
    );

    assert_eq!(szenario.ch4_chp_emission_factor, None);
}

#[test]
fn import_v2() {
    let json = include_str!("example_data_v2.json");
    let (input, szenario) = import_from_str(json).unwrap();

    let InputData {
        plant_name,
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    } = input;

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
        open_sludge_bags,
        open_sludge_storage_containers,
        sewage_sludge_for_disposal,
        transport_distance,
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

    assert_eq!(sewage_gas_produced, Some(1260000.0));
    assert_eq!(methane_fraction, Some(62.0));
    assert_eq!(on_site_power_generation, Some(2250897.0));
    assert_eq!(emission_factor_electricity_mix, Some(468.0));
    assert_eq!(gas_supply, Some(500.0));
    assert_eq!(purchase_of_biogas, Some(true));
    assert_eq!(total_power_consumption, Some(2683259.0));

    assert_eq!(open_sludge_bags, Some(true));
    assert_eq!(open_sludge_storage_containers, Some(true));
    assert_eq!(sewage_sludge_for_disposal, Some(3687.6));
    assert_eq!(transport_distance, Some(47.0));

    assert_eq!(operating_materials.fecl3, Some(12.345));
    assert_eq!(operating_materials.feclso4, Some(326.0));
    assert_eq!(operating_materials.caoh2, Some(326.26));
    assert_eq!(operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(szenario.n2o_emission_factor.custom_factor, Some(1.5));
    assert_eq!(
        szenario.n2o_emission_factor.calculation_method,
        N2oEmissionFactorCalcMethod::CustomFactor
    );

    assert_eq!(szenario.ch4_chp_emission_factor, None);
}
