use crate::{
    GramsPerKilowatthour, Kilometers, Kilowatthours, MilligramsPerLiter, Percent, Qubicmeters, Tons,
};

#[derive(Debug, Clone)]
pub struct Input {
    pub plant_name: Option<String>,
    pub population_equivalent: f64,
    pub wastewater: Qubicmeters,
    pub influent_average: AnnualAverageInfluent,
    pub effluent_average: AnnualAverageEffluent,
    pub energy_consumption: EnergyConsumption,
    pub sewage_sludge_treatment: SewageSludgeTreatment,
    pub operating_materials: OperatingMaterials,
}

#[derive(Debug, Clone)]
pub struct AnnualAverageInfluent {
    pub nitrogen: MilligramsPerLiter,
    pub chemical_oxygen_demand: Option<MilligramsPerLiter>,
    pub phosphorus: Option<MilligramsPerLiter>,
}

#[derive(Debug, Clone)]
pub struct AnnualAverageEffluent {
    pub nitrogen: MilligramsPerLiter,
    pub chemical_oxygen_demand: MilligramsPerLiter,
    pub phosphorus: Option<MilligramsPerLiter>,
}

#[derive(Debug, Clone)]
pub struct EnergyConsumption {
    pub sewage_gas_produced: Qubicmeters,
    pub methane_fraction: Percent,
    pub gas_supply: Option<Kilowatthours>,
    pub purchase_of_biogas: Option<bool>,
    pub total_power_consumption: Kilowatthours,
    pub on_site_power_generation: Kilowatthours,
    pub emission_factor_electricity_mix: GramsPerKilowatthour,
}

#[derive(Debug, Clone)]
pub struct SewageSludgeTreatment {
    pub open_sludge_bags: bool,
    pub open_sludge_storage_containers: bool,
    pub sewage_sludge_for_disposal: Tons,
    pub transport_distance: Kilometers,
}

#[derive(Debug, Clone)]
pub struct OperatingMaterials {
    pub fecl3: Tons,
    pub feclso4: Tons,
    pub caoh2: Tons,
    pub synthetic_polymers: Tons,
}
