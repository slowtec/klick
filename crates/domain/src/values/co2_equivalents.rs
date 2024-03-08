use crate::units::Tons;

#[derive(Debug, Clone)]
pub struct CO2Equivalents {
    pub n2o_plant: Tons,
    pub n2o_water: Tons,
    pub n2o_side_stream: Tons,
    pub n2o_emissions: Tons,
    pub ch4_plant: Tons,
    pub ch4_sludge_storage_containers: Tons,
    pub ch4_sludge_bags: Tons,
    pub ch4_water: Tons,
    pub ch4_combined_heat_and_power_plant: Tons,
    pub ch4_emissions: Tons,
    pub fossil_emissions: Tons,
    pub fecl3: Tons,
    pub feclso4: Tons,
    pub caoh2: Tons,
    pub synthetic_polymers: Tons,
    pub electricity_mix: Tons,
    pub oil_emissions: Tons,
    pub gas_emissions: Tons,
    pub operating_materials: Tons,
    pub sewage_sludge_transport: Tons,
    pub total_emissions: Tons,
    pub direct_emissions: Tons,
    pub indirect_emissions: Tons,
    pub other_indirect_emissions: Tons,
    pub excess_energy_co2_equivalent: Tons,
}

impl CO2Equivalents {
    pub fn to_csv(&self) -> String {
        // FIXME add n2o_side_stream fossil_emissions
        // FIXME add     pub oil_emissions: Tons,
        //     pub gas_emissions: Tons,
        let mut output: String = String::new();
        output += &format!("n2o_plant, {}\n", f64::from(self.n2o_plant));
        output += &format!("n2o_water, {}\n", f64::from(self.n2o_water));
        output += &format!("n2o_emissions, {}\n", f64::from(self.n2o_emissions));
        output += &format!("ch4_plant, {}\n", f64::from(self.ch4_plant));
        output += &format!(
            "ch4_sludge_storage_containers, {}\n",
            f64::from(self.ch4_sludge_storage_containers)
        );
        output += &format!("ch4_sludge_bags, {}\n", f64::from(self.ch4_sludge_bags));
        output += &format!("ch4_water, {}\n", f64::from(self.ch4_water));
        output += &format!(
            "ch4_combined_heat_and_power_plant, {}\n",
            f64::from(self.ch4_combined_heat_and_power_plant)
        );
        output += &format!("ch4_emissions, {}\n", f64::from(self.ch4_emissions));
        output += &format!("fecl3, {}\n", f64::from(self.fecl3));
        output += &format!("feclso4, {}\n", f64::from(self.feclso4));
        output += &format!("caoh2, {}\n", f64::from(self.caoh2));
        output += &format!(
            "synthetic_polymers, {}\n",
            f64::from(self.synthetic_polymers)
        );
        output += &format!("electricity_mix, {}\n", f64::from(self.electricity_mix));
        output += &format!(
            "operating_materials, {}\n",
            f64::from(self.operating_materials)
        );
        output += &format!(
            "sewage_sludge_transport, {}\n",
            f64::from(self.sewage_sludge_transport)
        );
        output += &format!("emissions, {}\n", f64::from(self.total_emissions));
        output += &format!("direct_emissions, {}\n", f64::from(self.direct_emissions));
        output += &format!(
            "indirect_emissions, {}\n",
            f64::from(self.indirect_emissions)
        );
        output += &format!(
            "other_indirect_emissions, {}\n",
            f64::from(self.other_indirect_emissions)
        );
        output += &format!(
            "excess_energy_co2_equivalent, {}\n",
            f64::from(self.excess_energy_co2_equivalent)
        );
        output
    }
}
