use std::ops::Sub;

use crate::units::Tons;

#[derive(Debug, Clone, PartialEq)]
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
    pub process_energy_savings: Tons,
    pub photovoltaic_expansion_savings: Tons,
    pub wind_expansion_savings: Tons,
    pub water_expansion_savings: Tons,
    pub district_heating_savings: Tons,
    pub fossil_energy_savings: Tons,
    pub indirect_emissions: Tons,
    pub other_indirect_emissions: Tons,
    pub excess_energy_co2_equivalent: Tons,
}

impl Sub for CO2Equivalents {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            n2o_plant: self.n2o_plant - rhs.n2o_plant,
            n2o_water: self.n2o_water - rhs.n2o_water,
            n2o_side_stream: self.n2o_side_stream - rhs.n2o_side_stream,
            n2o_emissions: self.n2o_emissions - rhs.n2o_emissions,
            ch4_plant: self.ch4_plant - rhs.ch4_plant,
            ch4_sludge_storage_containers: self.ch4_sludge_storage_containers
                - rhs.ch4_sludge_storage_containers,
            ch4_sludge_bags: self.ch4_sludge_bags - rhs.ch4_sludge_bags,
            ch4_water: self.ch4_water - rhs.ch4_water,
            ch4_combined_heat_and_power_plant: self.ch4_combined_heat_and_power_plant
                - rhs.ch4_combined_heat_and_power_plant,
            ch4_emissions: self.ch4_emissions - rhs.ch4_emissions,
            fossil_emissions: self.fossil_emissions - rhs.fossil_emissions,
            fecl3: self.fecl3 - rhs.fecl3,
            feclso4: self.feclso4 - rhs.feclso4,
            caoh2: self.caoh2 - rhs.caoh2,
            synthetic_polymers: self.synthetic_polymers - rhs.synthetic_polymers,
            electricity_mix: self.electricity_mix - rhs.electricity_mix,
            oil_emissions: self.oil_emissions - rhs.oil_emissions,
            gas_emissions: self.gas_emissions - rhs.gas_emissions,
            operating_materials: self.operating_materials - rhs.operating_materials,
            sewage_sludge_transport: self.sewage_sludge_transport - rhs.sewage_sludge_transport,
            total_emissions: self.total_emissions - rhs.total_emissions,
            direct_emissions: self.direct_emissions - rhs.direct_emissions,
            process_energy_savings: self.process_energy_savings - rhs.process_energy_savings,
            photovoltaic_expansion_savings: self.photovoltaic_expansion_savings
                - rhs.photovoltaic_expansion_savings,
            wind_expansion_savings: self.wind_expansion_savings - rhs.wind_expansion_savings,
            water_expansion_savings: self.water_expansion_savings - rhs.water_expansion_savings,
            district_heating_savings: self.district_heating_savings - rhs.district_heating_savings,
            fossil_energy_savings: self.fossil_energy_savings - rhs.fossil_energy_savings,
            indirect_emissions: self.indirect_emissions - rhs.indirect_emissions,
            other_indirect_emissions: self.other_indirect_emissions - rhs.other_indirect_emissions,
            excess_energy_co2_equivalent: self.excess_energy_co2_equivalent
                - rhs.excess_energy_co2_equivalent,
        }
    }
}
