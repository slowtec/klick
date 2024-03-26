use crate::{constants::*, units::*, *};

fn ch4_combined_heat_and_power_plant_computation_helper(
    scenario: EmissionFactorCalculationMethods,
    profile: EmissionInfluencingValues,
    ch4_chp_emission_factor: Option<CH4ChpEmissionFactorCalcMethod>,
) -> f64 {
    let mut s2 = scenario;
    s2.ch4 = ch4_chp_emission_factor;
    let EmissionsCalculationOutcome {
        co2_equivalents, ..
    } = calculate_emissions(profile, s2);

    f64::from(co2_equivalents.ch4_combined_heat_and_power_plant)
}

fn example_values() -> EmissionInfluencingValues {
    EmissionInfluencingValues {
        population_equivalent: 50_000.0,
        wastewater: Qubicmeters::new(2_135_250.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(94.0),
            chemical_oxygen_demand: MilligramsPerLiter::new(1_020.0),
            total_organic_carbohydrates: MilligramsPerLiter::new(382.5),
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(15.77),
            chemical_oxygen_demand: MilligramsPerLiter::new(47.18),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(420_000.0),
            methane_fraction: Percent::new(62.0),
            total_power_consumption: Kilowatthours::new(1_665_000.0),
            on_site_power_generation: Kilowatthours::new(810_000.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(420.0),
            heating_oil: Liters::new(0.0),
            gas_supply: Qubicmeters::new(0.0),
            purchase_of_biogas: false,
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            sludge_bags_factor: None,
            sludge_storage_containers_are_open: true,
            sludge_storage_containers_factor: None,
            sewage_sludge_for_disposal: Tons::new(3016.5),
            transport_distance: Kilometers::new(150.0),
            digester_count: Some(1),
        },
        operating_materials: OperatingMaterials {
            fecl3: Tons::new(310.5),
            feclso4: Tons::new(0.0),
            caoh2: Tons::new(0.0),
            synthetic_polymers: Tons::new(12.0),
        },
        emission_factors: EmissionFactors {
            co2_fossil: Factor::new(0.0),
            n2o_side_stream: Factor::new(0.0),
        },
        side_stream_treatment: SideStreamTreatment {
            total_nitrogen: Tons::new(0.0),
            side_stream_cover_is_open: true,
        },
        energy_emission_factors: EnergyEmissionFactors {
            process_energy_savings: Percent::new(0.0),
            fossil_energy_savings: Percent::new(0.0),
            district_heating: Kilowatthours::new(0.0),
            photovoltaic_energy_expansion: Kilowatthours::new(0.0),
            estimated_self_photovoltaic_usage: Percent::new(0.0),
            wind_energy_expansion: Kilowatthours::new(0.0),
            estimated_self_wind_energy_usage: Percent::new(0.0),
            water_energy_expansion: Kilowatthours::new(0.0),
            estimated_self_water_energy_usage: Percent::new(0.0),
        },
    }
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
        ch4: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(profile, scenario);

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

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.004_750_531_914_893_612);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 409.04973573749953);
    assert_eq!(f64::from(n2o_water), 72.2283544125);
    assert_eq!(f64::from(n2o_emissions), 481.2780901499995);
    assert_eq!(f64::from(ch4_plant), 293.33409);
    assert_eq!(f64::from(ch4_sludge_storage_containers), 104.62872000000002);
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.38675594);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.31436000000001
    );
    assert_eq!(f64::from(ch4_emissions), 612.0549359400001);
    assert_eq!(f64::from(fecl3), 122.64750000000001);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400000000000002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981175);
    assert_eq!(f64::from(total_emissions), 1625.4617010899997);
    assert_eq!(f64::from(direct_emissions), 1093.3330260899995);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.02867500000002);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_optimistic() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Optimistic,
        ch4: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(profile, scenario);

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

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.003);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 258.3182745);
    assert_eq!(f64::from(n2o_water), 72.2283544125);
    assert_eq!(f64::from(n2o_emissions), 330.54662891249995);
    assert_eq!(f64::from(ch4_plant), 293.33409);
    assert_eq!(f64::from(ch4_sludge_storage_containers), 104.62872000000002);
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.38675594);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.31436000000001
    );
    assert_eq!(f64::from(ch4_emissions), 612.0549359400001);
    assert_eq!(f64::from(fecl3), 122.64750000000001);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400000000000002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981175);
    assert_eq!(f64::from(total_emissions), 1474.7302398525);
    assert_eq!(f64::from(direct_emissions), 942.6015648525);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.02867500000002);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_pesimistic() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Pesimistic,
        ch4: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(profile, scenario);

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

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.008);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 688.848732);
    assert_eq!(f64::from(n2o_water), 72.2283544125);
    assert_eq!(f64::from(n2o_emissions), 761.0770864125001);
    assert_eq!(f64::from(ch4_plant), 293.33409);
    assert_eq!(f64::from(ch4_sludge_storage_containers), 104.62872000000002);
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.38675594);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.31436000000001
    );
    assert_eq!(f64::from(ch4_emissions), 612.0549359400001);
    assert_eq!(f64::from(fecl3), 122.64750000000001);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400000000000002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981175);
    assert_eq!(f64::from(total_emissions), 1905.2606973525003);
    assert_eq!(f64::from(direct_emissions), 1373.1320223525001);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.02867500000002);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_ipcc2019() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Ipcc2019,
        ch4: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(profile, scenario);

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

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.016);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 1377.697464);
    assert_eq!(f64::from(n2o_water), 72.2283544125);
    assert_eq!(f64::from(n2o_emissions), 1449.9258184125001);
    assert_eq!(f64::from(ch4_plant), 293.33409);
    assert_eq!(f64::from(ch4_sludge_storage_containers), 104.62872000000002);
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.38675594);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.31436000000001
    );
    assert_eq!(f64::from(ch4_emissions), 612.0549359400001);
    assert_eq!(f64::from(fecl3), 122.64750000000001);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400000000000002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981175);
    assert_eq!(f64::from(total_emissions), 2594.1094293525002);
    assert_eq!(f64::from(direct_emissions), 2061.9807543525003);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.02867500000002);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_custom_factor() {
    let profile = example_values();

    let scenario = EmissionFactorCalculationMethods {
        n2o: N2oEmissionFactorCalcMethod::Custom(Factor::new(1.0 / 100.0)),
        ch4: None,
    };

    let EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        ..
    } = calculate_emissions(profile, scenario);

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

    let CalculatedEmissionFactors { n2o, ch4 } = emission_factors;

    // create_test_results_on_changes_co2_equivalents_emission_factors(emission_factors);
    assert_eq!(f64::from(n2o), 0.01);
    assert_eq!(f64::from(ch4), 0.01);

    // create_test_results_on_changes_co2_equivalents(co2_equivalents);
    assert_eq!(f64::from(n2o_plant), 861.060915);
    assert_eq!(f64::from(n2o_water), 72.2283544125);
    assert_eq!(f64::from(n2o_emissions), 933.2892694125001);
    assert_eq!(f64::from(ch4_plant), 293.33409);
    assert_eq!(f64::from(ch4_sludge_storage_containers), 104.62872000000002);
    assert_eq!(f64::from(ch4_sludge_bags), 136.39101);
    assert_eq!(f64::from(ch4_water), 25.38675594);
    assert_eq!(
        f64::from(ch4_combined_heat_and_power_plant),
        52.31436000000001
    );
    assert_eq!(f64::from(ch4_emissions), 612.0549359400001);
    assert_eq!(f64::from(fecl3), 122.64750000000001);
    assert_eq!(f64::from(feclso4), 0.0);
    assert_eq!(f64::from(caoh2), 0.0);
    assert_eq!(f64::from(synthetic_polymers), 26.400000000000002);
    assert_eq!(f64::from(electricity_mix), 359.1);
    assert_eq!(f64::from(oil_emissions), 0.0);
    assert_eq!(f64::from(gas_emissions), 0.0);
    assert_eq!(f64::from(operating_materials), 149.0475);
    assert_eq!(f64::from(sewage_sludge_transport), 23.981175);
    assert_eq!(f64::from(total_emissions), 2077.4728803525);
    assert_eq!(f64::from(direct_emissions), 1545.3442053525);
    assert_eq!(f64::from(process_energy_savings), 0.0);
    assert_eq!(f64::from(photovoltaic_expansion_savings), 0.0);
    assert_eq!(f64::from(wind_expansion_savings), 0.0);
    assert_eq!(f64::from(water_expansion_savings), 0.0);
    assert_eq!(f64::from(district_heating_savings), 0.0);
    assert_eq!(f64::from(fossil_energy_savings), 0.0);
    assert_eq!(f64::from(indirect_emissions), 359.1);
    assert_eq!(f64::from(other_indirect_emissions), 173.02867500000002);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(n2o_side_stream), 0.0);
    assert_eq!(f64::from(fossil_emissions), 0.0);

    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        78.47154
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile,
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        130.785_900_000_000_03
    );
}

#[test]
fn calculate_ch4_slippage_sludge_bags_for_one_digester() {
    let expected = Tons::new(4.871_107_5);
    let digester_count = Some(1);
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
    assert_eq!(
        calculate_ch4_plant(50000.0, Tons::new(137.0), Tons::new(105.0), Tons::new(79.0)),
        Tons::new(321.0)
    );
    assert_eq!(
        calculate_ch4_plant(50000.0, Tons::new(0.0), Tons::new(105.0), Tons::new(79.0)),
        Tons::new(184.0)
    );
    assert_eq!(
        calculate_ch4_plant(50000.0, Tons::new(137.0), Tons::new(0.0), Tons::new(79.0)),
        Tons::new(216.0)
    );
    assert_eq!(
        calculate_ch4_plant(50000.0, Tons::new(0.0), Tons::new(0.0), Tons::new(79.0)),
        Tons::new(322.0)
    );
}

#[test]
fn test_calculate_fossil_emissions() {
    assert_eq!(
        calculate_fossil_emissions(
            MilligramsPerLiter::new(300.0),
            MilligramsPerLiter::new(0.0),
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
        calculate_process_energy_savings(Kilowatthours::new(1_665_000.0), Percent::new(20.0)),
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
