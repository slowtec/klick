use klick_domain::{
    constants::*, AnnualAverageEffluent, AnnualAverageInfluent, CH4ChpEmissionFactorCalcMethod,
    CO2Equivalents, EmissionFactors, EnergyConsumption, Factor, Kilowatthours, Mass,
    MilligramsPerLiter, N2oEmissionFactorCalcMethod, OperatingMaterials, OptimizationScenario,
    PlantProfile, Qubicmeters, SewageSludgeTreatment, Tons,
};

use crate::Output;

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn calculate_emissions(input: &PlantProfile, scenario: OptimizationScenario) -> Output {
    let PlantProfile {
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
    } = influent_average;

    let AnnualAverageEffluent {
        nitrogen: nitrogen_effluent,
        chemical_oxygen_demand: chemical_oxygen_demand_effluent,
    } = effluent_average;

    let EnergyConsumption {
        sewage_gas_produced,
        methane_fraction,
        total_power_consumption,
        on_site_power_generation,
        emission_factor_electricity_mix,
    } = energy_consumption;

    let SewageSludgeTreatment {
        sludge_bags_are_open,
        sludge_storage_containers_are_open,
        sewage_sludge_for_disposal,
        transport_distance,
        digester_count,
    } = sewage_sludge_treatment;

    let OperatingMaterials {
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
    } = operating_materials;

    let n2o_emission_factor = calculate_n2o_emission_factor(
        scenario.n2o_emission_factor,
        *nitrogen_influent,
        *nitrogen_effluent,
    );
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

    let ch4_slippage_sludge_bags = if *sludge_bags_are_open {
        let count = digester_count.unwrap_or(1);
        let volume = *sewage_gas_produced
            * *methane_fraction
            * (Factor::new(count as f64) * EMISSION_FACTOR_SLUDGE_BAGS);
        let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
        f64::from(mass) / 1_000.0
    } else {
        0.0
    }; // [t CH4 / a]

    let ch4_slippage_sludge_storage = if *sludge_storage_containers_are_open {
        let volume = *sewage_gas_produced * *methane_fraction * EMISSION_FACTOR_SLUDGE_STORAGE;
        let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
        f64::from(mass) / 1_000.0
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

    let ch4_emission_factor = match scenario.ch4_chp_emission_factor {
        None => Factor::new(0.01),
        Some(CH4ChpEmissionFactorCalcMethod::MicroGasTurbines) => Factor::new(0.01),
        Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine) => Factor::new(0.015),
        Some(CH4ChpEmissionFactorCalcMethod::JetEngine) => Factor::new(0.025),
        Some(CH4ChpEmissionFactorCalcMethod::Custom(f)) => f,
    };

    let volume = *sewage_gas_produced * *methane_fraction * ch4_emission_factor;
    let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
    let ch4_chp = f64::from(mass) / 1_000.0; // [t CH4 / a]

    let ch4_combined_heat_and_power_plant = Tons::new(ch4_chp * GWP_CH4);

    let ch4_emissions = ch4_sewage_treatment
        + ch4_sludge_storage_containers
        + ch4_sludge_bags
        + ch4_water
        + ch4_combined_heat_and_power_plant;

    let divisor6 = f64::from(10_i32.pow(6));

    let t = *total_power_consumption - *on_site_power_generation;
    let external_energy = if t >= Kilowatthours::new(0.0) {
        t
    } else {
        Kilowatthours::new(0.0)
    }; // [kwh/a]]
    let excess_energy_co2_equivalent = if t <= Kilowatthours::new(0.0) {
        Tons::new(-1.0 * f64::from(t * *emission_factor_electricity_mix) / divisor6)
    } else {
        Tons::new(0.0)
    }; // [kwh/a]]

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
        excess_energy_co2_equivalent,
    };

    let emission_factors = EmissionFactors {
        n2o: n2o_emission_factor,
        ch4: ch4_emission_factor,
    };

    Output {
        co2_equivalents,
        emission_factors,
    }
}

#[must_use]
pub fn calculate_n2o_emission_factor(
    calc_method: N2oEmissionFactorCalcMethod,
    nitrogen_influent: MilligramsPerLiter,
    nitrogen_effluent: MilligramsPerLiter,
) -> Factor {
    match calc_method {
        N2oEmissionFactorCalcMethod::TuWien2016 => {
            extrapolate_according_to_tu_wien_2016(nitrogen_influent, nitrogen_effluent)
        }
        N2oEmissionFactorCalcMethod::Optimistic => EMISSION_FACTOR_N2O_OPTIMISTIC.into(),
        N2oEmissionFactorCalcMethod::Pesimistic => EMISSION_FACTOR_N2O_PESIMISTIC.into(),
        N2oEmissionFactorCalcMethod::Ipcc2019 => EMISSION_FACTOR_N2O_IPCC2019.into(),
        N2oEmissionFactorCalcMethod::Custom(factor) => factor,
    }
}

#[must_use]
pub fn extrapolate_according_to_tu_wien_2016(
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
