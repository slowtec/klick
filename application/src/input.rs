use crate::{
    GramsPerKilowatthour, Kilometers, Kilowatthours, MilligramsPerLiter, Percent, Qubicmeters, Tons,
};

#[derive(Debug, Clone)]
pub struct Input {
    pub plant_name: Option<String>,
    pub population_values: f64,
    pub waste_water: Qubicmeters,
    pub inflow_averages: AnnualAveragesInflow,
    pub effluent_averages: AnnualAveragesEffluent,
    pub energy_consumption: EnergyConsumption,
    pub sewage_sludge_treatment: SewageSludgeTreatment,
    pub operating_materials: OperatingMaterials,
}

#[derive(Debug, Clone)]
pub struct AnnualAveragesInflow {
    pub nitrogen: MilligramsPerLiter,
    pub chemical_oxygen_demand: Option<MilligramsPerLiter>,
    pub phosphorus: Option<MilligramsPerLiter>,
}

#[derive(Debug, Clone)]
pub struct AnnualAveragesEffluent {
    pub nitrogen: MilligramsPerLiter,
    pub chemical_oxygen_demand: MilligramsPerLiter,
    pub phosphorus: Option<MilligramsPerLiter>,
}

#[derive(Debug, Clone)]
pub struct EnergyConsumption {
    pub sewage_gas_produced: Qubicmeters,
    pub methane_level: Percent,
    pub gas_supply: Option<Kilowatthours>,
    pub purchase_of_biogas: Option<bool>,
    pub total_power_consumption: Kilowatthours,
    pub in_house_power_generation: Kilowatthours,
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
