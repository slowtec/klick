use crate::{
    AnnualAveragesEffluent, AnnualAveragesInflow, CO2Equivalents, EnergyConsumption, Factor, Input,
    OperatingMaterials, Output, Percent, SewageSludgeTreatment,
};

const EMISSION_FACTOR_CH4_PLANT: f64 = 230.0; // g ch4 / (population values * year)
const EMISSION_FACTOR_CH4_WATER: f64 = 0.009; // 0,9 % of chemical oxygen demand effluent
const EMISSION_FACTOR_CH4_CHP: f64 = 1.164; // 1,164 g ch4 / kwh
const EMISSION_FACTOR_N2O_WATER: f64 = 0.005; // 0,5 % of nitrogen effulent
const EMISSION_FACTOR_SLUDGE_BAGS: f64 = 0.003; // 0.3 % of the total methane gas yield
const EMISSION_FACTOR_SLUDGE_STORAGE: f64 = 0.017; // 1,7 % of the total digester gas production
const EMISSION_FACTOR_FECL3: f64 = 395.0; // g co2 / kg solution
const EMISSION_FACTOR_FECLSO4: f64 = 76.0; // g co2 / kg solution
const EMISSION_FACTOR_CAOH2: f64 = 1055.3; // g co2 / kg solution
const EMISSION_FACTOR_POLYMERS: f64 = 2200.0; // g co2 / kg solution

const GWP_N2O: f64 = 273.0;
const GWP_CH4: f64 = 28.0;
const CONVERSION_FACTOR_CH4_M3_TO_KG: f64 = 0.7175; // kg/m^3 for standard cubic meters (GESTIS substance database)

const EMISSION_FACTOR_DIESEL: f64 = 3.24; // kg co2/l
const FUEL_CONSUMPTION: f64 = 0.033; // l/tkm

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum N2oEmissionFactorCalcMethod {
    ExtrapolatedParravicini,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    Custom(Factor),
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn calc(input: &Input, calc_method: N2oEmissionFactorCalcMethod) -> Output {
    let Input {
        plant_name: _,
        population_values,
        waste_water,
        inflow_averages,
        effluent_averages,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    } = input;

    let AnnualAveragesInflow {
        nitrogen: nitrogen_inflow,
        chemical_oxygen_demand: _,
        phosphorus: _,
    } = inflow_averages;

    let AnnualAveragesEffluent {
        nitrogen: nitrogen_effluent,
        chemical_oxygen_demand: chemical_oxygen_demand_effluent,
        phosphorus: _,
    } = effluent_averages;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_level,
        gas_supply: _,
        purchase_of_biogas: _,
        total_power_consumption,
        in_house_power_generation,
        emission_factor_electricity_mix,
    } = energy_consumption;

    let SewageSludgeTreatment {
        open_sludge_bags,
        open_sludge_storage_containers,
        sewage_sludge_for_disposal,
        transport_distance,
    } = sewage_sludge_treatment;

    let OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    } = operating_materials;

    let n2o_emission_factor =
        calculate_n2o_emission_factor(calc_method, *nitrogen_inflow, *nitrogen_effluent);
    debug_assert!(n2o_emission_factor < Factor::new(1.0));

    let (n2o_plant, n2o_water) = calculate_nitrous_oxide(
        *nitrogen_inflow,
        *nitrogen_effluent,
        *waste_water,
        n2o_emission_factor,
    );

    let ch4_sewage_treatment =
        population_values * EMISSION_FACTOR_CH4_PLANT / f64::from(10_i32.pow(6)); // [t CH4/a]
    let ch4_water = chemical_oxygen_demand_effluent / f64::from(10_i32.pow(9))
        * waste_water
        * f64::from(10_i32.pow(3))
        * EMISSION_FACTOR_CH4_WATER; // [t CH4/a]
    let ch4_bhkw = in_house_power_generation * EMISSION_FACTOR_CH4_CHP / f64::from(10_i32.pow(6)); // [t CH4/a]

    let ch4_slippage_sludge_bags = if *open_sludge_bags {
        sewage_gas_produced
            * methane_level
            * EMISSION_FACTOR_SLUDGE_BAGS
            * CONVERSION_FACTOR_CH4_M3_TO_KG
            / 1_000.0
    } else {
        0.0
    }; // [t CH4 / a]

    let ch4_slippage_sludge_storage = if *open_sludge_storage_containers {
        sewage_gas_produced
            * methane_level
            * EMISSION_FACTOR_SLUDGE_STORAGE
            * CONVERSION_FACTOR_CH4_M3_TO_KG
            / 10_000.0
    } else {
        0.0
    }; // [t CH4 / a]

    let n2o_plant = n2o_plant * GWP_N2O;
    let n2o_water = n2o_water * GWP_N2O;
    let n2o_emissions = n2o_plant + n2o_water;

    let ch4_sewage_treatment = ch4_sewage_treatment * GWP_CH4;
    let ch4_sludge_storage_containers = ch4_slippage_sludge_storage * GWP_CH4;
    let ch4_sludge_bags = ch4_slippage_sludge_bags * GWP_CH4;
    let ch4_water = ch4_water * GWP_CH4;
    let ch4_combined_heat_and_power_plant = ch4_bhkw * GWP_CH4;
    let ch4_emissions = ch4_sewage_treatment
        + ch4_sludge_storage_containers
        + ch4_sludge_bags
        + ch4_water
        + ch4_combined_heat_and_power_plant;

    let external_energy = total_power_consumption - in_house_power_generation; // [kwh/a]

    let divisor = f64::from(10_i32.pow(6));

    let electricity_mix = external_energy * emission_factor_electricity_mix / divisor;
    let synthetic_polymers = synthetic_polymers * EMISSION_FACTOR_POLYMERS / divisor;
    let fecl3 = fecl3 * EMISSION_FACTOR_FECL3 / divisor;
    let feclso4 = feclso4 * EMISSION_FACTOR_FECLSO4 / divisor;
    let caoh2 = caoh2 * EMISSION_FACTOR_CAOH2 / divisor;

    let operating_materials = synthetic_polymers + feclso4 + caoh2 + fecl3;

    let sewage_sludge_transport =
        sewage_sludge_for_disposal * transport_distance * FUEL_CONSUMPTION * EMISSION_FACTOR_DIESEL
            / 1_000.0;

    let direct_emissions = n2o_plant
        + n2o_water
        + ch4_sewage_treatment
        + ch4_water
        + ch4_combined_heat_and_power_plant
        + ch4_sludge_storage_containers
        + ch4_sludge_bags;
    let indirect_emissions = electricity_mix;
    let other_indirect_emissions = operating_materials + sewage_sludge_transport;
    let emissions = direct_emissions + indirect_emissions + other_indirect_emissions;

    let co2_equivalents = CO2Equivalents {
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
    };

    Output {
        co2_equivalents,
        n2o_emission_factor,
    }
}

fn calculate_n2o_emission_factor(
    calc_method: N2oEmissionFactorCalcMethod,
    nitrogen_inflow: f64,
    nitrogen_effluent: f64,
) -> Factor {
    match calc_method {
        N2oEmissionFactorCalcMethod::ExtrapolatedParravicini => {
            extrapolate_according_to_parravicini(nitrogen_inflow, nitrogen_effluent)
        }
        N2oEmissionFactorCalcMethod::Optimistic => Percent::new(0.3).as_factor(), // 0,3 % of the nitrogen inflow
        N2oEmissionFactorCalcMethod::Pesimistic => Percent::new(0.8).as_factor(), // 0,8 % of the nitrogen inflow
        N2oEmissionFactorCalcMethod::Ipcc2019 => Percent::new(1.6).as_factor(), // 1,6 % of the nitrogen inflow
        N2oEmissionFactorCalcMethod::Custom(factor) => factor,
    }
}

fn extrapolate_according_to_parravicini(nitrogen_inflow: f64, nitrogen_effluent: f64) -> Factor {
    let n_elim = (nitrogen_inflow - nitrogen_effluent) / nitrogen_inflow * 100.0;
    let mut ef = (-0.049 * n_elim + 4.553) / 100.0;
    if ef < 0.0 {
        ef = 0.002;
    }
    Factor::new(ef)
}

const CONVERSION_FACTOR_N_TO_N2O: f64 = 44.0 / 28.0;

fn calculate_nitrous_oxide(
    nitrogen_inflow: f64,
    nitrogen_effluent: f64,
    waste_water: f64,
    n2o_emission_factor: Factor,
) -> (f64, f64) {
    let n2o_anlage = nitrogen_inflow / f64::from(10_i32.pow(9))
        * waste_water
        * 1_000.0
        * n2o_emission_factor
        * CONVERSION_FACTOR_N_TO_N2O; // [t N2O/a]
    let n2o_gewaesser = nitrogen_effluent / f64::from(10_i32.pow(9))
        * waste_water
        * 1_000.0
        * EMISSION_FACTOR_N2O_WATER
        * CONVERSION_FACTOR_N_TO_N2O; // [t N2O/a]
    (n2o_anlage, n2o_gewaesser)
}

#[cfg(test)]
mod tests {
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
                fecl3: 0.0,
                feclso4: 326_000.0,
                caoh2: 326_260.0,
                synthetic_polymers: 23620.0,
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

        assert_eq!(n2o_plant, 327.970_500_000_001_83);
        assert_eq!(n2o_water, 126.125_999_999_999_99);
        assert_eq!(n2o_emissions, 454.096_500_000_001_8);
        assert_eq!(ch4_sewage_treatment, 772.800_000_000_000_1);
        assert_eq!(ch4_sludge_storage_containers, 26.680_323_600_000_005);
        assert_eq!(ch4_sludge_bags, 47.082_924);
        assert_eq!(ch4_water, 162.54);
        assert_eq!(ch4_combined_heat_and_power_plant, 73.361_235_024);
        assert_eq!(ch4_emissions, 1_082.464_482_624);
        assert_eq!(fecl3, 0.0);
        assert_eq!(feclso4, 24.776);
        assert_eq!(caoh2, 344.302_178);
        assert_eq!(synthetic_polymers, 51.964);
        assert_eq!(electricity_mix, 202.345_416);
        assert_eq!(operating_materials, 421.042_178_000_000_04);
        assert_eq!(sewage_sludge_transport, 18.531_075_024_000_003);
        assert_eq!(direct_emissions, 1_536.560_982_624_002);
        assert_eq!(indirect_emissions, 202.345_416);
        assert_eq!(other_indirect_emissions, 439.573_253_024_000_05);
        assert_eq!(emissions, 2_178.479_651_648_002_3);
        assert_eq!(
            n2o_emission_factor,
            Factor::new(0.001_253_278_688_524_597_2)
        );
    }
}
