use crate::OutputValueId;

pub trait ValueColor {
    fn color(&self) -> &'static str;
    fn color_light(&self) -> &'static str {
        self.color()
    }
}

const RED: &str = "red";
const RED_LIGHT: &str = "#ffb2b2";

const YELLOW: &str = "yellow";
const YELLOW_LIGHT: &str = "#fff5b2";

const ORANGE: &str = "orange";
const ORANGE_LIGHT: &str = "#ffe4b2";

const GREY: &str = "grey";
const GREY_LIGHT: &str = "lightgrey";

impl ValueColor for OutputValueId {
    fn color(&self) -> &'static str {
        match self {
            Self::N2oPlant
            | Self::N2oWater
            | Self::N2oSideStream
            | Self::N2oEmissions
            | Self::Ch4Plant
            | Self::Ch4SludgeStorageContainers
            | Self::Ch4SludgeBags
            | Self::Ch4Water
            | Self::Ch4CombinedHeatAndPowerPlant
            | Self::Ch4Emissions
            | Self::DirectEmissions
            | Self::FossilEmissions => RED,

            Self::Fecl3
            | Self::Feclso4
            | Self::Caoh2
            | Self::OperatingMaterials
            | Self::SewageSludgeTransport
            | Self::SyntheticPolymers
            | Self::OtherIndirectEmissions => YELLOW,

            Self::ElectricityMix
            | Self::OilEmissions
            | Self::GasEmissions
            | Self::IndirectEmissions => ORANGE,

            Self::TotalEmissions => GREY,

            _ => todo!(),
        }
    }

    fn color_light(&self) -> &'static str {
        match self {
            Self::N2oPlant
            | Self::N2oWater
            | Self::N2oSideStream
            | Self::N2oEmissions
            | Self::Ch4Plant
            | Self::Ch4SludgeStorageContainers
            | Self::Ch4SludgeBags
            | Self::Ch4Water
            | Self::Ch4CombinedHeatAndPowerPlant
            | Self::Ch4Emissions
            | Self::DirectEmissions
            | Self::FossilEmissions => RED_LIGHT,

            Self::Fecl3
            | Self::Feclso4
            | Self::Caoh2
            | Self::OperatingMaterials
            | Self::SewageSludgeTransport
            | Self::SyntheticPolymers
            | Self::OtherIndirectEmissions => YELLOW_LIGHT,

            Self::ElectricityMix
            | Self::OilEmissions
            | Self::GasEmissions
            | Self::IndirectEmissions => ORANGE_LIGHT,

            Self::TotalEmissions => GREY_LIGHT,

            _ => self.color(),
        }
    }
}
