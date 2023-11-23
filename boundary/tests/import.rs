use klick_boundary::{import_from_str, ImportError, N2oEmissionFactorCalcMethod, CURRENT_VERSION};

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
    assert_eq!(input.plant_name.as_deref(), Some("Example Plant"));
    assert_eq!(input.population_values, Some(120_000.0));
    assert_eq!(input.waste_water, Some(5_000_000.0));
    assert_eq!(input.inflow_averages.nitrogen, Some(122.0));
    assert_eq!(input.effluent_averages.nitrogen, Some(11.76));
    assert_eq!(input.effluent_averages.chemical_oxygen_demand, Some(129.0));

    assert_eq!(input.operating_materials.fecl3, Some(12.345));
    assert_eq!(input.operating_materials.feclso4, Some(326.0));
    assert_eq!(input.operating_materials.caoh2, Some(326.26));
    assert_eq!(input.operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(szenario.n2o_emission_factor.custom_factor, Some(1.5));
    assert_eq!(
        szenario.n2o_emission_factor.calculation_method,
        N2oEmissionFactorCalcMethod::CustomFactor
    );
}

#[test]
fn import_v2() {
    let json = include_str!("example_data_v2.json");
    let (input, szenario) = import_from_str(json).unwrap();
    assert_eq!(input.plant_name.as_deref(), Some("Example Plant"));
    assert_eq!(input.population_values, Some(120_000.0));
    assert_eq!(input.waste_water, Some(5_000_000.0));
    assert_eq!(input.inflow_averages.nitrogen, Some(122.0));
    assert_eq!(input.effluent_averages.nitrogen, Some(11.76));
    assert_eq!(input.effluent_averages.chemical_oxygen_demand, Some(129.0));

    assert_eq!(input.operating_materials.fecl3, Some(12.345));
    assert_eq!(input.operating_materials.feclso4, Some(326.0));
    assert_eq!(input.operating_materials.caoh2, Some(326.26));
    assert_eq!(input.operating_materials.synthetic_polymers, Some(23.62));

    assert_eq!(szenario.n2o_emission_factor.custom_factor, Some(1.5));
    assert_eq!(
        szenario.n2o_emission_factor.calculation_method,
        N2oEmissionFactorCalcMethod::CustomFactor
    );
}
