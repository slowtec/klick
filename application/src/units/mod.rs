use std::{
    fmt,
    ops::{Add, Div, Mul, Sub, SubAssign},
};
use klick_convert_numbers::format_de_number;

mod conversion;

#[cfg(test)]
mod tests;

macro_rules! quantity {
    (
      $(#[$quantity_attr:meta])* $quantity:ident;
      units {
          $($unit:ident, $conv:expr, $abbreviation:expr;)+
      }
    ) =>
    {
        $(#[$quantity_attr])*
        pub trait $quantity
        {
            const CONVERSION_FACTOR: f64;
            fn convert_to<T>(self) -> T where T: $quantity;

            #[doc(hidden)]
            fn new_from_base_value(v: f64) -> Self;
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
            }

            impl $quantity for $unit {
                const CONVERSION_FACTOR: f64 = $conv;
                fn convert_to<T>(self) -> T where T: $quantity {
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

            impl Mul<Factor> for $unit {
                type Output = $unit;
                fn mul(self, rhs: Factor) -> Self::Output {
                    $unit::new(self.0 * rhs.0)
                }
            }

            impl fmt::Display for $unit {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{} {}", format_de_number(self.0, ","), $abbreviation)
                }
            }
        )+
    };
}

quantity! {
    Ratio;
    units {
        Factor, 1.0, "";
        Percent, 0.01, "%";
    }
}

quantity! {
    Volume;
    units {
        Liters, 1.0, "l";
        Qubicmeters, 1_000.0, "m³";
    }
}

quantity! {
    Length;
    units {
        Kilometers, 1.0, "km";
    }
}

quantity! {
    Mass;
    units {
        Grams, 1.0, "g";
        Kilograms, 1_000.0, "kg";
        Tons, 1_000_000.0, "t";
    }
}

quantity! {
    Density;
    units {
        MilligramsPerLiter, 1.0, "mg/l";
        KilogramsPerQubicmeter, 1_000.0, "kg/m³";
        KilogramsPerLiter, 1_000_000.0, "kg/l";
    }
}

quantity! {
    Energy;
    units {
        Kilowatthours, 1.0, "kWh";
    }
}

quantity! {
    SpecificEnergyDensity;
    units {
        GramsPerKilowatthour, 1.0, "g/kWh";
    }
}

quantity! {
    FuelConsumption;
    units {
        LitersPerKilometer, 1.0, "l/km";
    }
}

quantity! {
    TransportFuelConsumption;
    units {
        LitersPerTonKilometer, 1.0, "l/tkm";
    }
}
