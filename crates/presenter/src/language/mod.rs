use klick_domain::{
    units::{Enum, Int, Scalar},
    Value,
};

use crate::value_labels::ValueLabel;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lng {
    En,
    De,
}

impl Lng {
    /// ISO 639-1 code
    #[must_use]
    pub const fn alpha_2(&self) -> &str {
        match self {
            Self::En => "en",
            Self::De => "de",
        }
    }

    #[must_use]
    const fn thousands_separator(&self) -> &str {
        match self {
            Self::En => ",",
            Self::De => ".",
        }
    }

    #[must_use]
    const fn decimal_separator(&self) -> &str {
        match self {
            Self::En => ".",
            Self::De => ",",
        }
    }

    #[must_use]
    pub fn format_number<N>(&self, n: N) -> String
    where
        N: Into<f64>,
    {
        self.format_f64(n.into(), None)
    }

    #[must_use]
    pub fn format_number_without_thousands_separators<N>(&self, n: N) -> String
    where
        N: Into<f64>,
    {
        n.into().to_string().replace('.', self.decimal_separator())
    }

    #[must_use]
    pub fn format_number_with_fixed_precision<N>(&self, n: N, precision: usize) -> String
    where
        N: Into<f64>,
    {
        self.format_f64(n.into(), Some(precision))
    }

    #[must_use]
    pub(crate) const fn format_bool(&self, x: bool) -> &str {
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

    #[must_use]
    pub fn format_value(&self, value: &Value) -> String {
        match value {
            Value::Scalar(scalar) => match scalar {
                Scalar::Float(float) => {
                    let v = f64::from(*float);
                    self.format_number(v)
                }
                Scalar::Bool(b) => self.format_bool(*b).to_string(),
                Scalar::Int(Int::Count(cnt)) => self.format_number(u64::from(*cnt) as f64),
            },
            Value::Text(txt) => txt.clone(),
            Value::Enum(v) => match v {
                Enum::N2oEmissionFactorCalcMethod(v) => v.label().to_string(),
                Enum::Ch4ChpEmissionFactorCalcMethod(v) => v.label().to_string(),
            },
        }
    }

    fn format_f64(&self, number: f64, precision: Option<usize>) -> String {
        let num_string = match precision {
            Some(precision) => format!("{number:.precision$}"),
            None => number.to_string(),
        };

        let (integer_str, decimal_str) = if let Some(pos) = num_string.find('.') {
            num_string.split_at(pos)
        } else {
            (&*num_string, "")
        };

        let integer_string = integer_str
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(self.thousands_separator());

        let decimal_str = decimal_str.replace('.', self.decimal_separator());

        format!("{integer_string}{decimal_str}")
    }

    pub fn parse_str_as_f64(&self, input: &str) -> Result<f64, String> {
        input
            .replace(self.thousands_separator(), "")
            .replace(self.decimal_separator(), ".")
            .trim()
            .parse::<f64>()
            .map_err(|err| format!("{err}"))
    }
}
