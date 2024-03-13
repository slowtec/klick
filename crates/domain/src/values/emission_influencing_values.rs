use crate::{
    units::{
        GramsPerKilowatthour, Kilometers, Kilowatthours, Liters, MilligramsPerLiter, Percent,
        Qubicmeters, QubicmetersPerHour, Tons,
    },
    EmissionFactors,
};

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct EmissionInfluencingValues {
    pub population_equivalent: f64,
    pub wastewater: Qubicmeters,
    pub influent_average: AnnualAverageInfluent,
    pub effluent_average: AnnualAverageEffluent,
    pub energy_consumption: EnergyConsumption,
    pub sewage_sludge_treatment: SewageSludgeTreatment,
    pub side_stream_treatment: SideStreamTreatment,
    pub operating_materials: OperatingMaterials,
    pub emission_factors: EmissionFactors,
    pub energy_emission_factors: EnergyEmissionFactors,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct AnnualAverageInfluent {
    pub nitrogen: MilligramsPerLiter,
    pub chemical_oxygen_demand: MilligramsPerLiter,
    pub total_organic_carbohydrates: MilligramsPerLiter,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct AnnualAverageEffluent {
    pub nitrogen: MilligramsPerLiter,
    pub chemical_oxygen_demand: MilligramsPerLiter,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct EnergyConsumption {
    pub sewage_gas_produced: Qubicmeters,
    pub methane_fraction: Percent,
    pub total_power_consumption: Kilowatthours,
    pub on_site_power_generation: Kilowatthours,
    pub emission_factor_electricity_mix: GramsPerKilowatthour,
    pub heating_oil: Liters,
    pub gas_supply: Qubicmeters,
    pub purchase_of_biogas: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct SideStreamTreatment {
    pub total_nitrogen: Tons,
    pub side_stream_cover_is_open: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct SewageSludgeTreatment {
    pub sludge_bags_are_open: bool,
    pub sludge_bags_factor: Option<QubicmetersPerHour>,
    pub sludge_storage_containers_are_open: bool,
    pub sludge_storage_containers_factor: Option<Percent>,
    pub sewage_sludge_for_disposal: Tons,
    pub transport_distance: Kilometers,
    pub digester_count: Option<u64>,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct OperatingMaterials {
    pub fecl3: Tons,
    pub feclso4: Tons,
    pub caoh2: Tons,
    pub synthetic_polymers: Tons,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct EnergyEmissionFactors {
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
