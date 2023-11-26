mod v2;

mod export;
mod import;

pub use self::{
    export::{export_to_string_pretty, export_to_vec_pretty},
    import::{import_from_slice, import_from_str, Error as ImportError},
    v2::*,
};

#[cfg(feature = "conversion")]
mod conversion;

pub const CURRENT_VERSION: u32 = 2;
