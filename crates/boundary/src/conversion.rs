use anyhow::bail;

use klick_domain::{self as domain, units::*};

use crate::*;

// -----   ----- //
//    Project    //
// -----   ----- //

impl From<ProjectId> for domain::ProjectId {
    fn from(from: ProjectId) -> Self {
        Self::from_uuid(from.0)
    }
}

impl From<domain::ProjectId> for ProjectId {
    fn from(from: domain::ProjectId) -> Self {
        Self(from.to_uuid())
    }
}

impl From<SavedProject> for domain::Project<FormData> {
    fn from(from: SavedProject) -> Self {
        let SavedProject {
            id,
            created_at,
            modified_at,
            data,
        } = from;
        let id = domain::ProjectId::from(id);

        Self {
            id,
            created_at,
            modified_at,
            data,
        }
    }
}

impl From<domain::Project<FormData>> for SavedProject {
    fn from(from: domain::Project<FormData>) -> Self {
        let domain::Project {
            id,
            created_at,
            modified_at,
            data,
        } = from;

        let id = id.into();

        Self {
            id,
            created_at,
            modified_at,
            data,
        }
    }
}

impl From<domain::Project<FormData>> for Project {
    fn from(from: domain::Project<FormData>) -> Self {
        Self::Saved(from.into())
    }
}

// -----   ----- //
//    Values     //
// -----   ----- //

impl TryFrom<FormData>
    for (
        domain::EmissionInfluencingValues,
        domain::EmissionFactorCalculationMethods,
    )
{
    type Error = anyhow::Error;
    fn try_from(from: FormData) -> Result<Self, Self::Error> {
        let FormData {
            project_title: _,
            plant_profile,
            sensitivity_parameters,
            optimization_scenario,
        } = from;

        // -----   ----- //
        // Plant Profile //
        // -----   ----- //

        let PlantProfile {
            plant_name: _,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            side_stream_treatment,
            operating_materials,
        } = plant_profile;

        let Some(population_equivalent) = population_equivalent else {
            bail!("missing population_values");
        };

        let Some(wastewater) = wastewater else {
            bail!("missing wastewater");
        };

        let wastewater = Qubicmeters::new(wastewater);

        let influent_average = influent_average.try_into()?;
        let effluent_average = effluent_average.try_into()?;
        let energy_consumption = energy_consumption.try_into()?;

        let SewageSludgeTreatment {
            sludge_bags_are_closed,
            sludge_storage_containers_are_closed,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        } = sewage_sludge_treatment;

        let Some(sludge_bags_are_closed) = sludge_bags_are_closed else {
            bail!("missing sludge_bags_are_closed");
        };
        let Some(sludge_storage_containers_are_closed) = sludge_storage_containers_are_closed
        else {
            bail!("missing sludge_storage_containers_are_closed");
        };
        let Some(sewage_sludge_for_disposal) = sewage_sludge_for_disposal else {
            bail!("missing sewage_sludge_for_disposal");
        };
        let Some(transport_distance) = transport_distance else {
            bail!("missing transport_distance");
        };

        let sewage_sludge_for_disposal = Tons::new(sewage_sludge_for_disposal);
        let transport_distance = Kilometers::new(transport_distance);

        let mut sludge_storage_containers_are_open = !sludge_storage_containers_are_closed;
        let mut sludge_bags_are_open = !sludge_bags_are_closed;

        let SideStreamTreatment { total_nitrogen } = side_stream_treatment;

        let operating_materials = operating_materials.try_into()?;

        // -----   ----- //
        //  Sensitivity  //
        // -----   ----- //

        let SensitivityParameters {
            n2o_emissions,
            ch4_chp_emissions,
            ch4_sewage_sludge_emissions,
            co2_fossil_emissions,
        } = sensitivity_parameters;

        let (n2o, n2o_side_stream) = n2o_emissions.try_into()?;
        let ch4 = ch4_chp_emissions
            .try_into()
            .map_err(|err| {
                log::warn!("Missing CH4 CHP emission factor calculation method: {err}");
            })
            .ok();

        let emission_factor_calculation_methods =
            domain::EmissionFactorCalculationMethods { n2o, ch4 };

        let SewageSludgeTreatmentEmissionsSensitivity {
            emission_factor_sludge_bags,
            emission_factor_sludge_storage_containers,
        } = ch4_sewage_sludge_emissions;

        let sludge_bags_factor = emission_factor_sludge_bags.map(QubicmetersPerHour::new);
        let sludge_storage_containers_factor =
            emission_factor_sludge_storage_containers.map(Percent::new);

        let FossilEmissonsSensitivity { emission_factor } = co2_fossil_emissions;

        let Some(fossil_emission_factor) = emission_factor else {
            bail!("missing fossil emission_factor");
        };
        let fossil_emission_factor = Percent::new(fossil_emission_factor).convert_to();

        // -----   ----- //
        //  Optimization //
        // -----   ----- //

        let OptimizationScenario {
            sewage_sludge_treatment,
            energy_emissions,
            side_stream_treatment,
        } = optimization_scenario;

        let SewageSludgeTreatmentScenario {
            sludge_bags_are_closed,
            sludge_storage_containers_are_closed,
        } = sewage_sludge_treatment;

        if let Some(sludge_bags_are_closed) = sludge_bags_are_closed {
            sludge_bags_are_open = !sludge_bags_are_closed;
        }
        if let Some(sludge_storage_containers_are_closed) = sludge_storage_containers_are_closed {
            sludge_storage_containers_are_open = !sludge_storage_containers_are_closed;
        }

        let EnergyEmissionScenario {
            process_energy_savings,
            fossil_energy_savings,
            photovoltaic_energy_expansion,
            estimated_self_photovoltaic_usage,
            wind_energy_expansion,
            estimated_self_wind_energy_usage,
            water_energy_expansion,
            estimated_self_water_energy_usage,
            district_heating,
        } = energy_emissions;

        let Some(process_energy_savings) = process_energy_savings else {
            bail!("missing process_energy_savings")
        };
        let Some(fossil_energy_savings) = fossil_energy_savings else {
            bail!("missing fossil_energy_savings")
        };
        let Some(photovoltaic_energy_expansion) = photovoltaic_energy_expansion else {
            bail!("missing photovoltaic_energy_expansion")
        };
        let Some(estimated_self_photovoltaic_usage) = estimated_self_photovoltaic_usage else {
            bail!("missing estimated_self_photovoltaic_usage")
        };
        let Some(wind_energy_expansion) = wind_energy_expansion else {
            bail!("missing wind_energy_expansion")
        };
        let Some(estimated_self_wind_energy_usage) = estimated_self_wind_energy_usage else {
            bail!("missing estimated_self_wind_energy_usage")
        };
        let Some(water_energy_expansion) = water_energy_expansion else {
            bail!("missing water_energy_expansion")
        };
        let Some(estimated_self_water_energy_usage) = estimated_self_water_energy_usage else {
            bail!("missing estimated_self_water_energy_usage")
        };
        let Some(district_heating) = district_heating else {
            bail!("missing district_heating")
        };

        let process_energy_savings = Percent::new(process_energy_savings);
        let fossil_energy_savings = Percent::new(fossil_energy_savings);
        let district_heating = Kilowatthours::new(district_heating);
        let photovoltaic_energy_expansion = Kilowatthours::new(photovoltaic_energy_expansion);
        let estimated_self_photovoltaic_usage = Percent::new(estimated_self_photovoltaic_usage);
        let wind_energy_expansion = Kilowatthours::new(wind_energy_expansion);
        let estimated_self_wind_energy_usage = Percent::new(estimated_self_wind_energy_usage);
        let water_energy_expansion = Kilowatthours::new(water_energy_expansion);
        let estimated_self_water_energy_usage = Percent::new(estimated_self_water_energy_usage);

        let SideStreamTreatmentScenario {
            side_stream_cover_is_closed,
        } = side_stream_treatment;

        let Some(side_stream_cover_is_closed) = side_stream_cover_is_closed else {
            bail!("missing side_stream_cover_is_closed");
        };
        let Some(total_nitrogen) = total_nitrogen else {
            bail!("missing total_nitrogen");
        };
        let total_nitrogen = Tons::new(total_nitrogen);

        let side_stream_cover_is_open = !side_stream_cover_is_closed;

        // -----   ----- //
        //    Assemble   //
        // -----   ----- //

        let energy_emission_factors = domain::EnergyEmissionFactors {
            process_energy_savings,
            fossil_energy_savings,
            district_heating,
            photovoltaic_energy_expansion,
            estimated_self_photovoltaic_usage,
            wind_energy_expansion,
            estimated_self_wind_energy_usage,
            water_energy_expansion,
            estimated_self_water_energy_usage,
        };

        let side_stream_treatment = domain::SideStreamTreatment {
            total_nitrogen,
            side_stream_cover_is_open,
        };

        let emission_factors = domain::EmissionFactors {
            n2o_side_stream,
            co2_fossil: fossil_emission_factor,
        };

        let sewage_sludge_treatment = domain::SewageSludgeTreatment {
            sludge_bags_are_open,
            sludge_storage_containers_are_open,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
            sludge_bags_factor,
            sludge_storage_containers_factor,
        };

        let emission_influencing_values = domain::EmissionInfluencingValues {
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
        };

        Ok((
            emission_influencing_values,
            emission_factor_calculation_methods,
        ))
    }
}

impl TryFrom<N2OEmissionsSensitivity> for (domain::N2oEmissionFactorCalcMethod, Factor) {
    type Error = anyhow::Error;

    fn try_from(from: N2OEmissionsSensitivity) -> Result<Self, Self::Error> {
        use domain::N2oEmissionFactorCalcMethod as D;
        use N2oEmissionFactorCalcMethod as M;

        let N2OEmissionsSensitivity {
            calculation_method,
            custom_emission_factor,
            side_stream_emission_factor,
        } = from;

        let Some(method) = calculation_method else {
            bail!("N2O emission factor calculation method is missing");
        };

        let Some(side_stream_factor) = side_stream_emission_factor else {
            bail!("N2O sidestream factor is missing");
        };
        let side_stream_factor = Percent::new(side_stream_factor).convert_to();

        let method = match method {
            M::TuWien2016 => D::TuWien2016,
            M::Optimistic => D::Optimistic,
            M::Pesimistic => D::Pesimistic,
            M::Ipcc2019 => D::Ipcc2019,
            M::CustomFactor => {
                let Some(factor) = custom_emission_factor else {
                    bail!("custom N2O emission factor is missing");
                };
                D::Custom(Percent::new(factor).convert_to())
            }
        };
        Ok((method, side_stream_factor))
    }
}

impl TryFrom<CH4ChpEmissionsSensitivity> for domain::CH4ChpEmissionFactorCalcMethod {
    type Error = anyhow::Error;

    fn try_from(from: CH4ChpEmissionsSensitivity) -> Result<Self, Self::Error> {
        use domain::CH4ChpEmissionFactorCalcMethod as D;
        use CH4ChpEmissionFactorCalcMethod as M;

        let Some(method) = from.calculation_method else {
            bail!("CH4 CHP emission factor calculation method is missing");
        };

        let f = match method {
            M::MicroGasTurbines => D::MicroGasTurbines,
            M::GasolineEngine => D::GasolineEngine,
            M::JetEngine => D::JetEngine,
            M::CustomFactor => {
                let Some(factor) = from.custom_emission_factor else {
                    bail!("custom CH4 CHP emission factor is missing");
                };
                D::Custom(Percent::new(factor).convert_to())
            }
        };
        Ok(f)
    }
}

impl TryFrom<EnergyConsumption> for domain::EnergyConsumption {
    type Error = anyhow::Error;

    fn try_from(from: EnergyConsumption) -> Result<Self, Self::Error> {
        let EnergyConsumption {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil,
        } = from;

        let Some(sewage_gas_produced) = sewage_gas_produced else {
            bail!("missing sewage_gas_produced");
        };
        let Some(methane_fraction) = methane_fraction else {
            bail!("missing methane_fraction");
        };
        let Some(total_power_consumption) = total_power_consumption else {
            bail!("missing total_power_consumption");
        };
        let Some(on_site_power_generation) = on_site_power_generation else {
            bail!("missing on_site_power_generation");
        };
        let Some(emission_factor_electricity_mix) = emission_factor_electricity_mix else {
            bail!("missing emission_factor_electricity_mix");
        };
        let Some(heating_oil) = heating_oil else {
            bail!("missing heating_oil");
        };
        let Some(gas_supply) = gas_supply else {
            bail!("missing gas_supply");
        };
        let Some(purchase_of_biogas) = purchase_of_biogas else {
            bail!("missing purchase_of_biogas");
        };

        let methane_fraction = Percent::new(methane_fraction);
        let sewage_gas_produced = Qubicmeters::new(sewage_gas_produced);
        let on_site_power_generation = Kilowatthours::new(on_site_power_generation);
        let total_power_consumption = Kilowatthours::new(total_power_consumption);
        let emission_factor_electricity_mix =
            GramsPerKilowatthour::new(emission_factor_electricity_mix);
        let heating_oil = Liters::new(heating_oil);
        let gas_supply = Qubicmeters::new(gas_supply);

        Ok(Self {
            sewage_gas_produced,
            methane_fraction,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil,
            gas_supply,
            purchase_of_biogas,
        })
    }
}

impl TryFrom<OperatingMaterials> for domain::OperatingMaterials {
    type Error = anyhow::Error;

    fn try_from(from: OperatingMaterials) -> Result<Self, Self::Error> {
        let OperatingMaterials {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        } = from;
        let Some(fecl3) = fecl3 else {
            bail!("missing fecl3");
        };
        let Some(feclso4) = feclso4 else {
            bail!("missing feclso4");
        };
        let Some(caoh2) = caoh2 else {
            bail!("missing caoh2");
        };
        let Some(synthetic_polymers) = synthetic_polymers else {
            bail!("missing synthetic_polymers");
        };

        let fecl3 = Tons::new(fecl3);
        let feclso4 = Tons::new(feclso4);
        let caoh2 = Tons::new(caoh2);
        let synthetic_polymers = Tons::new(synthetic_polymers);

        Ok(Self {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        })
    }
}

impl TryFrom<AnnualAverageInfluent> for domain::AnnualAverageInfluent {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverageInfluent) -> Result<Self, Self::Error> {
        let AnnualAverageInfluent {
            total_nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates,
        } = from;

        let Some(nitrogen) = total_nitrogen else {
            bail!("missing inflow nitrogen");
        };
        let Some(chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing inflow chemical_oxygen_demand");
        };
        let Some(total_organic_carbohydrates) = total_organic_carbohydrates else {
            bail!("missing inflow total_organic_carbohydrates");
        };

        let nitrogen = MilligramsPerLiter::new(nitrogen);
        let chemical_oxygen_demand = MilligramsPerLiter::new(chemical_oxygen_demand);
        let total_organic_carbohydrates = MilligramsPerLiter::new(total_organic_carbohydrates);
        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates,
        })
    }
}

impl TryFrom<AnnualAverageEffluent> for domain::AnnualAverageEffluent {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverageEffluent) -> Result<Self, Self::Error> {
        let AnnualAverageEffluent {
            total_nitrogen,
            chemical_oxygen_demand,
        } = from;

        let Some(nitrogen) = total_nitrogen else {
            bail!("missing effluent nitrogen");
        };
        let Some(chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing effluent chemical_oxygen_demand");
        };

        let chemical_oxygen_demand = MilligramsPerLiter::new(chemical_oxygen_demand);
        let nitrogen = MilligramsPerLiter::new(nitrogen);

        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
        })
    }
}
