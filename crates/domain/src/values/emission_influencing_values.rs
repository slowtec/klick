use crate::units::{
    GramsPerKilowatthour, Kilometers, Kilowatthours, MilligramsPerLiter, Percent, Qubicmeters, Tons,
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
    pub operating_materials: OperatingMaterials,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct AnnualAverageInfluent {
    pub nitrogen: MilligramsPerLiter,
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
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Copy))]
pub struct SewageSludgeTreatment {
    pub sludge_bags_are_open: bool,
    pub custom_sludge_bags_factor: Option<f64>,
    pub sludge_storage_containers_are_open: bool,
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
            "\npopulation_equivalent, {}",
            f64::from(self.population_equivalent)
        );
        output += &format!("\nwastewater, {}", f64::from(self.wastewater));
        output += &format!(
            "\ninfluent_average.nitrogen, {}",
            f64::from(self.influent_average.nitrogen)
        );
        output += &format!(
            "\neffluent_average.nitrogen, {}",
            f64::from(self.effluent_average.nitrogen)
        );
        output += &format!(
            "\neffluent_average.chemical_oxygen_demand, {}",
            f64::from(self.effluent_average.chemical_oxygen_demand)
        );
        output += &format!(
            "\nenergy_consumption.sewage_gas_produced, {}",
            f64::from(self.energy_consumption.sewage_gas_produced)
        );
        output += &format!(
            "\nenergy_consumption.methane_fraction, {}",
            f64::from(self.energy_consumption.methane_fraction)
        );
        output += &format!(
            "\nenergy_consumption.total_power_consumption, {}",
            f64::from(self.energy_consumption.total_power_consumption)
        );
        output += &format!(
            "\nenergy_consumption.on_site_power_generation, {}",
            f64::from(self.energy_consumption.on_site_power_generation)
        );
        output += &format!(
            "\nenergy_consumption.emission_factor_electricity_mix, {}",
            f64::from(self.energy_consumption.emission_factor_electricity_mix)
        );
        output += &format!(
            "\nsewage_sludge_treatment.sludge_bags_are_open, {}",
            self.sewage_sludge_treatment.sludge_bags_are_open
        );
        output += &format!(
            "\nsewage_sludge_treatment.custom_sludge_bags_factor, {}",
            f64::from(
                self.sewage_sludge_treatment
                    .custom_sludge_bags_factor
                    .unwrap_or(-0.0)
            )
        );
        output += &format!(
            "\nsewage_sludge_treatment.sludge_storage_containers_are_open, {}",
            self.sewage_sludge_treatment
                .sludge_storage_containers_are_open
        );
        output += &format!(
            "\nsewage_sludge_treatment.custom_sludge_storage_containers_factor, {}",
            f64::from(
                self.sewage_sludge_treatment
                    .custom_sludge_storage_containers_factor
                    .unwrap_or(0.0)
            )
        );
        output += &format!(
            "\nsewage_sludge_treatment.sewage_sludge_for_disposal, {}",
            f64::from(self.sewage_sludge_treatment.sewage_sludge_for_disposal)
        );
        output += &format!(
            "\nsewage_sludge_treatment.transport_distance, {}",
            f64::from(self.sewage_sludge_treatment.transport_distance)
        );
        output += &format!(
            "\nsewage_sludge_treatment.digester_count, {}",
            self.sewage_sludge_treatment.digester_count.unwrap_or(0)
        );
        output += &format!(
            "\noperating_materials.fecl3, {}",
            f64::from(self.operating_materials.fecl3)
        );
        output += &format!(
            "\noperating_materials.feclso4, {}",
            f64::from(self.operating_materials.feclso4)
        );
        output += &format!(
            "\noperating_materials.caoh2, {}",
            f64::from(self.operating_materials.caoh2)
        );
        output += &format!(
            "\noperating_materials.synthetic_polymers, {}",
            f64::from(self.operating_materials.synthetic_polymers)
        );
        output
    }
}
