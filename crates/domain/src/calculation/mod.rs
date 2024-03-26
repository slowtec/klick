#[cfg(test)]
mod tests;

#[allow(clippy::wildcard_imports)]
use crate::{
    constants::*,
    units::{
        Factor, Grams, Hours, Kilowatthours, Liters, Mass, MilligramsPerLiter, Percent,
        Qubicmeters, QubicmetersPerHour, Ratio, Time, Tons, Years,
    },
    AnnualAverageEffluent, AnnualAverageInfluent, CH4ChpEmissionFactorCalcMethod, CO2Equivalents,
    CalculatedEmissionFactors, EmissionFactorCalculationMethods, EmissionFactors,
    EmissionInfluencingValues, EmissionsCalculationOutcome, EnergyConsumption,
    EnergyEmissionFactors, N2oEmissionFactorCalcMethod, OperatingMaterials, SewageSludgeTreatment,
    SideStreamTreatment,
};

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn calculate_emissions(
    input: EmissionInfluencingValues,
    calculation_methods: EmissionFactorCalculationMethods,
) -> EmissionsCalculationOutcome {
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
        side_stream_treatment,
        operating_materials,
        emission_factors,
        energy_emission_factors,
    } = input;

    let AnnualAverageInfluent {
        chemical_oxygen_demand: chemical_oxygen_demand_influent,
        nitrogen: nitrogen_influent,
        total_organic_carbohydrates,
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
        heating_oil,
        gas_supply,
        purchase_of_biogas,
    } = energy_consumption;

    let SewageSludgeTreatment {
        sludge_bags_are_open,
        sludge_bags_factor,
        sludge_storage_containers_are_open,
        sludge_storage_containers_factor,
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

    let SideStreamTreatment {
        total_nitrogen,
        side_stream_cover_is_open,
    } = side_stream_treatment;

    let EmissionFactors {
        n2o_side_stream,
        co2_fossil,
    } = emission_factors;

    let EnergyEmissionFactors {
        process_energy_savings,
        fossil_energy_savings,
        district_heating,
        photovoltaic_energy_expansion,
        estimated_self_photovoltaic_usage,
        wind_energy_expansion,
        estimated_self_wind_energy_usage,
        water_energy_expansion,
        estimated_self_water_energy_usage,
    } = energy_emission_factors;

    // -------    ------ //
    //     Calculate     //
    // -------    ------ //

    let n2o_emission_factor = calculate_n2o_emission_factor(
        calculation_methods.n2o,
        nitrogen_influent,
        nitrogen_effluent,
    );
    debug_assert!(n2o_emission_factor < Factor::new(1.0));

    let (n2o_plant, n2o_water) = calculate_nitrous_oxide(
        nitrogen_influent,
        nitrogen_effluent,
        wastewater,
        n2o_emission_factor,
    );

    let ch4_water = chemical_oxygen_demand_effluent * wastewater * EMISSION_FACTOR_CH4_WATER;

    let ch4_slippage_sludge_bags = if sludge_bags_are_open {
        calculate_ch4_slippage_sludge_bags(digester_count, methane_fraction, sludge_bags_factor)
    } else {
        Tons::zero()
    };

    let ch4_slippage_sludge_storage = if sludge_storage_containers_are_open {
        calculate_ch4_slippage_sludge_storage(
            sewage_gas_produced,
            methane_fraction,
            sludge_storage_containers_factor,
        )
    } else {
        Tons::zero()
    };

    let n2o_plant = n2o_plant * GWP_N2O;
    let n2o_water = n2o_water * GWP_N2O;

    let n2o_side_stream =
        calculate_n2o_side_stream(total_nitrogen, n2o_side_stream, side_stream_cover_is_open);

    let fossil_emissions = calculate_fossil_emissions(
        total_organic_carbohydrates,
        chemical_oxygen_demand_influent,
        co2_fossil,
        wastewater,
    );

    let n2o_emissions = n2o_plant + n2o_water + n2o_side_stream;

    let ch4_sludge_storage_containers = ch4_slippage_sludge_storage * GWP_CH4;
    let ch4_sludge_bags = ch4_slippage_sludge_bags * GWP_CH4;
    let ch4_water = ch4_water.convert_to::<Tons>() * GWP_CH4;

    let (ch4_combined_heat_and_power_plant, ch4_emission_factor) =
        calculate_ch4_combined_heat_and_power_plant(
            calculation_methods.ch4,
            sewage_gas_produced,
            methane_fraction,
        );

    let ch4_plant = calculate_ch4_plant(
        population_equivalent,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_combined_heat_and_power_plant,
    );

    let ch4_emissions = ch4_plant
        + ch4_sludge_storage_containers
        + ch4_sludge_bags
        + ch4_water
        + ch4_combined_heat_and_power_plant;

    let power_production_consumption_difference =
        total_power_consumption - on_site_power_generation;

    let excess_energy_co2_equivalent =
        if power_production_consumption_difference.is_sign_negative() {
            power_production_consumption_difference
                * emission_factor_electricity_mix
                * Factor::new(-1.0)
        } else {
            Grams::zero()
        }
        .convert_to::<Tons>();

    let external_energy = if power_production_consumption_difference.is_sign_negative() {
        Kilowatthours::zero()
    } else {
        power_production_consumption_difference
    };

    let oil_emissions = calculate_oil_emissions(heating_oil);
    let gas_emissions = calculate_gas_emissions(gas_supply, purchase_of_biogas);

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

    let direct_emissions = ch4_emissions + n2o_emissions + fossil_emissions;

    let process_energy_savings =
        calculate_process_energy_savings(total_power_consumption, process_energy_savings);
    let photovoltaic_expansion_savings = calculate_photovoltaic_expansion_savings(
        photovoltaic_energy_expansion,
        estimated_self_photovoltaic_usage,
    );
    let wind_expansion_savings =
        calculate_wind_expansion_savings(wind_energy_expansion, estimated_self_wind_energy_usage);
    let water_expansion_savings = calculate_water_expansion_savings(
        water_energy_expansion,
        estimated_self_water_energy_usage,
    );

    let district_heating_savings: Tons = (district_heating
        * (EMISSION_FACTOR_STROM_MIX - EMISSION_FACTOR_HEAT_NETWORK))
        .convert_to::<Tons>();

    let fossil_energy_savings_emissions =
        calculate_oil_gas_savings(oil_emissions, gas_emissions, fossil_energy_savings);

    let oil_emissions_with_savings_applied = oil_emissions - oil_emissions * fossil_energy_savings;
    let gas_emissions_with_savings_applied = gas_emissions - gas_emissions * fossil_energy_savings;

    let electricity_mix_helper =
        (external_energy * emission_factor_electricity_mix).convert_to::<Tons>();

    let electricity_mix = {
        let mix = electricity_mix_helper
            - process_energy_savings
            - photovoltaic_expansion_savings
            - wind_expansion_savings
            - water_expansion_savings
            - district_heating_savings;
        if mix < Tons::zero() {
            Tons::zero()
        } else {
            mix
        }
    };

    let indirect_emissions =
        electricity_mix + oil_emissions_with_savings_applied + gas_emissions_with_savings_applied;
    let other_indirect_emissions = operating_materials + sewage_sludge_transport;
    let total_emissions = direct_emissions + indirect_emissions + other_indirect_emissions;

    // -------    ------ //
    //   Pack variables  //
    // -------    ------ //

    let co2_equivalents = CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_side_stream,
        n2o_emissions,
        ch4_plant,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
        fossil_emissions,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        oil_emissions: oil_emissions_with_savings_applied,
        gas_emissions: gas_emissions_with_savings_applied,
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        process_energy_savings,
        photovoltaic_expansion_savings,
        wind_expansion_savings,
        water_expansion_savings,
        district_heating_savings,
        fossil_energy_savings: fossil_energy_savings_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent,
    };

    let emission_factors = CalculatedEmissionFactors {
        n2o: n2o_emission_factor,
        ch4: ch4_emission_factor,
    };

    EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        calculation_methods,
    }
}

#[must_use]
pub fn calculate_ch4_slippage_sludge_bags(
    digester_count: Option<u64>,
    methane_fraction: Percent,
    sludge_bags_factor: Option<QubicmetersPerHour>,
) -> Tons {
    let count = Factor::new(digester_count.unwrap_or(0) as f64);
    let hours_per_year = Years::new(1.0).convert_to::<Hours>();
    let sludge_bags_factor = sludge_bags_factor.unwrap_or(EMISSION_FACTOR_SLUDGE_BAGS);
    let kilograms = sludge_bags_factor
        * hours_per_year
        * count
        * methane_fraction
        * CONVERSION_FACTOR_CH4_M3_TO_KG;
    kilograms.convert_to()
}

#[must_use]
pub fn calculate_ch4_slippage_sludge_storage(
    sewage_gas_produced: Qubicmeters,
    methane_fraction: Percent,
    sludge_storage_containers_factor: Option<Percent>,
) -> Tons {
    let sludge_storage_containers_factor =
        sludge_storage_containers_factor.unwrap_or(EMISSION_FACTOR_SLUDGE_STORAGE);
    let volume = sewage_gas_produced * methane_fraction * sludge_storage_containers_factor;
    let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
    mass.convert_to()
}

#[must_use]
pub fn calculate_n2o_emission_factor(
    calculation_method: N2oEmissionFactorCalcMethod,
    nitrogen_influent: MilligramsPerLiter,
    nitrogen_effluent: MilligramsPerLiter,
) -> Factor {
    match calculation_method {
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

#[must_use]
pub fn calculate_fossil_emissions(
    total_organic_carbohydrates: MilligramsPerLiter,
    chemical_oxygen_demand_influent: MilligramsPerLiter,
    co2_fossil_emission_factor: Factor,
    wastewater: Qubicmeters,
) -> Tons {
    let base = if total_organic_carbohydrates > MilligramsPerLiter::new(0.01) {
        total_organic_carbohydrates
    } else {
        chemical_oxygen_demand_influent * CONVERSION_FACTOR_TOC_TO_COD
    };
    let emissions = base * co2_fossil_emission_factor * wastewater * CONVERSION_FACTOR_C_TO_CO2;
    emissions.convert_to()
}

#[must_use]
pub fn calculate_n2o_side_stream(
    total_nitrogen: Tons,
    n2o_side_stream_emission_factor: Factor,
    side_stream_cover_is_open: bool,
) -> Tons {
    if !side_stream_cover_is_open {
        return Tons::zero();
    }
    total_nitrogen * n2o_side_stream_emission_factor * CONVERSION_FACTOR_N_TO_N2O * GWP_N2O
}

#[must_use]
pub fn calculate_ch4_plant(
    population_equivalent: f64,
    ch4_sludge_storage_containers: Tons,
    ch4_sludge_bags: Tons,
    ch4_combined_heat_and_power_plant: Tons,
) -> Tons {
    let ch4_plant = Grams::new(population_equivalent * EMISSION_FACTOR_CH4_PLANT * GWP_CH4)
        .convert_to::<Tons>();
    let ch4_processes =
        ch4_sludge_storage_containers + ch4_sludge_bags + ch4_combined_heat_and_power_plant;
    if ch4_processes >= ch4_plant {
        Tons::zero()
    } else {
        ch4_plant - ch4_processes
    }
}

#[must_use]
pub fn calculate_oil_emissions(oil_supply: Liters) -> Tons {
    (oil_supply * EMISSION_FACTOR_OIL).convert_to::<Tons>()
}

#[must_use]
pub fn calculate_gas_emissions(gas_supply: Qubicmeters, purchase_of_biogas: bool) -> Tons {
    let ef_gas = if purchase_of_biogas {
        EMISSION_FACTOR_BIOGAS
    } else {
        EMISSION_FACTOR_GAS
    };
    (gas_supply * ef_gas).convert_to::<Tons>()
}

#[must_use]
pub fn calculate_process_energy_savings(
    total_power_consumption: Kilowatthours,
    process_energy_savings: Percent,
) -> Tons {
    (total_power_consumption * process_energy_savings * EMISSION_FACTOR_STROM_MIX)
        .convert_to::<Tons>()
}

#[must_use]
pub fn calculate_photovoltaic_expansion_savings(
    photovoltaic_energy_expansion: Kilowatthours,
    estimated_self_photovoltaic_usage: Percent,
) -> Tons {
    (photovoltaic_energy_expansion * estimated_self_photovoltaic_usage * EMISSION_FACTOR_STROM_MIX)
        .convert_to::<Tons>()
}

#[must_use]
pub fn calculate_wind_expansion_savings(
    wind_energy_expansion: Kilowatthours,
    estimated_self_wind_energy_usage: Percent,
) -> Tons {
    (wind_energy_expansion * estimated_self_wind_energy_usage * EMISSION_FACTOR_STROM_MIX)
        .convert_to::<Tons>()
}

#[must_use]
pub fn calculate_water_expansion_savings(
    water_energy_expansion: Kilowatthours,
    estimated_self_water_energy_usage: Percent,
) -> Tons {
    (water_energy_expansion * estimated_self_water_energy_usage * EMISSION_FACTOR_STROM_MIX)
        .convert_to::<Tons>()
}
#[must_use]
pub fn calculate_oil_gas_savings(
    oil_emissions: Tons,
    gas_emissions: Tons,
    fossil_energy_savings: Percent,
) -> Tons {
    (oil_emissions + gas_emissions) * fossil_energy_savings
}

#[must_use]
pub fn calculate_all_n2o_emission_factor_scenarios(
    values: &EmissionInfluencingValues,
    custom_factor: Option<Factor>,
    ch4_chp_calc_method: Option<CH4ChpEmissionFactorCalcMethod>,
) -> Vec<(N2oEmissionFactorCalcMethod, EmissionsCalculationOutcome)> {
    let ch4 = ch4_chp_calc_method;

    // TuWien2016
    let n2o = N2oEmissionFactorCalcMethod::TuWien2016;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let result = calculate_emissions(values.clone(), methods);
    let tuwien2016_result = (n2o, result);

    // Optimistic
    let n2o = N2oEmissionFactorCalcMethod::Optimistic;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let result = calculate_emissions(values.clone(), methods);
    let optimistc_result = (n2o, result);

    // Pesimistic
    let n2o = N2oEmissionFactorCalcMethod::Pesimistic;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let result = calculate_emissions(values.clone(), methods);
    let pesimistic_result = (n2o, result);

    // Ipcc2019
    let n2o = N2oEmissionFactorCalcMethod::Ipcc2019;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let result = calculate_emissions(values.clone(), methods);
    let ipcc2019_result = (n2o, result);

    let mut results = vec![
        tuwien2016_result,
        optimistc_result,
        pesimistic_result,
        ipcc2019_result,
    ];

    let Some(factor) = custom_factor else {
        return results;
    };

    // Custom
    let n2o = N2oEmissionFactorCalcMethod::Custom(factor);
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let result = calculate_emissions(values.clone(), methods);
    let custom_result = (n2o, result);
    results.push(custom_result);

    results
}

#[must_use]
pub fn calculate_ch4_combined_heat_and_power_plant(
    calculation_method: Option<CH4ChpEmissionFactorCalcMethod>,
    sewage_gas_produced: Qubicmeters,
    methane_fraction: Percent,
) -> (Tons, Factor) {
    let ch4_emission_factor = match calculation_method {
        None => Factor::new(0.01),
        Some(CH4ChpEmissionFactorCalcMethod::MicroGasTurbines) => Factor::new(0.01),
        Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine) => Factor::new(0.015),
        Some(CH4ChpEmissionFactorCalcMethod::JetEngine) => Factor::new(0.025),
        Some(CH4ChpEmissionFactorCalcMethod::Custom(f)) => f,
    };

    let volume = sewage_gas_produced * methane_fraction * ch4_emission_factor;
    let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
    let ch4_chp = mass.convert_to::<Tons>();

    (ch4_chp * GWP_CH4, ch4_emission_factor)
}

const CH4_CHP_CALC_METHODS: [CH4ChpEmissionFactorCalcMethod; 3] = [
    CH4ChpEmissionFactorCalcMethod::MicroGasTurbines,
    CH4ChpEmissionFactorCalcMethod::GasolineEngine,
    CH4ChpEmissionFactorCalcMethod::JetEngine,
];

#[must_use]
pub fn calculate_all_ch4_chp_emission_factor_scenarios(
    values: &EmissionInfluencingValues,
    custom_factor: Option<Factor>,
) -> Vec<(CH4ChpEmissionFactorCalcMethod, Tons, Factor)> {
    let EmissionInfluencingValues {
        energy_consumption:
            EnergyConsumption {
                sewage_gas_produced,
                methane_fraction,
                ..
            },
        ..
    } = values;

    let mut results = CH4_CHP_CALC_METHODS
        .into_iter()
        .map(|method| {
            let (result, factor) = calculate_ch4_combined_heat_and_power_plant(
                Some(method),
                *sewage_gas_produced,
                *methane_fraction,
            );
            (method, result, factor)
        })
        .collect();

    let Some(factor) = custom_factor else {
        return results;
    };

    // Custom
    let method = CH4ChpEmissionFactorCalcMethod::Custom(factor);
    let (result, factor) = calculate_ch4_combined_heat_and_power_plant(
        Some(method),
        *sewage_gas_produced,
        *methane_fraction,
    );
    results.push((method, result, factor));

    results
}
