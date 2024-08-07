use std::collections::HashMap;

use thiserror::Error;

#[allow(clippy::wildcard_imports)]
use crate::{units::*, value_spec, InputValueId as Id, Value as V};

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

    SewageGasProduced,
    MethaneFraction,
    GasSupply,
    PurchaseOfBiogas,
    TotalPowerConsumption,
    OnSitePowerGeneration,
    EmissionFactorElectricityMix,
    HeatingOil,

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

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct EmissionInfluencingValues {
    pub population_equivalent: Count,
    pub wastewater: Qubicmeters,

    pub influent_nitrogen: MilligramsPerLiter,
    pub influent_chemical_oxygen_demand: MilligramsPerLiter,
    pub influent_total_organic_carbohydrates: MilligramsPerLiter,

    pub effluent_nitrogen: MilligramsPerLiter,
    pub effluent_chemical_oxygen_demand: MilligramsPerLiter,

    pub sewage_gas_produced: Qubicmeters,
    pub methane_fraction: Percent,
    pub total_power_consumption: Kilowatthours,
    pub on_site_power_generation: Kilowatthours,
    pub emission_factor_electricity_mix: GramsPerKilowatthour,
    pub heating_oil: Liters,
    pub gas_supply: Qubicmeters,
    pub purchase_of_biogas: bool,

    pub sludge_bags_are_open: bool,
    pub sludge_bags_factor: Option<QubicmetersPerHour>,
    pub sludge_storage_containers_are_open: bool,
    pub sludge_storage_containers_factor: Option<Percent>,
    pub sewage_sludge_for_disposal: Tons,
    pub transport_distance: Kilometers,
    pub digester_count: Option<u64>,

    pub side_stream_treatment_total_nitrogen: Tons,
    pub side_stream_cover_is_open: bool,

    pub operating_material_fecl3: Tons,
    pub operating_material_feclso4: Tons,
    pub operating_material_caoh2: Tons,
    pub operating_material_synthetic_polymers: Tons,

    pub emission_factor_n2o_side_stream: Factor,
    pub emission_factor_co2_fossil: Factor,

    pub process_energy_savings: Percent,
    pub fossil_energy_savings: Percent,
    pub district_heating: Kilowatthours,
    pub photovoltaic_energy_expansion: Kilowatthours,
    pub estimated_self_photovoltaic_usage: Percent,
    pub wind_energy_expansion: Kilowatthours,
    pub estimated_self_wind_energy_usage: Percent,
    pub water_energy_expansion: Kilowatthours,
    pub estimated_self_water_energy_usage: Percent,
}

#[derive(Debug, Error)]
#[error("The required value ({0:?}) is missing")]
pub struct MissingValueError(Id);

impl TryFrom<HashMap<Id, V>> for EmissionInfluencingValues {
    type Error = MissingValueError;

    fn try_from(mut from: HashMap<Id, Value>) -> Result<Self, Self::Error> {
        let population_equivalent =
            required_value(Id::PopulationEquivalent, &mut from)?.as_count_unchecked();
        let wastewater = required_value(Id::Wastewater, &mut from)?.as_qubicmeters_unchecked();
        let influent_nitrogen =
            required_value(Id::InfluentNitrogen, &mut from)?.as_milligrams_per_liter_unchecked();
        let influent_chemical_oxygen_demand =
            required_value(Id::InfluentChemicalOxygenDemand, &mut from)?
                .as_milligrams_per_liter_unchecked();
        let influent_total_organic_carbohydrates =
            required_value(Id::InfluentTotalOrganicCarbohydrates, &mut from)?
                .as_milligrams_per_liter_unchecked();

        let effluent_nitrogen =
            required_value(Id::EffluentNitrogen, &mut from)?.as_milligrams_per_liter_unchecked();
        let effluent_chemical_oxygen_demand =
            required_value(Id::EffluentChemicalOxygenDemand, &mut from)?
                .as_milligrams_per_liter_unchecked();

        let sewage_gas_produced =
            required_value(Id::SewageGasProduced, &mut from)?.as_qubicmeters_unchecked();
        let methane_fraction =
            required_value(Id::MethaneFraction, &mut from)?.as_percent_unchecked();
        let total_power_consumption =
            required_value(Id::TotalPowerConsumption, &mut from)?.as_kilowatthours_unchecked();
        let on_site_power_generation =
            required_value(Id::OnSitePowerGeneration, &mut from)?.as_kilowatthours_unchecked();
        let emission_factor_electricity_mix =
            required_value(Id::EmissionFactorElectricityMix, &mut from)?
                .as_grams_per_kilowatthour_unchecked();
        let heating_oil = required_value(Id::HeatingOil, &mut from)?.as_liters_unchecked();
        let gas_supply = required_value(Id::GasSupply, &mut from)?.as_qubicmeters_unchecked();
        let purchase_of_biogas =
            required_value(Id::PurchaseOfBiogas, &mut from)?.as_bool_unchecked();

        let sludge_bags_are_open =
            required_value(Id::SludgeTreatmentBagsAreOpen, &mut from)?.as_bool_unchecked();
        let sludge_bags_factor = optional_value(Id::SensitivitySludgeBagsCustomFactor, &mut from)
            .map(V::as_qubicmeters_per_hour_unchecked);
        let sludge_storage_containers_are_open =
            required_value(Id::SludgeTreatmentStorageContainersAreOpen, &mut from)?
                .as_bool_unchecked();
        let sludge_storage_containers_factor =
            optional_value(Id::SensitivitySludgeStorageCustomFactor, &mut from)
                .map(V::as_percent_unchecked);
        let sewage_sludge_for_disposal =
            required_value(Id::SludgeTreatmentDisposal, &mut from)?.as_tons_unchecked();
        let transport_distance = required_value(Id::SludgeTreatmentTransportDistance, &mut from)?
            .as_kilometers_unchecked();
        let digester_count: Option<u64> =
            optional_value(Id::SludgeTreatmentDigesterCount, &mut from)
                .map(V::as_count_unchecked)
                .map(Into::into);

        let side_stream_treatment_total_nitrogen =
            required_value(Id::SideStreamTreatmentTotalNitrogen, &mut from)?.as_tons_unchecked();
        let side_stream_cover_is_open =
            required_value(Id::ScenarioN2OSideStreamCoverIsOpen, &mut from)?.as_bool_unchecked();

        let operating_material_fecl3 =
            required_value(Id::OperatingMaterialFeCl3, &mut from)?.as_tons_unchecked();
        let operating_material_feclso4 =
            required_value(Id::OperatingMaterialFeClSO4, &mut from)?.as_tons_unchecked();
        let operating_material_caoh2 =
            required_value(Id::OperatingMaterialCaOH2, &mut from)?.as_tons_unchecked();
        let operating_material_synthetic_polymers =
            required_value(Id::OperatingMaterialSyntheticPolymers, &mut from)?.as_tons_unchecked();

        let emission_factor_n2o_side_stream =
            required_value(Id::SensitivityN2OSideStreamFactor, &mut from)?
                .as_percent_unchecked()
                .convert_to::<Factor>();
        let emission_factor_co2_fossil =
            required_value(Id::SensitivityCO2FossilCustomFactor, &mut from)?
                .as_percent_unchecked()
                .convert_to::<Factor>();

        let process_energy_savings =
            required_value(Id::ScenarioProcessEnergySaving, &mut from)?.as_percent_unchecked();
        let fossil_energy_savings =
            required_value(Id::ScenarioFossilEnergySaving, &mut from)?.as_percent_unchecked();
        let district_heating =
            required_value(Id::ScenarioDistrictHeating, &mut from)?.as_kilowatthours_unchecked();
        let photovoltaic_energy_expansion =
            required_value(Id::ScenarioPhotovoltaicEnergyExpansion, &mut from)?
                .as_kilowatthours_unchecked();
        let estimated_self_photovoltaic_usage =
            required_value(Id::ScenarioEstimatedSelfPhotovolaticUsage, &mut from)?
                .as_percent_unchecked();
        let wind_energy_expansion = required_value(Id::ScenarioWindEnergyExpansion, &mut from)?
            .as_kilowatthours_unchecked();
        let estimated_self_wind_energy_usage =
            required_value(Id::ScenarioEstimatedSelfWindEnergyUsage, &mut from)?
                .as_percent_unchecked();
        let water_energy_expansion = required_value(Id::ScenarioWaterEnergyExpansion, &mut from)?
            .as_kilowatthours_unchecked();
        let estimated_self_water_energy_usage =
            required_value(Id::ScenarioEstimatedSelfWaterEnergyUsage, &mut from)?
                .as_percent_unchecked();

        let input_values = Self {
            population_equivalent,
            wastewater,

            influent_nitrogen,
            influent_chemical_oxygen_demand,
            influent_total_organic_carbohydrates,

            effluent_nitrogen,
            effluent_chemical_oxygen_demand,

            sewage_gas_produced,
            methane_fraction,
            total_power_consumption,
            on_site_power_generation,
            emission_factor_electricity_mix,
            heating_oil,
            gas_supply,
            purchase_of_biogas,

            sludge_bags_are_open,
            sludge_bags_factor,
            sludge_storage_containers_are_open,
            sludge_storage_containers_factor,
            sewage_sludge_for_disposal,
            transport_distance,
            digester_count,

            side_stream_treatment_total_nitrogen,
            side_stream_cover_is_open,

            operating_material_fecl3,
            operating_material_feclso4,
            operating_material_caoh2,
            operating_material_synthetic_polymers,

            emission_factor_n2o_side_stream,
            emission_factor_co2_fossil,

            process_energy_savings,
            fossil_energy_savings,
            district_heating,
            photovoltaic_energy_expansion,
            estimated_self_photovoltaic_usage,
            wind_energy_expansion,
            estimated_self_wind_energy_usage,
            water_energy_expansion,
            estimated_self_water_energy_usage,
        };

        Ok(input_values)
    }
}

// TODO: make this private
pub fn required_value(id: Id, map: &mut HashMap<Id, V>) -> Result<V, MissingValueError> {
    let spec = value_spec(&id);
    let value = map
        .remove(&id)
        .or_else(|| spec.default_value().cloned())
        .ok_or(MissingValueError(id))?;
    debug_assert_eq!(spec.value_type(), value.value_type());
    Ok(value)
}

// TODO: make this private
pub fn optional_value(id: Id, map: &mut HashMap<Id, V>) -> Option<V> {
    let spec = value_spec(&id);
    let value = map.remove(&id).or_else(|| spec.default_value().cloned());
    debug_assert!(value
        .as_ref()
        .map(|v| spec.value_type() == v.value_type())
        .unwrap_or(true));
    value
}
