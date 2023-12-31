mod v1;
mod v2;
mod v3;

mod export;
mod import;

pub mod json_api;

pub use self::{
    export::{export_to_string_pretty, export_to_vec_pretty},
    import::{import_from_slice, import_from_str, Error as ImportError},
    v3::*,
};

#[cfg(feature = "conversion")]
mod conversion;

pub const CURRENT_VERSION: u32 = 3;
