use std::collections::HashMap;

use klick_value::{
    constants::*,
    specs::{InputValueId as In, OutputValueId as Out},
    units::{Value as V, *},
};

use crate::{output_value::required as out, *};

fn ch4_combined_heat_and_power_plant_computation_helper(
    values: &HashMap<Id, Value>,
    ch4_chp_emission_factor: Ch4ChpEmissionFactorCalcMethod,
) -> Tons {
    let mut values = values.clone();

    values.insert(
        In::SensitivityCH4ChpCalculationMethod.into(),
        V::ch4_chp_emission_factor_calc_method(ch4_chp_emission_factor),
    );

    let (co2_equivalents, _) = calculate(&values, None).unwrap();

    co2_equivalents
        .get(&Out::Ch4CombinedHeatAndPowerPlant.into())
        .cloned()
        .unwrap()
        .as_tons()
        .unwrap()
}

fn example_values() -> HashMap<Id, Value> {
    [
        (In::PopulationEquivalent, V::count(50_000)),
        (In::Wastewater, V::qubicmeters(2_135_250.0)),
        (In::InfluentNitrogen, V::milligrams_per_liter(94.0)),
        (
            In::InfluentChemicalOxygenDemand,
            V::milligrams_per_liter(1_020.0),
        ),
        (
            In::InfluentTotalOrganicCarbohydrates,
            V::milligrams_per_liter(382.5),
        ),
        (In::EffluentNitrogen, V::milligrams_per_liter(15.77)),
        (
            In::EffluentChemicalOxygenDemand,
            V::milligrams_per_liter(47.18),
        ),
        (In::SewageGasProduced, V::qubicmeters(420_000.0)),
        (In::TotalPowerConsumption, V::kilowatthours(1_665_000.0)),
        (In::OnSitePowerGeneration, V::kilowatthours(810_000.0)),
        (
            In::EmissionFactorElectricityMix,
            V::grams_per_kilowatthour(420.0),
        ),
        (In::SludgeTreatmentDisposal, V::tons(3016.5)),
        (In::SludgeTreatmentTransportDistance, V::kilometers(150.0)),
        (In::SludgeTreatmentDigesterCount, V::count(1)),
        (In::OperatingMaterialFeCl3, V::tons(310.5)),
        (In::OperatingMaterialSyntheticPolymers, V::tons(12.0)),
        (In::SensitivityCO2FossilCustomFactor, V::percent(0.0)),
    ]
    .into_iter()
    .map(|(id, value)| (id.into(), value))
    .collect()
}

// a helper to update the tests
// cargo test  -- --nocapture
#[allow(dead_code)]
fn create_test_results_on_changes_co2_equivalents_emission_factors(
    emission_factors: HashMap<Out, Factor>,
) {
    for (id, value) in emission_factors {
        println!(
            "assert_eq!(emission_factors.get(&Out::{id:?}).copied().unwrap(),Factor::new({:?}));",
            f64::from(value)
        );
    }
}

// a helper to update the tests
// cargo test  -- --nocapture
#[allow(dead_code)]
fn create_test_results_on_changes_co2_equivalents(co2_equivalents: &HashMap<Out, Tons>) {
    let values: Vec<_> = co2_equivalents.iter().collect();

    // NOTE:
    // You need to temp. add `#[derive(PartialOrd, Ord)]` to OutputValueId.
    // in order to sort the values by ID:
    //
    // values.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (id, value) in values {
        println!(
            "assert_eq!(out!(Out::{id:?},values).unwrap(),Tons::new({:?}));",
            f64::from(*value)
        );
    }
}

#[test]
fn calculate_with_n2o_emission_factor_method_by_tu_wien_2016() {
    let profile = example_values();

    let mut values = profile.clone();
    values.insert(
        In::SensitivityN2OCalculationMethod.into(),
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::TuWien2016),
    );

    let (values, _) = calculate(&values, None).unwrap();

    assert_eq!(
        out!(Out::N2oCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.0045049999999999995).into()
    );
    assert_eq!(
        out!(Out::Ch4ChpCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.01).into()
    );

    assert_eq!(
        out!(Out::N2oPlant, values).unwrap(),
        Tons::new(387.9079422074999)
    );
    assert_eq!(
        out!(Out::N2oWater, values).unwrap(),
        Tons::new(72.228_354_412_5)
    );
    assert_eq!(out!(Out::N2oSideStream, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::N2oEmissions, values).unwrap(),
        Tons::new(460.1362966199999)
    );
    assert_eq!(out!(Out::Ch4Plant, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Ch4SludgeStorageContainers, values).unwrap(),
        Tons::new(104.628_720_000_000_02)
    );
    assert_eq!(
        out!(Out::Ch4SludgeBags, values).unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(
        out!(Out::Ch4Water, values).unwrap(),
        Tons::new(25.386_755_94)
    );
    assert_eq!(
        out!(Out::Ch4CombinedHeatAndPowerPlant, values).unwrap(),
        Tons::new(52.314_360_000_000_01)
    );
    assert_eq!(
        out!(Out::Ch4Emissions, values).unwrap(),
        Tons::new(318.720_845_940_000_06)
    );
    assert_eq!(out!(Out::FossilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Fecl3, values).unwrap(),
        Tons::new(122.647_500_000_000_01)
    );
    assert_eq!(out!(Out::Feclso4, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::Caoh2, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::SyntheticPolymers, values).unwrap(),
        Tons::new(26.400_000_000_000_002)
    );
    assert_eq!(out!(Out::ElectricityMix, values).unwrap(), Tons::new(359.1));
    assert_eq!(out!(Out::OilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::GasEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::OperatingMaterials, values).unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        out!(Out::SewageSludgeTransport, values).unwrap(),
        Tons::new(23.981_175)
    );
    assert_eq!(
        out!(Out::TotalEmissions, values).unwrap(),
        Tons::new(1310.98581756)
    );
    assert_eq!(
        out!(Out::DirectEmissions, values).unwrap(),
        Tons::new(778.8571425599999)
    );
    assert_eq!(
        out!(Out::ProcessEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::PhotovoltaicExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WindExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WaterExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::DistrictHeatingSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::FossilEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::IndirectEmissions, values).unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        out!(Out::OtherIndirectEmissions, values).unwrap(),
        Tons::new(173.028_675_000_000_02)
    );
    assert_eq!(
        out!(Out::ExcessEnergyCo2Equivalent, values).unwrap(),
        Tons::zero()
    );

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &profile,
            Ch4ChpEmissionFactorCalcMethod::GasolineEngine
        ),
        Tons::new(78.47154)
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &profile,
            Ch4ChpEmissionFactorCalcMethod::JetEngine
        ),
        Tons::new(130.785_900_000_000_03)
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_optimistic() {
    let mut input_values = example_values();

    input_values.insert(
        In::SensitivityN2OCalculationMethod.into(),
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Optimistic),
    );
    assert!(input_values
        .get(&In::SensitivityCH4ChpCalculationMethod.into())
        .is_none());

    let (values, _) = calculate(&input_values, None).unwrap();

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(
        out!(Out::N2oCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.003).into()
    );
    assert_eq!(
        out!(Out::Ch4ChpCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.01).into()
    );

    // create_test_results_on_changes_co2_equivalents(&co2);
    assert_eq!(out!(Out::N2oPlant, values).unwrap(), Tons::new(258.3182745));
    assert_eq!(
        out!(Out::N2oWater, values).unwrap(),
        Tons::new(72.2283544125)
    );
    assert_eq!(out!(Out::N2oSideStream, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::N2oEmissions, values).unwrap(),
        Tons::new(330.54662891249995)
    );
    assert_eq!(out!(Out::Ch4Plant, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Ch4SludgeStorageContainers, values).unwrap(),
        Tons::new(104.62872000000002)
    );
    assert_eq!(
        out!(Out::Ch4SludgeBags, values).unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(out!(Out::Ch4Water, values).unwrap(), Tons::new(25.38675594));
    assert_eq!(
        out!(Out::Ch4CombinedHeatAndPowerPlant, values).unwrap(),
        Tons::new(52.31436000000001)
    );
    assert_eq!(
        out!(Out::Ch4Emissions, values).unwrap(),
        Tons::new(318.72084594000006)
    );
    assert_eq!(out!(Out::FossilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Fecl3, values).unwrap(),
        Tons::new(122.64750000000001)
    );
    assert_eq!(out!(Out::Feclso4, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::Caoh2, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::SyntheticPolymers, values).unwrap(),
        Tons::new(26.400000000000002)
    );
    assert_eq!(out!(Out::ElectricityMix, values).unwrap(), Tons::new(359.1));
    assert_eq!(out!(Out::OilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::GasEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::OperatingMaterials, values).unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        out!(Out::SewageSludgeTransport, values).unwrap(),
        Tons::new(23.981175)
    );
    assert_eq!(
        out!(Out::TotalEmissions, values).unwrap(),
        Tons::new(1181.3961498525)
    );
    assert_eq!(
        out!(Out::DirectEmissions, values).unwrap(),
        Tons::new(649.2674748525001)
    );
    assert_eq!(
        out!(Out::ProcessEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::PhotovoltaicExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WindExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WaterExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::DistrictHeatingSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::FossilEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::IndirectEmissions, values).unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        out!(Out::OtherIndirectEmissions, values).unwrap(),
        Tons::new(173.02867500000002)
    );
    assert_eq!(
        out!(Out::ExcessEnergyCo2Equivalent, values).unwrap(),
        Tons::zero()
    );

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &input_values,
            Ch4ChpEmissionFactorCalcMethod::GasolineEngine
        ),
        Tons::new(78.47154)
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &input_values,
            Ch4ChpEmissionFactorCalcMethod::JetEngine
        ),
        Tons::new(130.785_900_000_000_03)
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_pesimistic() {
    let profile = example_values();

    let mut values = profile.clone();
    values.insert(
        In::SensitivityN2OCalculationMethod.into(),
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Pesimistic),
    );
    let (values, _) = calculate(&values, None).unwrap();

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(
        out!(Out::N2oCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.008).into()
    );
    assert_eq!(
        out!(Out::Ch4ChpCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.01).into()
    );

    // create_test_results_on_changes_co2_equivalents(&values);
    assert_eq!(out!(Out::N2oPlant, values).unwrap(), Tons::new(688.848732));
    assert_eq!(
        out!(Out::N2oWater, values).unwrap(),
        Tons::new(72.2283544125)
    );
    assert_eq!(out!(Out::N2oSideStream, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::N2oEmissions, values).unwrap(),
        Tons::new(761.0770864125001)
    );
    assert_eq!(out!(Out::Ch4Plant, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Ch4SludgeStorageContainers, values).unwrap(),
        Tons::new(104.62872000000002)
    );
    assert_eq!(
        out!(Out::Ch4SludgeBags, values).unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(out!(Out::Ch4Water, values).unwrap(), Tons::new(25.38675594));
    assert_eq!(
        out!(Out::Ch4CombinedHeatAndPowerPlant, values).unwrap(),
        Tons::new(52.31436000000001)
    );
    assert_eq!(
        out!(Out::Ch4Emissions, values).unwrap(),
        Tons::new(318.72084594000006)
    );
    assert_eq!(out!(Out::FossilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Fecl3, values).unwrap(),
        Tons::new(122.64750000000001)
    );
    assert_eq!(out!(Out::Feclso4, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::Caoh2, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::SyntheticPolymers, values).unwrap(),
        Tons::new(26.400000000000002)
    );
    assert_eq!(out!(Out::ElectricityMix, values).unwrap(), Tons::new(359.1));
    assert_eq!(out!(Out::OilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::GasEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::OperatingMaterials, values).unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        out!(Out::SewageSludgeTransport, values).unwrap(),
        Tons::new(23.981175)
    );
    assert_eq!(
        out!(Out::TotalEmissions, values).unwrap(),
        Tons::new(1611.9266073525)
    );
    assert_eq!(
        out!(Out::DirectEmissions, values).unwrap(),
        Tons::new(1079.7979323525)
    );
    assert_eq!(
        out!(Out::ProcessEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::PhotovoltaicExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WindExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WaterExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::DistrictHeatingSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::FossilEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::IndirectEmissions, values).unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        out!(Out::OtherIndirectEmissions, values).unwrap(),
        Tons::new(173.02867500000002)
    );
    assert_eq!(
        out!(Out::ExcessEnergyCo2Equivalent, values).unwrap(),
        Tons::zero()
    );

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &profile,
            Ch4ChpEmissionFactorCalcMethod::GasolineEngine
        ),
        Tons::new(78.47154)
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &profile,
            Ch4ChpEmissionFactorCalcMethod::JetEngine
        ),
        Tons::new(130.785_900_000_000_03)
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_ipcc2019() {
    let profile = example_values();

    let mut values = profile.clone();
    values.insert(
        In::SensitivityN2OCalculationMethod.into(),
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Ipcc2019),
    );
    let (values, _) = calculate(&values, None).unwrap();

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(
        out!(Out::N2oCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.016).into()
    );
    assert_eq!(
        out!(Out::Ch4ChpCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.01).into()
    );

    // To genereate the tests:
    // - uncomment the next line:
    //   create_test_results_on_changes_co2_equivalents(&co2);
    // - and run
    //   cargo test --tests calculation::tests::calculate_with_n2o_emission_factor_method_ipcc2019 -- --nocapture
    assert_eq!(out!(Out::N2oPlant, values).unwrap(), Tons::new(1377.697464));
    assert_eq!(
        out!(Out::N2oWater, values).unwrap(),
        Tons::new(72.2283544125)
    );
    assert_eq!(out!(Out::N2oSideStream, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::N2oEmissions, values).unwrap(),
        Tons::new(1449.9258184125001)
    );
    assert_eq!(out!(Out::Ch4Plant, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Ch4SludgeStorageContainers, values).unwrap(),
        Tons::new(104.62872000000002)
    );
    assert_eq!(
        out!(Out::Ch4SludgeBags, values).unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(out!(Out::Ch4Water, values).unwrap(), Tons::new(25.38675594));
    assert_eq!(
        out!(Out::Ch4CombinedHeatAndPowerPlant, values).unwrap(),
        Tons::new(52.31436000000001)
    );
    assert_eq!(
        out!(Out::Ch4Emissions, values).unwrap(),
        Tons::new(318.72084594000006)
    );
    assert_eq!(out!(Out::FossilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Fecl3, values).unwrap(),
        Tons::new(122.64750000000001)
    );
    assert_eq!(out!(Out::Feclso4, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::Caoh2, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::SyntheticPolymers, values).unwrap(),
        Tons::new(26.400000000000002)
    );
    assert_eq!(out!(Out::ElectricityMix, values).unwrap(), Tons::new(359.1));
    assert_eq!(out!(Out::OilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::GasEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::OperatingMaterials, values).unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        out!(Out::SewageSludgeTransport, values).unwrap(),
        Tons::new(23.981175)
    );
    assert_eq!(
        out!(Out::TotalEmissions, values).unwrap(),
        Tons::new(2300.7753393525004)
    );
    assert_eq!(
        out!(Out::DirectEmissions, values).unwrap(),
        Tons::new(1768.6466643525002)
    );
    assert_eq!(
        out!(Out::ProcessEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::PhotovoltaicExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WindExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WaterExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::DistrictHeatingSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::FossilEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::IndirectEmissions, values).unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        out!(Out::OtherIndirectEmissions, values).unwrap(),
        Tons::new(173.02867500000002)
    );
    assert_eq!(
        out!(Out::ExcessEnergyCo2Equivalent, values).unwrap(),
        Tons::zero()
    );

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &profile,
            Ch4ChpEmissionFactorCalcMethod::GasolineEngine
        ),
        Tons::new(78.47154)
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            &profile,
            Ch4ChpEmissionFactorCalcMethod::JetEngine
        ),
        Tons::new(130.785_900_000_000_03)
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_custom_factor() {
    let profile = example_values();

    let mut values = profile.clone();
    values.insert(
        In::SensitivityN2OCalculationMethod.into(),
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Custom),
    );
    values.insert(In::SensitivityN2OCustomFactor.into(), V::percent(1.0));
    values.remove(&In::SensitivityCH4ChpCalculationMethod.into());

    let (values, _) = calculate(&values, None).unwrap();

    assert_eq!(
        out!(Out::N2oCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.01).into()
    );
    assert_eq!(
        out!(Out::Ch4ChpCalculatedEmissionFactor, values).unwrap(),
        Factor::new(0.01).into()
    );

    assert_eq!(out!(Out::N2oPlant, values).unwrap(), Tons::new(861.060_915));
    assert_eq!(
        out!(Out::N2oWater, values).unwrap(),
        Tons::new(72.228_354_412_5)
    );
    assert_eq!(out!(Out::N2oSideStream, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::N2oEmissions, values).unwrap(),
        Tons::new(933.289_269_412_500_1)
    );
    assert_eq!(out!(Out::Ch4Plant, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Ch4SludgeStorageContainers, values).unwrap(),
        Tons::new(104.628_720_000_000_02)
    );
    assert_eq!(
        out!(Out::Ch4SludgeBags, values).unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(
        out!(Out::Ch4Water, values).unwrap(),
        Tons::new(25.386_755_94)
    );
    assert_eq!(
        out!(Out::Ch4CombinedHeatAndPowerPlant, values).unwrap(),
        Tons::new(52.314_360_000_000_01)
    );
    assert_eq!(
        out!(Out::Ch4Emissions, values).unwrap(),
        Tons::new(318.720_845_940_000_06)
    );
    assert_eq!(out!(Out::FossilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::Fecl3, values).unwrap(),
        Tons::new(122.647_500_000_000_01)
    );
    assert_eq!(out!(Out::Feclso4, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::Caoh2, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::SyntheticPolymers, values).unwrap(),
        Tons::new(26.400_000_000_000_002)
    );
    assert_eq!(out!(Out::ElectricityMix, values).unwrap(), Tons::new(359.1));
    assert_eq!(out!(Out::OilEmissions, values).unwrap(), Tons::zero());
    assert_eq!(out!(Out::GasEmissions, values).unwrap(), Tons::zero());
    assert_eq!(
        out!(Out::OperatingMaterials, values).unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        out!(Out::SewageSludgeTransport, values).unwrap(),
        Tons::new(23.981_175)
    );
    assert_eq!(
        out!(Out::TotalEmissions, values).unwrap(),
        Tons::new(1_784.138_790_352_5)
    );
    assert_eq!(
        out!(Out::DirectEmissions, values).unwrap(),
        Tons::new(1_252.010_115_352_500_2)
    );
    assert_eq!(
        out!(Out::ProcessEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::PhotovoltaicExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WindExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::WaterExpansionSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::DistrictHeatingSavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::FossilEnergySavings, values).unwrap(),
        Tons::zero()
    );
    assert_eq!(
        out!(Out::IndirectEmissions, values).unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        out!(Out::OtherIndirectEmissions, values).unwrap(),
        Tons::new(173.028_675_000_000_02)
    );
    assert_eq!(
        out!(Out::ExcessEnergyCo2Equivalent, values).unwrap(),
        Tons::zero()
    );
}

#[test]
fn calculate_ch4_slippage_sludge_bags_for_one_digester() {
    let expected = Tons::new(4.871_107_5);
    let digester_count = Count::new(1);
    let methane_fraction = Percent::new(62.0);
    let sludge_bags_factor = Some(QubicmetersPerHour::new(1.25));
    let result =
        calculate_ch4_slippage_sludge_bags(digester_count, methane_fraction, sludge_bags_factor);
    assert_eq!(result, expected);
    assert_eq!(result * GWP_CH4, Tons::new(136.391_01));
}

#[test]
fn calculate_n2o_side_streams() {
    assert_eq!(
        calculate_n2o_side_stream(Tons::new(10.0), Factor::new(0.02), true),
        Tons::new(85.8)
    );
    assert_eq!(
        calculate_n2o_side_stream(Tons::new(60.0), Factor::new(0.02), true),
        Tons::new(514.8)
    );
    assert_eq!(
        calculate_n2o_side_stream(Tons::new(60.0), Factor::new(0.02), false),
        Tons::zero()
    );
}

#[test]
fn calculate_ch4_plant_test() {
    assert_eq!(calculate_ch4_plant(Count::new(50_000)), Tons::new(322.0));
    assert_eq!(calculate_ch4_plant(Count::zero()), Tons::zero());
}

#[test]
fn test_calculate_fossil_emissions() {
    assert_eq!(
        calculate_fossil_emissions(
            MilligramsPerLiter::new(300.0),
            MilligramsPerLiter::zero(),
            Factor::new(0.0385),
            Qubicmeters::new(2_135_250.0)
        ),
        Tons::new(90.427_837_500_000_01)
    );
    assert_eq!(
        calculate_fossil_emissions(
            MilligramsPerLiter::new(0.00001),
            MilligramsPerLiter::new(1020.0),
            Factor::new(0.0385),
            Qubicmeters::new(2_135_250.0)
        ),
        Tons::new(115.295_492_812_499_99)
    );
}

#[test]
fn calculate_oil_emissions_test() {
    // Heizöl 15.000 L/a * 2,6763kg CO2-Äq./L * 10-3 = 40,15 t CO2-Äq./a (Einfamilienhaus 3000 L/a)
    assert_eq!(
        calculate_oil_emissions(Liters::new(15000.0)),
        Tons::new(40.1445)
    );
}

#[test]
fn calculate_gas_emissions_test() {
    // Erdgas 10.000 m3/a * 2,0kg CO2-Äq./m3 * 10-3 = 20 t CO2-Äq./a
    assert_eq!(
        calculate_gas_emissions(Qubicmeters::new(10000.0), false),
        Tons::new(20.4)
    );
    // Biogas = 10.000 m3/a * 165,48 g CO2-Äq./m3 * 10-6 = 1,65 t CO2-Äq./a
    assert_eq!(
        calculate_gas_emissions(Qubicmeters::new(10000.0), true),
        Tons::new(1.654_815)
    );
}

#[test]
fn calculate_process_energy_savings_test() {
    assert_eq!(
        calculate_process_energy_savings(
            Kilowatthours::new(1_665_000.0),
            Percent::new(20.0),
            GramsPerKilowatthour::new(468.0)
        ),
        Tons::new(155.844)
    );
}

#[test]
fn calculate_photovoltaic_expansion_savings_test() {
    assert_eq!(
        calculate_photovoltaic_expansion_savings(Kilowatthours::new(5000.0), Percent::new(70.0)),
        Tons::new(1.638)
    );
}

#[test]
fn calculate_wind_expansion_savings_test() {
    assert_eq!(
        calculate_wind_expansion_savings(Kilowatthours::new(8500.0), Percent::new(30.0)),
        Tons::new(1.1934)
    );
}

#[test]
fn calculate_water_expansion_savings_test() {
    assert_eq!(
        calculate_water_expansion_savings(Kilowatthours::new(10000.0), Percent::new(20.0)),
        Tons::new(0.936)
    );
}

#[test]
fn calculate_oil_gas_savings_test() {
    assert_eq!(
        calculate_oil_gas_savings(Tons::new(40.15), Tons::new(20.0), Percent::new(20.0)),
        Tons::new(12.03)
    );
}

#[test]
fn extrapolate_according_to_tu_wien_2016_small() {
    assert_eq!(
        // nitrogen_influent: MilligramsPerLiter, nitrogen_effluent: MilligramsPerLiter,
        extrapolate_according_to_tu_wien_2016(
            MilligramsPerLiter::new(1000.0),
            MilligramsPerLiter::new(1.0)
        ),
        Factor::new(0.0)
    );
}

#[test]
fn extrapolate_according_to_tu_wien_2016_large() {
    assert_eq!(
        // nitrogen_influent: MilligramsPerLiter, nitrogen_effluent: MilligramsPerLiter,
        extrapolate_according_to_tu_wien_2016(
            MilligramsPerLiter::new(1.0),
            MilligramsPerLiter::new(1000.0)
        ),
        Factor::new(46.99662)
    );
}
