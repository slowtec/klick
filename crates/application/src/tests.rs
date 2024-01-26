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
        n2o_emission_factor: _,
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
            open_sludge_bags: true,
            open_sludge_storage_containers: true,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
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
        n2o_emission_factor,
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
    assert_eq!(f64::from(ch4_sludge_bags), 47.082_924);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_406.169_24);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 15.156_589_140_000_001);
    assert_eq!(f64::from(direct_emissions), 1_860.265_740_000_001_9);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 436.198_767_14);
    assert_eq!(f64::from(emissions), 2_498.809_923_140_002);
    assert_eq!(f64::from(n2o_emission_factor), 0.001_253_278_688_524_597_2);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        154.588_933_8
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        153.019_503_000_000_01
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
            open_sludge_bags: true,
            open_sludge_storage_containers: true,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
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
        n2o_emission_factor,
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
    assert_eq!(f64::from(ch4_sludge_bags), 47.082_924);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_406.169_24);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 15.156_589_140_000_001);
    assert_eq!(f64::from(direct_emissions), 2_317.365_239_999_999_6);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 436.198_767_14);
    assert_eq!(f64::from(emissions), 2_955.909_423_139_999_5);
    assert_eq!(f64::from(n2o_emission_factor), 0.003);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        154.588_933_8
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        153.019_503_000_000_01
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
            open_sludge_bags: true,
            open_sludge_storage_containers: true,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
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
        n2o_emission_factor,
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
    assert_eq!(f64::from(ch4_sludge_bags), 47.082_924);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_406.169_24);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 15.156_589_140_000_001);
    assert_eq!(f64::from(direct_emissions), 3_625.815_240_000_000_4);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 436.198_767_14);
    assert_eq!(f64::from(emissions), 4_264.359_423_14);
    assert_eq!(f64::from(n2o_emission_factor), 0.008);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        154.588_933_8
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        153.019_503_000_000_01
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
            open_sludge_bags: true,
            open_sludge_storage_containers: true,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
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
        n2o_emission_factor,
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
    assert_eq!(f64::from(ch4_sludge_bags), 47.082_924);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_406.169_24);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 15.156_589_140_000_001);
    assert_eq!(f64::from(direct_emissions), 5_719.335_24);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 436.198_767_14);
    assert_eq!(f64::from(emissions), 6_357.879_423_140_001);
    assert_eq!(f64::from(n2o_emission_factor), 0.016);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        154.588_933_8
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        153.019_503_000_000_01
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
            open_sludge_bags: true,
            open_sludge_storage_containers: true,
            sewage_sludge_for_disposal: Tons::new(3687.6),
            transport_distance: Kilometers::new(47.0),
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
        n2o_emission_factor,
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
    assert_eq!(f64::from(ch4_sludge_bags), 47.082_924);
    assert_eq!(f64::from(ch4_water), 162.54);
    assert_eq!(f64::from(ch4_emissions), 1_406.169_24);
    assert_eq!(f64::from(fecl3), 0.0);
    assert_eq!(f64::from(feclso4), 24.776);
    assert_eq!(f64::from(caoh2), 344.302_177_999_999_97);
    assert_eq!(f64::from(synthetic_polymers), 51.964);
    assert_eq!(f64::from(electricity_mix), 202.345_416);
    assert_eq!(f64::from(operating_materials), 421.042_178);
    assert_eq!(f64::from(sewage_sludge_transport), 15.156_589_140_000_001);
    assert_eq!(f64::from(direct_emissions), 4_149.195_240_000_001);
    assert_eq!(f64::from(indirect_emissions), 202.345_416);
    assert_eq!(f64::from(other_indirect_emissions), 436.198_767_14);
    assert_eq!(f64::from(emissions), 4_787.739_423_140_001);
    assert_eq!(f64::from(n2o_emission_factor), 0.01);
    assert_eq!(f64::from(excess_energy_co2_equivalent), 0.0);

    assert_eq!(f64::from(ch4_combined_heat_and_power_plant), 156.943_08); // MicroGasTurbines
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::GasolineEngine)
        ),
        154.588_933_8
    );
    assert_eq!(
        ch4_combined_heat_and_power_plant_computation_helper(
            scenario,
            profile.clone(),
            Some(domain::CH4ChpEmissionFactorCalcMethod::JetEngine)
        ),
        153.019_503_000_000_01
    );
}
