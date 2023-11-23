macro_rules! unit {

    ($name:ident, $abbreviation:expr, $description:expr) => {

        #[doc = concat!($description, "`[",$abbreviation, "].`")]
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name(f64);

        impl $name {
            #[must_use]
            pub const fn new(value: f64) -> Self {
                Self(value)
            }
        }

        impl From<$name> for f64 {
            fn from(from: $name) -> Self {
                from.0
            }
        }
    };
}

unit!(Kilometers, "km", "Kilometers");
unit!(Tons, "t", "Tons");

mod factor;
mod percent;

pub use self::{factor::*, percent::*};
