mod bar;

#[cfg(feature = "ssr")]
pub mod ssr;

pub use self::bar::Barchart;
pub use self::bar::BarchartArguments;