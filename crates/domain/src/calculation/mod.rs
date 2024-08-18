use std::collections::HashMap;

#[cfg(test)]
mod tests;

use klick_value::{
    constants::*,
    extract_optional_with_input_value_id as extract_optional,
    extract_required_with_input_value_id as extract_required,
    specs::{InputValueId as Id, MissingInputValueIdError, OutputValueId as Out},
    units::*,
};

#[allow(clippy::wildcard_imports)]
use crate::{EmissionsCalculationOutcome, Value as V};

mod emission_groups;

use self::emission_groups::calculate_emission_groups;

#[must_use]
#[allow(clippy::too_many_lines)] // TODO
pub fn calculate_emissions(
    input: &HashMap<Id, Value>,
) -> Result<EmissionsCalculationOutcome, MissingInputValueIdError> {
    // -------    ------ //
    //  Unpack variables //
    // -------    ------ //

    let from = input; // FIXME

    let population_equivalent = extract_required!(Id::PopulationEquivalent, &from)?;
    let wastewater = extract_required!(Id::Wastewater, &from)?;
    let influent_nitrogen = extract_required!(Id::InfluentNitrogen, &from)?;
    let influent_chemical_oxygen_demand =
        extract_required!(Id::InfluentChemicalOxygenDemand, &from)?;
    let influent_total_organic_carbohydrates =
        extract_required!(Id::InfluentTotalOrganicCarbohydrates, &from)?;

    let chemical_oxygen_demand_influent = influent_chemical_oxygen_demand;
    let nitrogen_influent = influent_nitrogen;
    let total_organic_carbohydrates = influent_total_organic_carbohydrates;

    let effluent_nitrogen = extract_required!(Id::EffluentNitrogen, &from)?;
    let effluent_chemical_oxygen_demand =
        extract_required!(Id::EffluentChemicalOxygenDemand, &from)?;

    let nitrogen_effluent = effluent_nitrogen;
    let chemical_oxygen_demand_effluent = effluent_chemical_oxygen_demand;

    let sewage_gas_produced = extract_required!(Id::SewageGasProduced, &from)?;
    let methane_fraction = extract_required!(Id::MethaneFraction, &from)?;
    let total_power_consumption = extract_required!(Id::TotalPowerConsumption, &from)?;
    let on_site_power_generation = extract_required!(Id::OnSitePowerGeneration, &from)?;
    let emission_factor_electricity_mix =
        extract_required!(Id::EmissionFactorElectricityMix, &from)?;
    let heating_oil = extract_required!(Id::HeatingOil, &from)?;
    let gas_supply = extract_required!(Id::GasSupply, &from)?;
    let purchase_of_biogas = extract_required!(Id::PurchaseOfBiogas, &from)?;

    let sludge_bags_are_open = extract_required!(Id::SludgeTreatmentBagsAreOpen, &from)?;
    let sludge_bags_factor = extract_optional!(Id::SensitivitySludgeBagsCustomFactor, &from);

    let sludge_storage_containers_are_open =
        extract_required!(Id::SludgeTreatmentStorageContainersAreOpen, &from)?;
    let sludge_storage_containers_factor =
        extract_optional!(Id::SensitivitySludgeStorageCustomFactor, &from);
    let sewage_sludge_for_disposal = extract_required!(Id::SludgeTreatmentDisposal, &from)?;
    let transport_distance = extract_required!(Id::SludgeTreatmentTransportDistance, &from)?;
    let digester_count = extract_required!(Id::SludgeTreatmentDigesterCount, &from)?;

    let side_stream_treatment_total_nitrogen =
        extract_required!(Id::SideStreamTreatmentTotalNitrogen, &from)?;
    let total_nitrogen = side_stream_treatment_total_nitrogen;

    let side_stream_cover_is_open = extract_required!(Id::ScenarioN2OSideStreamCoverIsOpen, &from)?;

    let operating_material_fecl3 = extract_required!(Id::OperatingMaterialFeCl3, &from)?;
    let operating_material_feclso4 = extract_required!(Id::OperatingMaterialFeClSO4, &from)?;
    let operating_material_caoh2 = extract_required!(Id::OperatingMaterialCaOH2, &from)?;
    let operating_material_synthetic_polymers =
        extract_required!(Id::OperatingMaterialSyntheticPolymers, &from)?;

    let fecl3 = operating_material_fecl3;
    let feclso4 = operating_material_feclso4;
    let caoh2 = operating_material_caoh2;
    let synthetic_polymers = operating_material_synthetic_polymers;

    let emission_factor_n2o_side_stream =
        extract_required!(Id::SensitivityN2OSideStreamFactor, &from)?.convert_to::<Factor>();
    let emission_factor_co2_fossil =
        extract_required!(Id::SensitivityCO2FossilCustomFactor, &from)?.convert_to::<Factor>();

    let n2o_side_stream = emission_factor_n2o_side_stream;
    let co2_fossil = emission_factor_co2_fossil;

    let process_energy_savings = extract_required!(Id::ScenarioProcessEnergySaving, &from)?;
    let fossil_energy_savings = extract_required!(Id::ScenarioFossilEnergySaving, &from)?;
    let district_heating = extract_required!(Id::ScenarioDistrictHeating, &from)?;
    let photovoltaic_energy_expansion =
        extract_required!(Id::ScenarioPhotovoltaicEnergyExpansion, &from)?;
    let estimated_self_photovoltaic_usage =
        extract_required!(Id::ScenarioEstimatedSelfPhotovolaticUsage, &from)?;
    let wind_energy_expansion = extract_required!(Id::ScenarioWindEnergyExpansion, &from)?;
    let estimated_self_wind_energy_usage =
        extract_required!(Id::ScenarioEstimatedSelfWindEnergyUsage, &from)?;
    let water_energy_expansion = extract_required!(Id::ScenarioWaterEnergyExpansion, &from)?;
    let estimated_self_water_energy_usage =
        extract_required!(Id::ScenarioEstimatedSelfWaterEnergyUsage, &from)?;

    let n2o_calculation_method = extract_required!(Id::SensitivityN2OCalculationMethod, from)?;
    let n2o_custom_factor = extract_optional!(Id::SensitivityN2OCustomFactor, from);

    // FIXME:
    // The default method does not produce the expected outcome.
    // Also have a look at the  `calculation_method` function.
    // It looks like `MicroGasTurbines` should be the default instead of `GasolineEngine`;
    let ch4_chp_calculation_method = from
        .get(&Id::SensitivityCH4ChpCalculationMethod)
        .cloned()
        .map(V::as_ch4_chp_emission_factor_calc_method_unchecked);
    let ch4_chp_custom_factor = extract_optional!(Id::SensitivityCH4ChpCustomFactor, from);

    // -------    ------ //
    //     Calculate     //
    // -------    ------ //

    let n2o_emission_factor = calculate_n2o_emission_factor(
        n2o_calculation_method,
        n2o_custom_factor,
        nitrogen_influent,
        nitrogen_effluent,
    );
    debug_assert!(nitrogen_influent > MilligramsPerLiter::new(0.1));

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

    let with_digestion =
        sewage_gas_produced > Qubicmeters::new(0.001) && digester_count > Count::zero();

    let ch4_sludge_storage_containers = if with_digestion {
        ch4_slippage_sludge_storage * GWP_CH4
    } else {
        Tons::zero()
    };

    let ch4_sludge_bags = if with_digestion {
        ch4_slippage_sludge_bags * GWP_CH4
    } else {
        Tons::zero()
    };

    let ch4_water = ch4_water.convert_to::<Tons>() * GWP_CH4;

    let (ch4_chp, ch4_emission_factor) = if with_digestion {
        calculate_ch4_chp(
            ch4_chp_calculation_method,
            ch4_chp_custom_factor,
            sewage_gas_produced,
            methane_fraction,
        )
    } else {
        (Tons::zero(), Factor::zero())
    };

    let ch4_plant = if with_digestion {
        Tons::zero()
    } else {
        calculate_ch4_plant(population_equivalent)
    };

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

    let sewage_sludge_transport = (sewage_sludge_for_disposal
        * FUEL_CONSUMPTION
        * transport_distance
        * EMISSION_FACTOR_DIESEL)
        .convert_to();

    let process_energy_savings = calculate_process_energy_savings(
        external_energy,
        process_energy_savings,
        emission_factor_electricity_mix,
    );

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

    let energy_savings = process_energy_savings
        + photovoltaic_expansion_savings
        + wind_expansion_savings
        + water_expansion_savings
        + district_heating_savings;

    let mut electricity_mix =
        (external_energy * emission_factor_electricity_mix).convert_to::<Tons>() - energy_savings;

    if electricity_mix.is_sign_negative() {
        electricity_mix = Tons::zero();
    }

    // -------    ------ //
    //   Pack variables  //
    // -------    ------ //

    let co2_equivalents = [
        (Out::N2oPlant, n2o_plant),
        (Out::N2oWater, n2o_water),
        (Out::N2oSideStream, n2o_side_stream),
        (Out::Ch4Plant, ch4_plant),
        (
            Out::Ch4SludgeStorageContainers,
            ch4_sludge_storage_containers,
        ),
        (Out::Ch4SludgeBags, ch4_sludge_bags),
        (Out::Ch4Water, ch4_water),
        (Out::Ch4CombinedHeatAndPowerPlant, ch4_chp),
        (Out::FossilEmissions, fossil_emissions),
        (Out::Fecl3, fecl3),
        (Out::Feclso4, feclso4),
        (Out::Caoh2, caoh2),
        (Out::SyntheticPolymers, synthetic_polymers),
        (Out::ElectricityMix, electricity_mix),
        (Out::OilEmissions, oil_emissions_with_savings_applied),
        (Out::GasEmissions, gas_emissions_with_savings_applied),
        (Out::SewageSludgeTransport, sewage_sludge_transport),
        (Out::ProcessEnergySavings, process_energy_savings),
        (
            Out::PhotovoltaicExpansionSavings,
            photovoltaic_expansion_savings,
        ),
        (Out::WindExpansionSavings, wind_expansion_savings),
        (Out::WaterExpansionSavings, water_expansion_savings),
        (Out::DistrictHeatingSavings, district_heating_savings),
        (Out::FossilEnergySavings, fossil_energy_savings_emissions),
        (Out::ExcessEnergyCo2Equivalent, excess_energy_co2_equivalent),
    ]
    .into_iter()
    .collect();

    let co2_equivalents = calculate_emission_groups(co2_equivalents, emission_groups::SANKEY_EDGES);

    let emission_factors = [
        (Out::N2oCalculatedEmissionFactor, n2o_emission_factor),
        (Out::Ch4ChpCalculatedEmissionFactor, ch4_emission_factor),
    ]
    .into_iter()
    .collect();

    let calculation_methods = [
        (
            Out::N2oEmissionFactorCalcMethod,
            Some(Value::n2o_emission_factor_calc_method(
                n2o_calculation_method,
            )),
        ),
        (
            Out::Ch4ChpEmissionFactorCalcMethod,
            ch4_chp_calculation_method.map(Value::ch4_chp_emission_factor_calc_method),
        ),
        (
            Out::N2oEmissionCustomFactor,
            n2o_custom_factor
                .map(|v| v.convert_to::<Factor>())
                .map(Into::into),
        ),
        (
            Out::Ch4ChpEmissionCustomFactor,
            ch4_chp_custom_factor
                .map(|v| v.convert_to::<Factor>())
                .map(Into::into),
        ),
    ]
    .into_iter()
    .filter_map(|(id, value)| value.map(|v| (id, v)))
    .collect();

    Ok(EmissionsCalculationOutcome {
        co2_equivalents,
        emission_factors,
        calculation_methods,
    })
}

#[must_use]
pub fn calculate_ch4_slippage_sludge_bags(
    digester_count: Count,
    methane_fraction: Percent,
    sludge_bags_factor: Option<QubicmetersPerHour>,
) -> Tons {
    #[allow(clippy::cast_precision_loss)]
    let count = Factor::new(u64::from(digester_count) as f64);

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
    custom_factor: Option<Percent>,
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
        N2oEmissionFactorCalcMethod::Custom => custom_factor.expect("custom N2O EF").into(),
    }
}

#[must_use]
pub fn extrapolate_according_to_tu_wien_2016(
    nitrogen_influent: MilligramsPerLiter,
    nitrogen_effluent: MilligramsPerLiter,
) -> Factor {
    let n_elim = Factor::new(1.0) - (nitrogen_effluent / nitrogen_influent);
    let ef = Percent::new(-0.047 * n_elim * 100.0 + 4.362);
    if ef.is_sign_negative() {
        Factor::new(0.0)
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
pub fn calculate_ch4_plant(population_equivalent: Count) -> Tons {
    Grams::new(u64::from(population_equivalent) as f64 * EMISSION_FACTOR_CH4_PLANT * GWP_CH4)
        .convert_to::<Tons>()
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
    external_energy: GramsPerKilowatthour,
) -> Tons {
    (total_power_consumption * process_energy_savings * external_energy).convert_to::<Tons>()
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
    values: &HashMap<Id, Value>,
) -> anyhow::Result<Vec<(N2oEmissionFactorCalcMethod, EmissionsCalculationOutcome)>> {
    let mut values = values.clone();
    let id = Id::SensitivityN2OCalculationMethod;

    //let ch4 = ch4_chp_calc_method;

    // TuWien2016
    let n2o = N2oEmissionFactorCalcMethod::TuWien2016;
    values.insert(id, V::n2o_emission_factor_calc_method(n2o));
    let result = calculate_emissions(&values)?;
    let tuwien2016_result = (n2o, result);

    // Optimistic
    let n2o = N2oEmissionFactorCalcMethod::Optimistic;
    values.insert(id, V::n2o_emission_factor_calc_method(n2o));
    let result = calculate_emissions(&values)?;
    let optimistc_result = (n2o, result);

    // Pesimistic
    let n2o = N2oEmissionFactorCalcMethod::Pesimistic;
    values.insert(id, V::n2o_emission_factor_calc_method(n2o));
    let result = calculate_emissions(&values)?;
    let pesimistic_result = (n2o, result);

    // Ipcc2019
    let n2o = N2oEmissionFactorCalcMethod::Ipcc2019;
    values.insert(id, V::n2o_emission_factor_calc_method(n2o));
    let result = calculate_emissions(&values)?;
    let ipcc2019_result = (n2o, result);

    let mut results = vec![
        tuwien2016_result,
        optimistc_result,
        pesimistic_result,
        ipcc2019_result,
    ];

    if values.get(&Id::SensitivityN2OCustomFactor).is_none() {
        return Ok(results);
    };

    // Custom
    let n2o = N2oEmissionFactorCalcMethod::Custom;
    values.insert(id, V::n2o_emission_factor_calc_method(n2o));
    let result = calculate_emissions(&values)?;
    let custom_result = (n2o, result);
    results.push(custom_result);

    Ok(results)
}

#[must_use]
pub fn calculate_ch4_chp(
    calculation_method: Option<Ch4ChpEmissionFactorCalcMethod>,
    custom_factor: Option<Percent>,
    sewage_gas_produced: Qubicmeters,
    methane_fraction: Percent,
) -> (Tons, Factor) {
    let ch4_emission_factor = match calculation_method {
        Some(Ch4ChpEmissionFactorCalcMethod::MicroGasTurbines) | None => Factor::new(0.01),
        Some(Ch4ChpEmissionFactorCalcMethod::GasolineEngine) => Factor::new(0.015),
        Some(Ch4ChpEmissionFactorCalcMethod::JetEngine) => Factor::new(0.025),
        Some(Ch4ChpEmissionFactorCalcMethod::Custom) => {
            custom_factor.expect("custom CH4 EF").into()
        }
    };

    let volume = sewage_gas_produced * methane_fraction * ch4_emission_factor;
    let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
    let ch4_chp = mass.convert_to::<Tons>();

    (ch4_chp * GWP_CH4, ch4_emission_factor)
}

const CH4_CHP_CALC_METHODS: [Ch4ChpEmissionFactorCalcMethod; 3] = [
    Ch4ChpEmissionFactorCalcMethod::MicroGasTurbines,
    Ch4ChpEmissionFactorCalcMethod::GasolineEngine,
    Ch4ChpEmissionFactorCalcMethod::JetEngine,
];

#[must_use]
pub fn calculate_all_ch4_chp_emission_factor_scenarios(
    sewage_gas_produced: Qubicmeters,
    methane_fraction: Percent,
    custom_factor: Option<Percent>,
) -> Vec<(Ch4ChpEmissionFactorCalcMethod, Tons, Factor)> {
    let mut results = CH4_CHP_CALC_METHODS
        .into_iter()
        .map(|method| {
            let (result, factor) =
                calculate_ch4_chp(Some(method), None, sewage_gas_produced, methane_fraction);
            (method, result, factor)
        })
        .collect();

    let Some(factor) = custom_factor else {
        return results;
    };

    // Custom
    let method = Ch4ChpEmissionFactorCalcMethod::Custom;
    let (result, factor) = calculate_ch4_chp(
        Some(method),
        Some(factor.into()), // TODO: avoid conversion
        sewage_gas_produced,
        methane_fraction,
    );
    results.push((method, result, factor));

    results
}
