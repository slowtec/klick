use std::collections::HashMap;

use klick_value::{
    constants::*,
    specs::{InputValueId as Id, OutputValueId as Out},
    units::{Value as V, *},
};

use crate::*;

fn ch4_combined_heat_and_power_plant_computation_helper(
    values: &HashMap<Id, Value>,
    ch4_chp_emission_factor: Ch4ChpEmissionFactorCalcMethod,
) -> Tons {
    let mut values = values.clone();

    values.insert(
        Id::SensitivityCH4ChpCalculationMethod,
        V::ch4_chp_emission_factor_calc_method(ch4_chp_emission_factor),
    );

    let EmissionsCalculationOutcome {
        co2_equivalents, ..
    } = calculate_emissions(&values).unwrap();

    co2_equivalents
        .get(&Out::Ch4CombinedHeatAndPowerPlant)
        .copied()
        .unwrap()
}

fn example_values() -> HashMap<Id, Value> {
    [
        (Id::PopulationEquivalent, V::count(50_000)),
        (Id::Wastewater, V::qubicmeters(2_135_250.0)),
        (Id::InfluentNitrogen, V::milligrams_per_liter(94.0)),
        (
            Id::InfluentChemicalOxygenDemand,
            V::milligrams_per_liter(1_020.0),
        ),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            V::milligrams_per_liter(382.5),
        ),
        (Id::EffluentNitrogen, V::milligrams_per_liter(15.77)),
        (
            Id::EffluentChemicalOxygenDemand,
            V::milligrams_per_liter(47.18),
        ),
        (Id::SewageGasProduced, V::qubicmeters(420_000.0)),
        (Id::TotalPowerConsumption, V::kilowatthours(1_665_000.0)),
        (Id::OnSitePowerGeneration, V::kilowatthours(810_000.0)),
        (
            Id::EmissionFactorElectricityMix,
            V::grams_per_kilowatthour(420.0),
        ),
        (Id::SludgeTreatmentDisposal, V::tons(3016.5)),
        (Id::SludgeTreatmentTransportDistance, V::kilometers(150.0)),
        (Id::SludgeTreatmentDigesterCount, V::count(1)),
        (Id::OperatingMaterialFeCl3, V::tons(310.5)),
        (Id::OperatingMaterialSyntheticPolymers, V::tons(12.0)),
        (Id::SensitivityCO2FossilCustomFactor, V::percent(0.0)),
    ]
    .into_iter()
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
            "assert_eq!(co2.get(&Out::{id:?}).copied().unwrap(),Tons::new({:?}));",
            f64::from(*value)
        );
    }
}

#[test]
fn calculate_with_n2o_emission_factor_method_by_tu_wien_2016() {
    let profile = example_values();

    let mut values = profile.clone();
    values.insert(
        Id::SensitivityN2OCalculationMethod,
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::TuWien2016),
    );

    let EmissionsCalculationOutcome {
        co2_equivalents: eq,
        values,
        ..
    } = calculate_emissions(&values).unwrap();

    assert_eq!(
        values
            .get(&Out::N2oCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.0045049999999999995).into()
    );
    assert_eq!(
        values
            .get(&Out::Ch4ChpCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.01).into()
    );

    assert_eq!(
        *eq.get(&Out::N2oPlant).unwrap(),
        Tons::new(387.9079422074999)
    );
    assert_eq!(
        *eq.get(&Out::N2oWater).unwrap(),
        Tons::new(72.228_354_412_5)
    );
    assert_eq!(*eq.get(&Out::N2oSideStream).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::N2oEmissions).unwrap(),
        Tons::new(460.1362966199999)
    );
    assert_eq!(*eq.get(&Out::Ch4Plant).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::Ch4SludgeStorageContainers).unwrap(),
        Tons::new(104.628_720_000_000_02)
    );
    assert_eq!(*eq.get(&Out::Ch4SludgeBags).unwrap(), Tons::new(136.39101));
    assert_eq!(*eq.get(&Out::Ch4Water).unwrap(), Tons::new(25.386_755_94));
    assert_eq!(
        *eq.get(&Out::Ch4CombinedHeatAndPowerPlant).unwrap(),
        Tons::new(52.314_360_000_000_01)
    );
    assert_eq!(
        *eq.get(&Out::Ch4Emissions).unwrap(),
        Tons::new(318.720_845_940_000_06)
    );
    assert_eq!(*eq.get(&Out::FossilEmissions).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::Fecl3).unwrap(),
        Tons::new(122.647_500_000_000_01)
    );
    assert_eq!(*eq.get(&Out::Feclso4).unwrap(), Tons::new(0.0));
    assert_eq!(*eq.get(&Out::Caoh2).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::SyntheticPolymers).unwrap(),
        Tons::new(26.400_000_000_000_002)
    );
    assert_eq!(*eq.get(&Out::ElectricityMix).unwrap(), Tons::new(359.1));
    assert_eq!(*eq.get(&Out::OilEmissions).unwrap(), Tons::new(0.0));
    assert_eq!(*eq.get(&Out::GasEmissions).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::OperatingMaterials).unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        *eq.get(&Out::SewageSludgeTransport).unwrap(),
        Tons::new(23.981_175)
    );
    assert_eq!(
        *eq.get(&Out::TotalEmissions).unwrap(),
        Tons::new(1310.98581756)
    );
    assert_eq!(
        *eq.get(&Out::DirectEmissions).unwrap(),
        Tons::new(778.8571425599999)
    );
    assert_eq!(*eq.get(&Out::ProcessEnergySavings).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::PhotovoltaicExpansionSavings).unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(*eq.get(&Out::WindExpansionSavings).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::WaterExpansionSavings).unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        *eq.get(&Out::DistrictHeatingSavings).unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(*eq.get(&Out::FossilEnergySavings).unwrap(), Tons::new(0.0));
    assert_eq!(*eq.get(&Out::IndirectEmissions).unwrap(), Tons::new(359.1));
    assert_eq!(
        *eq.get(&Out::OtherIndirectEmissions).unwrap(),
        Tons::new(173.028_675_000_000_02)
    );
    assert_eq!(
        *eq.get(&Out::ExcessEnergyCo2Equivalent).unwrap(),
        Tons::new(0.0)
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
        Id::SensitivityN2OCalculationMethod,
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Optimistic),
    );
    assert!(input_values
        .get(&Id::SensitivityCH4ChpCalculationMethod)
        .is_none());
    let EmissionsCalculationOutcome {
        co2_equivalents,
        values,
        ..
    } = calculate_emissions(&input_values).unwrap();

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(
        values
            .get(&Out::N2oCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.003).into()
    );
    assert_eq!(
        values
            .get(&Out::Ch4ChpCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.01).into()
    );

    let co2 = co2_equivalents;

    // create_test_results_on_changes_co2_equivalents(&co2);
    assert_eq!(
        co2.get(&Out::N2oPlant).copied().unwrap(),
        Tons::new(258.3182745)
    );
    assert_eq!(
        co2.get(&Out::N2oWater).copied().unwrap(),
        Tons::new(72.2283544125)
    );
    assert_eq!(
        co2.get(&Out::N2oSideStream).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::N2oEmissions).copied().unwrap(),
        Tons::new(330.54662891249995)
    );
    assert_eq!(co2.get(&Out::Ch4Plant).copied().unwrap(), Tons::new(0.0));
    assert_eq!(
        co2.get(&Out::Ch4SludgeStorageContainers).copied().unwrap(),
        Tons::new(104.62872000000002)
    );
    assert_eq!(
        co2.get(&Out::Ch4SludgeBags).copied().unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(
        co2.get(&Out::Ch4Water).copied().unwrap(),
        Tons::new(25.38675594)
    );
    assert_eq!(
        co2.get(&Out::Ch4CombinedHeatAndPowerPlant)
            .copied()
            .unwrap(),
        Tons::new(52.31436000000001)
    );
    assert_eq!(
        co2.get(&Out::Ch4Emissions).copied().unwrap(),
        Tons::new(318.72084594000006)
    );
    assert_eq!(
        co2.get(&Out::FossilEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::Fecl3).copied().unwrap(),
        Tons::new(122.64750000000001)
    );
    assert_eq!(co2.get(&Out::Feclso4).copied().unwrap(), Tons::new(0.0));
    assert_eq!(co2.get(&Out::Caoh2).copied().unwrap(), Tons::new(0.0));
    assert_eq!(
        co2.get(&Out::SyntheticPolymers).copied().unwrap(),
        Tons::new(26.400000000000002)
    );
    assert_eq!(
        co2.get(&Out::ElectricityMix).copied().unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        co2.get(&Out::OilEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::GasEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::OperatingMaterials).copied().unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        co2.get(&Out::SewageSludgeTransport).copied().unwrap(),
        Tons::new(23.981175)
    );
    assert_eq!(
        co2.get(&Out::TotalEmissions).copied().unwrap(),
        Tons::new(1181.3961498525)
    );
    assert_eq!(
        co2.get(&Out::DirectEmissions).copied().unwrap(),
        Tons::new(649.2674748525001)
    );
    assert_eq!(
        co2.get(&Out::ProcessEnergySavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::PhotovoltaicExpansionSavings)
            .copied()
            .unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::WindExpansionSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::WaterExpansionSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::DistrictHeatingSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::FossilEnergySavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::IndirectEmissions).copied().unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        co2.get(&Out::OtherIndirectEmissions).copied().unwrap(),
        Tons::new(173.02867500000002)
    );
    assert_eq!(
        co2.get(&Out::ExcessEnergyCo2Equivalent).copied().unwrap(),
        Tons::new(0.0)
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
        Id::SensitivityN2OCalculationMethod,
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Pesimistic),
    );
    let EmissionsCalculationOutcome {
        co2_equivalents,
        values,
        ..
    } = calculate_emissions(&values).unwrap();

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(
        values
            .get(&Out::N2oCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.008).into()
    );
    assert_eq!(
        values
            .get(&Out::Ch4ChpCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.01).into()
    );

    let co2 = co2_equivalents;

    create_test_results_on_changes_co2_equivalents(&co2);
    assert_eq!(
        co2.get(&Out::N2oPlant).copied().unwrap(),
        Tons::new(688.848732)
    );
    assert_eq!(
        co2.get(&Out::N2oWater).copied().unwrap(),
        Tons::new(72.2283544125)
    );
    assert_eq!(
        co2.get(&Out::N2oSideStream).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::N2oEmissions).copied().unwrap(),
        Tons::new(761.0770864125001)
    );
    assert_eq!(co2.get(&Out::Ch4Plant).copied().unwrap(), Tons::new(0.0));
    assert_eq!(
        co2.get(&Out::Ch4SludgeStorageContainers).copied().unwrap(),
        Tons::new(104.62872000000002)
    );
    assert_eq!(
        co2.get(&Out::Ch4SludgeBags).copied().unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(
        co2.get(&Out::Ch4Water).copied().unwrap(),
        Tons::new(25.38675594)
    );
    assert_eq!(
        co2.get(&Out::Ch4CombinedHeatAndPowerPlant)
            .copied()
            .unwrap(),
        Tons::new(52.31436000000001)
    );
    assert_eq!(
        co2.get(&Out::Ch4Emissions).copied().unwrap(),
        Tons::new(318.72084594000006)
    );
    assert_eq!(
        co2.get(&Out::FossilEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::Fecl3).copied().unwrap(),
        Tons::new(122.64750000000001)
    );
    assert_eq!(co2.get(&Out::Feclso4).copied().unwrap(), Tons::new(0.0));
    assert_eq!(co2.get(&Out::Caoh2).copied().unwrap(), Tons::new(0.0));
    assert_eq!(
        co2.get(&Out::SyntheticPolymers).copied().unwrap(),
        Tons::new(26.400000000000002)
    );
    assert_eq!(
        co2.get(&Out::ElectricityMix).copied().unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        co2.get(&Out::OilEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::GasEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::OperatingMaterials).copied().unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        co2.get(&Out::SewageSludgeTransport).copied().unwrap(),
        Tons::new(23.981175)
    );
    assert_eq!(
        co2.get(&Out::TotalEmissions).copied().unwrap(),
        Tons::new(1611.9266073525)
    );
    assert_eq!(
        co2.get(&Out::DirectEmissions).copied().unwrap(),
        Tons::new(1079.7979323525)
    );
    assert_eq!(
        co2.get(&Out::ProcessEnergySavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::PhotovoltaicExpansionSavings)
            .copied()
            .unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::WindExpansionSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::WaterExpansionSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::DistrictHeatingSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::FossilEnergySavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::IndirectEmissions).copied().unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        co2.get(&Out::OtherIndirectEmissions).copied().unwrap(),
        Tons::new(173.02867500000002)
    );
    assert_eq!(
        co2.get(&Out::ExcessEnergyCo2Equivalent).copied().unwrap(),
        Tons::new(0.0)
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
        Id::SensitivityN2OCalculationMethod,
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Ipcc2019),
    );
    let EmissionsCalculationOutcome {
        co2_equivalents,
        values,
        ..
    } = calculate_emissions(&values).unwrap();

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(
        values
            .get(&Out::N2oCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.016).into()
    );
    assert_eq!(
        values
            .get(&Out::Ch4ChpCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.01).into()
    );

    let co2 = co2_equivalents;

    // To genereate the tests:
    // - uncomment the next line:
    //   create_test_results_on_changes_co2_equivalents(&co2);
    // - and run
    //   cargo test --tests calculation::tests::calculate_with_n2o_emission_factor_method_ipcc2019 -- --nocapture
    assert_eq!(
        co2.get(&Out::N2oPlant).copied().unwrap(),
        Tons::new(1377.697464)
    );
    assert_eq!(
        co2.get(&Out::N2oWater).copied().unwrap(),
        Tons::new(72.2283544125)
    );
    assert_eq!(
        co2.get(&Out::N2oSideStream).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::N2oEmissions).copied().unwrap(),
        Tons::new(1449.9258184125001)
    );
    assert_eq!(co2.get(&Out::Ch4Plant).copied().unwrap(), Tons::new(0.0));
    assert_eq!(
        co2.get(&Out::Ch4SludgeStorageContainers).copied().unwrap(),
        Tons::new(104.62872000000002)
    );
    assert_eq!(
        co2.get(&Out::Ch4SludgeBags).copied().unwrap(),
        Tons::new(136.39101)
    );
    assert_eq!(
        co2.get(&Out::Ch4Water).copied().unwrap(),
        Tons::new(25.38675594)
    );
    assert_eq!(
        co2.get(&Out::Ch4CombinedHeatAndPowerPlant)
            .copied()
            .unwrap(),
        Tons::new(52.31436000000001)
    );
    assert_eq!(
        co2.get(&Out::Ch4Emissions).copied().unwrap(),
        Tons::new(318.72084594000006)
    );
    assert_eq!(
        co2.get(&Out::FossilEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::Fecl3).copied().unwrap(),
        Tons::new(122.64750000000001)
    );
    assert_eq!(co2.get(&Out::Feclso4).copied().unwrap(), Tons::new(0.0));
    assert_eq!(co2.get(&Out::Caoh2).copied().unwrap(), Tons::new(0.0));
    assert_eq!(
        co2.get(&Out::SyntheticPolymers).copied().unwrap(),
        Tons::new(26.400000000000002)
    );
    assert_eq!(
        co2.get(&Out::ElectricityMix).copied().unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        co2.get(&Out::OilEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::GasEmissions).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::OperatingMaterials).copied().unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        co2.get(&Out::SewageSludgeTransport).copied().unwrap(),
        Tons::new(23.981175)
    );
    assert_eq!(
        co2.get(&Out::TotalEmissions).copied().unwrap(),
        Tons::new(2300.7753393525004)
    );
    assert_eq!(
        co2.get(&Out::DirectEmissions).copied().unwrap(),
        Tons::new(1768.6466643525002)
    );
    assert_eq!(
        co2.get(&Out::ProcessEnergySavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::PhotovoltaicExpansionSavings)
            .copied()
            .unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::WindExpansionSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::WaterExpansionSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::DistrictHeatingSavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::FossilEnergySavings).copied().unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        co2.get(&Out::IndirectEmissions).copied().unwrap(),
        Tons::new(359.1)
    );
    assert_eq!(
        co2.get(&Out::OtherIndirectEmissions).copied().unwrap(),
        Tons::new(173.02867500000002)
    );
    assert_eq!(
        co2.get(&Out::ExcessEnergyCo2Equivalent).copied().unwrap(),
        Tons::new(0.0)
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
        Id::SensitivityN2OCalculationMethod,
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Custom),
    );
    values.insert(Id::SensitivityN2OCustomFactor, V::percent(1.0));
    values.remove(&Id::SensitivityCH4ChpCalculationMethod);

    let EmissionsCalculationOutcome {
        co2_equivalents: eq,
        values,
        ..
    } = calculate_emissions(&values).unwrap();

    assert_eq!(
        values
            .get(&Out::N2oCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.01).into()
    );
    assert_eq!(
        values
            .get(&Out::Ch4ChpCalculatedEmissionFactor)
            .cloned()
            .unwrap(),
        Factor::new(0.01).into()
    );

    assert_eq!(*eq.get(&Out::N2oPlant).unwrap(), Tons::new(861.060_915));
    assert_eq!(
        *eq.get(&Out::N2oWater).unwrap(),
        Tons::new(72.228_354_412_5)
    );
    assert_eq!(*eq.get(&Out::N2oSideStream).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::N2oEmissions).unwrap(),
        Tons::new(933.289_269_412_500_1)
    );
    assert_eq!(*eq.get(&Out::Ch4Plant).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::Ch4SludgeStorageContainers).unwrap(),
        Tons::new(104.628_720_000_000_02)
    );
    assert_eq!(*eq.get(&Out::Ch4SludgeBags).unwrap(), Tons::new(136.39101));
    assert_eq!(*eq.get(&Out::Ch4Water).unwrap(), Tons::new(25.386_755_94));
    assert_eq!(
        *eq.get(&Out::Ch4CombinedHeatAndPowerPlant).unwrap(),
        Tons::new(52.314_360_000_000_01)
    );
    assert_eq!(
        *eq.get(&Out::Ch4Emissions).unwrap(),
        Tons::new(318.720_845_940_000_06)
    );
    assert_eq!(*eq.get(&Out::FossilEmissions).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::Fecl3).unwrap(),
        Tons::new(122.647_500_000_000_01)
    );
    assert_eq!(*eq.get(&Out::Feclso4).unwrap(), Tons::new(0.0));
    assert_eq!(*eq.get(&Out::Caoh2).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::SyntheticPolymers).unwrap(),
        Tons::new(26.400_000_000_000_002)
    );
    assert_eq!(*eq.get(&Out::ElectricityMix).unwrap(), Tons::new(359.1));
    assert_eq!(*eq.get(&Out::OilEmissions).unwrap(), Tons::new(0.0));
    assert_eq!(*eq.get(&Out::GasEmissions).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::OperatingMaterials).unwrap(),
        Tons::new(149.0475)
    );
    assert_eq!(
        *eq.get(&Out::SewageSludgeTransport).unwrap(),
        Tons::new(23.981_175)
    );
    assert_eq!(
        *eq.get(&Out::TotalEmissions).unwrap(),
        Tons::new(1_784.138_790_352_5)
    );
    assert_eq!(
        *eq.get(&Out::DirectEmissions).unwrap(),
        Tons::new(1_252.010_115_352_500_2)
    );
    assert_eq!(*eq.get(&Out::ProcessEnergySavings).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::PhotovoltaicExpansionSavings).unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(*eq.get(&Out::WindExpansionSavings).unwrap(), Tons::new(0.0));
    assert_eq!(
        *eq.get(&Out::WaterExpansionSavings).unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(
        *eq.get(&Out::DistrictHeatingSavings).unwrap(),
        Tons::new(0.0)
    );
    assert_eq!(*eq.get(&Out::FossilEnergySavings).unwrap(), Tons::new(0.0));
    assert_eq!(*eq.get(&Out::IndirectEmissions).unwrap(), Tons::new(359.1));
    assert_eq!(
        *eq.get(&Out::OtherIndirectEmissions).unwrap(),
        Tons::new(173.028_675_000_000_02)
    );
    assert_eq!(
        *eq.get(&Out::ExcessEnergyCo2Equivalent).unwrap(),
        Tons::new(0.0)
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
        Tons::new(0.0)
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
