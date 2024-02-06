use super::*;
use klick_domain as domain;
use klick_domain::*;

fn ch4_combined_heat_and_power_plant_computation_helper(
    scenario: OptimizationScenario,
    profile: PlantProfile,
    ch4_chp_emission_factor: Option<domain::CH4ChpEmissionFactorCalcMethod>,
) -> f64 {
    let mut s2 = scenario;
    s2.ch4_chp_emission_factor = ch4_chp_emission_factor;
    let Output {
        co2_equivalents,
        emission_factors: _,
    } = calculate_emissions(&profile, s2);

    f64::from(co2_equivalents.ch4_combined_heat_and_power_plant)
}

#[test]
fn calculate_with_n2o_emission_factor_method_by_tu_wien_2016() {
    let profile = PlantProfile {
        plant_name: None,
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
            chemical_oxygen_demand: None,
            phosphorus: None,
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
            phosphorus: None,
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            sludge_storage_containers_are_open: true,
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

    let scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorCalcMethod::TuWien2016,
        ch4_chp_emission_factor: None,
    };

    let Output {
        co2_equivalents,
        emission_factors,
    } = calculate_emissions(&profile, scenario);

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
        emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 327.970_500_000_001_83);
    assert_eq!(f64::from(n2o_water), 126.125_999_999_999_99);
    assert_eq!(f64::from(n2o_emissions), 454.096_500_000_001_8);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        266.803_235_999_999_97
    );
    assert_eq!(f64::from(ch4_sludge_bags), 5.8853655);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_364.971_681_5);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 1819.0681815000019);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(emissions), 2451.6415871000017);
    assert_eq!(f64::from(emission_factors.n2o), 0.001_253_278_688_524_597_2);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_620_000_000_04
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.35769999999997
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_optimistic() {
    let profile = PlantProfile {
        plant_name: None,
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
            chemical_oxygen_demand: None,
            phosphorus: None,
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
            phosphorus: None,
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            sludge_storage_containers_are_open: true,
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

    let scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorCalcMethod::Optimistic,
        ch4_chp_emission_factor: None,
    };

    let Output {
        co2_equivalents,
        emission_factors,
    } = calculate_emissions(&profile, scenario);

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
        emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 785.07);
    assert_eq!(f64::from(n2o_water), 126.125_999_999_999_99);
    assert_eq!(f64::from(n2o_emissions), 911.196);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        266.803_235_999_999_97
    );
    assert_eq!(f64::from(ch4_sludge_bags), 5.8853655);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_364.971_681_5);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 2276.1676814999996);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(emissions), 2908.7410870999997);
    assert_eq!(f64::from(emission_factors.n2o), 0.003);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_620_000_000_04
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_pesimistic() {
    let profile = PlantProfile {
        plant_name: None,
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
            chemical_oxygen_demand: None,
            phosphorus: None,
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
            phosphorus: None,
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            sludge_storage_containers_are_open: true,
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

    let scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorCalcMethod::Pesimistic,
        ch4_chp_emission_factor: None,
    };

    let Output {
        co2_equivalents,
        emission_factors,
    } = calculate_emissions(&profile, scenario);

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
        emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 2_093.52);
    assert_eq!(f64::from(n2o_water), 126.125_999_999_999_99);
    assert_eq!(f64::from(n2o_emissions), 2_219.646);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        266.803_235_999_999_97
    );
    assert_eq!(f64::from(ch4_sludge_bags), 5.8853655);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_364.971_681_5);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 3584.6176815000003);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(emissions), 4217.1910871);
    assert_eq!(f64::from(emission_factors.n2o), 0.008);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_620_000_000_04
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_ipcc2019() {
    let profile = PlantProfile {
        plant_name: None,
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
            chemical_oxygen_demand: None,
            phosphorus: None,
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
            phosphorus: None,
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            sludge_storage_containers_are_open: true,
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

    let scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorCalcMethod::Ipcc2019,
        ch4_chp_emission_factor: None,
    };

    let Output {
        co2_equivalents,
        emission_factors,
    } = calculate_emissions(&profile, scenario);

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
        emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 4_187.04);
    assert_eq!(f64::from(n2o_water), 126.125_999_999_999_99);
    assert_eq!(f64::from(n2o_emissions), 4_313.166);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        266.803_235_999_999_97
    );
    assert_eq!(f64::from(ch4_sludge_bags), 5.8853655);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_364.971_681_5);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 5678.1376815);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(emissions), 6310.7110871);
    assert_eq!(f64::from(emission_factors.n2o), 0.016);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_620_000_000_04
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}

#[test]
fn calculate_with_n2o_emission_factor_method_custom_factor() {
    let profile = PlantProfile {
        plant_name: None,
        population_equivalent: 120_000.0,
        wastewater: Qubicmeters::new(5_000_000.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: MilligramsPerLiter::new(122.0),
            chemical_oxygen_demand: None,
            phosphorus: None,
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: MilligramsPerLiter::new(11.76),
            chemical_oxygen_demand: MilligramsPerLiter::new(129.0),
            phosphorus: None,
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Qubicmeters::new(1_260_000.0),
            methane_fraction: Percent::new(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: Kilowatthours::new(2_683_259.0),
            on_site_power_generation: Kilowatthours::new(2_250_897.0),
            emission_factor_electricity_mix: GramsPerKilowatthour::new(468.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_open: true,
            sludge_storage_containers_are_open: true,
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

    let scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorCalcMethod::Custom(Factor::new(1.0 / 100.0)),
        ch4_chp_emission_factor: None,
    };

    let Output {
        co2_equivalents,
        emission_factors,
    } = calculate_emissions(&profile, scenario);

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
        emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    } = co2_equivalents;

    assert_eq!(f64::from(n2o_plant), 2_616.900_000_000_000_5);
    assert_eq!(f64::from(n2o_water), 126.125_999_999_999_99);
    assert_eq!(f64::from(n2o_emissions), 2_743.026_000_000_000_7);
    assert_eq!(f64::from(ch4_sewage_treatment), 772.800_000_000_000_1);
    assert_eq!(
        f64::from(ch4_sludge_storage_containers),
        266.803_235_999_999_97
    );
    assert_eq!(f64::from(ch4_sludge_bags), 5.8853655);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_364.971_681_5);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 9.1858116);
    assert_eq!(f64::from(direct_emissions), 4107.997681500001);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 430.2279896);
    assert_eq!(f64::from(emissions), 4740.571087100001);
    assert_eq!(f64::from(emission_factors.n2o), 0.01);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        235.414_620_000_000_04
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        392.357_699_999_999_97
    );
}
