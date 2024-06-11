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

        let AnnualAverageInfluent {
            total_nitrogen,
            chemical_oxygen_demand,
            total_organic_carbohydrates,
        } = influent_average;

        let Some(nitrogen) = total_nitrogen else {
            bail!("missing inflow nitrogen");
        };
        let Some(chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing inflow chemical_oxygen_demand");
        };
        let Some(total_organic_carbohydrates) = total_organic_carbohydrates else {
            bail!("missing inflow total_organic_carbohydrates");
        };

        let influent_nitrogen = MilligramsPerLiter::new(nitrogen);
        let influent_chemical_oxygen_demand = MilligramsPerLiter::new(chemical_oxygen_demand);
        let influent_total_organic_carbohydrates =
            MilligramsPerLiter::new(total_organic_carbohydrates);

        let AnnualAverageEffluent {
            total_nitrogen,
            chemical_oxygen_demand,
        } = effluent_average;

        let Some(nitrogen) = total_nitrogen else {
            bail!("missing effluent nitrogen");
        };
        let Some(chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing effluent chemical_oxygen_demand");
        };

        let effluent_chemical_oxygen_demand = MilligramsPerLiter::new(chemical_oxygen_demand);
        let effluent_nitrogen = MilligramsPerLiter::new(nitrogen);

        let EnergyConsumption {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil,
        } = energy_consumption;

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

        let OperatingMaterials {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        } = operating_materials;

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

        let operating_material_fecl3 = Tons::new(fecl3);
        let operating_material_feclso4 = Tons::new(feclso4);
        let operating_material_caoh2 = Tons::new(caoh2);
        let operating_material_synthetic_polymers = Tons::new(synthetic_polymers);

        // -----   ----- //
        //  Sensitivity  //
        // -----   ----- //

        let SensitivityParameters {
            n2o_emissions,
            ch4_chp_emissions,
            ch4_sewage_sludge_emissions,
            co2_fossil_emissions,
        } = sensitivity_parameters;

        let (n2o, n2o_side_stream, n2o_custom_factor) = n2o_emissions.try_into()?;
        let (ch4, ch4_custom_factor) = ch4_chp_emissions.try_into()?;

        let emission_factor_calculation_methods = domain::EmissionFactorCalculationMethods {
            n2o,
            n2o_custom_factor,
            ch4,
            ch4_custom_factor,
        };

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
            if sludge_bags_are_closed {
                sludge_bags_are_open = false;
            }
        }
        if let Some(sludge_storage_containers_are_closed) = sludge_storage_containers_are_closed {
            if sludge_storage_containers_are_closed {
                sludge_storage_containers_are_open = false;
            }
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

        let emission_influencing_values = domain::EmissionInfluencingValues {
            population_equivalent,
            wastewater,

            influent_nitrogen,
            influent_chemical_oxygen_demand,
            influent_total_organic_carbohydrates,

            effluent_chemical_oxygen_demand,
            effluent_nitrogen,

            sludge_bags_are_open,
            sludge_bags_factor,
            sludge_storage_containers_are_open,
            sludge_storage_containers_factor,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,

            side_stream_treatment_total_nitrogen: total_nitrogen,
            side_stream_cover_is_open,

            operating_material_fecl3,
            operating_material_feclso4,
            operating_material_caoh2,
            operating_material_synthetic_polymers,

            emission_factor_n2o_side_stream: n2o_side_stream,
            emission_factor_co2_fossil: fossil_emission_factor,

            sewage_gas_produced,
            methane_fraction,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil,
            gas_supply,
            purchase_of_biogas,

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

        Ok((
            emission_influencing_values,
            emission_factor_calculation_methods,
        ))
    }
}

impl TryFrom<N2OEmissionsSensitivity>
    for (
        domain::units::N2oEmissionFactorCalcMethod,
        Factor,
        Option<Factor>,
    )
{
    type Error = anyhow::Error;

    fn try_from(from: N2OEmissionsSensitivity) -> Result<Self, Self::Error> {
        use crate::N2oEmissionFactorCalcMethod as M;
        use domain::units::N2oEmissionFactorCalcMethod as D;

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
                if custom_emission_factor.is_none() {
                    bail!("custom N2O emission factor is missing");
                }
                D::Custom
            }
        };
        let custom_factor = custom_emission_factor.map(|f| Percent::new(f).convert_to());
        Ok((method, side_stream_factor, custom_factor))
    }
}

impl TryFrom<CH4ChpEmissionsSensitivity>
    for (
        Option<domain::units::Ch4ChpEmissionFactorCalcMethod>,
        Option<Factor>,
    )
{
    type Error = anyhow::Error;

    fn try_from(from: CH4ChpEmissionsSensitivity) -> Result<Self, Self::Error> {
        use crate::CH4ChpEmissionFactorCalcMethod as M;
        use domain::units::Ch4ChpEmissionFactorCalcMethod as D;

        let method = from.calculation_method.map(|method| match method {
            M::MicroGasTurbines => D::MicroGasTurbines,
            M::GasolineEngine => D::GasolineEngine,
            M::JetEngine => D::JetEngine,
            M::CustomFactor => D::Custom,
        });

        let custom_factor = from
            .custom_emission_factor
            .map(|f| Percent::new(f).convert_to());

        if method == Some(D::Custom) && custom_factor.is_none() {
            bail!("custom CH4 CHP emission factor is missing");
        }

        Ok((method, custom_factor))
    }
}

impl From<domain::units::N2oEmissionFactorCalcMethod> for crate::N2oEmissionFactorCalcMethod {
    fn from(from: domain::units::N2oEmissionFactorCalcMethod) -> Self {
        use domain::units::N2oEmissionFactorCalcMethod as FROM;
        match from {
            FROM::TuWien2016 => Self::TuWien2016,
            FROM::Optimistic => Self::Optimistic,
            FROM::Pesimistic => Self::Pesimistic,
            FROM::Ipcc2019 => Self::Ipcc2019,
            FROM::Custom => Self::CustomFactor,
        }
    }
}

impl From<domain::units::Ch4ChpEmissionFactorCalcMethod> for crate::CH4ChpEmissionFactorCalcMethod {
    fn from(from: domain::units::Ch4ChpEmissionFactorCalcMethod) -> Self {
        use domain::units::Ch4ChpEmissionFactorCalcMethod as FROM;
        match from {
            FROM::MicroGasTurbines => Self::MicroGasTurbines,
            FROM::GasolineEngine => Self::GasolineEngine,
            FROM::JetEngine => Self::JetEngine,
            FROM::Custom => Self::CustomFactor,
        }
    }
}

impl From<crate::N2oEmissionFactorCalcMethod> for domain::units::N2oEmissionFactorCalcMethod {
    fn from(from: crate::N2oEmissionFactorCalcMethod) -> Self {
        use crate::N2oEmissionFactorCalcMethod as FROM;
        match from {
            FROM::TuWien2016 => Self::TuWien2016,
            FROM::Optimistic => Self::Optimistic,
            FROM::Pesimistic => Self::Pesimistic,
            FROM::Ipcc2019 => Self::Ipcc2019,
            FROM::CustomFactor => Self::Custom,
        }
    }
}

impl From<crate::CH4ChpEmissionFactorCalcMethod> for domain::units::Ch4ChpEmissionFactorCalcMethod {
    fn from(from: crate::CH4ChpEmissionFactorCalcMethod) -> Self {
        use crate::CH4ChpEmissionFactorCalcMethod as FROM;
        match from {
            FROM::MicroGasTurbines => Self::MicroGasTurbines,
            FROM::GasolineEngine => Self::GasolineEngine,
            FROM::JetEngine => Self::JetEngine,
            FROM::CustomFactor => Self::Custom,
        }
    }
}
