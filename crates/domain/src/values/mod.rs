use std::collections::HashMap;

use derive_more::From;

pub mod constants;
pub mod specs;
pub mod units;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum ValueId {
    Custom(String),
    In(specs::InputValueId),
    Out(specs::OutputValueId),
}

impl ValueId {
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}

pub type Edge = (ValueId, ValueId);
pub type Edges = Vec<Edge>;
pub type Values = HashMap<ValueId, units::Value>;

#[derive(Debug, Clone, PartialEq)]
pub struct CalculationOutcome {
    pub input: Values,
    pub output: Option<Values>,
    pub graph: Option<Edges>,

    // Used to create bar chart input
    pub sensitivity_n2o_calculations: Option<Vec<(units::N2oEmissionFactorCalcMethod, Values)>>,

    // Used to create bar chart input
    pub sensitivity_ch4_chp_calculations: Option<
        Vec<(
            units::Ch4ChpEmissionFactorCalcMethod,
            units::Tons,
            units::Factor,
        )>,
    >,
}
