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

pub mod json_api;

pub use self::{
    export::{export_to_string, export_to_string_pretty, export_to_vec_pretty},
    import::{import_from_slice, import_from_str, Error as ImportError},
    v8::*,
};

#[cfg(feature = "conversion")]
mod conversion;

pub const CURRENT_VERSION: u32 = 8;

#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct CalculationOutcome {
    // a.k.a "Model One"
    pub profile: EvaluationData,

    // a.k.a "Model Two"
    pub sensitivity: EvaluationData,

    // Used to create bar chart input
    pub sensitivity_n2o_calculations: Vec<(
        domain::N2oEmissionFactorCalcMethod,
        domain::EmissionsCalculationOutcome,
    )>,

    // Used to create bar chart input
    pub sensitivity_ch4_chp_calculations:
        Vec<(domain::CH4ChpEmissionFactorCalcMethod, Tons, Factor)>,

    // a.k.a "Model Three"
    pub recommendation: EvaluationData,
}

#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct EvaluationData {
    pub input: FormData,
    pub output: domain::EmissionsCalculationOutcome,
}
