use fluent_templates::static_loader;

mod bar_chart;
mod csv;
mod language;
mod sankey_chart;
mod tables;
mod value_color;
mod value_input_field;
mod value_labels;
mod value_metadata;
mod value_units;

pub use klick_domain::*;

pub use self::{
    bar_chart::*, csv::*, language::*, sankey_chart::*, tables::*, value_color::*,
    value_input_field::*, value_labels::*, value_metadata::*, value_units::*,
};

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "de",
    };
}
