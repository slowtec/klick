#[allow(clippy::wildcard_imports)]
mod bar;
mod sankey;

#[cfg(test)]
mod tests;

#[cfg(feature = "ssr")]
pub mod ssr;

pub use self::{
    bar::{Arguments as BarChartArguments, Chart as BarChart},
    sankey::{Chart as SankeyChart, Sankey as SankeyData},
};
