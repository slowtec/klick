use super::*;

#[test]
fn calculate_with_n2o_emission_factor_method_by_parravicini() {
    let input = Input {
        plant_name: None,
        population_values: 120_000.0,
        waste_water: 5_000_000.0,
        inflow_averages: AnnualAveragesInflow {
            nitrogen: 122.0,
            chemical_oxygen_demand: None,
            phosphorus: None,
        },
        effluent_averages: AnnualAveragesEffluent {
            nitrogen: 11.76,
            chemical_oxygen_demand: 129.0,
            phosphorus: None,
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: 1_260_000.0,
            methane_level: Percent::new(62.0),
            gas_supply: None,
            purchase_of_biogas: None,
            total_power_consumption: 2_683_259.0,
            in_house_power_generation: 2_250_897.0,
            emission_factor_electricity_mix: 468.0,
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            open_sludge_bags: true,
            open_sludge_storage_containers: true,
            sewage_sludge_for_disposal: 3687.6,
            transport_distance: 47.0,
        },
        operating_materials: OperatingMaterials {
            fecl3: Tons::new(0.0),
            feclso4: Tons::new(326.0),
            caoh2: Tons::new(326.26),
            synthetic_polymers: Tons::new(23.62),
        },
    };

    let method = N2oEmissionFactorCalcMethod::ExtrapolatedParravicini;
    let Output {
        co2_equivalents,
        n2o_emission_factor,
    } = calc(&input, method);

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
    } = co2_equivalents;

    let approx_eq = |a: f64, b: f64| {
        (a - b).abs() < f64::EPSILON // TODO: What precision is required?
    };

    assert!(approx_eq(n2o_plant, 327.970_500_000_001_83));
    assert!(approx_eq(n2o_water, 126.125_999_999_999_99));
    assert!(approx_eq(n2o_emissions, 454.096_500_000_001_8));
    assert!(approx_eq(ch4_sewage_treatment, 772.800_000_000_000_1));
    assert!(approx_eq(
        ch4_sludge_storage_containers,
        26.680_323_600_000_005
    ));
    assert!(approx_eq(ch4_sludge_bags, 47.082_924));
    assert!(approx_eq(ch4_water, 162.54));
    assert!(approx_eq(ch4_combined_heat_and_power_plant, 73.361_235_024));
    assert!(approx_eq(ch4_emissions, 1_082.464_482_624));
    assert!(approx_eq(fecl3, 0.0));
    assert!(approx_eq(feclso4, 24.776));
    assert!(approx_eq(caoh2, 344.302_177_999_999_97));
    assert!(approx_eq(synthetic_polymers, 51.964));
    assert!(approx_eq(electricity_mix, 202.345_416));
    assert!(approx_eq(operating_materials, 421.042_178));
    assert!(approx_eq(sewage_sludge_transport, 18.531_075_024_000_003));
    assert!(approx_eq(direct_emissions, 1_536.560_982_624_002));
    assert!(approx_eq(indirect_emissions, 202.345_416));
    assert!(approx_eq(other_indirect_emissions, 439.573_253_024));
    assert!(approx_eq(emissions, 2_178.479_651_648_002));
    assert!(approx_eq(
        f64::from(n2o_emission_factor),
        0.001_253_278_688_524_597_2
    ));
}
