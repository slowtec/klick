use std::{
    fmt,
    ops::{Add, Div, Mul, Sub, SubAssign},
};

use derive_more::From;
use paste::paste;

mod conversion;

#[cfg(test)]
mod tests;

macro_rules! quantity {
    (
        $(#[$quantity_attr:meta])*
        $quantity:ident {
            $($unit:ident, $conv:expr, $abbreviation:expr;)+
        }
    ) =>
    {
        paste!{
            $(#[$quantity_attr])*
            pub trait [<$quantity Ext>]
            {
                const CONVERSION_FACTOR: f64;
                fn convert_to<T>(self) -> T where T: [<$quantity Ext>];

                #[doc(hidden)]
                fn new_from_base_value(v: f64) -> Self;
            }

            #[derive(Debug, Clone, Copy, PartialEq, From)]
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
            }

            // units
            $(
                #[doc = concat!(stringify!($unit), " `[",$abbreviation, "].`")]
                #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
                pub struct $unit(f64);

                impl $unit {
                    #[must_use]
                    pub const fn new(value: f64) -> Self {
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
                    const CONVERSION_FACTOR: f64 = $conv;
                    fn convert_to<T>(self) -> T where T: [<$quantity Ext>] {
                        let base = self.0 * Self::CONVERSION_FACTOR;
                        T::new_from_base_value(base)
                    }

                    #[doc(hidden)]
                    fn new_from_base_value(base_value: f64) -> Self {
                        Self::new(base_value / Self::CONVERSION_FACTOR)
                    }
                }

                impl From<$unit> for f64 {
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

macro_rules! quantities {
    (
        $(
            $(#[$quantity_attr:meta])*
            $quantity:ident {
                $($unit:ident, $conv:expr, $abbreviation:expr;)+
            }
        )+
    ) => {
        paste! {

            #[derive(Debug, Clone, Copy, PartialEq, From)]
            pub enum Quantity {
                $(
                  $quantity($quantity),
                )+
            }

            #[derive(Debug, Clone, Copy, PartialEq, From)]
            pub enum QuantityType {
                $(
                  $quantity([<$quantity Type>]),
                )+
            }

            impl Quantity {
                pub const fn quantity_type(&self) -> QuantityType {
                    match self {
                        $(
                            Self::$quantity(q) => match q {
                                $(
                                   $quantity::$unit(_) => QuantityType::$quantity([<$quantity Type>]::$unit),
                                )+
                            }
                        )+
                    }
                }
            }

            $(
                $(
                    impl From<$unit> for Quantity {
                        fn from(from: $unit) -> Self {
                            Self::from($quantity::from(from))
                        }
                    }
                )+
            )+

            $(
                quantity!{
                    $(#[$quantity_attr])*
                    $quantity {
                        $($unit, $conv, $abbreviation;)+
                    }
                }
            )+
        }
    };
}

quantities! {
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
    Time {
        Hours, 1.0, "h";
        Days, 24.0, "d";
        Years, 24.0 * 365.0, "y";
    }
}
