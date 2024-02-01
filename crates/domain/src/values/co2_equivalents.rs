use crate::units::Tons;

#[derive(Debug, Clone)]
pub struct CO2Equivalents {
    pub n2o_plant: Tons,
    pub n2o_water: Tons,
    pub n2o_emissions: Tons,
    pub ch4_sewage_treatment: Tons,
    pub ch4_sludge_storage_containers: Tons,
    pub ch4_sludge_bags: Tons,
    pub ch4_water: Tons,
    pub ch4_combined_heat_and_power_plant: Tons,
    pub ch4_emissions: Tons,
    pub fecl3: Tons,
    pub feclso4: Tons,
    pub caoh2: Tons,
    pub synthetic_polymers: Tons,
    pub electricity_mix: Tons,
    pub operating_materials: Tons,
    pub sewage_sludge_transport: Tons,
    pub emissions: Tons,
    pub direct_emissions: Tons,
    pub indirect_emissions: Tons,
    pub other_indirect_emissions: Tons,
    pub excess_energy_co2_equivalent: Tons,
}

impl CO2Equivalents {
    pub fn to_csv(&self) -> String {
        let mut output: String = String::new();
        output += &format!("\nn2o_plant, {}", f64::from(self.n2o_plant));
        output += &format!("\nn2o_water, {}", f64::from(self.n2o_water));
        output += &format!("\nn2o_emissions, {}", f64::from(self.n2o_emissions));
        output += &format!(
            "\nch4_sewage_treatment, {}",
            f64::from(self.ch4_sewage_treatment)
        );
        output += &format!(
            "\nch4_sludge_storage_containers, {}",
            f64::from(self.ch4_sludge_storage_containers)
        );
        output += &format!("\nch4_sludge_bags, {}", f64::from(self.ch4_sludge_bags));
        output += &format!("\nch4_water, {}", f64::from(self.ch4_water));
        output += &format!(
            "\nch4_combined_heat_and_power_plant, {}",
            f64::from(self.ch4_combined_heat_and_power_plant)
        );
        output += &format!("\nch4_emissions, {}", f64::from(self.ch4_emissions));
        output += &format!("\nfecl3, {}", f64::from(self.fecl3));
        output += &format!("\nfeclso4, {}", f64::from(self.feclso4));
        output += &format!("\ncaoh2, {}", f64::from(self.caoh2));
        output += &format!(
            "\nsynthetic_polymers, {}",
            f64::from(self.synthetic_polymers)
        );
        output += &format!("\nelectricity_mix, {}", f64::from(self.electricity_mix));
        output += &format!(
            "\noperating_materials, {}",
            f64::from(self.operating_materials)
        );
        output += &format!(
            "\nsewage_sludge_transport, {}",
            f64::from(self.sewage_sludge_transport)
        );
        output += &format!("\nemissions, {}", f64::from(self.emissions));
        output += &format!("\ndirect_emissions, {}", f64::from(self.direct_emissions));
        output += &format!(
            "\nindirect_emissions, {}",
            f64::from(self.indirect_emissions)
        );
        output += &format!(
            "\nother_indirect_emissions, {}",
            f64::from(self.other_indirect_emissions)
        );
        output += &format!(
            "\nexcess_energy_co2_equivalent, {}",
            f64::from(self.excess_energy_co2_equivalent)
        );
        output
    }
}
