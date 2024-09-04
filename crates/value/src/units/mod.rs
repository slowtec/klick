use std::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use derive_more::From;
use num_derive::{FromPrimitive, ToPrimitive};
use paste::paste;
use strum::AsRefStr;

mod conversion;

#[cfg(test)]
mod tests;

macro_rules! float {
    (
        $(#[$quantity_attr:meta])*
        $quantity:ident($base_type:ty) {
            $($unit:ident, $conv:expr, $abbreviation:expr;)+
        }
    ) =>
    {
        paste!{
            $(#[$quantity_attr])*
            pub trait [<$quantity Ext>]
            {
                const CONVERSION_FACTOR: $base_type;
                fn convert_to<T>(self) -> T where T: [<$quantity Ext>];

                #[doc(hidden)]
                fn new_from_base_value(v: $base_type) -> Self;
            }

            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
            pub enum $quantity {
                $(
                  $unit($unit),
                )+
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum [<$quantity Type>] {
                $(
                  $unit,
                )+
            }

            impl $quantity {
                #[must_use]
                pub fn convert_to<T>(self) -> Self
                    where T: [<$quantity Ext>] + Into<Self>
                {
                    match self {
                        $(
                            Self::$unit(value) => value.convert_to::<T>().into(),
                        )+
                    }
                }

                #[must_use]
                pub const fn [<$quantity:snake _type>](&self) -> [<$quantity Type>] {
                    match self {
                        $(
                          Self::$unit(_) => [<$quantity Type>]::$unit,
                        )+
                    }
                }

                $(
                    #[allow(irrefutable_let_patterns)]
                    #[must_use]
                    pub const fn [<as_ $unit:snake>](self) -> Option<$unit> {
                        let Self::$unit(v) = self else {
                            return None;
                        };
                        Some(v)
                    }

                    #[must_use]
                    pub fn [<unchecked_ $unit:snake>](self) -> $unit {
                        self.[<as_ $unit:snake>]().expect(concat!(stringify!($unit), " value"))
                    }
                )+
            }

            // units
            $(
                #[doc = concat!(stringify!($unit), " `[",$abbreviation, "]`.")]
                #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
                pub struct $unit($base_type);

                impl ValueTypeExt for $unit {
                    const VALUE_TYPE: ValueType = ValueType::Scalar(
                        ScalarType::Float(
                            FloatType::$quantity(
                                [<$quantity Type>]::$unit
                            )
                        )
                    );
                }

                impl Default for $unit {
                  fn default() -> Self {
                      Self::zero()
                  }
                }

                impl $unit {
                    #[must_use]
                    pub const fn new(value: $base_type) -> Self {
                        Self(value)
                    }

                    #[must_use]
                    pub const fn zero() -> Self {
                        Self(0.0)
                    }

                    #[must_use]
                    pub fn is_sign_negative(&self) -> bool {
                        self.0.is_sign_negative()
                    }

                    #[must_use]
                    pub fn round(&self, precision: usize) -> Self {
                        let scaling_factor = 10_f64.powi(i32::try_from(precision).unwrap());
                        Self((self.0 * scaling_factor).round() / scaling_factor)
                    }
                }

                impl [<$quantity Ext>] for $unit {
                    const CONVERSION_FACTOR: $base_type = $conv;
                    fn convert_to<T>(self) -> T where T: [<$quantity Ext>] {
                        let base = self.0 * Self::CONVERSION_FACTOR;
                        T::new_from_base_value(base)
                    }

                    #[doc(hidden)]
                    fn new_from_base_value(base_value: $base_type) -> Self {
                        Self::new(base_value / Self::CONVERSION_FACTOR)
                    }
                }

                impl From<$unit> for $base_type {
                    fn from(from: $unit) -> Self {
                        from.0
                    }
                }

                impl Add<$unit> for $unit {
                    type Output = $unit;

                    fn add(self, rhs: $unit) -> Self::Output {
                        Self::new(self.0 + rhs.0)
                    }
                }

                impl AddAssign<$unit> for $unit {
                    fn add_assign(&mut self, rhs: $unit) {
                        self.0 += rhs.0;
                    }
                }

                impl Sub<$unit> for $unit {
                    type Output = $unit;

                    fn sub(self, rhs: $unit) -> Self::Output  {
                        Self::new(self.0 - rhs.0)
                    }
                }

                impl Sub<&$unit> for $unit {
                    type Output = $unit;

                    fn sub(self, rhs: &$unit) -> Self::Output  {
                        Self::new(self.0 - rhs.0)
                    }
                }

                impl Sub<&$unit> for &$unit {
                    type Output = $unit;

                    fn sub(self, rhs: &$unit) -> Self::Output  {
                        Self::Output::new(self.0 - rhs.0)
                    }
                }

                impl SubAssign<$unit> for $unit {
                    fn sub_assign(&mut self, rhs: Self) {
                        self.0 = self.0 - rhs.0;
                    }
                }

                impl Div<$unit> for $unit {
                    type Output = Factor;

                    fn div(self, rhs: $unit) -> Self::Output {
                        Factor::new(self.0 / rhs.0)
                    }
                }

                impl Mul<Percent> for $unit {
                    type Output = $unit;
                    fn mul(self, rhs: Percent) -> Self::Output {
                        $unit::new(self.0 * rhs.0 / 100.0)
                    }
                }

                impl Mul<&Percent> for $unit {
                    type Output = $unit;
                    fn mul(self, rhs: &Percent) -> Self::Output {
                        $unit::new(self.0 * rhs.0 / 100.0)
                    }
                }

                impl Mul<&Percent> for &$unit {
                    type Output = $unit;
                    fn mul(self, rhs: &Percent) -> Self::Output {
                        $unit::new(self.0 * rhs.0 / 100.0)
                    }
                }


                impl Mul<Factor> for $unit {
                    type Output = $unit;
                    fn mul(self, rhs: Factor) -> Self::Output {
                        $unit::new(self.0 * rhs.0)
                    }
                }

                impl Mul<&Factor> for $unit {
                    type Output = $unit;
                    fn mul(self, rhs: &Factor) -> Self::Output {
                        $unit::new(self.0 * rhs.0)
                    }
                }

                impl Mul<Factor> for &$unit {
                    type Output = $unit;
                    fn mul(self, rhs: Factor) -> Self::Output {
                        $unit::new(self.0 * rhs.0)
                    }
                }

                impl fmt::Display for $unit {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "{}{}", self.0, $abbreviation)
                    }
                }
            )+
        }
    };
}

macro_rules! floats {
    (
         $(
             $(#[$float_attr:meta])*
             $float:ident {
                 $($unit:ident, $conv:expr, $abbreviation:expr;)+
             }
         )+
    ) =>
    {
        paste! {

            pub trait UnitAbbreviation {
                fn abbreviation(&self) -> &'static str;
            }

            /// Typed floating-point number.
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
            pub enum Float {
                $(
                  $float($float),
                )+
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
            pub enum FloatType {
                $(
                  $float([<$float Type>]),
                )+
            }

            impl FloatType {
                $(
                    $(
                        #[must_use] pub const fn [<$unit:snake>]() -> Self {
                            Self::$float([<$float Type>]::$unit)
                        }
                    )+
                )+
            }

            impl UnitAbbreviation for FloatType {
                fn abbreviation(&self) -> &'static str {
                    match self {
                        $(
                            $(
                                Self::$float([<$float Type>]::$unit) => $abbreviation,
                            )+
                        )+
                    }
                }
            }

            impl UnitAbbreviation for Float {
                fn abbreviation(&self) -> &'static str {
                    match self {
                        $(
                            Self::$float(q) => match q {
                                $(
                                   $float::$unit(_) => $abbreviation,
                                )+
                            }
                        )+
                    }
                }
            }

            impl Float {
                #[must_use]
                pub const fn from_f64_with_type(value: f64, float_type: FloatType) -> Self {
                    match float_type {
                        $(
                            $(
                                FloatType::$float([<$float Type>]::$unit) => Self::[<$unit:snake>](value),
                            )+
                        )+
                    }
                }

                #[must_use]
                pub const fn float_type(&self) -> FloatType {
                    match self {
                        $(
                            Self::$float(q) => match q {
                                $(
                                   $float::$unit(_) => FloatType::$float([<$float Type>]::$unit),
                                )+
                            }
                        )+
                    }
                }

                $(
                    #[must_use]
                    pub const fn [<$float:snake>](v: $float) -> Self {
                        Self::$float(v)
                    }
                )+

                $(
                    $(
                        #[must_use]
                        pub const fn [<$unit:snake>](v: f64) -> Self {
                            Self::$float($float::$unit($unit::new(v)))
                        }
                    )+
                )+

                $(
                    #[allow(irrefutable_let_patterns)]
                    #[must_use]
                    pub const fn [<as_ $float:snake>](self) -> Option<$float> {
                        let Self::$float(v) = self else {
                            return None;
                        };
                        Some(v)
                    }

                    #[must_use]
                    pub fn [<as_ $float:snake _unchecked>](self) -> $float {
                        self.[<as_ $float:snake>]()
                            .ok_or_else(||
                              format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                                  expected_type = stringify!($float),
                                  actual_type = self.float_type(),
                                  value = self
                              )
                            )
                            .unwrap()
                    }
                )+

                $(
                    $(
                        #[allow(irrefutable_let_patterns)]
                        #[must_use]
                        pub const fn [<as_ $unit:snake>](self) -> Option<$unit> {
                            let Self::$float(v) = self else {
                                return None;
                            };
                            v.[<as_ $unit:snake>]()
                        }

                        #[must_use]
                        pub fn [<as_ $unit:snake _unchecked>](self) -> $unit {
                            self.[<as_ $unit:snake>]()
                                .ok_or_else(||
                                  format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                                      expected_type = stringify!($unit),
                                      actual_type = self.float_type(),
                                      value = self
                                  )
                                )
                                .unwrap()
                        }
                    )+
                )+
            }

            impl From<Float> for f64 {
                fn from(from: Float) -> Self {
                    match from {
                        $(
                            $(
                                Float::$float($float::$unit(v)) => f64::from(v),
                            )+
                        )+
                    }
                }
            }

            $(
                $(
                    impl From<$unit> for Float {
                        fn from(from: $unit) -> Self {
                            Self::from($float::from(from))
                        }
                    }
                )+
            )+

            $(
                float!{
                    $(#[$float_attr])*
                    $float(f64) {
                        $($unit, $conv, $abbreviation;)+
                    }
                }
            )+
        }
    };
}

macro_rules! integers {
    (
        $(
            $(#[int_attr:meta])*
            $int:ident, $int_base_type:ty;
        )+
    ) =>
    {
        paste! {
            /// Typed integer value.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum Int {
                $(
                    $int($int),
                )+
            }

            impl Int {
                #[must_use]
                pub const fn int_type(&self) -> IntType {
                    match self {
                        $(
                          Self::$int(_) => IntType::$int,
                        )+
                    }
                }

                $(
                    #[must_use]
                    pub const fn [<$int:snake>](v: $int_base_type) -> Self {
                        Self::$int($int::new(v))
                    }
                )+

                $(
                    #[allow(irrefutable_let_patterns)]
                    #[must_use]
                    pub const fn [<as_ $int:snake>](self) -> Option<$int> {
                        let Self::$int(v) = self else {
                            return None;
                        };
                        Some(v)
                    }
                )+
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum IntType {
                $(
                    $int,
                )+
            }

            $(
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
                pub struct $int($int_base_type);

                impl $int {
                    #[must_use]
                    pub const fn new(v: $int_base_type) -> Self {
                        Self(v)
                    }

                    #[must_use]
                    pub const fn zero() -> Self {
                        Self(0)
                    }
                }

                impl From<$int> for $int_base_type {
                    fn from(v: $int) -> Self {
                        v.0
                    }
                }

                impl ValueTypeExt for $int {
                    const VALUE_TYPE: ValueType = ValueType::Scalar(ScalarType::Int(IntType::$int));
                }
            )+
        }
    };
}

macro_rules! values {
    (
        scalars {
            floats {
                $(
                    $(#[$float_attr:meta])*
                    $float:ident {
                        $($unit:ident, $conv:expr, $abbreviation:expr;)+
                    }
                )+
            }
            integers {
                $(
                    $(#[int_attr:meta])*
                    $int:ident, $int_base_type:ty;
                )+
            }
            enums {
                $(
                    $(#[$enum_attr:meta])*
                    $enum_name:ident {
                        $($enum_body:tt)*
                    }
                )+
            }
        }
    ) => {
        paste! {

            floats! {
                $(
                    $(#[$float_attr])*
                    $float {
                        $($unit, $conv, $abbreviation;)+
                    }
                )+
            }

            integers! {
                $(
                    #[int_attr:meta]
                    $int, $int_base_type;
                )+
            }

            pub trait ValueTypeExt {
                const VALUE_TYPE : ValueType;
            }

            impl ValueTypeExt for bool {
                const VALUE_TYPE : ValueType = ValueType::Scalar(ScalarType::Bool);
            }

            impl ValueTypeExt for String {
                const VALUE_TYPE : ValueType = ValueType::Text;
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, From)]
            pub enum Enum {
                $(
                    $enum_name($enum_name),
                )+
            }

            impl Enum {
                #[must_use]
                pub const fn enum_type(&self) -> EnumType {
                    match self {
                        $(
                            Self::$enum_name(_) => EnumType::$enum_name,
                        )+
                    }
                }

                $(
                    #[allow(irrefutable_let_patterns)]
                    #[must_use]
                    pub const fn [<as_ $enum_name:snake>](self) -> Option<$enum_name> {
                        let Self::$enum_name(v) = self else {
                            return None;
                        };
                        Some(v)
                    }

                    #[must_use]
                    pub fn [<as_ $enum_name:snake _unchecked>](self) -> $enum_name {
                        self.[<as_ $enum_name:snake>]()
                            .ok_or_else(||
                              format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                                  expected_type = stringify!($enum_name),
                                  actual_type = self.enum_type(),
                                  value = self
                              )
                            )
                            .unwrap()
                    }
                )+
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum EnumType {
                $(
                    $enum_name,
                )+
            }

            $(
                $(#[$enum_attr])*
                #[derive(Debug, Clone, Copy, PartialEq, Eq)]
                pub enum $enum_name {
                    $($enum_body)*
                }

                impl ValueTypeExt for $enum_name {
                    const VALUE_TYPE: ValueType = ValueType::Enum(EnumType::$enum_name);
                }
            )+

            /// A typed value.
            #[derive(Debug, Clone, PartialEq, From)]
            pub enum Value {
                Scalar(Scalar),
                Text(String),
                Enum(Enum),
            }

            impl Value {
                #[must_use]
                pub const fn value_type(&self) -> ValueType {
                    match self {
                        Self::Scalar(s) => ValueType::Scalar(s.scalar_type()),
                        Self::Text(_) => ValueType::Text,
                        Self::Enum(e) => ValueType::Enum(e.enum_type()),
                    }
                }

                // --- constructors --- //

                $(
                    $(
                        #[must_use]
                        pub const fn [<$unit:snake>](v: f64) -> Self {
                            Self::Scalar(
                              Scalar::Float(Float::[<$unit:snake>](v))
                            )
                        }

                    )+
                )+

                $(
                    #[must_use]
                    pub const fn [<$int:snake>](v: $int_base_type) -> Self {
                        Self::Scalar(Scalar::Int(Int::$int($int::new(v))))
                    }
                )+

                $(
                    #[must_use]
                    pub const fn [<$enum_name:snake>](v: $enum_name) -> Self {
                        Self::Enum(Enum::$enum_name(v))
                    }
                )+

                #[must_use]
                pub const fn bool(v: bool) -> Self {
                    Self::Scalar(Scalar::Bool(v))
                }

                pub fn text(s: impl Into<String>) -> Self {
                    Self::Text(s.into())
                }

                // --- getters --- //


                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_scalar(self) -> Option<Scalar> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_scalar_unchecked(self) -> Scalar {
                    self.clone().as_scalar()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(Scalar),
                              actual_type = self.value_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_text(self) -> Option<String> {
                    let Self::Text(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_text_unchecked(self) -> String {
                    self.clone().as_text()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(String),
                              actual_type = self.value_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_float(self) -> Option<Float> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    v.as_float()
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_float_unchecked(self) -> Float {
                    self.clone().as_float()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(Float),
                              actual_type = self.value_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_int(self) -> Option<Int> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    v.as_int()
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_int_unchecked(self) -> Int {
                    self.clone().as_int()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(Int),
                              actual_type = self.value_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_bool(self) -> Option<bool> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    v.as_bool()
                }

                #[must_use]
                // TODO: rename 'as' -> 'into'
                pub fn as_bool_unchecked(self) -> bool {
                    self.clone().as_bool()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(bool),
                              actual_type = self.value_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }

                $(
                    $(
                        #[must_use]
                        pub fn [<as_ $unit:snake>](self) -> Option<$unit> {
                            let Self::Scalar(v) = self else {
                                return None;
                            };
                            v.[<as_ $unit:snake>]()
                        }

                        #[must_use]
                        pub fn [<as_ $unit:snake _unchecked>](self) -> $unit {
                            self.clone().[<as_ $unit:snake>]()
                                .ok_or_else(||
                                  format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                                      expected_type = stringify!($unit),
                                      actual_type = self.value_type(),
                                      value = self
                                  )
                                )
                                .unwrap()
                        }
                    )+
                )+

                $(
                    #[must_use]
                    pub fn [<as_ $int:snake>](self) -> Option<$int> {
                        let Self::Scalar(v) = self else {
                            return None;
                        };
                        v.[<as_ $int:snake>]()
                    }

                    #[must_use]
                    pub fn [<as_ $int:snake _unchecked>](self) -> $int {
                        self.clone().[<as_ $int:snake>]()
                            .ok_or_else(||
                              format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                                  expected_type = stringify!($init),
                                  actual_type = self.value_type(),
                                  value = self
                              )
                            )
                            .unwrap()
                    }
                )+

                $(
                    #[must_use]
                    pub fn [<as_ $enum_name:snake>](self) -> Option<$enum_name> {
                        let Self::Enum(v) = self else {
                            return None;
                        };
                        v.[<as_ $enum_name:snake>]()
                    }

                    #[must_use]
                    pub fn [<as_ $enum_name:snake _unchecked>](self) -> $enum_name {
                        self.clone().[<as_ $enum_name:snake>]()
                            .ok_or_else(||
                              format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                                  expected_type = stringify!($enum_name),
                                  actual_type = self.value_type(),
                                  value = self
                              )
                            )
                            .unwrap()
                    }
                )+
            }

            impl<T> From<T> for Value
            where
                Float: From<T>,
            {
                fn from(from: T) -> Self {
                    Self::Scalar(Scalar::Float(Float::from(from)))
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum ValueType {
                Scalar(ScalarType),
                Text,
                Enum(EnumType),
            }

            impl ValueType {
                $(
                    $(
                        #[must_use]
                        pub const fn [<$unit:snake>]() -> Self {
                            Self::Scalar(
                                ScalarType::Float(
                                  FloatType::[<$unit:snake>]()
                                )
                            )
                        }
                    )+
                )+

                $(
                    #[must_use]
                    pub const fn [<$int:snake>]() -> Self {
                        Self::Scalar(ScalarType::Int(IntType::$int))
                    }
                )+

                $(
                    #[must_use]
                    pub const fn [<$enum_name:snake>]() -> Self {
                        Self::Enum(EnumType::$enum_name)
                    }
                )+

                #[must_use]
                pub const fn bool() -> Self {
                    Self::Scalar(ScalarType::Bool)
                }

                #[must_use]
                pub const fn text() -> Self {
                    Self::Text
                }
            }

            /// Elementary value that represents only a single data unit or value.
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
            pub enum Scalar {
                Float(Float),
                Int(Int),
                Bool(bool),
            }

            impl Scalar {
                #[must_use]
                pub const fn scalar_type(&self) -> ScalarType {
                    match self {
                      Self::Float(f) => ScalarType::Float(f.float_type()),
                      Self::Int(i) => ScalarType::Int(i.int_type()),
                      Self::Bool(_) => ScalarType::Bool,
                    }
                }

                $(
                    $(
                        #[must_use]
                        pub const fn [<$unit:snake>](v: f64) -> Self {
                            Self::Float(Float::[<$unit:snake>](v))
                        }
                    )+
                )+

                $(
                    #[must_use]
                    pub const fn [<$int:snake>](v: $int_base_type) -> Self {
                        Self::Int(Int::[<$int:snake>](v))
                    }
                )+

                #[must_use]
                pub const fn bool(v: bool) -> Self {
                    Self::Bool(v)
                }

                $(
                    $(
                        #[must_use]
                        pub const fn [<as_ $unit:snake>](self) -> Option<$unit> {
                            let Self::Float(v) = self else {
                                return None;
                            };
                            v.[<as_ $unit:snake>]()
                        }
                    )+
                )+

                $(
                    #[must_use]
                    pub const fn [<as_ $int:snake>](self) -> Option<$int> {
                        let Self::Int(v) = self else {
                            return None;
                        };
                        v.[<as_ $int:snake>]()
                    }
                )+

                #[must_use]
                pub const fn as_float(self) -> Option<Float> {
                    let Self::Float(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                #[must_use]
                pub fn as_float_unchecked(self) -> Float {
                    self.as_float()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(Float),
                              actual_type = self.scalar_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }

                #[must_use]
                pub const fn as_int(self) -> Option<Int> {
                    let Self::Int(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                #[must_use]
                pub fn as_int_unchecked(self) -> Int {
                    self.as_int()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(Int),
                              actual_type = self.scalar_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }

                #[must_use]
                pub const fn as_bool(self) -> Option<bool> {
                    let Self::Bool(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                #[must_use]
                pub fn as_bool_unchecked(self) -> bool {
                    self.as_bool()
                        .ok_or_else(||
                          format!("expected a {expected_type} but found a {actual_type:?}: {value:?}",
                              expected_type = stringify!(bool),
                              actual_type = self.scalar_type(),
                              value = self
                          )
                        )
                        .unwrap()
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum ScalarType {
                Float(FloatType),
                Int(IntType),
                Bool,
            }
        }
    };
}

values! {
    scalars {
        floats {
            Ratio {
                Factor, 1.0, "";
                Percent, 0.01, "%";
            }
            Volume {
                Liters, 1.0, "l";
                Qubicmeters, 1_000.0, "m³";
            }
            Length {
                Kilometers, 1.0, "km";
            }
            Mass {
                Grams, 1.0, "g";
                Kilograms, 1_000.0, "kg";
                Tons, 1_000_000.0, "t";
            }
            Density {
                MilligramsPerLiter, 1.0, "mg/l";
                KilogramsPerQubicmeter, 1_000.0, "kg/m³";
                KilogramsPerLiter, 1_000_000.0, "kg/l";
            }
            Energy {
                Kilowatthours, 1.0, "kWh";
            }
            SpecificEnergyDensity {
                GramsPerKilowatthour, 1.0, "g/kWh";
            }
            FuelConsumption {
                LitersPerKilometer, 1.0, "l/km";
            }
            TransportFuelConsumption {
                LitersPerTonKilometer, 1.0, "l/tkm";
            }
            FlowRate {
                QubicmetersPerHour, 1.0, "m³/h";
            }
            Time { // TODO: use integer type?
                Hours, 1.0, "h";
                Days, 24.0, "d";
                Years, 24.0 * 365.0, "y";
            }
        }
        integers {
            Count, u64;
        }
        enums {
            #[derive(Default, FromPrimitive, ToPrimitive, AsRefStr)]
            N2oEmissionFactorCalcMethod {
              #[default]
              TuWien2016,
              Optimistic,
              Pesimistic,
              Ipcc2019,
              Custom,
            }
            #[derive(Default, FromPrimitive, ToPrimitive, AsRefStr)]
            Ch4ChpEmissionFactorCalcMethod {
                #[default]
                MicroGasTurbines,
                GasolineEngine,
                JetEngine,
                Custom,
            }
        }
    }
}
