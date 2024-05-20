#[allow(clippy::wildcard_imports)]
use crate::units::*;

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct EmissionInfluencingValues {
    pub population_equivalent: f64,
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
