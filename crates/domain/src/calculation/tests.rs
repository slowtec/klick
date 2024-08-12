use std::collections::HashMap;

use crate::{constants::*, units::*, InputValueId as Id, OutputValueId as Out, Value as V, *};

fn ch4_combined_heat_and_power_plant_computation_helper(
    scenario: EmissionFactorCalculationMethods,
    profile: &HashMap<Id, Value>,
    ch4_chp_emission_factor: Option<Ch4ChpEmissionFactorCalcMethod>,
) -> f64 {
    let mut s2 = scenario;
    s2.ch4 = ch4_chp_emission_factor;
    let EmissionsCalculationOutcome {
        co2_equivalents, ..
    } = calculate_emissions(&profile, s2).unwrap();

    f64::from(
        co2_equivalents
            .get(&Out::Ch4CombinedHeatAndPowerPlant)
            .copied()
            .unwrap(),
    )
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
    emission_factors: CalculatedEmissionFactors,
) {
    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;
    println!("assert_eq!(f64::from(n2o), {:?});", f64::from(n2o));
    println!("assert_eq!(f64::from(ch4), {:?});", f64::from(ch4));
}

// a helper to update the tests
// cargo test  -- --nocapture
#[allow(dead_code)]
fn create_test_results_on_changes_co2_equivalents(co2_equivalents: CO2Equivalents) {
    let CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_emissions,
        ch4_plant,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        oil_emissions,
        gas_emissions,
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        process_energy_savings,
        photovoltaic_expansion_savings,
        wind_expansion_savings,
        water_expansion_savings,
        district_heating_savings,
        fossil_energy_savings,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
        n2o_side_stream,
        fossil_emissions,
    } = co2_equivalents;

    println!(
        "assert_eq!(f64::from(n2o_plant), {:?});",
        f64::from(n2o_plant)
    );
    println!(
        "assert_eq!(f64::from(n2o_water), {:?});",
        f64::from(n2o_water)
    );
    println!(
        "assert_eq!(f64::from(n2o_emissions), {:?});",
        f64::from(n2o_emissions)
    );
    println!(
        "assert_eq!(f64::from(ch4_plant), {:?});",
        f64::from(ch4_plant)
    );
    println!(
        "assert_eq!(f64::from(ch4_sludge_storage_containers), {:?});",
        f64::from(ch4_sludge_storage_containers)
    );
    println!(
        "assert_eq!(f64::from(ch4_sludge_bags), {:?});",
        f64::from(ch4_sludge_bags)
    );
    println!(
        "assert_eq!(f64::from(ch4_water), {:?});",
        f64::from(ch4_water)
    );
    println!(
        "assert_eq!(f64::from(ch4_combined_heat_and_power_plant), {:?});",
        f64::from(ch4_combined_heat_and_power_plant)
    );
    println!(
        "assert_eq!(f64::from(ch4_emissions), {:?});",
        f64::from(ch4_emissions)
    );
    println!("assert_eq!(f64::from(fecl3), {:?});", f64::from(fecl3));
    println!("assert_eq!(f64::from(feclso4), {:?});", f64::from(feclso4));
    println!("assert_eq!(f64::from(caoh2), {:?});", f64::from(caoh2));
    println!(
        "assert_eq!(f64::from(synthetic_polymers), {:?});",
        f64::from(synthetic_polymers)
    );
    println!(
        "assert_eq!(f64::from(electricity_mix), {:?});",
        f64::from(electricity_mix)
    );
    println!(
        "assert_eq!(f64::from(oil_emissions), {:?});",
        f64::from(oil_emissions)
    );
    println!(
        "assert_eq!(f64::from(gas_emissions), {:?});",
        f64::from(gas_emissions)
    );
    println!(
        "assert_eq!(f64::from(operating_materials), {:?});",
        f64::from(operating_materials)
    );
    println!(
        "assert_eq!(f64::from(sewage_sludge_transport), {:?});",
        f64::from(sewage_sludge_transport)
    );
    println!(
        "assert_eq!(f64::from(total_emissions), {:?});",
        f64::from(total_emissions)
    );
    println!(
        "assert_eq!(f64::from(direct_emissions), {:?});",
        f64::from(direct_emissions)
    );
    println!(
        "assert_eq!(f64::from(process_energy_savings), {:?});",
        f64::from(process_energy_savings)
    );
    println!(
        "assert_eq!(f64::from(photovoltaic_expansion_savings), {:?});",
        f64::from(photovoltaic_expansion_savings)
    );
    println!(
        "assert_eq!(f64::from(wind_expansion_savings), {:?});",
        f64::from(wind_expansion_savings)
    );
    println!(
        "assert_eq!(f64::from(water_expansion_savings), {:?});",
        f64::from(water_expansion_savings)
    );
    println!(
        "assert_eq!(f64::from(district_heating_savings), {:?});",
        f64::from(district_heating_savings)
    );
    println!(
        "assert_eq!(f64::from(fossil_energy_savings), {:?});",
        f64::from(fossil_energy_savings)
    );
    println!(
        "assert_eq!(f64::from(indirect_emissions), {:?});",
        f64::from(indirect_emissions)
    );
    println!(
        "assert_eq!(f64::from(other_indirect_emissions), {:?});",
        f64::from(other_indirect_emissions)
    );
    println!(
        "assert_eq!(f64::from(excess_energy_co2_equivalent), {:?});",
        f64::from(excess_energy_co2_equivalent)
    );
    println!(
        "assert_eq!(f64::from(n2o_side_stream), {:?});",
        f64::from(n2o_side_stream)
    );
    println!(
        "assert_eq!(f64::from(fossil_emissions), {:?});",
        f64::from(fossil_emissions)
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_by_tu_wien_2016() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::TuWien2016,
        n2o_custom_factor: None, // Kein benutzerdefinierter Faktor
        ch4: None,
        ch4_custom_factor: None,
    };
    let mut values = profile.clone();
    values.insert(
        Id::SensitivityN2OCalculationMethod,
        V::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::TuWien2016),
    );

    let EmissionsCalculationOutcome {
        co2_equivalents: eq,
        emission_factors,
        ..
    } = calculate_emissions(&values, scenario).unwrap();

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    assert_eq!(n2o, Factor::new(0.0045049999999999995));
    assert_eq!(ch4, Factor::new(0.01));

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
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_optimistic() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Optimistic,
        n2o_custom_factor: None,
        ch4: None,
        ch4_custom_factor: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(&profile, scenario).unwrap();

    let CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_emissions,
        ch4_plant,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        oil_emissions,
        gas_emissions,
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        process_energy_savings,
        photovoltaic_expansion_savings,
        wind_expansion_savings,
        water_expansion_savings,
        district_heating_savings,
        fossil_energy_savings,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
        n2o_side_stream,
        fossil_emissions,
    } = co2_equivalents.try_into().unwrap();

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.003);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 258.318_274_5);
    assert_eq!(f64::from(n2o_water), 72.228_354_412_5);
    assert_eq!(f64::from(n2o_emissions), 330.546_628_912_499_95);
    assert_eq!(f64::from(ch4_plant), 0.0);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        104.628_720_000_000_02
    );
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.386_755_94);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.314_360_000_000_01
    );
    assert_eq!(f64::from(ch4_emissions), 318.720_845_940_000_06);
    assert_eq!(f64::from(fecl3), 122.647_500_000_000_01);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400_000_000_000_002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981_175);
    assert_eq!(f64::from(total_emissions), 1_181.396_149_852_5);
    assert_eq!(f64::from(direct_emissions), 649.267_474_852_500_1);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.028_675_000_000_02);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_pesimistic() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Pesimistic,
        n2o_custom_factor: None,
        ch4: None,
        ch4_custom_factor: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(&profile, scenario).unwrap();

    let CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_emissions,
        ch4_plant,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        oil_emissions,
        gas_emissions,
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        process_energy_savings,
        photovoltaic_expansion_savings,
        wind_expansion_savings,
        water_expansion_savings,
        district_heating_savings,
        fossil_energy_savings,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
        n2o_side_stream,
        fossil_emissions,
    } = co2_equivalents.try_into().unwrap();

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.008);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 688.848_732);
    assert_eq!(f64::from(n2o_water), 72.228_354_412_5);
    assert_eq!(f64::from(n2o_emissions), 761.077_086_412_500_1);
    assert_eq!(f64::from(ch4_plant), 0.0);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        104.628_720_000_000_02
    );
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.386_755_94);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.314_360_000_000_01
    );
    assert_eq!(f64::from(ch4_emissions), 318.720_845_940_000_06);
    assert_eq!(f64::from(fecl3), 122.647_500_000_000_01);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400_000_000_000_002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981_175);
    assert_eq!(f64::from(total_emissions), 1_611.926_607_352_5);
    assert_eq!(f64::from(direct_emissions), 1_079.797_932_352_5);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.028_675_000_000_02);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_ipcc2019() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Ipcc2019,
        n2o_custom_factor: None,
        ch4: None,
        ch4_custom_factor: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(&profile, scenario).unwrap();

    let CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_emissions,
        ch4_plant,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        oil_emissions,
        gas_emissions,
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        process_energy_savings,
        photovoltaic_expansion_savings,
        wind_expansion_savings,
        water_expansion_savings,
        district_heating_savings,
        fossil_energy_savings,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
        n2o_side_stream,
        fossil_emissions,
    } = co2_equivalents.try_into().unwrap();

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.016);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 1_377.697_464);
    assert_eq!(f64::from(n2o_water), 72.228_354_412_5);
    assert_eq!(f64::from(n2o_emissions), 1_449.925_818_412_500_1);
    assert_eq!(f64::from(ch4_plant), 0.0);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        104.628_720_000_000_02
    );
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.386_755_94);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.314_360_000_000_01
    );
    assert_eq!(f64::from(ch4_emissions), 318.720_845_940_000_06);
    assert_eq!(f64::from(fecl3), 122.647_500_000_000_01);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400_000_000_000_002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981_175);
    assert_eq!(f64::from(total_emissions), 2_300.775_339_352_500_4);
    assert_eq!(f64::from(direct_emissions), 1_768.646_664_352_500_2);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.028_675_000_000_02);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            &profile,
            Some(Ch4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_custom_factor() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Custom,
        n2o_custom_factor: Some(Factor::new(1.0 / 100.0)),
        ch4: None,
        ch4_custom_factor: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents: eq,
        emission_factors,
        ..
    } = calculate_emissions(&profile, scenario).unwrap();

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    assert_eq!(n2o, Factor::new(0.01));
    assert_eq!(ch4, Factor::new(0.01));

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
    let digester_count = 1;
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
