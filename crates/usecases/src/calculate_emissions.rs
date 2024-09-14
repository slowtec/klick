use std::{collections::HashMap, hash::BuildHasher};

use klick_domain::{
    constants::*, optional_input_value_id as optional, required_input_value_id as required,
    units::*, InputValueId as In, OutputValueId as Out, Value as V, ValueId as Id,
};

use crate::{
    calculate_all_ch4_chp_emission_factor_scenarios, calculate_all_n2o_emission_factor_scenarios,
    calculate_ch4_chp,
};

pub type Edge = (Id, Id);
pub type Edges = Vec<Edge>;
pub type Values = HashMap<Id, Value>;

#[derive(Debug, Clone, PartialEq)]
pub struct CalculationOutcome {
    pub input: Values,
    pub output: Option<Values>,
    pub graph: Option<Edges>,

    // Used to create bar chart input
    pub sensitivity_n2o_calculations: Option<Vec<(N2oEmissionFactorCalcMethod, Values)>>,

    // Used to create bar chart input
    pub sensitivity_ch4_chp_calculations:
        Option<Vec<(Ch4ChpEmissionFactorCalcMethod, Tons, Factor)>>,
}

#[must_use]
pub fn calculate_emissions(
    input: &HashMap<Id, Value>,
    custom_edges: Option<&[(Id, Id)]>,
    custom_leafs: Vec<Id>,
) -> CalculationOutcome {
    log::debug!("Calculate");

    let mut calc_output = calculate(input, custom_edges)
        .map_err(|err| log::warn!("{err}"))
        .ok();

    let custom_sum: Option<Tons> = calc_output.clone().map(|(values, _)| {
        values
            .iter()
            .filter_map(|(id, value)| {
                if id.is_custom() && custom_leafs.iter().any(|x| x == id) {
                    let v = value.clone().as_tons().unwrap_or_else(|| Tons::new(0.0));
                    Some(v)
                } else {
                    None
                }
            })
            .fold(Tons::new(0.0), |acc, element| acc + element)
    });

    if let Some((values, _)) = &mut calc_output {
        if let Some(sum) = custom_sum {
            values.insert(Id::Out(Out::AdditionalCustomEmissions), Value::from(sum));
        }
    }

    log::debug!("Calculate all N2O emission factor scenarios");
    let maybe_graph = calc_output.clone().map(|(_, graph)| graph).clone();

    let sensitivity_n2o_calculations =
        calculate_all_n2o_emission_factor_scenarios(input, maybe_graph.as_deref())
            .ok()
            .map(|results| {
                results
                    .into_iter()
                    .map(|(method, (values, _))| (method, values))
                    .collect()
            });

    let sensitivity_ch4_chp_calculations = {
        log::debug!("Calculate all CH4 CHP emission factor scenarios");

        let sewage_gas_produced = required!(In::ProfileSewageGasProduced, &input).unwrap();
        let methane_fraction = required!(In::ProfileMethaneFraction, &input).unwrap();
        let custom_ch4_chp_emission_factor = optional!(In::SensitivityCH4ChpCustomFactor, &input);
        let results = calculate_all_ch4_chp_emission_factor_scenarios(
            sewage_gas_produced,
            methane_fraction,
            custom_ch4_chp_emission_factor,
        );
        Some(results)
    };

    let input = input.clone();
    let output = calc_output.as_ref().map(|(values, _)| values.clone());
    let graph = calc_output.as_ref().map(|(_, graph)| graph.clone());

    log::debug!("Calculation finished");

    CalculationOutcome {
        input,
        output,
        graph,
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
    }
}

// TODO: rename function
pub fn calculate(
    values: &Values,
    custom_edges: Option<&[Edge]>,
) -> anyhow::Result<(Values, Edges)> {
    let input_values = extract_input_values(values).collect();
    let custom_values = extract_custom_emission_values(values);
    let (emissions, factors, methods) = emissions_factors_and_methods(&input_values)?;

    let all_emission_values = emissions
        .into_iter()
        .map(|(id, value)| (id.into(), value))
        .chain(custom_values)
        .collect();

    let graph = emission_graph(custom_edges);

    let co2_equivalents =
        crate::emission_groups::calculate_emission_groups(all_emission_values, &graph);

    let co2_values = co2_equivalents
        .into_iter()
        .map(|(id, v)| (id, Value::from(v)))
        .chain(
            factors
                .into_iter()
                .map(|(id, v)| (id.into(), Value::from(v))),
        )
        .chain(methods.into_iter().map(|(id, v)| (id.into(), v)))
        .collect();

    Ok((co2_values, graph))
}

/// `calculate_emission_groups` assumes leaft to root node ordering for building the sums
/// this also removes edges which don't end up in `target`, which is usually `Out::TotalEmissions`
#[must_use]
pub fn sort_and_filter_nodes(edges: Vec<(Id, Id)>, target: Id) -> Vec<(Id, Id)> {
    let mut targets: Vec<Id> = vec![target];
    let mut ret: Vec<(Id, Id)> = vec![];
    let mut next_edges: Vec<(Id, Id)>;
    loop {
        next_edges = edges
            .clone()
            .into_iter()
            .filter(|(_, target)| targets.iter().any(|t| t == target))
            .collect::<Vec<_>>();
        let next_targets: Vec<Id> = next_edges
            .iter()
            .map(|(source, _)| source.clone())
            .collect::<Vec<_>>();
        ret.append(&mut next_edges);
        if next_targets.is_empty() {
            break;
        }
        targets = next_targets;
    }
    ret.reverse();
    ret
}

#[must_use]
fn emission_graph(custom_edges: Option<&[Edge]>) -> Edges {
    let mut edges = crate::emission_groups::SANKEY_EDGES
        .iter()
        .map(|(from, to)| (Id::from(*from), Id::from(*to)))
        .collect::<Vec<_>>();
    if let Some(custom_edges) = custom_edges {
        edges.extend(
            custom_edges
                .iter()
                .map(|(from, to)| (from.clone(), to.clone())),
        );
    }
    sort_and_filter_nodes(edges, Id::Out(Out::TotalEmissions))
}

pub fn extract_input_values<T, S>(values: &HashMap<Id, T, S>) -> impl Iterator<Item = (In, T)> + '_
where
    T: Clone,
    S: BuildHasher,
{
    values.iter().filter_map(|(id, value)| {
        let Id::In(id) = id else { return None };
        Some((*id, value.clone())) // TODO: avoid clone
    })
}

fn extract_custom_emission_values(
    values: &HashMap<Id, Value>,
) -> impl Iterator<Item = (Id, Tons)> + '_ {
    values
        .iter()
        .filter(|(id, _)| id.is_custom())
        .filter_map(|(id, value)| {
            // Currently we assume all custom emisson values are of type `Tons`.
            value
                .clone() // TODO: avoid clone
                .as_tons()
                .map(|v| (id.clone(), v)) // TODO: avoid clone
        })
}

fn emissions_factors_and_methods(
    input: &HashMap<In, Value>,
) -> anyhow::Result<(Vec<(Out, Tons)>, Vec<(Out, Factor)>, Vec<(Out, Value)>)> {
    // -------    ------ //
    //  Unpack variables //
    // -------    ------ //

    let from = input; // FIXME

    let population_equivalent = required!(In::ProfilePopulationEquivalent, &from)?;
    let wastewater = required!(In::ProfileWastewater, &from)?;
    let influent_nitrogen = required!(In::ProfileInfluentNitrogen, &from)?;
    let influent_chemical_oxygen_demand =
        required!(In::ProfileInfluentChemicalOxygenDemand, &from)?;
    let influent_total_organic_carbohydrates =
        required!(In::ProfileInfluentTotalOrganicCarbohydrates, &from)?;

    let chemical_oxygen_demand_influent = influent_chemical_oxygen_demand;
    let nitrogen_influent = influent_nitrogen;
    let total_organic_carbohydrates = influent_total_organic_carbohydrates;

    let effluent_nitrogen = required!(In::ProfileEffluentNitrogen, &from)?;
    let effluent_chemical_oxygen_demand =
        required!(In::ProfileEffluentChemicalOxygenDemand, &from)?;

    let nitrogen_effluent = effluent_nitrogen;
    let chemical_oxygen_demand_effluent = effluent_chemical_oxygen_demand;

    let sewage_gas_produced = required!(In::ProfileSewageGasProduced, &from)?;
    let methane_fraction = required!(In::ProfileMethaneFraction, &from)?;
    let total_power_consumption = required!(In::ProfileTotalPowerConsumption, &from)?;
    let on_site_power_generation = required!(In::ProfileOnSitePowerGeneration, &from)?;
    let emission_factor_electricity_mix =
        required!(In::ProfileEmissionFactorElectricityMix, &from)?;
    let heating_oil = required!(In::ProfileHeatingOil, &from)?;
    let gas_supply = required!(In::ProfileGasSupply, &from)?;
    let purchase_of_biogas = required!(In::ProfilePurchaseOfBiogas, &from)?;

    let sludge_bags_are_open_recommendation =
        required!(In::RecommendationSludgeBagsAreOpen, &from)?;
    let sludge_bags_are_open_profile = required!(In::ProfileSludgeBagsAreOpen, &from)?;

    let sludge_bags_factor = optional!(In::SensitivitySludgeBagsCustomFactor, &from);

    let sludge_storage_containers_are_open_recommendation =
        required!(In::RecommendationSludgeStorageContainersAreOpen, &from)?;
    let sludge_storage_containers_are_open_profile =
        required!(In::ProfileSludgeStorageContainersAreOpen, &from)?;
    let sludge_storage_containers_factor =
        optional!(In::SensitivitySludgeStorageCustomFactor, &from);
    let sewage_sludge_for_disposal = required!(In::ProfileSludgeDisposal, &from)?;
    let transport_distance = required!(In::ProfileSludgeTransportDistance, &from)?;
    let digester_count = required!(In::ProfileSludgeDigesterCount, &from)?;

    let side_stream_treatment_total_nitrogen =
        required!(In::ProfileSideStreamTotalNitrogen, &from)?;
    let total_nitrogen = side_stream_treatment_total_nitrogen;

    let side_stream_cover_is_open = required!(In::RecommendationN2OSideStreamCoverIsOpen, &from)?;

    let operating_material_fecl3 = required!(In::ProfileOperatingMaterialFeCl3, &from)?;
    let operating_material_feclso4 = required!(In::ProfileOperatingMaterialFeClSO4, &from)?;
    let operating_material_caoh2 = required!(In::ProfileOperatingMaterialCaOH2, &from)?;
    let operating_material_synthetic_polymers =
        required!(In::ProfileOperatingMaterialSyntheticPolymers, &from)?;

    let fecl3 = operating_material_fecl3;
    let feclso4 = operating_material_feclso4;
    let caoh2 = operating_material_caoh2;
    let synthetic_polymers = operating_material_synthetic_polymers;

    let emission_factor_n2o_side_stream =
        required!(In::SensitivityN2OSideStreamFactor, &from)?.convert_to::<Factor>();
    let emission_factor_co2_fossil =
        required!(In::SensitivityCO2FossilCustomFactor, &from)?.convert_to::<Factor>();

    let n2o_side_stream = emission_factor_n2o_side_stream;
    let co2_fossil = emission_factor_co2_fossil;

    let process_energy_savings = required!(In::RecommendationProcessEnergySaving, &from)?;
    let fossil_energy_savings = required!(In::RecommendationFossilEnergySaving, &from)?;
    let district_heating = required!(In::RecommendationDistrictHeating, &from)?;
    let photovoltaic_energy_expansion =
        required!(In::RecommendationPhotovoltaicEnergyExpansion, &from)?;
    let estimated_self_photovoltaic_usage =
        required!(In::RecommendationEstimatedSelfPhotovolaticUsage, &from)?;
    let wind_energy_expansion = required!(In::RecommendationWindEnergyExpansion, &from)?;
    let estimated_self_wind_energy_usage =
        required!(In::RecommendationEstimatedSelfWindEnergyUsage, &from)?;
    let water_energy_expansion = required!(In::RecommendationWaterEnergyExpansion, &from)?;
    let estimated_self_water_energy_usage =
        required!(In::RecommendationEstimatedSelfWaterEnergyUsage, &from)?;

    let n2o_calculation_method = required!(In::SensitivityN2OCalculationMethod, from)?;
    let n2o_custom_factor = optional!(In::SensitivityN2OCustomFactor, from);

    // FIXME:
    // The default method does not produce the expected outcome.
    // Also have a look at the  `calculation_method` function.
    // It looks like `MicroGasTurbines` should be the default instead of `GasolineEngine`;
    let ch4_chp_calculation_method = from
        .get(&In::SensitivityCH4ChpCalculationMethod)
        .cloned()
        .map(V::as_ch4_chp_emission_factor_calc_method_unchecked);
    let ch4_chp_custom_factor = optional!(In::SensitivityCH4ChpCustomFactor, from);

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

    let ch4_slippage_sludge_bags =
        if sludge_bags_are_open_recommendation && sludge_bags_are_open_profile {
            calculate_ch4_slippage_sludge_bags(digester_count, methane_fraction, sludge_bags_factor)
        } else {
            Tons::zero()
        };

    let ch4_slippage_sludge_storage = if sludge_storage_containers_are_open_recommendation
        && sludge_storage_containers_are_open_profile
    {
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

    let district_heating_savings = (district_heating
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

    let values: [(Out, Tons); 24] = [
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
    ];

    let emission_factors = [
        (Out::N2oCalculatedEmissionFactor, n2o_emission_factor),
        (Out::Ch4ChpCalculatedEmissionFactor, ch4_emission_factor),
    ];

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
                .map(RatioExt::convert_to::<Factor>)
                .map(Into::into),
        ),
        (
            Out::Ch4ChpEmissionCustomFactor,
            ch4_chp_custom_factor
                .map(RatioExt::convert_to::<Factor>)
                .map(Into::into),
        ),
    ]
    .into_iter()
    .filter_map(|(id, value)| value.map(|v| (id, v)))
    .collect();

    Ok((
        values.to_vec(),
        emission_factors.to_vec(),
        calculation_methods,
    ))
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
#[allow(clippy::missing_panics_doc)]
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
