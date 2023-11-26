use crate::{
    constants::*, AnnualAverageEffluent, AnnualAverageInfluent, CO2Equivalents, EnergyConsumption,
    Factor, Input, Mass, MilligramsPerLiter, OperatingMaterials, Output, Qubicmeters,
    SewageSludgeTreatment, Tons,
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
        population_equivalent,
        wastewater,
        influent_average,
        effluent_average,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    } = input;

    let AnnualAverageInfluent {
        nitrogen: nitrogen_influent,
        chemical_oxygen_demand: _,
        phosphorus: _,
    } = influent_average;

    let AnnualAverageEffluent {
        nitrogen: nitrogen_effluent,
        chemical_oxygen_demand: chemical_oxygen_demand_effluent,
        phosphorus: _,
    } = effluent_average;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        gas_supply: _,
        purchase_of_biogas: _,
        total_power_consumption,
        on_site_power_generation,
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
        calculate_n2o_emission_factor(calc_method, *nitrogen_influent, *nitrogen_effluent);
    debug_assert!(n2o_emission_factor < Factor::new(1.0));

    let (n2o_plant, n2o_water) = calculate_nitrous_oxide(
        *nitrogen_influent,
        *nitrogen_effluent,
        *wastewater,
        n2o_emission_factor,
    );

    let ch4_sewage_treatment =
        population_equivalent * EMISSION_FACTOR_CH4_PLANT / f64::from(10_i32.pow(6)); // [t CH4/a]
    let ch4_water = f64::from(*chemical_oxygen_demand_effluent * *wastewater) / 1000.0
        * EMISSION_FACTOR_CH4_WATER; // [t CH4/a]
    let ch4_bhkw =
        f64::from(*on_site_power_generation * EMISSION_FACTOR_CH4_CHP) / f64::from(10_i32.pow(6)); // [t CH4/a]

    let ch4_slippage_sludge_bags = if *open_sludge_bags {
        let volume = *sewage_gas_produced * *methane_fraction * EMISSION_FACTOR_SLUDGE_BAGS;
        let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
        f64::from(mass) / 1_000.0
    } else {
        0.0
    }; // [t CH4 / a]

    let ch4_slippage_sludge_storage = if *open_sludge_storage_containers {
        let volume = *sewage_gas_produced * *methane_fraction * EMISSION_FACTOR_SLUDGE_STORAGE;
        let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
        f64::from(mass) / 10_000.0
    } else {
        0.0
    }; // [t CH4 / a]

    let n2o_plant = Tons::new(n2o_plant * GWP_N2O);
    let n2o_water = Tons::new(n2o_water * GWP_N2O);
    let n2o_emissions = n2o_plant + n2o_water;

    let ch4_sewage_treatment = Tons::new(ch4_sewage_treatment * GWP_CH4);
    let ch4_sludge_storage_containers = Tons::new(ch4_slippage_sludge_storage * GWP_CH4);
    let ch4_sludge_bags = Tons::new(ch4_slippage_sludge_bags * GWP_CH4);
    let ch4_water = Tons::new(ch4_water * GWP_CH4);
    let ch4_combined_heat_and_power_plant = Tons::new(ch4_bhkw * GWP_CH4);
    let ch4_emissions = ch4_sewage_treatment
        + ch4_sludge_storage_containers
        + ch4_sludge_bags
        + ch4_water
        + ch4_combined_heat_and_power_plant;

    let external_energy = *total_power_consumption - *on_site_power_generation; // [kwh/a]

    let divisor6 = f64::from(10_i32.pow(6));
    let electricity_mix =
        Tons::new(f64::from(external_energy * *emission_factor_electricity_mix) / divisor6);

    let divisor3 = f64::from(10_i32.pow(3));
    let synthetic_polymers =
        Tons::new(f64::from(*synthetic_polymers) * EMISSION_FACTOR_POLYMERS / divisor3);
    let fecl3 = Tons::new(f64::from(*fecl3) * EMISSION_FACTOR_FECL3 / divisor3);
    let feclso4 = Tons::new(f64::from(*feclso4) * EMISSION_FACTOR_FECLSO4 / divisor3);
    let caoh2 = Tons::new(f64::from(*caoh2) * EMISSION_FACTOR_CAOH2 / divisor3);

    let operating_materials = synthetic_polymers + feclso4 + caoh2 + fecl3;

    let sewage_sludge_transport = (*sewage_sludge_for_disposal
        * FUEL_CONSUMPTION
        * *transport_distance
        * EMISSION_FACTOR_DIESEL)
        .convert_to();

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

#[must_use]
pub fn calculate_n2o_emission_factor(
    calc_method: N2oEmissionFactorCalcMethod,
    nitrogen_influent: MilligramsPerLiter,
    nitrogen_effluent: MilligramsPerLiter,
) -> Factor {
    match calc_method {
        N2oEmissionFactorCalcMethod::ExtrapolatedParravicini => {
            extrapolate_according_to_parravicini(nitrogen_influent, nitrogen_effluent)
        }
        N2oEmissionFactorCalcMethod::Optimistic => EMISSION_FACTOR_N2O_OPTIMISTIC.into(),
        N2oEmissionFactorCalcMethod::Pesimistic => EMISSION_FACTOR_N2O_PESIMISTIC.into(),
        N2oEmissionFactorCalcMethod::Ipcc2019 => EMISSION_FACTOR_N2O_IPCC2019.into(),
        N2oEmissionFactorCalcMethod::Custom(factor) => factor,
    }
}

#[must_use]
pub fn extrapolate_according_to_parravicini(
    nitrogen_influent: MilligramsPerLiter,
    nitrogen_effluent: MilligramsPerLiter,
) -> Factor {
    let n_elim = f64::from((nitrogen_influent - nitrogen_effluent) / nitrogen_influent) * 100.0;
    let mut ef = (-0.049 * n_elim + 4.553) / 100.0;
    if ef < 0.0 {
        ef = 0.002;
    }
    Factor::new(ef)
}

const CONVERSION_FACTOR_N_TO_N2O: f64 = 44.0 / 28.0;

fn calculate_nitrous_oxide(
    nitrogen_influent: MilligramsPerLiter,
    nitrogen_effluent: MilligramsPerLiter,
    wastewater: Qubicmeters,
    n2o_emission_factor: Factor,
) -> (f64, f64) {
    let n2o_anlage = f64::from(wastewater * nitrogen_influent) / 1_000.0
        * n2o_emission_factor
        * CONVERSION_FACTOR_N_TO_N2O; // [t N2O/a]
    let n2o_gewaesser = f64::from(nitrogen_effluent * wastewater) / 1_000.0
        * EMISSION_FACTOR_N2O_WATER
        * CONVERSION_FACTOR_N_TO_N2O; // [t N2O/a]
    (n2o_anlage, n2o_gewaesser)
}
