use anyhow::bail;

use klick_application as app;
use klick_domain as domain;

use crate::{
    AnnualAverage, CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, EnergyConsumption,
    InputData, N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, OperatingMaterials,
    Scenario, SewageSludgeTreatment,
};

impl TryFrom<Scenario> for app::Scenario {
    type Error = anyhow::Error;

    fn try_from(from: Scenario) -> Result<Self, Self::Error> {
        let Scenario {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        } = from;

        let n2o_emission_factor = n2o_emission_factor.try_into()?;
        let ch4_chp_emission_factor = ch4_chp_emission_factor.map(TryInto::try_into).transpose()?;

        Ok(app::Scenario {
            n2o_emission_factor,
            ch4_chp_emission_factor,
        })
    }
}

impl TryFrom<N2oEmissionFactorScenario> for app::N2oEmissionFactorCalcMethod {
    type Error = anyhow::Error;

    fn try_from(from: N2oEmissionFactorScenario) -> Result<Self, Self::Error> {
        use app::N2oEmissionFactorCalcMethod as A;
        use N2oEmissionFactorCalcMethod as M;

        let f = match from.calculation_method {
            M::TuWien2016 => A::TuWien2016,
            M::Optimistic => A::Optimistic,
            M::Pesimistic => A::Pesimistic,
            M::Ipcc2019 => A::Ipcc2019,
            M::CustomFactor => {
                let Some(factor) = from.custom_factor else {
                    bail!("custom N2O emission factor is missing");
                };
                A::Custom(domain::Factor::new(factor))
            }
        };
        Ok(f)
    }
}

impl TryFrom<CH4ChpEmissionFactorScenario> for app::CH4ChpEmissionFactorCalcMethod {
    type Error = anyhow::Error;

    fn try_from(from: CH4ChpEmissionFactorScenario) -> Result<Self, Self::Error> {
        use app::CH4ChpEmissionFactorCalcMethod as A;
        use CH4ChpEmissionFactorCalcMethod as M;

        let f = match from.calculation_method {
            M::MicroGasTurbines => A::MicroGasTurbines,
            M::GasolineEngine => A::GasolineEngine,
            M::JetEngine => A::JetEngine,
            M::CustomFactor => {
                let Some(factor) = from.custom_factor else {
                    bail!("custom N2O emission factor is missing");
                };
                A::Custom(domain::Factor::new(factor))
            }
        };
        Ok(f)
    }
}

impl TryFrom<InputData> for domain::PlantProfile {
    type Error = anyhow::Error;

    fn try_from(from: InputData) -> Result<Self, Self::Error> {
        let InputData {
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

impl TryFrom<SewageSludgeTreatment> for domain::SewageSludgeTreatment {
    type Error = anyhow::Error;

    fn try_from(from: SewageSludgeTreatment) -> Result<Self, Self::Error> {
        let SewageSludgeTreatment {
            open_sludge_bags,
            open_sludge_storage_containers,
            sewage_sludge_for_disposal,
            transport_distance,
        } = from;
        let Some(open_sludge_bags) = open_sludge_bags else {
            bail!("missing open_sludge_bags");
        };
        let Some(open_sludge_storage_containers) = open_sludge_storage_containers else {
            bail!("missing open_sludge_storage_containers");
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
            open_sludge_bags,
            open_sludge_storage_containers,
            sewage_sludge_for_disposal,
            transport_distance,
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
