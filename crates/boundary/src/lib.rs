use std::collections::HashMap;

use klick_domain::{
    self as domain,
    units::{Factor, Tons},
};

mod export;
mod import;

mod v1;
mod v2;
mod v3;
mod v4;
mod v5;
mod v6;
mod v7;
mod v8;

mod calculation;
pub mod json_api;

pub use self::{
    calculation::calculate,
    export::{export_to_string, export_to_string_pretty, export_to_vec_pretty},
    import::{import_from_slice, import_from_str, Error as ImportError},
    v8::*,
};

#[cfg(feature = "conversion")]
mod conversion;

pub const CURRENT_VERSION: u32 = 8;

type Values = HashMap<domain::Id, domain::Value>;

#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct CalculationOutcome {
    pub input: Values,
    pub output: Option<Values>,

    // Used to create bar chart input
    pub sensitivity_n2o_calculations:
        Option<Vec<(domain::units::N2oEmissionFactorCalcMethod, Values)>>,

    // Used to create bar chart input
    pub sensitivity_ch4_chp_calculations:
        Option<Vec<(domain::units::Ch4ChpEmissionFactorCalcMethod, Tons, Factor)>>,
}
