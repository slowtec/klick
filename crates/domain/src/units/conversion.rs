use std::ops::Mul;

#[allow(clippy::wildcard_imports)]
use super::*;

macro_rules! direct_multiply {
    (
      $unit_a:ident, $unit_b:ident, $output:ident
    ) => {
        impl Mul<$unit_a> for $unit_b {
            type Output = $output;
            fn mul(self, rhs: $unit_a) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }

        impl Mul<$unit_b> for $unit_a {
            type Output = $output;
            fn mul(self, rhs: $unit_b) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }
    };
}

direct_multiply!(Qubicmeters, KilogramsPerQubicmeter, Kilograms);
direct_multiply!(Kilowatthours, GramsPerKilowatthour, Grams);
direct_multiply!(LitersPerTonKilometer, Tons, LitersPerKilometer);
direct_multiply!(Kilometers, LitersPerKilometer, Liters);
direct_multiply!(KilogramsPerLiter, Liters, Kilograms);

impl From<Percent> for Factor {
    fn from(from: Percent) -> Factor {
        Factor::new(from.0 / 100.0)
    }
}

impl Mul<Factor> for f64 {
    type Output = f64;

    fn mul(self, rhs: Factor) -> f64 {
        self * rhs.0
    }
}

impl Mul<Percent> for f64 {
    type Output = f64;
    fn mul(self, value: Percent) -> Self::Output {
        self * value.0 / 100.0
    }
}

impl Mul<Qubicmeters> for MilligramsPerLiter {
    type Output = Kilograms;
    fn mul(self, rhs: Qubicmeters) -> Self::Output {
        let kg_p_m3 = self.convert_to::<KilogramsPerQubicmeter>();
        Self::Output::new(kg_p_m3.0 * rhs.0)
    }
}

impl Mul<MilligramsPerLiter> for Qubicmeters {
    type Output = Kilograms;
    fn mul(self, rhs: MilligramsPerLiter) -> Self::Output {
        let kg_p_m3 = rhs.convert_to::<KilogramsPerQubicmeter>();
        Self::Output::new(self.0 * kg_p_m3.0)
    }
}
