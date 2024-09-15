use std::collections::{HashMap, HashSet};

use klick_domain::{CalculationOutcome, InputValueId as In, Value, ValueId as Id};

use crate::calculate_emissions;

const PROFILE_IDS: &[In] = &[
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
];

pub fn calculate_profile(form_data: HashMap<In, Value>) -> CalculationOutcome {
    let values: HashMap<_, _> = form_data
        .into_iter()
        .map(|(id, value)| (Id::from(id), value))
        .collect();

    let custom_leafs = vec![];
    let custom_edges = None;

    let profile_ids: HashSet<_> = PROFILE_IDS.iter().copied().map(Id::from).collect();

    let values: HashMap<_, _> = values
        .into_iter()
        .filter(|(i, _)| profile_ids.contains(i))
        .collect();

    calculate_emissions(&values, custom_edges, custom_leafs)
}
