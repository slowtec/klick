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

#[test]
fn calculate_with_n2o_emission_factor_method_by_tu_wien_2016() {
    let profile = EmissionInfluencingValues {
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open: true,
            custom_sludge_storage_containers_factor: None,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
            digester_count: None,
        },
        operating_materials: OperatingMaterials {
            fecl3: Tons::new(0.0),
            feclso4: Tons::new(326.0),
            caoh2: Tons::new(326.26),
            synthetic_polymers: Tons::new(23.62),
        },
    };

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
        ch4_sewage_treatment,
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
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 327.970_500_000_001_83);
    assert_eq!(f64::from(n2o_water), 126.126);
    assert_eq!(f64::from(n2o_emissions), 454.096_500_000_001_8);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        251.108_928_000_000_05
    );
    assert_eq!(ch4_sludge_bags, Tons::zero());
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_343.392_008);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers.round(3)), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 1_797.488_508_000_002);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(total_emissions), 2_430.061_913_600_002);
    assert_eq!(f64::from(emission_factors.n2o), 0.001_253_278_688_524_597_2);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_619_999_999_99
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.35769999999997
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_optimistic() {
    let profile = EmissionInfluencingValues {
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open: true,
            custom_sludge_storage_containers_factor: None,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
            digester_count: None,
        },
        operating_materials: OperatingMaterials {
            fecl3: Tons::new(0.0),
            feclso4: Tons::new(326.0),
            caoh2: Tons::new(326.26),
            synthetic_polymers: Tons::new(23.62),
        },
    };

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
        ch4_sewage_treatment,
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
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 785.07);
    assert_eq!(f64::from(n2o_water), 126.126);
    assert_eq!(f64::from(n2o_emissions), 911.196);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        251.108_928_000_000_05
    );
    assert_eq!(f64::from(ch4_sludge_bags), 0.0);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_343.392_008);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers.round(3)), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 2_254.588_008);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(total_emissions), 2_887.161_413_600_000_3);
    assert_eq!(f64::from(emission_factors.n2o), 0.003);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_619_999_999_99
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_pesimistic() {
    let profile = EmissionInfluencingValues {
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open: true,
            custom_sludge_storage_containers_factor: None,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
            digester_count: None,
        },
        operating_materials: OperatingMaterials {
            fecl3: Tons::new(0.0),
            feclso4: Tons::new(326.0),
            caoh2: Tons::new(326.26),
            synthetic_polymers: Tons::new(23.62),
        },
    };

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
        ch4_sewage_treatment,
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
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 2_093.52);
    assert_eq!(f64::from(n2o_water), 126.126);
    assert_eq!(f64::from(n2o_emissions), 2_219.646);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        251.108_928_000_000_05
    );
    assert_eq!(f64::from(ch4_sludge_bags), 0.0);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_343.392_008);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers.round(3)), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 3_563.038_008_000_000_4);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(total_emissions), 4_195.611_413_600_001);
    assert_eq!(f64::from(emission_factors.n2o), 0.008);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_619_999_999_99
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_ipcc2019() {
    let profile = EmissionInfluencingValues {
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open: true,
            custom_sludge_storage_containers_factor: None,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
            digester_count: None,
        },
        operating_materials: OperatingMaterials {
            fecl3: Tons::new(0.0),
            feclso4: Tons::new(326.0),
            caoh2: Tons::new(326.26),
            synthetic_polymers: Tons::new(23.62),
        },
    };

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
        ch4_sewage_treatment,
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
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 4_187.04);
    assert_eq!(f64::from(n2o_water), 126.126);
    assert_eq!(f64::from(n2o_emissions), 4_313.166);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        251.108_928_000_000_05
    );
    assert_eq!(f64::from(ch4_sludge_bags), 0.0);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_343.392_008);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers.round(3)), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 5_656.558_008);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(total_emissions), 6_289.131_413_6);
    assert_eq!(f64::from(emission_factors.n2o), 0.016);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);
    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_619_999_999_99
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_custom_factor() {
    let profile = EmissionInfluencingValues {
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open: true,
            custom_sludge_storage_containers_factor: None,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
            digester_count: None,
        },
        operating_materials: OperatingMaterials {
            fecl3: Tons::new(0.0),
            feclso4: Tons::new(326.0),
            caoh2: Tons::new(326.26),
            synthetic_polymers: Tons::new(23.62),
        },
    };

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
        ch4_sewage_treatment,
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
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    const PRECISION: usize = 3;

    assert_eq!(n2o_plant.round(PRECISION), Tons::new(2_616.9));
    assert_eq!(n2o_water.round(PRECISION), Tons::new(126.126));
    assert_eq!(n2o_emissions.round(PRECISION), Tons::new(2_743.026));
    assert_eq!(ch4_sewage_treatment.round(PRECISION), Tons::new(772.8));
    assert_eq!(
        ch4_sludge_storage_containers.round(PRECISION),
        Tons::new(251.109)
    );
    assert_eq!(ch4_sludge_bags.round(PRECISION), Tons::zero());
    assert_eq!(ch4_water.round(PRECISION), Tons::new(162.54));
    assert_eq!(ch4_emissions.round(PRECISION), Tons::new(1_343.392));
    assert_eq!(fecl3.round(PRECISION), Tons::zero());
    assert_eq!(feclso4.round(PRECISION), Tons::new(24.776));
    assert_eq!(caoh2.round(PRECISION), Tons::new(344.302));
    assert_eq!(synthetic_polymers.round(PRECISION), Tons::new(51.964));
    assert_eq!(electricity_mix.round(PRECISION), Tons::new(202.345));
    assert_eq!(operating_materials.round(PRECISION), Tons::new(421.042));
    assert_eq!(sewage_sludge_transport.round(PRECISION), Tons::new(9.186));
    assert_eq!(direct_emissions.round(PRECISION), Tons::new(4_086.418));
    assert_eq!(indirect_emissions.round(PRECISION), Tons::new(202.345));
    assert_eq!(
        other_indirect_emissions.round(PRECISION),
        Tons::new(430.228)
    );
    assert_eq!(total_emissions.round(PRECISION), Tons::new(4_718.991));
    assert_eq!(emission_factors.n2o.round(PRECISION), Factor::new(0.01));
    assert_eq!(excess_energy_co2_equivalent.round(PRECISION), Tons::zero());
    assert_eq!(
        ch4_combined_heat_and_power_plant.round(PRECISION),
        Tons::new(156.943)
    ); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_619_999_999_99
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}

#[test]
fn calculate_ch4_slippage_sludge_bags_for_one_digester() {
    let expected = Tons::new(4.8711075);
    let digester_count = Some(1);
    let methane_fraction = Percent::new(62.0);
    let custom_sludge_bags_factor: Option<f64> = Some(1.25);
    let result = calculate_ch4_slippage_sludge_bags(
        digester_count,
        methane_fraction,
        custom_sludge_bags_factor,
    );
    assert_eq!(result, expected);
    assert_eq!(result * GWP_CH4, Tons::new(136.391_01));
}
