use anyhow::bail;

use klick_application as app;

use crate::{
    AnnualAverages, EnergyConsumption, InputData, N2oEmissionFactorCalcMethod,
    N2oEmissionFactorScenario, OperatingMaterials, SewageSludgeTreatment,
};

impl TryFrom<N2oEmissionFactorScenario> for app::N2oEmissionFactorCalcMethod {
    type Error = anyhow::Error;

    fn try_from(from: N2oEmissionFactorScenario) -> Result<Self, Self::Error> {
        use app::N2oEmissionFactorCalcMethod as A;
        use N2oEmissionFactorCalcMethod as M;

        let f = match from.calculation_method {
            M::ExtrapolatedParravicini => A::ExtrapolatedParravicini,
            M::Optimistic => A::Optimistic,
            M::Pesimistic => A::Pesimistic,
            M::Ipcc2019 => A::Ipcc2019,
            M::CustomFactor => {
                let Some(factor) = from.custom_factor else {
                    bail!("custom N2O emission factor is missing");
                };
                A::Custom(app::Factor::new(factor))
            }
        };
        Ok(f)
    }
}

impl From<app::N2oEmissionFactorCalcMethod> for N2oEmissionFactorScenario {
    fn from(from: app::N2oEmissionFactorCalcMethod) -> Self {
        use app::N2oEmissionFactorCalcMethod as A;
        use N2oEmissionFactorCalcMethod as M;

        let calculation_method = match from {
            A::ExtrapolatedParravicini => M::ExtrapolatedParravicini,
            A::Optimistic => M::Optimistic,
            A::Pesimistic => M::Pesimistic,
            A::Ipcc2019 => M::Ipcc2019,
            A::Custom(_) => M::CustomFactor,
        };

        let custom_factor = if let A::Custom(factor) = from {
            Some(factor)
        } else {
            None
        }
        .map(f64::from);

        Self {
            calculation_method,
            custom_factor,
        }
    }
}

impl TryFrom<InputData> for app::Input {
    type Error = anyhow::Error;

    fn try_from(from: InputData) -> Result<Self, Self::Error> {
        let InputData {
            plant_name,
            population_values,
            waste_water,
            inflow_averages,
            effluent_averages,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        } = from;

        let Some(population_values) = population_values else {
            bail!("missing population_values");
        };

        let Some(waste_water) = waste_water else {
            bail!("missing waste_water");
        };

        let inflow_averages = inflow_averages.try_into()?;
        let effluent_averages = effluent_averages.try_into()?;
        let energy_consumption = energy_consumption.try_into()?;
        let sewage_sludge_treatment = sewage_sludge_treatment.try_into()?;
        let operating_materials = operating_materials.try_into()?;

        Ok(Self {
            plant_name,
            population_values,
            waste_water,
            inflow_averages,
            effluent_averages,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        })
    }
}

impl TryFrom<EnergyConsumption> for app::EnergyConsumption {
    type Error = anyhow::Error;

    fn try_from(from: EnergyConsumption) -> Result<Self, Self::Error> {
        let EnergyConsumption {
            sewage_gas_produced,
            methane_level,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            in_house_power_generation,
            emission_factor_electricity_mix,
        } = from;

        let Some(sewage_gas_produced) = sewage_gas_produced else {
            bail!("missing sewage_gas_produced");
        };
        let Some(methane_level) = methane_level else {
            bail!("missing methane_level");
        };
        let Some(total_power_consumption) = total_power_consumption else {
            bail!("missing total_power_consumption");
        };
        let Some(in_house_power_generation) = in_house_power_generation else {
            bail!("missing in_house_power_generation");
        };
        let Some(emission_factor_electricity_mix) = emission_factor_electricity_mix else {
            bail!("missing emission_factor_electricity_mix");
        };

        let methane_level = app::Percent::new(methane_level);

        Ok(Self {
            sewage_gas_produced,
            methane_level,
            gas_supply,
            purchase_of_biogas,
            total_power_consumption,
            in_house_power_generation,
            emission_factor_electricity_mix,
        })
    }
}

impl TryFrom<SewageSludgeTreatment> for app::SewageSludgeTreatment {
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
        Ok(Self {
            open_sludge_bags,
            open_sludge_storage_containers,
            sewage_sludge_for_disposal,
            transport_distance,
        })
    }
}

impl TryFrom<OperatingMaterials> for app::OperatingMaterials {
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

        let fecl3 = app::Tons::new(fecl3);
        let feclso4 = app::Tons::new(feclso4);
        let caoh2 = app::Tons::new(caoh2);
        let synthetic_polymers = app::Tons::new(synthetic_polymers);

        Ok(Self {
            fecl3,
            feclso4,
            caoh2,
            synthetic_polymers,
        })
    }
}

impl TryFrom<AnnualAverages> for app::AnnualAveragesInflow {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverages) -> Result<Self, Self::Error> {
        let AnnualAverages {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        } = from;

        let Some(nitrogen) = nitrogen else {
            bail!("missing inflow nitrogen");
        };
        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        })
    }
}

impl TryFrom<AnnualAverages> for app::AnnualAveragesEffluent {
    type Error = anyhow::Error;

    fn try_from(from: AnnualAverages) -> Result<Self, Self::Error> {
        let AnnualAverages {
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
        Ok(Self {
            nitrogen,
            chemical_oxygen_demand,
            phosphorus,
        })
    }
}
