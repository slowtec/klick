mod value_ids;
mod value_labels;

pub use self::{value_ids::*, value_labels::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lng {
    En,
    De,
}

impl Lng {
    #[must_use]
    pub const fn thousands_separator(&self) -> &str {
        match self {
            Self::En => ",",
            Self::De => ".",
        }
    }

    #[must_use]
    pub const fn decimal_separator(&self) -> &str {
        match self {
            Self::En => ".",
            Self::De => ",",
        }
    }

    pub fn format_number<N>(&self, n: N) -> String
    where
        N: Into<f64>,
    {
        n.into().to_string().replace('.', self.decimal_separator())
    }

    pub fn format_number_with_precision<N>(&self, n: N, precision: usize) -> String
    where
        N: Into<f64>,
    {
        let n = n.into();
        format!("{n:.precision$}").replace('.', self.decimal_separator())
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn format_number_with_thousands_seperator<N>(&self, n: N) -> String
    where
        N: Into<f64>,
    {
        format!("{:.0}", n.into())
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(self.thousands_separator())
    }
}

#[test]
fn format_german_numbers() {
    assert_eq!(Lng::De.format_number(0.6), "0,6");
    assert_eq!(Lng::De.format_number(6.0), "6");
    assert_eq!(Lng::De.format_number(6.01), "6,01");

    assert_eq!(Lng::De.format_number_with_precision(6.0, 3), "6,000");
    assert_eq!(Lng::De.format_number_with_precision(0.6, 3), "0,600");
}

#[test]
fn format_german_numbers_with_thousands_separator() {
    assert_eq!(
        Lng::De.format_number_with_thousands_seperator(6_000.0),
        "6.000"
    );

    // TODO: bug or feature?
    // assert_eq!(Lng::De.format_number_with_thousands_seperator(6_000.1), "6.000,1");
}

#[test]
fn format_english_numbers() {
    assert_eq!(Lng::En.format_number(0.6), "0.6");
    assert_eq!(Lng::En.format_number(6.0), "6");
    assert_eq!(Lng::En.format_number(6.01), "6.01");
}

#[test]
fn format_english_numbers_with_thousands_separator() {
    assert_eq!(
        Lng::En.format_number_with_thousands_seperator(6_000.0),
        "6,000"
    );

    // TODO: bug or feature?
    // assert_eq!(Lng::En.format_number_with_thousands_seperator(6_000.01), "6,000.01");
}
