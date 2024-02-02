use anyhow::bail;

use klick_domain as domain;

use crate::{
    AnnualAverage, CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, EnergyConsumption,
    N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, OperatingMaterials,
    OptimizationScenario, PlantProfile, Project, ProjectData, ProjectId, SavedProject,
    SewageSludgeTreatment,
};

impl TryFrom<OptimizationScenario> for domain::OptimizationScenario {
    type Error = anyhow::Error;

    fn try_from(from: OptimizationScenario) -> Result<Self, Self::Error> {
        let OptimizationScenario {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        } = from;

        let n2o_emission_factor = n2o_emission_factor.try_into()?;
        let ch4_chp_emission_factor = ch4_chp_emission_factor.map(TryInto::try_into).transpose()?;

        Ok(Self {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        })
    }
}

impl From<domain::OptimizationScenario> for OptimizationScenario {
    fn from(from: domain::OptimizationScenario) -> Self {
        let domain::OptimizationScenario {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        } = from;

        let n2o_emission_factor = n2o_emission_factor.into();
        let ch4_chp_emission_factor = ch4_chp_emission_factor.map(Into::into);

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
                D::Custom(domain::Factor::new(factor))
            }
        };
        Ok(f)
    }
}

impl From<domain::N2oEmissionFactorCalcMethod> for N2oEmissionFactorScenario {
    fn from(from: domain::N2oEmissionFactorCalcMethod) -> Self {
        use domain::N2oEmissionFactorCalcMethod as D;
        use N2oEmissionFactorCalcMethod as M;

        let calculation_method = match from {
            D::TuWien2016 => M::TuWien2016,
            D::Optimistic => M::Optimistic,
            D::Pesimistic => M::Pesimistic,
            D::Ipcc2019 => M::Ipcc2019,
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
                D::Custom(domain::Factor::new(factor))
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

impl TryFrom<PlantProfile> for domain::PlantProfile {
    type Error = anyhow::Error;

    fn try_from(from: PlantProfile) -> Result<Self, Self::Error> {
        let PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
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
        let operating_materials = operating_materials.try_into()?;

        let wastewater = domain::Qubicmeters::new(wastewater);

        Ok(Self {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        })
    }
}

impl From<domain::PlantProfile> for PlantProfile {
    fn from(from: domain::PlantProfile) -> Self {
        let domain::PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        } = from;

        let influent_average = influent_average.into();
        let effluent_average = effluent_average.into();
        let energy_consumption = energy_consumption.into();
        let sewage_sludge_treatment = sewage_sludge_treatment.into();
        let operating_materials = operating_materials.into();

        let population_equivalent = Some(population_equivalent);
        let wastewater = Some(wastewater.into());

        Self {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
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

        let methane_fraction = domain::Percent::new(methane_fraction);
        let sewage_gas_produced = domain::Qubicmeters::new(sewage_gas_produced);
        let on_site_power_generation = domain::Kilowatthours::new(on_site_power_generation);
        let total_power_consumption = domain::Kilowatthours::new(total_power_consumption);
        let gas_supply = gas_supply.map(domain::Kilowatthours::new);
        let emission_factor_electricity_mix =
            domain::GramsPerKilowatthour::new(emission_factor_electricity_mix);

        Ok(Self {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
        })
    }
}

impl From<domain::EnergyConsumption> for EnergyConsumption {
    fn from(from: domain::EnergyConsumption) -> Self {
        let domain::EnergyConsumption {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
        } = from;

        let sewage_gas_produced = Some(sewage_gas_produced.into());
        let methane_fraction = Some(methane_fraction.into());

        let total_power_consumption = Some(total_power_consumption.into());
        let on_site_power_generation = Some(on_site_power_generation.into());
        let emission_factor_electricity_mix = Some(emission_factor_electricity_mix.into());

        let gas_supply = gas_supply.map(Into::into);

        Self {
            sewage_gas_produced,
            methane_fraction,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
        }
    }
}

impl TryFrom<SewageSludgeTreatment> for domain::SewageSludgeTreatment {
    type Error = anyhow::Error;

    fn try_from(from: SewageSludgeTreatment) -> Result<Self, Self::Error> {
        let SewageSludgeTreatment {
            sludge_bags_are_open,
            sludge_storage_containers_are_open,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        } = from;
        let Some(sludge_bags_are_open) = sludge_bags_are_open else {
            bail!("missing sludge_bags_are_open");
        };
        let Some(sludge_storage_containers_are_open) = sludge_storage_containers_are_open else {
            bail!("missing sludge_storage_containers_are_open");
        };
        let Some(sewage_sludge_for_disposal) = sewage_sludge_for_disposal else {
            bail!("missing sewage_sludge_for_disposal");
        };
        let Some(transport_distance) = transport_distance else {
            bail!("missing transport_distance");
        };
        let sewage_sludge_for_disposal = domain::Tons::new(sewage_sludge_for_disposal);
        let transport_distance = domain::Kilometers::new(transport_distance);
        Ok(Self {
            sludge_bags_are_open,
            sludge_storage_containers_are_open,
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
            sludge_storage_containers_are_open,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,
        } = from;
        let sludge_bags_are_open = Some(sludge_bags_are_open);
        let sludge_storage_containers_are_open = Some(sludge_storage_containers_are_open);
        let sewage_sludge_for_disposal = Some(sewage_sludge_for_disposal.into());
        let transport_distance = Some(transport_distance.into());
        let digester_count = digester_count.map(Into::into);
        Self {
            sludge_bags_are_open,
            sludge_storage_containers_are_open,
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

        let fecl3 = domain::Tons::new(fecl3);
        let feclso4 = domain::Tons::new(feclso4);
        let caoh2 = domain::Tons::new(caoh2);
        let synthetic_polymers = domain::Tons::new(synthetic_polymers);

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

impl TryFrom<AnnualAverage> for domain::AnnualAverageInfluent {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverage) -> Result<Self, Self::Error> {
        let AnnualAverage {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        } = from;

        let Some(nitrogen) = nitrogen else {
            bail!("missing inflow nitrogen");
        };

        let phosphorus = phosphorus.map(domain::MilligramsPerLiter::new);
        let chemical_oxygen_demand = chemical_oxygen_demand.map(domain::MilligramsPerLiter::new);
        let nitrogen = domain::MilligramsPerLiter::new(nitrogen);

        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        })
    }
}

impl From<domain::AnnualAverageInfluent> for AnnualAverage {
    fn from(from: domain::AnnualAverageInfluent) -> Self {
        let domain::AnnualAverageInfluent {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        } = from;

        let nitrogen = Some(nitrogen.into());

        let phosphorus = phosphorus.map(Into::into);
        let chemical_oxygen_demand = chemical_oxygen_demand.map(Into::into);

        Self {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        }
    }
}

impl TryFrom<AnnualAverage> for domain::AnnualAverageEffluent {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverage) -> Result<Self, Self::Error> {
        let AnnualAverage {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        } = from;

        let Some(nitrogen) = nitrogen else {
            bail!("missing effluent nitrogen");
        };
        let Some(chemical_oxygen_demand) = chemical_oxygen_demand else {
            bail!("missing effluent chemical_oxygen_demand");
        };

        let phosphorus = phosphorus.map(domain::MilligramsPerLiter::new);
        let chemical_oxygen_demand = domain::MilligramsPerLiter::new(chemical_oxygen_demand);
        let nitrogen = domain::MilligramsPerLiter::new(nitrogen);

        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        })
    }
}

impl From<domain::AnnualAverageEffluent> for AnnualAverage {
    fn from(from: domain::AnnualAverageEffluent) -> Self {
        let domain::AnnualAverageEffluent {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        } = from;

        let nitrogen = Some(nitrogen.into());
        let chemical_oxygen_demand = Some(chemical_oxygen_demand.into());

        let phosphorus = phosphorus.map(Into::into);

        Self {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        }
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
