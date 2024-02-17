#[cfg(test)]
mod tests;

use crate::{
    constants::*,
    units::{
        Factor, Grams, Hours, Kilowatthours, Mass, MilligramsPerLiter, Percent, Qubicmeters, Ratio,
        Time, Tons, Years,
    },
    AnnualAverageEffluent, AnnualAverageInfluent, CH4ChpEmissionFactorCalcMethod, CO2Equivalents,
    EmissionFactorCalculationMethods, EmissionFactors, EmissionInfluencingValues,
    EnergyConsumption, N2oEmissionFactorCalcMethod, OperatingMaterials, SewageSludgeTreatment,
};

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn calculate_emissions(
    input: EmissionInfluencingValues,
    calc_methods: EmissionFactorCalculationMethods,
) -> (
    CO2Equivalents,
    EmissionFactors,
    EmissionFactorCalculationMethods,
) {
    // -------    ------ //
    //  Unpack variables //
    // -------    ------ //

    let EmissionInfluencingValues {
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

    // -------    ------ //
    //     Calculate     //
    // -------    ------ //

    let n2o_emission_factor =
        calculate_n2o_emission_factor(calc_methods.n2o, nitrogen_influent, nitrogen_effluent);
    debug_assert!(n2o_emission_factor < Factor::new(1.0));

    let (n2o_plant, n2o_water) = calculate_nitrous_oxide(
        nitrogen_influent,
        nitrogen_effluent,
        wastewater,
        n2o_emission_factor,
    );

    let ch4_sewage_treatment =
        Grams::new(population_equivalent * EMISSION_FACTOR_CH4_PLANT).convert_to::<Tons>();

    let ch4_water = chemical_oxygen_demand_effluent * wastewater * EMISSION_FACTOR_CH4_WATER;

    let ch4_slippage_sludge_bags = if sludge_bags_are_open {
        calculate_ch4_slippage_sludge_bags(digester_count, methane_fraction)
    } else {
        Tons::zero()
    };

    let ch4_slippage_sludge_storage = if sludge_storage_containers_are_open {
        calculate_ch4_slippage_sludge_storage(sewage_gas_produced, methane_fraction)
    } else {
        Tons::zero()
    };

    let n2o_plant = n2o_plant * GWP_N2O;
    let n2o_water = n2o_water * GWP_N2O;
    let n2o_emissions = n2o_plant + n2o_water;

    let ch4_sewage_treatment = ch4_sewage_treatment * GWP_CH4;
    let ch4_sludge_storage_containers = ch4_slippage_sludge_storage * GWP_CH4;
    let ch4_sludge_bags = ch4_slippage_sludge_bags * GWP_CH4;
    let ch4_water = ch4_water.convert_to::<Tons>() * GWP_CH4;

    let ch4_emission_factor = match calc_methods.ch4 {
        None => Factor::new(0.01),
        Some(CH4ChpEmissionFactorCalcMethod::MicroGasTurbines) => Factor::new(0.01),
        Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine) => Factor::new(0.015),
        Some(CH4ChpEmissionFactorCalcMethod::JetEngine) => Factor::new(0.025),
        Some(CH4ChpEmissionFactorCalcMethod::Custom(f)) => f,
    };

    let volume = sewage_gas_produced * methane_fraction * ch4_emission_factor;
    let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
    let ch4_chp = mass.convert_to::<Tons>();

    let ch4_combined_heat_and_power_plant = ch4_chp * GWP_CH4;

    let ch4_emissions = ch4_sewage_treatment
        + ch4_sludge_storage_containers
        + ch4_sludge_bags
        + ch4_water
        + ch4_combined_heat_and_power_plant;

    let mut external_energy = total_power_consumption - on_site_power_generation;

    if external_energy.is_sign_negative() {
        external_energy = Kilowatthours::zero();
    }

    let excess_energy_co2_equivalent = if external_energy.is_sign_negative() {
        external_energy * emission_factor_electricity_mix * Factor::new(-1.0)
    } else {
        Grams::zero()
    }
    .convert_to::<Tons>();

    let electricity_mix = (external_energy * emission_factor_electricity_mix).convert_to::<Tons>();
    let synthetic_polymers = synthetic_polymers * EMISSION_FACTOR_POLYMERS;
    let fecl3 = fecl3 * EMISSION_FACTOR_FECL3;
    let feclso4 = feclso4 * EMISSION_FACTOR_FECLSO4;
    let caoh2 = caoh2 * EMISSION_FACTOR_CAOH2;

    let operating_materials = synthetic_polymers + feclso4 + caoh2 + fecl3;

    let sewage_sludge_transport = (sewage_sludge_for_disposal
        * FUEL_CONSUMPTION
        * transport_distance
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

    // -------    ------ //
    //   Pack variables  //
    // -------    ------ //

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

    (co2_equivalents, emission_factors, calc_methods)
}

#[must_use]
pub fn calculate_ch4_slippage_sludge_bags(
    digester_count: Option<u64>,
    methane_fraction: Percent,
) -> Tons {
    let count = Factor::new(digester_count.unwrap_or(0) as f64);
    let hours_per_year = Years::new(1.0).convert_to::<Hours>();
    let kilograms = EMISSION_FACTOR_SLUDGE_BAGS
        * hours_per_year
        * count
        * methane_fraction
        * GWP_CH4
        * CONVERSION_FACTOR_CH4_M3_TO_KG;
    kilograms.convert_to()
}

#[must_use]
pub fn calculate_ch4_slippage_sludge_storage(
    sewage_gas_produced: Qubicmeters,
    methane_fraction: Percent,
) -> Tons {
    let volume = sewage_gas_produced * methane_fraction * EMISSION_FACTOR_SLUDGE_STORAGE;
    let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
    mass.convert_to()
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
    let n_elim = (nitrogen_influent - nitrogen_effluent) / nitrogen_influent;
    let ef = Percent::new(-0.049 * n_elim * 100.0 + 4.553);
    if ef.is_sign_negative() {
        Factor::new(0.002)
    } else {
        ef.convert_to::<Factor>()
    }
}

#[must_use]
pub fn calculate_nitrous_oxide(
    nitrogen_influent: MilligramsPerLiter,
    nitrogen_effluent: MilligramsPerLiter,
    wastewater: Qubicmeters,
    n2o_emission_factor: Factor,
) -> (Tons, Tons) {
    let n2o_anlage =
        wastewater * nitrogen_influent * n2o_emission_factor * CONVERSION_FACTOR_N_TO_N2O;
    let n2o_gewaesser =
        nitrogen_effluent * wastewater * EMISSION_FACTOR_N2O_WATER * CONVERSION_FACTOR_N_TO_N2O;
    (
        n2o_anlage.convert_to::<Tons>(),
        n2o_gewaesser.convert_to::<Tons>(),
    )
}
