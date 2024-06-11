use derive_more::From;

use crate::units::{Quantity, QuantityType};

mod calculation_outcome;
mod co2_equivalents;
mod emission_factors;
mod emission_influencing_values;
mod value_ids;

pub use self::{
    calculation_outcome::*, co2_equivalents::*, emission_factors::*,
    emission_influencing_values::*, value_ids::*,
};

#[derive(Debug, Clone, PartialEq, From)]
pub enum Value {
    #[from(ignore)]
    Quantity(Quantity),
    Bool(bool),
    Text(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueType {
    Quantity(QuantityType),
    Bool,
    Text,
}

impl Value {
    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        match self {
            Self::Quantity(q) => ValueType::Quantity(q.quantity_type()),
            Self::Bool(_) => ValueType::Bool,
            Self::Text(_) => ValueType::Text,
        }
    }

    pub fn as_quantity(self) -> Option<Quantity> {
        let Self::Quantity(v) = self else {
            return None;
        };
        Some(v)
    }

    pub fn as_bool(self) -> Option<bool> {
        let Self::Bool(v) = self else {
            return None;
        };
        Some(v)
    }

    pub fn as_text(self) -> Option<String> {
        let Self::Text(v) = self else {
            return None;
        };
        Some(v)
    }
}

impl<T> From<T> for Value
where
    Quantity: From<T>,
{
    fn from(from: T) -> Self {
        Self::Quantity(Quantity::from(from))
    }
}
