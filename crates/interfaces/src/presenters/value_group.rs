use klick_domain::ValueId;

pub trait ValueGroupPresenter {
    fn present_value_group(&self, group: ValueGroupId) -> (String, Vec<ValueId>);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueGroupId {
    PlantDetails,
    InfluentParameters,
    EffluentParameters,
    EnergyConsumption,
    SludgeTreatment,
    SideStreamTreatment,
    OperatingMaterials,
    N2OEmissions,
    CH4ChpEmissions,
    CH4SludgeEmissions,
    FossilCO2Emissions,
}
