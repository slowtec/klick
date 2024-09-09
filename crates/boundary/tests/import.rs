use std::collections::HashMap;

use klick_boundary::{import_from_str, ImportError, CURRENT_VERSION};
use klick_domain::{units::*, InputValueId as In, Value as V};

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
    let project = import_from_str(json).unwrap();
    let data = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    assert_eq!(
        data.get(&In::PlantName)
            .cloned()
            .map(V::as_text_unchecked)
            .as_deref(),
        Some("Example Plant")
    );
    assert_eq!(
        data.get(&In::PopulationEquivalent)
            .cloned()
            .map(V::as_count_unchecked),
        Some(Count::new(120_000))
    );
    assert_eq!(
        data.get(&In::Wastewater),
        Some(&V::qubicmeters(5_000_000.0))
    );

    assert_eq!(
        data.get(&In::InfluentNitrogen),
        Some(&V::milligrams_per_liter(122.0))
    );
    assert_eq!(
        data.get(&In::InfluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(333.0))
    );

    assert_eq!(
        data.get(&In::EffluentNitrogen),
        Some(&V::milligrams_per_liter(11.76))
    );
    assert_eq!(
        data.get(&In::EffluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(129.0))
    );

    assert_eq!(
        data.get(&In::SewageGasProduced),
        Some(&Value::qubicmeters(1_260_000.0))
    );
    assert_eq!(data.get(&In::MethaneFraction), Some(&Value::percent(62.0)));
    assert_eq!(
        data.get(&In::OnSitePowerGeneration),
        Some(&Value::kilowatthours(2_250_897.0))
    );
    assert_eq!(
        data.get(&In::EmissionFactorElectricityMix),
        Some(&Value::grams_per_kilowatthour(468.0))
    );
    assert_eq!(data.get(&In::GasSupply), Some(&Value::qubicmeters(500.0)));
    assert_eq!(data.get(&In::PurchaseOfBiogas), Some(&Value::bool(true)));
    assert_eq!(
        data.get(&In::TotalPowerConsumption),
        Some(&Value::kilowatthours(2_683_259.0))
    );

    assert_eq!(
        data.get(&In::SludgeTreatmentBagsAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&In::SensitivitySludgeBagsCustomFactor), None);
    assert_eq!(
        data.get(&In::SludgeTreatmentStorageContainersAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&In::SensitivitySludgeStorageCustomFactor), None);
    assert_eq!(
        data.get(&In::SludgeTreatmentDisposal),
        Some(&Value::tons(3687.6))
    );
    assert_eq!(
        data.get(&In::SludgeTreatmentTransportDistance),
        Some(&Value::kilometers(47.0))
    );
    assert_eq!(data.get(&In::SludgeTreatmentDigesterCount), None);

    assert_eq!(
        data.get(&In::OperatingMaterialFeCl3),
        Some(&V::tons(12.345))
    );
    assert_eq!(
        data.get(&In::OperatingMaterialFeClSO4),
        Some(&V::tons(326.0))
    );
    assert_eq!(
        data.get(&In::OperatingMaterialCaOH2),
        Some(&V::tons(326.26))
    );
    assert_eq!(
        data.get(&In::OperatingMaterialSyntheticPolymers),
        Some(&V::tons(23.62))
    );
    assert_eq!(
        data.get(&In::SensitivityN2OCustomFactor),
        Some(&Value::percent(1.5))
    );
    assert_eq!(
        data.get(&In::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Custom)
    );
    assert_eq!(data.get(&In::SensitivityCH4ChpCustomFactor), None);
}

#[test]
fn import_v2() {
    let json = include_str!("example_data_v2.json");
    let project = import_from_str(json).unwrap();
    let data = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    assert_eq!(
        data.get(&In::PlantName)
            .cloned()
            .map(V::as_text_unchecked)
            .as_deref(),
        Some("Example Plant")
    );
    assert_eq!(
        data.get(&In::PopulationEquivalent)
            .cloned()
            .map(V::as_count_unchecked),
        Some(Count::new(120_000))
    );
    assert_eq!(
        data.get(&In::Wastewater),
        Some(&V::qubicmeters(5_000_000.0))
    );

    assert_eq!(
        data.get(&In::InfluentNitrogen),
        Some(&V::milligrams_per_liter(122.0))
    );
    assert_eq!(
        data.get(&In::InfluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(333.0))
    );

    assert_eq!(
        data.get(&In::EffluentNitrogen),
        Some(&V::milligrams_per_liter(11.76))
    );
    assert_eq!(
        data.get(&In::EffluentChemicalOxygenDemand),
        Some(&V::milligrams_per_liter(129.0))
    );

    assert_eq!(
        data.get(&In::SewageGasProduced),
        Some(&Value::qubicmeters(1_260_000.0))
    );
    assert_eq!(data.get(&In::MethaneFraction), Some(&Value::percent(62.0)));
    assert_eq!(
        data.get(&In::OnSitePowerGeneration),
        Some(&Value::kilowatthours(2_250_897.0))
    );
    assert_eq!(
        data.get(&In::EmissionFactorElectricityMix),
        Some(&Value::grams_per_kilowatthour(468.0))
    );
    assert_eq!(data.get(&In::GasSupply), Some(&Value::qubicmeters(500.0)));
    assert_eq!(data.get(&In::PurchaseOfBiogas), Some(&Value::bool(true)));
    assert_eq!(
        data.get(&In::TotalPowerConsumption),
        Some(&Value::kilowatthours(2_683_259.0))
    );

    assert_eq!(
        data.get(&In::SludgeTreatmentBagsAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&In::SensitivitySludgeBagsCustomFactor), None);
    assert_eq!(
        data.get(&In::SludgeTreatmentStorageContainersAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(data.get(&In::SensitivitySludgeStorageCustomFactor), None);
    assert_eq!(
        data.get(&In::SludgeTreatmentDisposal),
        Some(&Value::tons(3687.6))
    );
    assert_eq!(
        data.get(&In::SludgeTreatmentTransportDistance),
        Some(&Value::kilometers(47.0))
    );
    assert_eq!(data.get(&In::SludgeTreatmentDigesterCount), None);

    assert_eq!(
        data.get(&In::OperatingMaterialFeCl3),
        Some(&V::tons(12.345))
    );
    assert_eq!(
        data.get(&In::OperatingMaterialFeClSO4),
        Some(&V::tons(326.0))
    );
    assert_eq!(
        data.get(&In::OperatingMaterialCaOH2),
        Some(&V::tons(326.26))
    );
    assert_eq!(
        data.get(&In::OperatingMaterialSyntheticPolymers),
        Some(&V::tons(23.62))
    );

    assert_eq!(
        data.get(&In::SensitivityN2OCustomFactor),
        Some(&Value::percent(1.5))
    );
    assert_eq!(
        data.get(&In::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Custom)
    );
    assert_eq!(data.get(&In::SensitivityCH4ChpCustomFactor), None);
}

#[test]
fn import_v3() {
    let json = include_str!("example_data_v3.json");
    let project = import_from_str(json).unwrap();
    let project = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    assert_eq!(
        project
            .get(&In::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::TuWien2016)
    );
    assert_eq!(project.get(&In::SensitivityCH4ChpCustomFactor), None);
}

#[test]
fn import_v5() {
    let json = include_str!("example_data_v5.json");
    let project = import_from_str(json).unwrap();
    let project = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    assert_eq!(
        project
            .get(&In::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );
    assert_eq!(project.get(&In::SensitivityCH4ChpCustomFactor), None);
    assert_eq!(
        project
            .get(&In::SludgeTreatmentBagsAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
    assert_eq!(
        project
            .get(&In::SludgeTreatmentStorageContainersAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
}

#[test]
fn import_v6() {
    let json = include_str!("example_data_v6.json");
    let project = import_from_str(json).unwrap();
    let project = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    let project = project;

    assert_eq!(
        project
            .get(&In::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );
    assert_eq!(project.get(&In::SensitivityCH4ChpCustomFactor), None);
    assert_eq!(
        project
            .get(&In::SludgeTreatmentBagsAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
    assert_eq!(
        project
            .get(&In::SludgeTreatmentStorageContainersAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
}

#[test]
fn import_v7() {
    let json = include_str!("example_data_v7.json");
    let project = import_from_str(json).unwrap();
    let project = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    assert_eq!(
        project
            .get(&In::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );
    assert_eq!(project.get(&In::SensitivityCH4ChpCustomFactor), None);
    assert_eq!(
        project.get(&In::SensitivitySludgeBagsCustomFactor),
        Some(&QubicmetersPerHour::new(1.12).into())
    );
    assert_eq!(
        project.get(&In::SensitivitySludgeStorageCustomFactor),
        Some(&Percent::new(1.13).into())
    );
}

#[test]
fn import_v8() {
    let json = include_str!("example_data_v8.json");
    let project = import_from_str(json).unwrap();
    let form_data = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    assert_eq!(
        form_data
            .get(&In::SensitivityN2OCalculationMethod)
            .cloned()
            .map(V::as_n2o_emission_factor_calc_method_unchecked),
        Some(N2oEmissionFactorCalcMethod::Ipcc2019)
    );

    assert_eq!(
        form_data
            .get(&In::InfluentTotalOrganicCarbohydrates)
            .cloned()
            .map(V::as_milligrams_per_liter_unchecked)
            .map(f64::from),
        Some(101.99)
    );

    assert_eq!(
        form_data.get(&In::SideStreamTreatmentTotalNitrogen),
        Some(&Value::tons(101.4))
    );

    assert_eq!(
        form_data
            .get(&In::SludgeTreatmentBagsAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );

    assert_eq!(
        form_data
            .get(&In::SludgeTreatmentStorageContainersAreOpen)
            .cloned()
            .map(Value::as_bool_unchecked),
        Some(true)
    );
    assert_eq!(
        form_data.get(&In::SludgeTreatmentBagsAreOpen),
        Some(&Value::bool(true))
    );
    assert_eq!(
        form_data.get(&In::SludgeTreatmentStorageContainersAreOpen),
        Some(&Value::bool(true))
    );
}

#[test]
fn import_v9() {
    let json = include_str!("example_data_v9.json");
    let project = import_from_str(json).unwrap();
    let form_data = HashMap::<In, Value>::try_from(project.form_data().clone()).unwrap();

    assert_eq!(form_data.get(&In::ProjectName), None);

    assert_eq!(
        form_data.get(&In::PlantName).cloned(),
        Some(Value::text("Muster Klärwerk".to_string()))
    );

    assert_eq!(
        form_data.get(&In::PopulationEquivalent).cloned(),
        Some(Value::count(50000))
    );

    assert_eq!(
        form_data.get(&In::Wastewater).cloned(),
        Some(Value::qubicmeters(2_135_250.0))
    );

    assert_eq!(
        form_data.get(&In::InfluentNitrogen).cloned(),
        Some(Value::milligrams_per_liter(94.0))
    );

    assert_eq!(
        form_data.get(&In::InfluentChemicalOxygenDemand).cloned(),
        Some(Value::milligrams_per_liter(1020.0))
    );

    assert_eq!(
        form_data
            .get(&In::InfluentTotalOrganicCarbohydrates)
            .cloned(),
        Some(Value::milligrams_per_liter(101.99))
    );

    assert_eq!(
        form_data.get(&In::EffluentNitrogen).cloned(),
        Some(Value::milligrams_per_liter(15.77))
    );

    assert_eq!(
        form_data.get(&In::EffluentChemicalOxygenDemand).cloned(),
        Some(Value::milligrams_per_liter(47.18))
    );

    assert_eq!(
        form_data.get(&In::SewageGasProduced).cloned(),
        Some(Value::qubicmeters(420_000.0))
    );

    assert_eq!(
        form_data.get(&In::MethaneFraction).cloned(),
        Some(Value::percent(62.0))
    );

    assert_eq!(
        form_data.get(&In::PurchaseOfBiogas).cloned(),
        Some(Value::bool(false))
    );

    assert_eq!(
        form_data.get(&In::TotalPowerConsumption).cloned(),
        Some(Value::kilowatthours(1_665_000.0))
    );

    assert_eq!(
        form_data.get(&In::OnSitePowerGeneration).cloned(),
        Some(Value::kilowatthours(810_000.0))
    );

    assert_eq!(
        form_data.get(&In::EmissionFactorElectricityMix).cloned(),
        Some(Value::grams_per_kilowatthour(420.0))
    );

    assert_eq!(
        form_data.get(&In::HeatingOil).cloned(),
        Some(Value::liters(2132.12))
    );

    assert_eq!(
        form_data
            .get(&In::SideStreamTreatmentTotalNitrogen)
            .cloned(),
        Some(Value::tons(101.4))
    );

    assert_eq!(
        form_data.get(&In::SludgeTreatmentBagsAreOpen).cloned(),
        Some(Value::bool(true))
    );

    assert_eq!(
        form_data
            .get(&In::SludgeTreatmentStorageContainersAreOpen)
            .cloned(),
        Some(Value::bool(true))
    );

    assert_eq!(
        form_data.get(&In::SludgeTreatmentDisposal).cloned(),
        Some(Value::tons(3016.5))
    );

    assert_eq!(
        form_data
            .get(&In::SludgeTreatmentTransportDistance)
            .cloned(),
        Some(Value::kilometers(150.0))
    );

    assert_eq!(
        form_data.get(&In::SludgeTreatmentDigesterCount).cloned(),
        Some(Value::count(3))
    );

    assert_eq!(
        form_data.get(&In::OperatingMaterialFeCl3).cloned(),
        Some(Value::tons(310.5))
    );

    assert_eq!(
        form_data.get(&In::OperatingMaterialFeClSO4).cloned(),
        Some(Value::tons(0.0))
    );

    assert_eq!(
        form_data.get(&In::OperatingMaterialCaOH2).cloned(),
        Some(Value::tons(0.0))
    );

    assert_eq!(
        form_data
            .get(&In::OperatingMaterialSyntheticPolymers)
            .cloned(),
        Some(Value::tons(12.0))
    );

    assert_eq!(
        form_data.get(&In::SensitivityN2OSideStreamFactor).cloned(),
        Some(Value::percent(5.1))
    );

    assert_eq!(
        form_data
            .get(&In::SensitivityCO2FossilCustomFactor)
            .cloned(),
        Some(Value::percent(4.3))
    );

    assert_eq!(
        form_data.get(&In::SensitivityN2OCalculationMethod).cloned(),
        Some(Value::n2o_emission_factor_calc_method(
            N2oEmissionFactorCalcMethod::Ipcc2019
        ))
    );

    assert_eq!(
        form_data
            .get(&In::ScenarioN2OSideStreamCoverIsOpen)
            .cloned(),
        Some(Value::bool(true))
    );

    assert_eq!(
        form_data.get(&In::ScenarioN2OSideStreamFactor).cloned(),
        Some(Value::factor(101.4))
    );
}
