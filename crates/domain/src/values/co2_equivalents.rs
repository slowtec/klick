use std::{collections::HashMap, ops::Sub};

use klick_value::{specs::OutputValueId, units::Tons};

// TODO: Remove
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

impl From<CO2Equivalents> for HashMap<OutputValueId, Tons> {
    fn from(item: CO2Equivalents) -> Self {
        let mut map = HashMap::new();
        map.insert(OutputValueId::N2oPlant, item.n2o_plant);
        map.insert(OutputValueId::N2oWater, item.n2o_water);
        map.insert(OutputValueId::N2oSideStream, item.n2o_side_stream);
        map.insert(OutputValueId::N2oEmissions, item.n2o_emissions);
        map.insert(OutputValueId::Ch4Plant, item.ch4_plant);
        map.insert(
            OutputValueId::Ch4SludgeStorageContainers,
            item.ch4_sludge_storage_containers,
        );
        map.insert(OutputValueId::Ch4SludgeBags, item.ch4_sludge_bags);
        map.insert(OutputValueId::Ch4Water, item.ch4_water);
        map.insert(
            OutputValueId::Ch4CombinedHeatAndPowerPlant,
            item.ch4_combined_heat_and_power_plant,
        );
        map.insert(OutputValueId::Ch4Emissions, item.ch4_emissions);
        map.insert(OutputValueId::FossilEmissions, item.fossil_emissions);
        map.insert(OutputValueId::Fecl3, item.fecl3);
        map.insert(OutputValueId::Feclso4, item.feclso4);
        map.insert(OutputValueId::Caoh2, item.caoh2);
        map.insert(OutputValueId::SyntheticPolymers, item.synthetic_polymers);
        map.insert(OutputValueId::ElectricityMix, item.electricity_mix);
        map.insert(OutputValueId::OilEmissions, item.oil_emissions);
        map.insert(OutputValueId::GasEmissions, item.gas_emissions);
        map.insert(OutputValueId::OperatingMaterials, item.operating_materials);
        map.insert(
            OutputValueId::SewageSludgeTransport,
            item.sewage_sludge_transport,
        );
        map.insert(OutputValueId::TotalEmissions, item.total_emissions);
        map.insert(OutputValueId::DirectEmissions, item.direct_emissions);
        map.insert(
            OutputValueId::ProcessEnergySavings,
            item.process_energy_savings,
        );
        map.insert(
            OutputValueId::PhotovoltaicExpansionSavings,
            item.photovoltaic_expansion_savings,
        );
        map.insert(
            OutputValueId::WindExpansionSavings,
            item.wind_expansion_savings,
        );
        map.insert(
            OutputValueId::WaterExpansionSavings,
            item.water_expansion_savings,
        );
        map.insert(
            OutputValueId::DistrictHeatingSavings,
            item.district_heating_savings,
        );
        map.insert(
            OutputValueId::FossilEnergySavings,
            item.fossil_energy_savings,
        );
        map.insert(OutputValueId::IndirectEmissions, item.indirect_emissions);
        map.insert(
            OutputValueId::OtherIndirectEmissions,
            item.other_indirect_emissions,
        );
        map.insert(
            OutputValueId::ExcessEnergyCo2Equivalent,
            item.excess_energy_co2_equivalent,
        );

        map
    }
}

impl TryFrom<HashMap<OutputValueId, Tons>> for CO2Equivalents {
    type Error = String;

    fn try_from(mut map: HashMap<OutputValueId, Tons>) -> Result<Self, Self::Error> {
        let result = CO2Equivalents {
            n2o_plant: map
                .remove(&OutputValueId::N2oPlant)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::N2oPlant))?,
            n2o_water: map
                .remove(&OutputValueId::N2oWater)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::N2oWater))?,
            n2o_side_stream: map
                .remove(&OutputValueId::N2oSideStream)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::N2oSideStream))?,
            n2o_emissions: map
                .remove(&OutputValueId::N2oEmissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::N2oEmissions))?,
            ch4_plant: map
                .remove(&OutputValueId::Ch4Plant)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::Ch4Plant))?,
            ch4_sludge_storage_containers: map
                .remove(&OutputValueId::Ch4SludgeStorageContainers)
                .ok_or_else(|| {
                format!(
                    "missing field: {:?}",
                    OutputValueId::Ch4SludgeStorageContainers
                )
            })?,
            ch4_sludge_bags: map
                .remove(&OutputValueId::Ch4SludgeBags)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::Ch4SludgeBags))?,
            ch4_water: map
                .remove(&OutputValueId::Ch4Water)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::Ch4Water))?,
            ch4_combined_heat_and_power_plant: map
                .remove(&OutputValueId::Ch4CombinedHeatAndPowerPlant)
                .ok_or_else(|| {
                    format!(
                        "missing field: {:?}",
                        OutputValueId::Ch4CombinedHeatAndPowerPlant
                    )
                })?,
            ch4_emissions: map
                .remove(&OutputValueId::Ch4Emissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::Ch4Emissions))?,
            fossil_emissions: map
                .remove(&OutputValueId::FossilEmissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::FossilEmissions))?,
            fecl3: map
                .remove(&OutputValueId::Fecl3)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::Fecl3))?,
            feclso4: map
                .remove(&OutputValueId::Feclso4)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::Feclso4))?,
            caoh2: map
                .remove(&OutputValueId::Caoh2)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::Caoh2))?,
            synthetic_polymers: map
                .remove(&OutputValueId::SyntheticPolymers)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::SyntheticPolymers))?,
            electricity_mix: map
                .remove(&OutputValueId::ElectricityMix)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::ElectricityMix))?,
            oil_emissions: map
                .remove(&OutputValueId::OilEmissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::OilEmissions))?,
            gas_emissions: map
                .remove(&OutputValueId::GasEmissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::GasEmissions))?,
            operating_materials: map
                .remove(&OutputValueId::OperatingMaterials)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::OperatingMaterials))?,
            sewage_sludge_transport: map
                .remove(&OutputValueId::SewageSludgeTransport)
                .ok_or_else(|| {
                    format!("missing field: {:?}", OutputValueId::SewageSludgeTransport)
                })?,
            total_emissions: map
                .remove(&OutputValueId::TotalEmissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::TotalEmissions))?,
            direct_emissions: map
                .remove(&OutputValueId::DirectEmissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::DirectEmissions))?,
            process_energy_savings: map
                .remove(&OutputValueId::ProcessEnergySavings)
                .ok_or_else(|| {
                    format!("missing field: {:?}", OutputValueId::ProcessEnergySavings)
                })?,
            photovoltaic_expansion_savings: map
                .remove(&OutputValueId::PhotovoltaicExpansionSavings)
                .ok_or_else(|| {
                    format!(
                        "missing field: {:?}",
                        OutputValueId::PhotovoltaicExpansionSavings
                    )
                })?,
            wind_expansion_savings: map
                .remove(&OutputValueId::WindExpansionSavings)
                .ok_or_else(|| {
                    format!("missing field: {:?}", OutputValueId::WindExpansionSavings)
                })?,
            water_expansion_savings: map
                .remove(&OutputValueId::WaterExpansionSavings)
                .ok_or_else(|| {
                    format!("missing field: {:?}", OutputValueId::WaterExpansionSavings)
                })?,
            district_heating_savings: map
                .remove(&OutputValueId::DistrictHeatingSavings)
                .ok_or_else(|| {
                    format!("missing field: {:?}", OutputValueId::DistrictHeatingSavings)
                })?,
            fossil_energy_savings: map.remove(&OutputValueId::FossilEnergySavings).ok_or_else(
                || format!("missing field: {:?}", OutputValueId::FossilEnergySavings),
            )?,
            indirect_emissions: map
                .remove(&OutputValueId::IndirectEmissions)
                .ok_or_else(|| format!("missing field: {:?}", OutputValueId::IndirectEmissions))?,
            other_indirect_emissions: map
                .remove(&OutputValueId::OtherIndirectEmissions)
                .ok_or_else(|| {
                    format!("missing field: {:?}", OutputValueId::OtherIndirectEmissions)
                })?,
            excess_energy_co2_equivalent: map
                .remove(&OutputValueId::ExcessEnergyCo2Equivalent)
                .ok_or_else(|| {
                    format!(
                        "missing field: {:?}",
                        OutputValueId::ExcessEnergyCo2Equivalent
                    )
                })?,
        };

        Ok(result)
    }
}
