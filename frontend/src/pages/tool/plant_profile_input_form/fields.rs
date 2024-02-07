use std::collections::HashMap;

use klick_app_components::forms::{self, FieldSignal};
use klick_boundary::{
    AnnualAverage, EnergyConsumption, OperatingMaterials, PlantProfile, SewageSludgeTreatment,
};
use leptos::*;

use klick_presenter::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    SewageSludgeTreatmentId,
};

use super::field_sets::Id;
use crate::forms::MissingField;

pub type RequiredField = forms::RequiredField<Id>;

pub fn read_input_fields(
    signals: &HashMap<Id, forms::FieldSignal>,
    required_fields: &Vec<RequiredField>,
) -> (PlantProfile, Vec<MissingField<Id>>) {
    let missing_fields = required_fields
        .iter()
        .filter_map(|field| {
            let Some(field_signal) = signals.get(&field.id) else {
                return None;
            };
            let is_missing = match field_signal {
                FieldSignal::Float { output, .. } => output.get().is_none(),
                FieldSignal::UnsignedInteger { output, .. } => output.get().is_none(),
                FieldSignal::Text(signal) => signal.get().is_none(),
                FieldSignal::Bool(signal) => !signal.get(),
                FieldSignal::Selection(signal) => signal.get().is_none(),
            };
            if is_missing {
                Some(MissingField::new(field.id, field.field_id.clone()))
            } else {
                None
            }
        })
        .collect();

    let plant_name = signals
        .get(&Id::PlantName.into())
        .and_then(FieldSignal::get_text);
    let population_equivalent = signals
        .get(&Id::PopulationEquivalent.into())
        .and_then(FieldSignal::get_float);
    let wastewater = signals
        .get(&Id::Wastewater.into())
        .and_then(FieldSignal::get_float);

    let influent_average = AnnualAverage {
        nitrogen: signals
            .get(&Id::from(AnnualAverageInfluentId::Nitrogen).into())
            .and_then(FieldSignal::get_float),
        chemical_oxygen_demand: signals
            .get(&Id::from(AnnualAverageInfluentId::ChemicalOxygenDemand).into())
            .and_then(FieldSignal::get_float),
        phosphorus: signals
            .get(&Id::from(AnnualAverageInfluentId::Phosphorus).into())
            .and_then(FieldSignal::get_float),
    };

    let effluent_average = AnnualAverage {
        nitrogen: signals
            .get(&Id::from(AnnualAverageEffluentId::Nitrogen).into())
            .and_then(FieldSignal::get_float),
        chemical_oxygen_demand: signals
            .get(&Id::from(AnnualAverageEffluentId::ChemicalOxygenDemand).into())
            .and_then(FieldSignal::get_float),
        phosphorus: signals
            .get(&Id::from(AnnualAverageEffluentId::Phosphorus).into())
            .and_then(FieldSignal::get_float),
    };

    let energy_consumption = EnergyConsumption {
        sewage_gas_produced: signals
            .get(&Id::from(EnergyConsumptionId::SewageGasProduced).into())
            .and_then(FieldSignal::get_float),
        methane_fraction: signals
            .get(&Id::from(EnergyConsumptionId::MethaneFraction).into())
            .and_then(FieldSignal::get_float),
        gas_supply: signals
            .get(&Id::from(EnergyConsumptionId::GasSupply).into())
            .and_then(FieldSignal::get_float),
        purchase_of_biogas: signals
            .get(&Id::from(EnergyConsumptionId::PurchaseOfBiogas).into())
            .and_then(FieldSignal::get_bool),
        total_power_consumption: signals
            .get(&Id::from(EnergyConsumptionId::TotalPowerConsumption).into())
            .and_then(FieldSignal::get_float),
        on_site_power_generation: signals
            .get(&Id::from(EnergyConsumptionId::OnSitePowerGeneration).into())
            .and_then(FieldSignal::get_float),
        emission_factor_electricity_mix: signals
            .get(&Id::from(EnergyConsumptionId::EmissionFactorElectricityMix).into())
            .and_then(FieldSignal::get_float),
    };

    let sewage_sludge_treatment = SewageSludgeTreatment {
        sludge_bags_are_open: Some(true),
        sludge_storage_containers_are_open: Some(true),
        sewage_sludge_for_disposal: signals
            .get(&Id::from(SewageSludgeTreatmentId::SewageSludgeForDisposal).into())
            .and_then(FieldSignal::get_float),
        transport_distance: signals
            .get(&Id::from(SewageSludgeTreatmentId::TransportDistance).into())
            .and_then(FieldSignal::get_float),
        digester_count: signals
            .get(&Id::from(SewageSludgeTreatmentId::DigesterCount).into())
            .and_then(FieldSignal::get_unsigned_integer),
    };

    let operating_materials = OperatingMaterials {
        fecl3: signals
            .get(&Id::from(OperatingMaterialId::FeCl3).into())
            .and_then(FieldSignal::get_float),
        feclso4: signals
            .get(&Id::from(OperatingMaterialId::FeClSO4).into())
            .and_then(FieldSignal::get_float),
        caoh2: signals
            .get(&Id::from(OperatingMaterialId::CaOH2).into())
            .and_then(FieldSignal::get_float),
        synthetic_polymers: signals
            .get(&Id::from(OperatingMaterialId::SyntheticPolymers).into())
            .and_then(FieldSignal::get_float),
    };

    (
        PlantProfile {
            plant_name,
            population_equivalent,
            wastewater,
            influent_average,
            effluent_average,
            energy_consumption,
            sewage_sludge_treatment,
            operating_materials,
        },
        missing_fields,
    )
}
