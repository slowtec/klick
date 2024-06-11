use std::{
    fmt,
    ops::{Add, Div, Mul, Sub, SubAssign},
};

use derive_more::From;
use paste::paste;

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

            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum [<$quantity Type>] {
                $(
                  $unit,
                )+
            }

            impl $quantity {
                pub fn convert_to<T>(self) -> Self
                    where T: [<$quantity Ext>] + Into<Self>
                {
                    match self {
                        $(
                            Self::$unit(value) => value.convert_to::<T>().into(),
                        )+
                    }
                }

                pub const fn [<$quantity:snake _type>](&self) -> [<$quantity Type>] {
                    match self {
                        $(
                          Self::$unit(_) => [<$quantity Type>]::$unit,
                        )+
                    }
                }

                $(
                    #[allow(irrefutable_let_patterns)]
                    pub fn [<as_ $unit:snake>](self) -> Option<$unit> {
                        let Self::$unit(v) = self else {
                            return None;
                        };
                        Some(v)
                    }

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
                        let scaling_factor = 10_f64.powi(precision as i32);
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

            /// Typed floating-point number.
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
            pub enum Float {
                $(
                  $float($float),
                )+
            }

            #[derive(Debug, Clone, Copy, PartialEq, From)]
            pub enum FloatType {
                $(
                  $float([<$float Type>]),
                )+
            }

            impl Float {

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
                    pub const fn [<new_ $float:snake>](v: $float) -> Self {
                        Self::$float(v)
                    }
                )+

                $(
                    $(
                        pub const fn [<new_ $unit:snake>](v: f64) -> Self {
                            Self::$float($float::$unit($unit::new(v)))
                        }
                    )+
                )+

                $(
                    #[allow(irrefutable_let_patterns)]
                    pub fn [<as_ $float:snake>](self) -> Option<$float> {
                        let Self::$float(v) = self else {
                            return None;
                        };
                        Some(v)
                    }

                    pub fn [<as_ $float:snake _unchecked>](self) -> $float {
                        self.[<as_ $float:snake>]().expect(concat!(stringify!($float), " value"))
                    }
                )+

                $(
                    $(
                        #[allow(irrefutable_let_patterns)]
                        pub fn [<as_ $unit:snake>](self) -> Option<$unit> {
                            let Self::$float(v) = self else {
                                return None;
                            };
                            v.[<as_ $unit:snake>]()
                        }

                        pub fn [<as_ $unit:snake _unchecked>](self) -> $unit {
                            self.[<as_ $unit:snake>]().expect(concat!(stringify!($unit)," value"))
                        }
                    )+
                )+
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
                pub const fn int_type(&self) -> IntType {
                    match self {
                        $(
                          Self::$int(_) => IntType::$int,
                        )+
                    }
                }

                $(
                    pub const fn [<new_ $int:snake>](v: $int_base_type) -> Self {
                        Self::$int($int::new(v))
                    }
                )+

                $(
                    #[allow(irrefutable_let_patterns)]
                    pub fn [<as_ $int:snake>](self) -> Option<$int> {
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
                    pub const fn new(v: $int_base_type) -> Self {
                        Self(v)
                    }
                }

                impl From<$int> for $int_base_type {
                    fn from(v: $int) -> Self {
                        v.0
                    }
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

            /// A typed value.
            #[derive(Debug, Clone, PartialEq, From)]
            pub enum Value {
                Scalar(Scalar),
                Text(String),
            }

            impl Value {
                #[must_use]
                pub const fn value_type(&self) -> ValueType {
                    match self {
                        Self::Scalar(s) => ValueType::Scalar(s.scalar_type()),
                        Self::Text(_) => ValueType::Text,
                    }
                }

                // --- constructors --- //

                $(
                    $(
                        pub const fn [<new_ $unit:snake>](v: f64) -> Self {
                            Self::Scalar(
                              Scalar::Float(Float::[<new_ $unit:snake>](v))
                            )
                        }

                    )+
                )+

                $(
                    pub const fn [<new_ $int:snake>](v: $int_base_type) -> Self {
                        Self::Scalar(Scalar::Int(Int::$int($int::new(v))))
                    }
                )+

                pub const fn new_bool(v: bool) -> Self {
                    Self::Scalar(Scalar::Bool(v))
                }

                pub fn new_text(s: impl Into<String>) -> Self {
                    Self::Text(s.into())
                }

                // --- getters --- //

                pub fn as_scalar(self) -> Option<Scalar> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                pub fn as_scalar_unchecked(self) -> Scalar {
                    self.as_scalar().expect("scalar value")
                }

                pub fn as_text(self) -> Option<String> {
                    let Self::Text(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                pub fn as_text_unchecked(self) -> String {
                    self.as_text().expect("text value")
                }

                pub fn as_float(self) -> Option<Float> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    v.as_float()
                }

                pub fn as_float_unchecked(self) -> Float {
                    self.as_float().expect("float value")
                }

                pub fn as_int(self) -> Option<Int> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    v.as_int()
                }

                pub fn as_int_unchecked(self) -> Int {
                    self.as_int().expect("integer value")
                }

                pub fn as_bool(self) -> Option<bool> {
                    let Self::Scalar(v) = self else {
                        return None;
                    };
                    v.as_bool()
                }

                pub fn as_bool_unchecked(self) -> bool {
                    self.as_bool().expect("bool value")
                }


                $(
                    $(
                        pub fn [<as_ $unit:snake>](self) -> Option<$unit> {
                            let Self::Scalar(v) = self else {
                                return None;
                            };
                            v.[<as_ $unit:snake>]()
                        }

                        pub fn [<as_ $unit:snake _unchecked>](self) -> $unit {
                            self.[<as_ $unit:snake>]().expect(concat!(stringify!($unit)," value"))
                        }
                    )+
                )+

                $(
                    pub fn [<as_ $int:snake>](self) -> Option<$int> {
                        let Self::Scalar(v) = self else {
                            return None;
                        };
                        v.[<as_ $int:snake>]()
                    }

                    pub fn [<as_ $int:snake _unchecked>](self) -> $int {
                        self.[<as_ $int:snake>]().expect(concat!(stringify!($int)," value"))
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
            }

            /// Elementary value that represents only a single data unit or value.
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, From)]
            pub enum Scalar {
                Float(Float),
                Int(Int),
                Bool(bool),
            }

            impl Scalar {
                pub const fn scalar_type(&self) -> ScalarType {
                    match self {
                      Self::Float(f) => ScalarType::Float(f.float_type()),
                      Self::Int(i) => ScalarType::Int(i.int_type()),
                      Self::Bool(_) => ScalarType::Bool,
                    }
                }

                $(
                    $(
                        pub const fn [<new_ $unit:snake>](v: f64) -> Self {
                            Self::Float(Float::[<new_ $unit:snake>](v))
                        }
                    )+
                )+

                $(
                    pub const fn [<new_ $int:snake>](v: $int_base_type) -> Self {
                        Self::Int(Int::[<new_ $int:snake>](v))
                    }
                )+

                pub const fn new_bool(v: bool) -> Self {
                    Self::Bool(v)
                }

                $(
                    $(
                        pub fn [<as_ $unit:snake>](self) -> Option<$unit> {
                            let Self::Float(v) = self else {
                                return None;
                            };
                            v.[<as_ $unit:snake>]()
                        }
                    )+
                )+

                $(
                    pub fn [<as_ $int:snake>](self) -> Option<$int> {
                        let Self::Int(v) = self else {
                            return None;
                        };
                        v.[<as_ $int:snake>]()
                    }
                )+

                pub fn as_float(self) -> Option<Float> {
                    let Self::Float(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                pub fn as_float_unchecked(self) -> Float {
                    self.as_float().expect("float value")
                }

                pub fn as_int(self) -> Option<Int> {
                    let Self::Int(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                pub fn as_int_unchecked(self) -> Int {
                    self.as_int().expect("integer value")
                }

                pub fn as_bool(self) -> Option<bool> {
                    let Self::Bool(v) = self else {
                        return None;
                    };
                    Some(v)
                }

                pub fn as_bool_unchecked(self) -> bool {
                    self.as_bool().expect("bool value")
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq)]
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
    }
}
