use std::collections::{HashMap, HashSet};

use klick_domain::{CalculationOutcome, InputValueId as In, Value, ValueId as Id};

use crate::calculate_emissions;

const RECOMMENDATION_IDS: &[In] = &[
    In::ProjectName,
    In::ProfilePlantName,
    In::ProfilePopulationEquivalent,
    In::ProfileWastewater,
    In::ProfileInfluentNitrogen,
    In::ProfileInfluentChemicalOxygenDemand,
    In::ProfileInfluentTotalOrganicCarbohydrates,
    In::ProfileEffluentNitrogen,
    In::ProfileEffluentChemicalOxygenDemand,
    In::ProfileSewageGasProduced,
    In::ProfileMethaneFraction,
    In::ProfileGasSupply,
    In::ProfilePurchaseOfBiogas,
    In::ProfileTotalPowerConsumption,
    In::ProfileOnSitePowerGeneration,
    In::ProfileEmissionFactorElectricityMix,
    In::ProfileHeatingOil,
    In::ProfileSideStreamTotalNitrogen,
    In::ProfileOperatingMaterialFeCl3,
    In::ProfileOperatingMaterialFeClSO4,
    In::ProfileOperatingMaterialCaOH2,
    In::ProfileOperatingMaterialSyntheticPolymers,
    In::ProfileSludgeBagsAreOpen,
    In::ProfileSludgeStorageContainersAreOpen,
    In::ProfileSludgeDisposal,
    In::ProfileSludgeTransportDistance,
    In::ProfileSludgeDigesterCount,
    In::SensitivityN2OCalculationMethod,
    In::SensitivityCH4ChpCalculationMethod,
    In::SensitivityN2OCustomFactor,
    In::SensitivityN2OSideStreamFactor,
    In::SensitivityCH4ChpCustomFactor,
    In::SensitivityCO2FossilCustomFactor,
    In::SensitivitySludgeBagsCustomFactor,
    In::SensitivitySludgeStorageCustomFactor,
    In::RecommendationSludgeBagsAreOpen,
    In::RecommendationSludgeStorageContainersAreOpen,
    In::RecommendationN2OSideStreamCoverIsOpen,
    In::RecommendationProcessEnergySaving,
    In::RecommendationFossilEnergySaving,
    In::RecommendationDistrictHeating,
    In::RecommendationPhotovoltaicEnergyExpansion,
    In::RecommendationEstimatedSelfPhotovolaticUsage,
    In::RecommendationWindEnergyExpansion,
    In::RecommendationEstimatedSelfWindEnergyUsage,
    In::RecommendationWaterEnergyExpansion,
    In::RecommendationEstimatedSelfWaterEnergyUsage,
];

pub fn calculate_recommendation(
    form_data: HashMap<Id, Value>,
    custom_edges: Option<&[(Id, Id)]>,
    custom_leafs: Vec<Id>,
) -> CalculationOutcome {
    let recommendation_ids: HashSet<_> = RECOMMENDATION_IDS.iter().copied().map(Id::from).collect();
    let values = form_data
        .into_iter()
        .filter(|(i, _)| recommendation_ids.contains(i) || i.is_custom())
        .collect();

    calculate_emissions(&values, custom_edges, custom_leafs)
}
