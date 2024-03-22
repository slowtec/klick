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

    // TODO: rename to format_float
    pub fn format_number<N>(&self, n: N) -> String
    where
        N: Into<f64>,
    {
        n.into().to_string().replace('.', self.decimal_separator())
    }

    // TODO: rename to format_float_with_precision
    pub fn format_number_with_precision<N>(&self, n: N, precision: usize) -> String
    where
        N: Into<f64>,
    {
        let n = n.into();
        format!("{n:.precision$}").replace('.', self.decimal_separator())
    }

    #[allow(clippy::missing_panics_doc)]
    // TODO: rename to format_float_with_thousands_seperator
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

    #[must_use]
    pub const fn format_bool(&self, x: bool) -> &str {
        match self {
            Lng::De => {
                if x {
                    "Ja"
                } else {
                    "Nein"
                }
            }
            Lng::En => {
                if x {
                    "Yes"
                } else {
                    "No"
                }
            }
        }
    }
}
