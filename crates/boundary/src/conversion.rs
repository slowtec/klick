use klick_domain::{self as domain};

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

impl From<SavedProject> for domain::Project<JsonFormData> {
    fn from(from: SavedProject) -> Self {
        let SavedProject {
            id,
            created_at,
            modified_at,
            form_data: data,
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

impl From<domain::Project<JsonFormData>> for SavedProject {
    fn from(from: domain::Project<JsonFormData>) -> Self {
        let domain::Project {
            id,
            created_at,
            modified_at,
            data: form_data,
        } = from;

        let id = id.into();

        Self {
            id,
            created_at,
            modified_at,
            form_data,
        }
    }
}

impl From<domain::Project<JsonFormData>> for Project {
    fn from(from: domain::Project<JsonFormData>) -> Self {
        Self::Saved(from.into())
    }
}

// -----   ----- //
//    Values     //
// -----   ----- //

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

impl TryFrom<JsonFormData> for HashMap<domain::InputValueId, domain::Value> {
    type Error = anyhow::Error;

    fn try_from(from: JsonFormData) -> Result<Self, Self::Error> {
        from.0
            .into_iter()
            .filter(|(_, value)| !value.is_null())
            .map(|(id, value)| {
                id.value_from_json(value)
                    .map(|domain_value| (id.into(), domain_value))
            })
            .collect::<Result<HashMap<_, _>, _>>()
    }
}

impl TryFrom<v8::FormData> for JsonFormData {
    type Error = anyhow::Error;

    fn try_from(from: v8::FormData) -> Result<Self, Self::Error> {
        Ok(from
            .into_iter()
            .map(|(id, value)| {
                let id = v9::InputValueId::from(id);
                id.value_to_json(value).map(|json_value| (id, json_value))
            })
            .collect::<Result<HashMap<_, _>, _>>()?
            .into())
    }
}

impl From<v9::InputValueId> for domain::InputValueId {
    fn from(from: v9::InputValueId) -> Self {
        use v9::InputValueId as In;
        match from {
            In::ProjectName => Self::ProjectName,
            In::PlantName => Self::PlantName,
            In::PopulationEquivalent => Self::PopulationEquivalent,
            In::Wastewater => Self::Wastewater,
            In::InfluentNitrogen => Self::InfluentNitrogen,
            In::InfluentChemicalOxygenDemand => Self::InfluentChemicalOxygenDemand,
            In::InfluentTotalOrganicCarbohydrates => Self::InfluentTotalOrganicCarbohydrates,
            In::EffluentNitrogen => Self::EffluentNitrogen,
            In::EffluentChemicalOxygenDemand => Self::EffluentChemicalOxygenDemand,
            In::SewageGasProduced => Self::SewageGasProduced,
            In::MethaneFraction => Self::MethaneFraction,
            In::GasSupply => Self::GasSupply,
            In::PurchaseOfBiogas => Self::PurchaseOfBiogas,
            In::TotalPowerConsumption => Self::TotalPowerConsumption,
            In::OnSitePowerGeneration => Self::OnSitePowerGeneration,
            In::EmissionFactorElectricityMix => Self::EmissionFactorElectricityMix,
            In::HeatingOil => Self::HeatingOil,
            In::SideStreamTreatmentTotalNitrogen => Self::SideStreamTreatmentTotalNitrogen,
            In::OperatingMaterialFeCl3 => Self::OperatingMaterialFeCl3,
            In::OperatingMaterialFeClSO4 => Self::OperatingMaterialFeClSO4,
            In::OperatingMaterialCaOH2 => Self::OperatingMaterialCaOH2,
            In::OperatingMaterialSyntheticPolymers => Self::OperatingMaterialSyntheticPolymers,
            In::SensitivityN2OCalculationMethod => Self::SensitivityN2OCalculationMethod,
            In::SensitivityN2OCustomFactor => Self::SensitivityN2OCustomFactor,
            In::SensitivityN2OSideStreamFactor => Self::SensitivityN2OSideStreamFactor,
            In::SensitivityCH4ChpCalculationMethod => Self::SensitivityCH4ChpCalculationMethod,
            In::SensitivityCH4ChpCustomFactor => Self::SensitivityCH4ChpCustomFactor,
            In::SensitivityCO2FossilCustomFactor => Self::SensitivityCO2FossilCustomFactor,
            In::SensitivitySludgeBagsCustomFactor => Self::SensitivitySludgeBagsCustomFactor,
            In::SensitivitySludgeStorageCustomFactor => Self::SensitivitySludgeStorageCustomFactor,
            In::SludgeTreatmentBagsAreOpen => Self::SludgeTreatmentBagsAreOpen,
            In::SludgeTreatmentStorageContainersAreOpen => {
                Self::SludgeTreatmentStorageContainersAreOpen
            }
            In::SludgeTreatmentDisposal => Self::SludgeTreatmentDisposal,
            In::SludgeTreatmentTransportDistance => Self::SludgeTreatmentTransportDistance,
            In::SludgeTreatmentDigesterCount => Self::SludgeTreatmentDigesterCount,
            In::ScenarioSludgeBagsAreOpen => Self::ScenarioSludgeBagsAreOpen,
            In::ScenarioSludgeStorageContainersAreOpen => {
                Self::ScenarioSludgeStorageContainersAreOpen
            }
            In::ScenarioN2OSideStreamFactor => Self::ScenarioN2OSideStreamFactor,
            In::ScenarioN2OSideStreamCoverIsOpen => Self::ScenarioN2OSideStreamCoverIsOpen,
            In::ScenarioProcessEnergySaving => Self::ScenarioProcessEnergySaving,
            In::ScenarioFossilEnergySaving => Self::ScenarioFossilEnergySaving,
            In::ScenarioDistrictHeating => Self::ScenarioDistrictHeating,
            In::ScenarioPhotovoltaicEnergyExpansion => Self::ScenarioPhotovoltaicEnergyExpansion,
            In::ScenarioEstimatedSelfPhotovolaticUsage => {
                Self::ScenarioEstimatedSelfPhotovolaticUsage
            }
            In::ScenarioWindEnergyExpansion => Self::ScenarioWindEnergyExpansion,
            In::ScenarioEstimatedSelfWindEnergyUsage => Self::ScenarioEstimatedSelfWindEnergyUsage,
            In::ScenarioWaterEnergyExpansion => Self::ScenarioWaterEnergyExpansion,
            In::ScenarioEstimatedSelfWaterEnergyUsage => {
                Self::ScenarioEstimatedSelfWaterEnergyUsage
            }
            In::AdditionalCustomEmissions => Self::AdditionalCustomEmissions,
        }
    }
}

impl From<domain::InputValueId> for v9::InputValueId {
    fn from(from: domain::InputValueId) -> Self {
        use domain::InputValueId as In;
        match from {
            In::ProjectName => Self::ProjectName,
            In::PlantName => Self::PlantName,
            In::PopulationEquivalent => Self::PopulationEquivalent,
            In::Wastewater => Self::Wastewater,
            In::InfluentNitrogen => Self::InfluentNitrogen,
            In::InfluentChemicalOxygenDemand => Self::InfluentChemicalOxygenDemand,
            In::InfluentTotalOrganicCarbohydrates => Self::InfluentTotalOrganicCarbohydrates,
            In::EffluentNitrogen => Self::EffluentNitrogen,
            In::EffluentChemicalOxygenDemand => Self::EffluentChemicalOxygenDemand,
            In::SewageGasProduced => Self::SewageGasProduced,
            In::MethaneFraction => Self::MethaneFraction,
            In::GasSupply => Self::GasSupply,
            In::PurchaseOfBiogas => Self::PurchaseOfBiogas,
            In::TotalPowerConsumption => Self::TotalPowerConsumption,
            In::OnSitePowerGeneration => Self::OnSitePowerGeneration,
            In::EmissionFactorElectricityMix => Self::EmissionFactorElectricityMix,
            In::HeatingOil => Self::HeatingOil,
            In::SideStreamTreatmentTotalNitrogen => Self::SideStreamTreatmentTotalNitrogen,
            In::OperatingMaterialFeCl3 => Self::OperatingMaterialFeCl3,
            In::OperatingMaterialFeClSO4 => Self::OperatingMaterialFeClSO4,
            In::OperatingMaterialCaOH2 => Self::OperatingMaterialCaOH2,
            In::OperatingMaterialSyntheticPolymers => Self::OperatingMaterialSyntheticPolymers,
            In::SensitivityN2OCalculationMethod => Self::SensitivityN2OCalculationMethod,
            In::SensitivityN2OCustomFactor => Self::SensitivityN2OCustomFactor,
            In::SensitivityN2OSideStreamFactor => Self::SensitivityN2OSideStreamFactor,
            In::SensitivityCH4ChpCalculationMethod => Self::SensitivityCH4ChpCalculationMethod,
            In::SensitivityCH4ChpCustomFactor => Self::SensitivityCH4ChpCustomFactor,
            In::SensitivityCO2FossilCustomFactor => Self::SensitivityCO2FossilCustomFactor,
            In::SensitivitySludgeBagsCustomFactor => Self::SensitivitySludgeBagsCustomFactor,
            In::SensitivitySludgeStorageCustomFactor => Self::SensitivitySludgeStorageCustomFactor,
            In::SludgeTreatmentBagsAreOpen => Self::SludgeTreatmentBagsAreOpen,
            In::SludgeTreatmentStorageContainersAreOpen => {
                Self::SludgeTreatmentStorageContainersAreOpen
            }
            In::SludgeTreatmentDisposal => Self::SludgeTreatmentDisposal,
            In::SludgeTreatmentTransportDistance => Self::SludgeTreatmentTransportDistance,
            In::SludgeTreatmentDigesterCount => Self::SludgeTreatmentDigesterCount,
            In::ScenarioSludgeBagsAreOpen => Self::ScenarioSludgeBagsAreOpen,
            In::ScenarioSludgeStorageContainersAreOpen => {
                Self::ScenarioSludgeStorageContainersAreOpen
            }
            In::ScenarioN2OSideStreamFactor => Self::ScenarioN2OSideStreamFactor,
            In::ScenarioN2OSideStreamCoverIsOpen => Self::ScenarioN2OSideStreamCoverIsOpen,
            In::ScenarioProcessEnergySaving => Self::ScenarioProcessEnergySaving,
            In::ScenarioFossilEnergySaving => Self::ScenarioFossilEnergySaving,
            In::ScenarioDistrictHeating => Self::ScenarioDistrictHeating,
            In::ScenarioPhotovoltaicEnergyExpansion => Self::ScenarioPhotovoltaicEnergyExpansion,
            In::ScenarioEstimatedSelfPhotovolaticUsage => {
                Self::ScenarioEstimatedSelfPhotovolaticUsage
            }
            In::ScenarioWindEnergyExpansion => Self::ScenarioWindEnergyExpansion,
            In::ScenarioEstimatedSelfWindEnergyUsage => Self::ScenarioEstimatedSelfWindEnergyUsage,
            In::ScenarioWaterEnergyExpansion => Self::ScenarioWaterEnergyExpansion,
            In::ScenarioEstimatedSelfWaterEnergyUsage => {
                Self::ScenarioEstimatedSelfWaterEnergyUsage
            }
            In::AdditionalCustomEmissions => Self::AdditionalCustomEmissions,
        }
    }
}
