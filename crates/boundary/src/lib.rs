mod export;
mod import;
mod v1;
mod v2;
mod v3;
mod v4;
mod v5;
mod v6;

pub mod json_api;

pub use self::{
    export::{export_to_csv_pretty, export_to_string, export_to_string_pretty, export_to_vec_pretty},
    import::{import_from_slice, import_from_str, Error as ImportError},
    v6::*,
};

#[cfg(feature = "conversion")]
mod conversion;

pub const CURRENT_VERSION: u32 = 6;
