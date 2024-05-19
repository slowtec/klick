use derive_more::From;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize, From)]
pub enum ProfileValueId {
    PlantName,
    PopulationEquivalent,
    Wastewater,
    InfluentAverage(AnnualAverageInfluentId),
    EffluentAverage(AnnualAverageEffluentId),
    EnergyConsumption(EnergyConsumptionId),
    SewageSludgeTreatment(SewageSludgeTreatmentId),
    SideStreamTreatment(SideStreamTreatmentId),
    OperatingMaterials(OperatingMaterialId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum AnnualAverageInfluentId {
    Nitrogen,
    ChemicalOxygenDemand,
    TotalOrganicCarbohydrates,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum AnnualAverageEffluentId {
    Nitrogen,
    ChemicalOxygenDemand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum EnergyConsumptionId {
    SewageGasProduced,
    MethaneFraction,
    GasSupply,
    PurchaseOfBiogas,
    TotalPowerConsumption,
    OnSitePowerGeneration,
    EmissionFactorElectricityMix,
    HeatingOil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum SideStreamTreatmentId {
    TotalNitrogen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum SewageSludgeTreatmentId {
    SludgeBags,
    SludgeStorageContainers,
    SewageSludgeForDisposal,
    TransportDistance,
    DigesterCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum OperatingMaterialId {
    FeCl3,
    FeClSO4,
    CaOH2,
    SyntheticPolymers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum ScenarioFieldId {
    N2OSideStreamFactor,
    N2OSideStreamCoverIsOpen,
    ProcessEnergySaving,
    FossilEnergySaving,
    DistrictHeating,
    PhotovoltaicEnergyExpansion,
    EstimatedSelfPhotovolaticUsage,
    WindEnergyExpansion,
    EstimatedSelfWindEnergyUsage,
    WaterEnergyExpansion,
    EstimatedSelfWaterEnergyUsage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum SensitivityParameterId {
    N2OCalculationMethod,
    N2OCustomFactor,
    N2OSideStreamFactor,
    CH4ChpCalculationMethod,
    CH4ChpCustomFactor,
    CO2FossilCustomFactor,
    SludgeBagsCustomFactor,
    SludgeStorageCustomFactor,
}
