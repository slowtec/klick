#[allow(clippy::wildcard_imports)]
mod bar;
mod sankey;

#[cfg(feature = "ssr")]
pub mod ssr;

pub use self::{
    bar::{Arguments as BarChartArguments, Chart as BarChart},
    sankey::{Chart as SankeyChart, Color, Sankey as SankeyData},
};
