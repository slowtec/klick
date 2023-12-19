use std::fmt;

use super::*;

// TODO: What precision is required?
const EPSILON: f64 = 0.000_000_000_001;

#[test]
fn calculate_with_n2o_emission_factor_method_by_parravicini() {
    let input = Input {
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
        combined_heat_and_power_plant_ch4_emission_factor: None,
    };

    let method = N2oEmissionFactorCalcMethod::ExtrapolatedParravicini;
    let Output {
        co2_equivalents,
        n2o_emission_factor,
    } = calculate_emissions(&input, method);

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

    fn assert_approx_eq<T>(a: T, b: f64)
    where
        T: Into<f64> + fmt::Display,
    {
        let a = a.into();
        assert!(
            (a - b).abs() < EPSILON,
            "Values not close enough:\n  left = {a}\n right = {b}"
        );
    }

    assert_approx_eq(n2o_plant, 327.970_500_000_001_83);
    assert_approx_eq(n2o_water, 126.125_999_999_999_99);
    assert_approx_eq(n2o_emissions, 454.096_500_000_001);
    assert_approx_eq(ch4_sewage_treatment, 772.8);
    assert_approx_eq(ch4_sludge_storage_containers, 26.680_323_6);
    assert_approx_eq(ch4_sludge_bags, 47.082_924);
    assert_approx_eq(ch4_water, 162.54);
    assert_approx_eq(ch4_combined_heat_and_power_plant, 70.840_230_384);
    assert_approx_eq(ch4_emissions, 1_079.943_477_984);
    assert_approx_eq(fecl3, 0.0);
    assert_approx_eq(feclso4, 24.776);
    assert_approx_eq(caoh2, 344.302_177_999_999_97);
    assert_approx_eq(synthetic_polymers, 51.964);
    assert_approx_eq(electricity_mix, 202.345_416);
    assert_approx_eq(operating_materials, 421.042_178);
    assert_approx_eq(sewage_sludge_transport, 15.156_589_14);
    assert_approx_eq(direct_emissions, 1_534.039_977_984_002);
    assert_approx_eq(indirect_emissions, 202.345_416);
    assert_approx_eq(other_indirect_emissions, 436.198_767_14);
    assert_approx_eq(emissions, 2_172.584_161_124_002);
    assert_approx_eq(n2o_emission_factor, 0.001_253_278_688_524_597_2);
}
