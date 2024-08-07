use klick_boundary::{import_from_str, FormData, ImportError, Project, CURRENT_VERSION};
use klick_domain::{units::*, InputValueId as Id, Value as V};

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

    let Project::Unsaved(data) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    let data = FormData::from(data);

    assert_eq!(
        data.get(&Id::PlantName)
            .cloned()
            .map(V::as_text_unchecked)
            .as_deref(),
        Some("Example Plant")
    );
    assert_eq!(
        data.get(&Id::PopulationEquivalent)
            .cloned()
            .map(V::as_count_unchecked),
        Some(Count::new(120_000))
    );
    assert_eq!(
        data.get(&Id::Wastewater),
        Some(&V::qubicmeters(5_000_000.0))
    );

    assert_eq!(
        data.get(&Id::InfluentNitrogen),
        Some(&V::milligrams_per_liter(122.0))
    );
    assert_eq!(
        data.get(&Id::InfluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(333.0))
    );

    assert_eq!(
        data.get(&Id::EffluentNitrogen),
        Some(&V::milligrams_per_liter(11.76))
    );
    assert_eq!(
        data.get(&Id::EffluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(129.0))
    );

    assert_eq!(
        data.get(&Id::SewageGasProduced),
        Some(&Value::qubicmeters(1_260_000.0))
    );
    assert_eq!(data.get(&Id::MethaneFraction), Some(&Value::percent(62.0)));
    assert_eq!(
        data.get(&Id::OnSitePowerGeneration),
        Some(&Value::kilowatthours(2_250_897.0))
    );
    assert_eq!(
        data.get(&Id::EmissionFactorElectricityMix),
        Some(&Value::grams_per_kilowatthour(468.0))
    );
    assert_eq!(data.get(&Id::GasSupply), Some(&Value::qubicmeters(500.0)));
    assert_eq!(data.get(&Id::PurchaseOfBiogas), Some(&Value::bool(true)));
    assert_eq!(
        data.get(&Id::TotalPowerConsumption),
        Some(&Value::kilowatthours(2_683_259.0))
    );

    assert_eq!(
        data.get(&Id::SludgeTreatmentBagsAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&Id::SensitivitySludgeBagsCustomFactor), None);
    assert_eq!(
        data.get(&Id::SludgeTreatmentStorageContainersAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&Id::SensitivitySludgeStorageCustomFactor), None);
    assert_eq!(
        data.get(&Id::SludgeTreatmentDisposal),
        Some(&Value::tons(3687.6))
    );
    assert_eq!(
        data.get(&Id::SludgeTreatmentTransportDistance),
        Some(&Value::kilometers(47.0))
    );
    assert_eq!(data.get(&Id::SludgeTreatmentDigesterCount), None);

    assert_eq!(
        data.get(&Id::OperatingMaterialFeCl3),
        Some(&V::tons(12.345))
    );
    assert_eq!(
        data.get(&Id::OperatingMaterialFeClSO4),
        Some(&V::tons(326.0))
    );
    assert_eq!(
        data.get(&Id::OperatingMaterialCaOH2),
        Some(&V::tons(326.26))
    );
    assert_eq!(
        data.get(&Id::OperatingMaterialSyntheticPolymers),
        Some(&V::tons(23.62))
    );
    assert_eq!(
        data.get(&Id::SensitivityN2OCustomFactor),
        Some(&Value::percent(1.5))
    );
    assert_eq!(
        data.get(&Id::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Custom)
    );
    assert_eq!(data.get(&Id::SensitivityCH4ChpCustomFactor), None);
}

#[test]
fn import_v2() {
    let json = include_str!("example_data_v2.json");

    let Project::Unsaved(data) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    let data = FormData::from(data);

    assert_eq!(
        data.get(&Id::PlantName)
            .cloned()
            .map(V::as_text_unchecked)
            .as_deref(),
        Some("Example Plant")
    );
    assert_eq!(
        data.get(&Id::PopulationEquivalent)
            .cloned()
            .map(V::as_count_unchecked),
        Some(Count::new(120_000))
    );
    assert_eq!(
        data.get(&Id::Wastewater),
        Some(&V::qubicmeters(5_000_000.0))
    );

    assert_eq!(
        data.get(&Id::InfluentNitrogen),
        Some(&V::milligrams_per_liter(122.0))
    );
    assert_eq!(
        data.get(&Id::InfluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(333.0))
    );

    assert_eq!(
        data.get(&Id::EffluentNitrogen),
        Some(&V::milligrams_per_liter(11.76))
    );
    assert_eq!(
        data.get(&Id::EffluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(129.0))
    );

    assert_eq!(
        data.get(&Id::SewageGasProduced),
        Some(&Value::qubicmeters(1_260_000.0))
    );
    assert_eq!(data.get(&Id::MethaneFraction), Some(&Value::percent(62.0)));
    assert_eq!(
        data.get(&Id::OnSitePowerGeneration),
        Some(&Value::kilowatthours(2_250_897.0))
    );
    assert_eq!(
        data.get(&Id::EmissionFactorElectricityMix),
        Some(&Value::grams_per_kilowatthour(468.0))
    );
    assert_eq!(data.get(&Id::GasSupply), Some(&Value::qubicmeters(500.0)));
    assert_eq!(data.get(&Id::PurchaseOfBiogas), Some(&Value::bool(true)));
    assert_eq!(
        data.get(&Id::TotalPowerConsumption),
        Some(&Value::kilowatthours(2_683_259.0))
    );

    assert_eq!(
        data.get(&Id::SludgeTreatmentBagsAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&Id::SensitivitySludgeBagsCustomFactor), None);
    assert_eq!(
        data.get(&Id::SludgeTreatmentStorageContainersAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&Id::SensitivitySludgeStorageCustomFactor), None);
    assert_eq!(
        data.get(&Id::SludgeTreatmentDisposal),
        Some(&Value::tons(3687.6))
    );
    assert_eq!(
        data.get(&Id::SludgeTreatmentTransportDistance),
        Some(&Value::kilometers(47.0))
    );
    assert_eq!(data.get(&Id::SludgeTreatmentDigesterCount), None);

    assert_eq!(
        data.get(&Id::OperatingMaterialFeCl3),
        Some(&V::tons(12.345))
    );
    assert_eq!(
        data.get(&Id::OperatingMaterialFeClSO4),
        Some(&V::tons(326.0))
    );
    assert_eq!(
        data.get(&Id::OperatingMaterialCaOH2),
        Some(&V::tons(326.26))
    );
    assert_eq!(
        data.get(&Id::OperatingMaterialSyntheticPolymers),
        Some(&V::tons(23.62))
    );

    assert_eq!(
        data.get(&Id::SensitivityN2OCustomFactor),
        Some(&Value::percent(1.5))
    );
    assert_eq!(
        data.get(&Id::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Custom)
    );
    assert_eq!(data.get(&Id::SensitivityCH4ChpCustomFactor), None);
}

#[test]
fn import_v3() {
    let json = include_str!("example_data_v3.json");

    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    let project = FormData::from(project);

    assert_eq!(
        project
            .get(&Id::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::TuWien2016)
    );
    assert_eq!(project.get(&Id::SensitivityCH4ChpCustomFactor), None);
}

#[test]
fn import_v5() {
    let json = include_str!("example_data_v5.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    let project = FormData::from(project);

    assert_eq!(
        project
            .get(&Id::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );
    assert_eq!(project.get(&Id::SensitivityCH4ChpCustomFactor), None);
    assert_eq!(
        project
            .get(&Id::SludgeTreatmentBagsAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
    assert_eq!(
        project
            .get(&Id::SludgeTreatmentStorageContainersAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
}

#[test]
fn import_v6() {
    let json = include_str!("example_data_v6.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    let project = FormData::from(project);

    assert_eq!(
        project
            .get(&Id::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );
    assert_eq!(project.get(&Id::SensitivityCH4ChpCustomFactor), None);
    assert_eq!(
        project
            .get(&Id::SludgeTreatmentBagsAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
    assert_eq!(
        project
            .get(&Id::SludgeTreatmentStorageContainersAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
}

#[test]
fn import_v7() {
    let json = include_str!("example_data_v7.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    let project = FormData::from(project);

    assert_eq!(
        project
            .get(&Id::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );
    assert_eq!(project.get(&Id::SensitivityCH4ChpCustomFactor), None);
    assert_eq!(
        project.get(&Id::SensitivitySludgeBagsCustomFactor),
        Some(&QubicmetersPerHour::new(1.12).into())
    );
    assert_eq!(
        project.get(&Id::SensitivitySludgeStorageCustomFactor),
        Some(&Percent::new(1.13).into())
    );
}

#[test]
fn import_v8() {
    let json = include_str!("example_data_v8.json");
    let Project::Unsaved(project) = import_from_str(json).unwrap() else {
        panic!("expected unsaved project");
    };

    let project = FormData::from(project);

    assert_eq!(
        project
            .get(&Id::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );

    assert_eq!(
        project
            .get(&Id::InfluentTotalOrganicCarbohydrates)
            .cloned()
            .map(V::as_milligrams_per_liter_unchecked)
            .map(f64::from),
        Some(101.99)
    );

    assert_eq!(
        project.get(&Id::SideStreamTreatmentTotalNitrogen),
        Some(&Value::tons(101.4))
    );

    assert_eq!(
        project
            .get(&Id::SludgeTreatmentBagsAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );

    assert_eq!(
        project
            .get(&Id::SludgeTreatmentStorageContainersAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
    assert_eq!(
        project.get(&Id::SludgeTreatmentBagsAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(
        project.get(&Id::SludgeTreatmentStorageContainersAreOpen),
        Some(&Value::bool(true))
    );
}
