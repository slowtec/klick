use crate::{
    constants::*, AnnualAveragesEffluent, AnnualAveragesInflow, CO2Equivalents, EnergyConsumption,
    Factor, Input, OperatingMaterials, Output, SewageSludgeTreatment,
};

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
pub fn calculate_emissions(input: &Input, calc_method: N2oEmissionFactorCalcMethod) -> Output {
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
        * EMISSION_FACTOR_CH4_WATER.as_factor(); // [t CH4/a]
    let ch4_bhkw = in_house_power_generation * EMISSION_FACTOR_CH4_CHP / f64::from(10_i32.pow(6)); // [t CH4/a]

    let ch4_slippage_sludge_bags = if *open_sludge_bags {
        sewage_gas_produced
            * methane_level
            * EMISSION_FACTOR_SLUDGE_BAGS.as_factor()
            * CONVERSION_FACTOR_CH4_M3_TO_KG
            / 1_000.0
    } else {
        0.0
    }; // [t CH4 / a]

    let ch4_slippage_sludge_storage = if *open_sludge_storage_containers {
        sewage_gas_produced
            * methane_level
            * EMISSION_FACTOR_SLUDGE_STORAGE.as_factor()
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

    let divisor6 = f64::from(10_i32.pow(6));
    let electricity_mix = external_energy * emission_factor_electricity_mix / divisor6;

    let divisor3 = f64::from(10_i32.pow(3));
    let synthetic_polymers = f64::from(*synthetic_polymers) * EMISSION_FACTOR_POLYMERS / divisor3;
    let fecl3 = f64::from(*fecl3) * EMISSION_FACTOR_FECL3 / divisor3;
    let feclso4 = f64::from(*feclso4) * EMISSION_FACTOR_FECLSO4 / divisor3;
    let caoh2 = f64::from(*caoh2) * EMISSION_FACTOR_CAOH2 / divisor3;

    let operating_materials = synthetic_polymers + feclso4 + caoh2 + fecl3;

    let sewage_sludge_transport = f64::from(*sewage_sludge_for_disposal)
        * f64::from(*transport_distance)
        * FUEL_CONSUMPTION
        * EMISSION_FACTOR_DIESEL
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

pub fn calculate_n2o_emission_factor(
    calc_method: N2oEmissionFactorCalcMethod,
    nitrogen_inflow: f64,
    nitrogen_effluent: f64,
) -> Factor {
    match calc_method {
        N2oEmissionFactorCalcMethod::ExtrapolatedParravicini => {
            extrapolate_according_to_parravicini(nitrogen_inflow, nitrogen_effluent)
        }
        N2oEmissionFactorCalcMethod::Optimistic => EMISSION_FACTOR_N2O_OPTIMISTIC.as_factor(),
        N2oEmissionFactorCalcMethod::Pesimistic => EMISSION_FACTOR_N2O_PESIMISTIC.as_factor(),
        N2oEmissionFactorCalcMethod::Ipcc2019 => EMISSION_FACTOR_N2O_IPCC2019.as_factor(),
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
        * EMISSION_FACTOR_N2O_WATER.as_factor()
        * CONVERSION_FACTOR_N_TO_N2O; // [t N2O/a]
    (n2o_anlage, n2o_gewaesser)
}
