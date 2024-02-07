use derive_more::From;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

// TODO:
// Actually, we should derive the IDs directly from the domain layer.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize, From)]
pub enum ProfileValueId {
    PlantName,
    PopulationEquivalent,
    Wastewater,
    InfluentAverage(AnnualAverageInfluentId),
    EffluentAverage(AnnualAverageEffluentId),
    EnergyConsumption(EnergyConsumptionId),
    SewageSludgeTreatment(SewageSludgeTreatmentId),
    OperatingMaterials(OperatingMaterialId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum AnnualAverageInfluentId {
    Nitrogen,
    ChemicalOxygenDemand,
    Phosphorus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum AnnualAverageEffluentId {
    Nitrogen,
    ChemicalOxygenDemand,
    Phosphorus,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, AsRefStr, Serialize, Deserialize)]
pub enum SewageSludgeTreatmentId {
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
