use std::ops::{Div, Mul};

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

        impl Mul<$unit_a> for &$unit_b {
            type Output = $output;
            fn mul(self, rhs: $unit_a) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }

        impl Mul<&$unit_a> for $unit_b {
            type Output = $output;
            fn mul(self, rhs: &$unit_a) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }

        impl Mul<$unit_b> for $unit_a {
            type Output = $output;
            fn mul(self, rhs: $unit_b) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }

        impl Mul<&$unit_b> for $unit_a {
            type Output = $output;
            fn mul(self, rhs: &$unit_b) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }

        impl Mul<$unit_b> for &$unit_a {
            type Output = $output;
            fn mul(self, rhs: $unit_b) -> Self::Output {
                Self::Output::new(self.0 * rhs.0)
            }
        }
    };
}

macro_rules! multiply {
    ($Output:ty: |$lhs:ident: $Lhs:ty, $rhs:ident: $Rhs:ty| $body:block) => {
        impl Mul<$Rhs> for $Lhs {
            type Output = $Output;
            fn mul(self, $rhs: $Rhs) -> Self::Output {
                let $lhs = self;
                $body
            }
        }

        impl Mul<$Rhs> for &$Lhs {
            type Output = $Output;
            fn mul(self, $rhs: $Rhs) -> Self::Output {
                let $lhs = self;
                $body
            }
        }

        impl Mul<&$Rhs> for $Lhs {
            type Output = $Output;
            fn mul(self, $rhs: &$Rhs) -> Self::Output {
                let $lhs = self;
                $body
            }
        }

        impl Mul<&$Rhs> for &$Lhs {
            type Output = $Output;
            fn mul(self, $rhs: &$Rhs) -> Self::Output {
                let $lhs = self;
                $body
            }
        }
    };
}

macro_rules! direct_divide {
    (
      $unit_a:ident, $unit_b:ident, $output:ident
    ) => {
        impl Div<$unit_a> for $unit_b {
            type Output = $output;
            fn div(self, rhs: $unit_a) -> Self::Output {
                Self::Output::new(self.0 / rhs.0)
            }
        }
    };
}

direct_multiply!(Qubicmeters, KilogramsPerQubicmeter, Kilograms);
direct_multiply!(Kilowatthours, GramsPerKilowatthour, Grams);
direct_multiply!(LitersPerTonKilometer, Tons, LitersPerKilometer);
direct_multiply!(Kilometers, LitersPerKilometer, Liters);
direct_multiply!(KilogramsPerLiter, Liters, Kilograms);
direct_multiply!(QubicmetersPerHour, Hours, Qubicmeters);

direct_divide!(Qubicmeters, Hours, QubicmetersPerHour);
direct_divide!(Liters, Kilometers, LitersPerKilometer);

multiply!(
    Kilograms: |lhs: MilligramsPerLiter, rhs: Qubicmeters|{
        let kg_p_m3 = lhs.convert_to::<KilogramsPerQubicmeter>();
        Kilograms::new(kg_p_m3.0 * rhs.0)
    }
);

multiply!(
    Kilograms: |lhs: Qubicmeters, rhs: MilligramsPerLiter|{
        let kg_p_m3 = rhs.convert_to::<KilogramsPerQubicmeter>();
        Kilograms::new(kg_p_m3.0 * lhs.0)
    }
);

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
