use fluent_templates::{langid, LanguageIdentifier};

use klick_domain::{
    units::{Enum, Int, Scalar},
    Value,
};

use crate::value_labels::ValueLabel;

#[cfg(test)]
mod tests;

const GERMAN: LanguageIdentifier = langid!("de");
const ENGLISH: LanguageIdentifier = langid!("en");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lng {
    En,
    De,
}

impl Lng {
    pub fn try_from_id(id: &LanguageIdentifier) -> Option<Self> {
        match *id {
            GERMAN => Some(Self::De),
            ENGLISH => Some(Self::En),
            _ => None,
        }
    }

    pub const fn id(&self) -> LanguageIdentifier {
        match self {
            Self::En => ENGLISH,
            Self::De => GERMAN,
        }
    }

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
                Enum::N2oEmissionFactorCalcMethod(v) => v.label(*self).to_string(),
                Enum::Ch4ChpEmissionFactorCalcMethod(v) => v.label(*self).to_string(),
            },
        }
    }

    fn format_f64(&self, number: f64, precision: Option<usize>) -> String {
        // Handle negative numbers by separating the sign
        let (sign, abs_number) = if number < 0.0 {
            ("-", number.abs())
        } else {
            ("", number)
        };

        // Format the number string based on the precision
        let num_string = match precision {
            Some(precision) => format!("{:.precision$}", abs_number),
            None => abs_number.to_string(),
        };

        // Split into integer and decimal parts
        let (integer_str, decimal_str) = if let Some(pos) = num_string.find('.') {
            num_string.split_at(pos)
        } else {
            (&*num_string, "")
        };

        // Format the integer part with the thousands separator
        let integer_string = integer_str
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(self.thousands_separator());

        // Replace decimal point with the correct separator
        let decimal_str = decimal_str.replace('.', self.decimal_separator());

        // Combine sign, integer, and decimal parts
        format!("{}{}{}", sign, integer_string, decimal_str)
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
