#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputValueId {
    ProjectName,
    PlantName,
    PopulationEquivalent,
    Wastewater,
    InfluentNitrogen,
    InfluentChemicalOxygenDemand,
    InfluentTotalOrganicCarbohydrates,
    EffluentNitrogen,
    EffluentChemicalOxygenDemand,
    SideStreamTreatmentTotalNitrogen,
    OperatingMaterialFeCl3,
    OperatingMaterialFeClSO4,
    OperatingMaterialCaOH2,
    OperatingMaterialSyntheticPolymers,
    SensitivityN2OCalculationMethod,
    SensitivityN2OCustomFactor,
    SensitivityN2OSideStreamFactor,
    SensitivityCH4ChpCalculationMethod,
    SensitivityCH4ChpCustomFactor,
    SensitivityCO2FossilCustomFactor,
    SensitivitySludgeBagsCustomFactor,
    SensitivitySludgeStorageCustomFactor,
    SludgeTreatmentBagsAreOpen,
    SludgeTreatmentStorageContainersAreOpen,
    SludgeTreatmentDisposal,
    SludgeTreatmentTransportDistance,
    SludgeTreatmentDigesterCount,
    SewageGasProduced,
    MethaneFraction,
    GasSupply,
    PurchaseOfBiogas,
    TotalPowerConsumption,
    OnSitePowerGeneration,
    EmissionFactorElectricityMix,
    HeatingOil,
    ScenarioSludgeBagsAreOpen,
    ScenarioSludgeStorageContainersAreOpen,
    ScenarioN2OSideStreamFactor,
    ScenarioN2OSideStreamCoverIsOpen,
    ScenarioProcessEnergySaving,
    ScenarioFossilEnergySaving,
    ScenarioDistrictHeating,
    ScenarioPhotovoltaicEnergyExpansion,
    ScenarioEstimatedSelfPhotovolaticUsage,
    ScenarioWindEnergyExpansion,
    ScenarioEstimatedSelfWindEnergyUsage,
    ScenarioWaterEnergyExpansion,
    ScenarioEstimatedSelfWaterEnergyUsage,
}