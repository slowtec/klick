// TODO: merge both barchart components
mod barchart;
mod barchart_radioinputs;
mod sankey;

#[cfg(feature = "ssr")]
pub mod ssr;

pub use self::{
    barchart::{BarChart, BarChartArguments},
    barchart_radioinputs::{BarChartRadioInput, BarChartRadioInputArguments},
    sankey::{SankeyChart, Color, SankeyData},
};
