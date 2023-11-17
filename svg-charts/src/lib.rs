#[allow(clippy::wildcard_imports)]

mod bar;

#[cfg(feature = "ssr")]
pub mod ssr;

pub use self::bar::{Arguments as BarChartArguments, Chart as BarChart};
