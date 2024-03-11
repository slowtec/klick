use anyhow::bail;

use klick_domain as domain;

use crate::{
    AnnualAverageEffluent, AnnualAverageInfluent, CH4ChpEmissionFactorCalcMethod,
    CH4ChpEmissionFactorScenario, CustomEmissionFactors, EnergyConsumption, EnergyEmissionFactors,
    N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, OperatingMaterials,
    OptimizationScenario, PlantProfile, Project, ProjectData, ProjectId, SavedProject,
    SewageSludgeTreatment, SideStreamTreatment,
};

impl TryFrom<OptimizationScenario> for domain::EmissionFactorCalculationMethods {
    type Error = anyhow::Error;

    fn try_from(from: OptimizationScenario) -> Result<Self, Self::Error> {
        let OptimizationScenario {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        } = from;

        let n2o = n2o_emission_factor.try_into()?;
        let ch4 = ch4_chp_emission_factor.map(TryInto::try_into).transpose()?;

        Ok(Self { n2o, ch4 })
    }
}

impl From<domain::EmissionFactorCalculationMethods> for OptimizationScenario {
    fn from(from: domain::EmissionFactorCalculationMethods) -> Self {
        let domain::EmissionFactorCalculationMethods { n2o, ch4 } = from;

        let n2o_emission_factor = n2o.into();
        let ch4_chp_emission_factor = ch4.map(Into::into);

        Self {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        }
    }
}

impl TryFrom<N2oEmissionFactorScenario> for domain::N2oEmissionFactorCalcMethod {
    type Error = anyhow::Error;

    fn try_from(from: N2oEmissionFactorScenario) -> Result<Self, Self::Error> {
        use domain::N2oEmissionFactorCalcMethod as D;
        use N2oEmissionFactorCalcMethod as M;

        let f = match from.calculation_method {
            M::TuWien2016 => D::TuWien2016,
            M::Optimistic => D::Optimistic,
            M::Pesimistic => D::Pesimistic,
            M::Ipcc2019 => D::Ipcc2019,
            M::CustomFactor => {
                let Some(factor) = from.custom_factor else {
                    bail!("custom N2O emission factor is missing");
                };
                D::Custom(domain::units::Factor::new(factor))
            }
        };
        Ok(f)
    }
}

impl From<domain::N2oEmissionFactorCalcMethod> for N2oEmissionFactorScenario {
    fn from(from: domain::N2oEmissionFactorCalcMethod) -> Self {
        let calculation_method = from.into();
        let custom_factor = match from {
            domain::N2oEmissionFactorCalcMethod::Custom(f) => Some(f.into()),
            _ => None,
        };
        Self {
            calculation_method,
            custom_factor,
        }
    }
}

impl From<domain::N2oEmissionFactorCalcMethod> for N2oEmissionFactorCalcMethod {
    fn from(from: domain::N2oEmissionFactorCalcMethod) -> Self {
        use domain::N2oEmissionFactorCalcMethod as D;
        use N2oEmissionFactorCalcMethod as M;

        match from {
            D::TuWien2016 => M::TuWien2016,
            D::Optimistic => M::Optimistic,
            D::Pesimistic => M::Pesimistic,
            D::Ipcc2019 => M::Ipcc2019,
            D::Custom(_) => M::CustomFactor,
        }
    }
}

impl TryFrom<CH4ChpEmissionFactorScenario> for domain::CH4ChpEmissionFactorCalcMethod {
    type Error = anyhow::Error;

    fn try_from(from: CH4ChpEmissionFactorScenario) -> Result<Self, Self::Error> {
        use domain::CH4ChpEmissionFactorCalcMethod as D;
        use CH4ChpEmissionFactorCalcMethod as M;

        let f = match from.calculation_method {
            M::MicroGasTurbines => D::MicroGasTurbines,
            M::GasolineEngine => D::GasolineEngine,
            M::JetEngine => D::JetEngine,
            M::CustomFactor => {
                let Some(factor) = from.custom_factor else {
                    bail!("custom N2O emission factor is missing");
                };
                D::Custom(domain::units::Factor::new(factor))
            }
        };
        Ok(f)
    }
}

impl From<domain::CH4ChpEmissionFactorCalcMethod> for CH4ChpEmissionFactorScenario {
    fn from(from: domain::CH4ChpEmissionFactorCalcMethod) -> Self {
        use domain::CH4ChpEmissionFactorCalcMethod as D;
        use CH4ChpEmissionFactorCalcMethod as M;

        let calculation_method = match from {
            D::MicroGasTurbines => M::MicroGasTurbines,
            D::GasolineEngine => M::GasolineEngine,
            D::JetEngine => M::JetEngine,
            D::Custom(_) => M::CustomFactor,
        };
        let custom_factor = match from {
            D::Custom(f) => Some(f.into()),
            _ => None,
        };
        Self {
            calculation_method,
            custom_factor,
        }
    }
}

impl TryFrom<PlantProfile> for domain::EmissionInfluencingValues {
    type Error = anyhow::Error;

    fn try_from(from: PlantProfile) -> Result<Self, Self::Error> {
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
            emission_factors,
            energy_emission_factors,
        } = from;

        let Some(population_equivalent) = population_equivalent else {
            bail!("missing population_values");
        };

        let Some(wastewater) = wastewater else {
            bail!("missing wastewater");
        };

        let influent_average = influent_average.try_into()?;
        let effluent_average = effluent_average.try_into()?;
        let energy_consumption = energy_consumption.try_into()?;
        let sewage_sludge_treatment = sewage_sludge_treatment.try_into()?;
        let side_stream_treatment = side_stream_treatment.try_into()?;
        let operating_materials = operating_materials.try_into()?;
        let emission_factors = emission_factors.try_into()?;
        let wastewater = domain::units::Qubicmeters::new(wastewater);
        let energy_emission_factors = energy_emission_factors.try_into()?;

        Ok(Self {
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
        })
    }
}

impl From<domain::EmissionInfluencingValues> for PlantProfile {
    fn from(from: domain::EmissionInfluencingValues) -> Self {
        let domain::EmissionInfluencingValues {
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
        } = from;

        let influent_average = influent_average.into();
        let effluent_average = effluent_average.into();
        let energy_consumption = energy_consumption.into();
        let sewage_sludge_treatment = sewage_sludge_treatment.into();
        let side_stream_treatment = side_stream_treatment.into();
        let operating_materials = operating_materials.into();
        let emission_factors = emission_factors.into();
        let energy_emission_factors = energy_emission_factors.into();

        let population_equivalent = Some(population_equivalent);
        let wastewater = Some(wastewater.into());
        let plant_name = None;

        Self {
            plant_name,
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
        }
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

        let methane_fraction = domain::units::Percent::new(methane_fraction);
        let sewage_gas_produced = domain::units::Qubicmeters::new(sewage_gas_produced);
        let on_site_power_generation = domain::units::Kilowatthours::new(on_site_power_generation);
        let total_power_consumption = domain::units::Kilowatthours::new(total_power_consumption);
        let emission_factor_electricity_mix =
            domain::units::GramsPerKilowatthour::new(emission_factor_electricity_mix);
        let heating_oil = domain::units::Liters::new(heating_oil);
        let gas_supply = domain::units::Qubicmeters::new(gas_supply);

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

impl From<domain::EnergyConsumption> for EnergyConsumption {
    fn from(from: domain::EnergyConsumption) -> Self {
        let domain::EnergyConsumption {
            sewage_gas_produced,
            methane_fraction,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil,
            gas_supply,
            purchase_of_biogas,
        } = from;

        let sewage_gas_produced = Some(sewage_gas_produced.into());
        let methane_fraction = Some(methane_fraction.into());

        let total_power_consumption = Some(total_power_consumption.into());
        let on_site_power_generation = Some(on_site_power_generation.into());
        let emission_factor_electricity_mix = Some(emission_factor_electricity_mix.into());
        let heating_oil = Some(heating_oil.into());

        let gas_supply = Some(gas_supply.into());
        let purchase_of_biogas = Some(purchase_of_biogas.into());

        Self {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil,
        }
    }
}

impl TryFrom<SideStreamTreatment> for domain::SideStreamTreatment {
    type Error = anyhow::Error;

    fn try_from(from: SideStreamTreatment) -> Result<Self, Self::Error> {
        let SideStreamTreatment {
            total_nitrogen,
            side_stream_cover_is_open,
        } = from;

        let Some(total_nitrogen) = total_nitrogen else {
            bail!("missing total_nitrogen");
        };
        let Some(side_stream_cover_is_open) = side_stream_cover_is_open else {
            bail!("missing side_stream_cover_is_open");
        };

        let total_nitrogen = domain::units::Tons::new(total_nitrogen);

        Ok(Self {
            total_nitrogen,
            side_stream_cover_is_open,
        })
    }
}

impl From<domain::SideStreamTreatment> for SideStreamTreatment {
    fn from(from: domain::SideStreamTreatment) -> Self {
        let domain::SideStreamTreatment {
            total_nitrogen,
            side_stream_cover_is_open,
        } = from;

        let total_nitrogen = Some(total_nitrogen.into());
        let side_stream_cover_is_open = Some(side_stream_cover_is_open);

        Self {
            total_nitrogen,
            side_stream_cover_is_open,
        }
    }
}

impl TryFrom<SewageSludgeTreatment> for domain::SewageSludgeTreatment {
    type Error = anyhow::Error;

    fn try_from(from: SewageSludgeTreatment) -> Result<Self, Self::Error> {
        let SewageSludgeTreatment {
            sludge_bags_are_open,
            sludge_bags_are_open_recommendation,
            custom_sludge_bags_factor,
            sludge_storage_containers_are_open,
            sludge_storage_containers_are_open_recommendation,
            custom_sludge_storage_containers_factor,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        } = from;
        let Some(sludge_bags_are_open) = sludge_bags_are_open else {
            bail!("missing sludge_bags_are_open");
        };
        let Some(sludge_bags_are_open_recommendation) = sludge_bags_are_open_recommendation else {
            bail!("missing sludge_bags_are_open_recommendation");
        };
        let Some(sludge_storage_containers_are_open) = sludge_storage_containers_are_open else {
            bail!("missing sludge_storage_containers_are_open");
        };
        let Some(sludge_storage_containers_are_open_recommendation) =
            sludge_storage_containers_are_open_recommendation
        else {
            bail!("missing sludge_storage_containers_are_open_recommendation");
        };
        let Some(sewage_sludge_for_disposal) = sewage_sludge_for_disposal else {
            bail!("missing sewage_sludge_for_disposal");
        };
        let Some(transport_distance) = transport_distance else {
            bail!("missing transport_distance");
        };
        let sewage_sludge_for_disposal = domain::units::Tons::new(sewage_sludge_for_disposal);
        let transport_distance = domain::units::Kilometers::new(transport_distance);
        let custom_sludge_bags_factor =
            custom_sludge_bags_factor.map(domain::units::QubicmetersPerHour::new);
        let custom_sludge_storage_containers_factor =
            custom_sludge_storage_containers_factor.map(domain::units::Percent::new);

        Ok(Self {
            sludge_bags_are_open,
            sludge_bags_are_open_recommendation,
            custom_sludge_bags_factor,
            sludge_storage_containers_are_open,
            sludge_storage_containers_are_open_recommendation,
            custom_sludge_storage_containers_factor,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        })
    }
}

impl From<domain::SewageSludgeTreatment> for SewageSludgeTreatment {
    fn from(from: domain::SewageSludgeTreatment) -> Self {
        let domain::SewageSludgeTreatment {
            sludge_bags_are_open,
            sludge_bags_are_open_recommendation,
            custom_sludge_bags_factor,
            sludge_storage_containers_are_open,
            sludge_storage_containers_are_open_recommendation,
            custom_sludge_storage_containers_factor,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        } = from;
        let sludge_bags_are_open = Some(sludge_bags_are_open);
        let sludge_bags_are_open_recommendation = Some(sludge_bags_are_open_recommendation);
        let sludge_storage_containers_are_open = Some(sludge_storage_containers_are_open);
        let sludge_storage_containers_are_open_recommendation =
            Some(sludge_storage_containers_are_open_recommendation);
        let sewage_sludge_for_disposal = Some(sewage_sludge_for_disposal.into());
        let transport_distance = Some(transport_distance.into());
        let digester_count = digester_count.map(Into::into);
        let custom_sludge_bags_factor = custom_sludge_bags_factor.map(Into::into);
        let custom_sludge_storage_containers_factor =
            custom_sludge_storage_containers_factor.map(Into::into);

        Self {
            sludge_bags_are_open,
            sludge_bags_are_open_recommendation,
            custom_sludge_bags_factor,
            sludge_storage_containers_are_open,
            sludge_storage_containers_are_open_recommendation,
            custom_sludge_storage_containers_factor,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        }
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

        let fecl3 = domain::units::Tons::new(fecl3);
        let feclso4 = domain::units::Tons::new(feclso4);
        let caoh2 = domain::units::Tons::new(caoh2);
        let synthetic_polymers = domain::units::Tons::new(synthetic_polymers);

        Ok(Self {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        })
    }
}

impl From<domain::OperatingMaterials> for OperatingMaterials {
    fn from(from: domain::OperatingMaterials) -> Self {
        let domain::OperatingMaterials {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        } = from;

        let fecl3 = Some(fecl3.into());
        let feclso4 = Some(feclso4.into());
        let caoh2 = Some(caoh2.into());
        let synthetic_polymers = Some(synthetic_polymers.into());

        Self {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        }
    }
}

impl TryFrom<CustomEmissionFactors> for domain::EmissionFactors {
    // FIXME rename into something better
    type Error = anyhow::Error;

    fn try_from(from: CustomEmissionFactors) -> Result<Self, Self::Error> {
        let CustomEmissionFactors {
            n2o_side_stream,
            co2_fossil,
        } = from;
        let Some(n2o_side_stream) = n2o_side_stream else {
            bail!("missing n2o_side_stream");
        };
        let Some(co2_fossil) = co2_fossil else {
            bail!("missing co2_fossil");
        };
        let n2o_side_stream = domain::units::Factor::new(n2o_side_stream / 100.0);
        let co2_fossil = domain::units::Factor::new(co2_fossil / 100.0);

        Ok(Self {
            n2o_side_stream,
            co2_fossil,
        })
    }
}

impl From<domain::EmissionFactors> for CustomEmissionFactors {
    fn from(from: domain::EmissionFactors) -> Self {
        let domain::EmissionFactors {
            n2o_side_stream,
            co2_fossil,
        } = from;

        let n2o_side_stream = Some(n2o_side_stream.into());
        let co2_fossil = Some(co2_fossil.into());

        Self {
            n2o_side_stream,
            co2_fossil,
        }
    }
}

impl TryFrom<AnnualAverageInfluent> for domain::AnnualAverageInfluent {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverageInfluent) -> Result<Self, Self::Error> {
        let AnnualAverageInfluent {
            nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates,
        } = from;

        let Some(nitrogen) = nitrogen else {
            bail!("missing inflow nitrogen");
        };
        let Some(chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing inflow chemical_oxygen_demand");
        };
        let Some(total_organic_carbohydrates) = total_organic_carbohydrates else {
            bail!("missing inflow total_organic_carbohydrates");
        };

        let nitrogen = domain::units::MilligramsPerLiter::new(nitrogen);
        let chemical_oxygen_demand = domain::units::MilligramsPerLiter::new(chemical_oxygen_demand);
        let total_organic_carbohydrates =
            domain::units::MilligramsPerLiter::new(total_organic_carbohydrates);
        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates,
        })
    }
}

impl From<domain::AnnualAverageInfluent> for AnnualAverageInfluent {
    fn from(from: domain::AnnualAverageInfluent) -> Self {
        let domain::AnnualAverageInfluent {
            nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates,
        } = from;

        let nitrogen = Some(nitrogen.into());
        let total_organic_carbohydrates = Some(total_organic_carbohydrates.into());
        let chemical_oxygen_demand = Some(chemical_oxygen_demand.into());

        Self {
            nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates,
        }
    }
}

impl TryFrom<AnnualAverageEffluent> for domain::AnnualAverageEffluent {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverageEffluent) -> Result<Self, Self::Error> {
        let AnnualAverageEffluent {
            nitrogen,
            chemical_oxygen_demand,
        } = from;

        let Some(nitrogen) = nitrogen else {
            bail!("missing effluent nitrogen");
        };
        let Some(chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing effluent chemical_oxygen_demand");
        };

        let chemical_oxygen_demand = domain::units::MilligramsPerLiter::new(chemical_oxygen_demand);
        let nitrogen = domain::units::MilligramsPerLiter::new(nitrogen);

        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
        })
    }
}

impl From<domain::AnnualAverageEffluent> for AnnualAverageEffluent {
    fn from(from: domain::AnnualAverageEffluent) -> Self {
        let domain::AnnualAverageEffluent {
            nitrogen,
            chemical_oxygen_demand,
        } = from;

        let nitrogen = Some(nitrogen.into());
        let chemical_oxygen_demand = Some(chemical_oxygen_demand.into());

        Self {
            nitrogen,
            chemical_oxygen_demand,
        }
    }
}

impl From<domain::EnergyEmissionFactors> for EnergyEmissionFactors {
    fn from(from: domain::EnergyEmissionFactors) -> Self {
        let domain::EnergyEmissionFactors {
            process_energy_savings,
            fossil_energy_savings,
            district_heating,
            photovoltaic_energy_expansion,
            estimated_self_photovoltaic_usage,
            wind_energy_expansion,
            estimated_self_wind_energy_usage,
            water_energy_expansion,
            estimated_self_water_energy_usage,
        } = from;

        let process_energy_savings = Some(process_energy_savings.into());
        let fossil_energy_savings = Some(fossil_energy_savings.into());
        let district_heating = Some(district_heating.into());
        let photovoltaic_energy_expansion = Some(photovoltaic_energy_expansion.into());
        let estimated_self_photovoltaic_usage = Some(estimated_self_photovoltaic_usage.into());
        let wind_energy_expansion = Some(wind_energy_expansion.into());
        let estimated_self_wind_energy_usage = Some(estimated_self_wind_energy_usage.into());
        let water_energy_expansion = Some(water_energy_expansion.into());
        let estimated_self_water_energy_usage = Some(estimated_self_water_energy_usage.into());

        Self {
            process_energy_savings,
            fossil_energy_savings,
            district_heating,
            photovoltaic_energy_expansion,
            estimated_self_photovoltaic_usage,
            wind_energy_expansion,
            estimated_self_wind_energy_usage,
            water_energy_expansion,
            estimated_self_water_energy_usage,
        }
    }
}

impl TryFrom<EnergyEmissionFactors> for domain::EnergyEmissionFactors {
    type Error = anyhow::Error;

    fn try_from(from: EnergyEmissionFactors) -> Result<Self, Self::Error> {
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
        } = from;
        let Some(process_energy_savings) = process_energy_savings else {
            bail!("missing process_energy_savings");
        };
        let Some(fossil_energy_savings) = fossil_energy_savings else {
            bail!("missing fossil_energy_savings");
        };
        let Some(district_heating) = district_heating else {
            bail!("missing district_heating");
        };
        let Some(photovoltaic_energy_expansion) = photovoltaic_energy_expansion else {
            bail!("missing photovoltaic_energy_expansion");
        };
        let Some(estimated_self_photovoltaic_usage) = estimated_self_photovoltaic_usage else {
            bail!("missing estimated_self_photovoltaic_usage");
        };
        let Some(wind_energy_expansion) = wind_energy_expansion else {
            bail!("missing wind_energy_expansion");
        };
        let Some(estimated_self_wind_energy_usage) = estimated_self_wind_energy_usage else {
            bail!("missing estimated_self_wind_energy_usage");
        };
        let Some(water_energy_expansion) = water_energy_expansion else {
            bail!("missing water_energy_expansion");
        };
        let Some(estimated_self_water_energy_usage) = estimated_self_water_energy_usage else {
            bail!("missing estimated_self_water_energy_usage");
        };

        let process_energy_savings = domain::units::Percent::new(process_energy_savings);
        let fossil_energy_savings = domain::units::Percent::new(fossil_energy_savings);
        let district_heating = domain::units::Kilowatthours::new(district_heating);
        let photovoltaic_energy_expansion =
            domain::units::Kilowatthours::new(photovoltaic_energy_expansion);
        let estimated_self_photovoltaic_usage =
            domain::units::Percent::new(estimated_self_photovoltaic_usage);
        let wind_energy_expansion = domain::units::Kilowatthours::new(wind_energy_expansion);
        let estimated_self_wind_energy_usage =
            domain::units::Percent::new(estimated_self_wind_energy_usage);
        let water_energy_expansion = domain::units::Kilowatthours::new(water_energy_expansion);
        let estimated_self_water_energy_usage =
            domain::units::Percent::new(estimated_self_water_energy_usage);

        Ok(Self {
            process_energy_savings,
            fossil_energy_savings,
            district_heating,
            photovoltaic_energy_expansion,
            estimated_self_photovoltaic_usage,
            wind_energy_expansion,
            estimated_self_wind_energy_usage,
            water_energy_expansion,
            estimated_self_water_energy_usage,
        })
    }
}

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

impl From<domain::Project<ProjectData>> for Project {
    fn from(from: domain::Project<ProjectData>) -> Self {
        Self::Saved(from.into())
    }
}

impl From<SavedProject> for domain::Project<ProjectData> {
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

impl From<domain::Project<ProjectData>> for SavedProject {
    fn from(from: domain::Project<ProjectData>) -> Self {
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
