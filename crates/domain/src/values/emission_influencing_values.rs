use crate::units::{
    GramsPerKilowatthour, Kilometers, Kilowatthours, MilligramsPerLiter, Percent, Qubicmeters, Tons,
};

use crate::CustomEmissionFactors;

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
    pub emission_factors: CustomEmissionFactors,
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
    pub heating_oil: Qubicmeters,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct SideStreamTreatment {
    pub total_nitrogen: Tons,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct SewageSludgeTreatment {
    pub sludge_bags_are_open: bool,
    pub sludge_bags_are_open_recommendation: bool,
    pub custom_sludge_bags_factor: Option<f64>,
    pub sludge_storage_containers_are_open: bool,
    pub sludge_storage_containers_are_open_recommendation: bool,
    pub custom_sludge_storage_containers_factor: Option<f64>,
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

impl EmissionInfluencingValues {
    pub fn to_csv(&self) -> String {
        let mut output: String = String::new();
        // make this multiple lines
        output += &format!(
            "population_equivalent, {}\n",
            f64::from(self.population_equivalent)
        );
        output += &format!("wastewater, {}\n", f64::from(self.wastewater));
        output += &format!(
            "influent_average.nitrogen, {}\n",
            f64::from(self.influent_average.nitrogen)
        );
        output += &format!(
            "effluent_average.nitrogen, {}\n",
            f64::from(self.effluent_average.nitrogen)
        );
        output += &format!(
            "effluent_average.chemical_oxygen_demand, {}\n",
            f64::from(self.effluent_average.chemical_oxygen_demand)
        );
        output += &format!(
            "energy_consumption.sewage_gas_produced, {}\n",
            f64::from(self.energy_consumption.sewage_gas_produced)
        );
        output += &format!(
            "energy_consumption.methane_fraction, {}\n",
            f64::from(self.energy_consumption.methane_fraction)
        );
        output += &format!(
            "energy_consumption.total_power_consumption, {}\n",
            f64::from(self.energy_consumption.total_power_consumption)
        );
        output += &format!(
            "energy_consumption.on_site_power_generation, {}\n",
            f64::from(self.energy_consumption.on_site_power_generation)
        );
        output += &format!(
            "energy_consumption.emission_factor_electricity_mix, {}\n",
            f64::from(self.energy_consumption.emission_factor_electricity_mix)
        );
        output += &format!(
            "sewage_sludge_treatment.sludge_bags_are_open, {}\n",
            self.sewage_sludge_treatment.sludge_bags_are_open
        );
        output += &format!(
            "sewage_sludge_treatment.custom_sludge_bags_factor, {}\n",
            f64::from(
                self.sewage_sludge_treatment
                    .custom_sludge_bags_factor
                    .unwrap_or(-0.0)
            )
        );
        output += &format!(
            "sewage_sludge_treatment.sludge_storage_containers_are_open, {}\n",
            self.sewage_sludge_treatment
                .sludge_storage_containers_are_open
        );
        output += &format!(
            "sewage_sludge_treatment.custom_sludge_storage_containers_factor, {}\n",
            f64::from(
                self.sewage_sludge_treatment
                    .custom_sludge_storage_containers_factor
                    .unwrap_or(0.0)
            )
        );
        output += &format!(
            "sewage_sludge_treatment.sewage_sludge_for_disposal, {}\n",
            f64::from(self.sewage_sludge_treatment.sewage_sludge_for_disposal)
        );
        output += &format!(
            "sewage_sludge_treatment.transport_distance, {}\n",
            f64::from(self.sewage_sludge_treatment.transport_distance)
        );
        output += &format!(
            "sewage_sludge_treatment.digester_count, {}\n",
            self.sewage_sludge_treatment.digester_count.unwrap_or(0)
        );
        output += &format!(
            "operating_materials.fecl3, {}\n",
            f64::from(self.operating_materials.fecl3)
        );
        output += &format!(
            "operating_materials.feclso4, {}\n",
            f64::from(self.operating_materials.feclso4)
        );
        output += &format!(
            "operating_materials.caoh2, {}\n",
            f64::from(self.operating_materials.caoh2)
        );
        output += &format!(
            "operating_materials.synthetic_polymers, {}\n",
            f64::from(self.operating_materials.synthetic_polymers)
        );
        output
    }
}
