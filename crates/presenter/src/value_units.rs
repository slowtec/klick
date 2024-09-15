use klick_domain::{InputValueId, OutputValueId, ValueId};

// TODO:
// Actually, we should derive the units directly from the domain layer
// (specifically from `domain::units`),
// but since we also display untyped values from the system profile here,
// we sometimes lack the information.
// We should therefore either reflect the `PlantProfile` in the domain layer
// or typify the values in the boundary layer.
// As long as we do not have this, we define the units here manually.
pub trait ValueUnit {
    /// Unit abbreviation for usage in LaTeX.
    ///
    /// The package `siunitx` is required
    /// in your LaTeX document, so add
    /// `\usepackage{siunitx}` to the header.
    fn unit_as_latex(&self) -> Option<&'static str> {
        None
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        None
    }
}

const LATEX_PERCENT: &str = "\\%";
const LATEX_QUBICMETERS: &str = r"\si{\metre}^3";
const LATEX_KILOMETERS: &str = r"\si{\kilo\metre}";
const LATEX_TONS: &str = r"\si{\tonne}";
const LATEX_MILLIGRAMSPERLITER: &str = r"\si{\milli\gram\per\liter}";
const LATEX_KILOWATTHOURS: &str = r"\si{\kilo\watt\hour}";
const LATEX_GRAMSPERKILOWATTHOUR: &str = r"\si{\gram\per\kilo\watt\hour}";

const TEXT_PERCENT: &str = "%";
const TEXT_QUBICMETERS: &str = "m³";
const TEXT_KILOMETERS: &str = "km";
const TEXT_TONS: &str = "t";
const TEXT_MILLIGRAMSPERLITER: &str = "mg/l";
const TEXT_KILOWATTHOURS: &str = "kWh";
const TEXT_GRAMSPERKILOWATTHOUR: &str = "g/kWh";
const TEXT_QUBICMETERS_PER_HOUR: &str = "m³/h";

impl ValueUnit for OutputValueId {}

impl ValueUnit for InputValueId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::ProfilePlantName => None,
            Self::ProfilePopulationEquivalent => None,
            Self::ProfileWastewater => Some(LATEX_QUBICMETERS),
            Self::ProfileInfluentNitrogen | Self::ProfileInfluentChemicalOxygenDemand => {
                Some(LATEX_MILLIGRAMSPERLITER)
            }
            Self::ProfileInfluentTotalOrganicCarbohydrates => Some(LATEX_MILLIGRAMSPERLITER),
            Self::ProfileEffluentNitrogen | Self::ProfileEffluentChemicalOxygenDemand => {
                Some(LATEX_MILLIGRAMSPERLITER)
            }
            Self::ProfileSewageGasProduced => Some(LATEX_QUBICMETERS),
            Self::ProfileMethaneFraction => Some(LATEX_PERCENT),
            Self::ProfileGasSupply => Some(LATEX_KILOWATTHOURS),
            Self::ProfilePurchaseOfBiogas => None,
            Self::ProfileTotalPowerConsumption => Some(LATEX_KILOWATTHOURS),
            Self::ProfileOnSitePowerGeneration => Some(LATEX_KILOWATTHOURS),
            Self::ProfileEmissionFactorElectricityMix => Some(LATEX_GRAMSPERKILOWATTHOUR),
            Self::ProfileHeatingOil => Some(LATEX_TONS),
            Self::ProfileSideStreamTotalNitrogen => Some(LATEX_TONS),
            Self::ProfileSludgeBagsAreOpen => None, // FIXME implement latex representation
            Self::ProfileSludgeStorageContainersAreOpen => None, // FIXME implement latex representation
            Self::ProfileSludgeDisposal => Some(LATEX_TONS),
            Self::ProfileSludgeTransportDistance => Some(LATEX_KILOMETERS),
            Self::ProfileSludgeDigesterCount => None,
            Self::ProfileOperatingMaterialFeCl3
            | Self::ProfileOperatingMaterialFeClSO4
            | Self::ProfileOperatingMaterialCaOH2
            | Self::ProfileOperatingMaterialSyntheticPolymers => Some(LATEX_TONS),
            // FIXME
            _ => None,
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::ProfilePlantName => None,
            Self::ProfilePopulationEquivalent => None,
            Self::ProfileWastewater => Some(TEXT_QUBICMETERS),
            Self::ProfileInfluentNitrogen | Self::ProfileInfluentChemicalOxygenDemand => {
                Some(TEXT_MILLIGRAMSPERLITER)
            }
            Self::ProfileInfluentTotalOrganicCarbohydrates => Some(TEXT_MILLIGRAMSPERLITER),
            Self::ProfileEffluentNitrogen | Self::ProfileEffluentChemicalOxygenDemand => {
                Some(TEXT_MILLIGRAMSPERLITER)
            }
            Self::ProfileSewageGasProduced => Some(TEXT_QUBICMETERS),
            Self::ProfileMethaneFraction => Some(TEXT_PERCENT),
            Self::ProfileGasSupply => Some(TEXT_KILOWATTHOURS),
            Self::ProfilePurchaseOfBiogas => None,
            Self::ProfileTotalPowerConsumption => Some(TEXT_KILOWATTHOURS),
            Self::ProfileOnSitePowerGeneration => Some(TEXT_KILOWATTHOURS),
            Self::ProfileEmissionFactorElectricityMix => Some(TEXT_GRAMSPERKILOWATTHOUR),
            Self::ProfileHeatingOil => Some(TEXT_TONS),
            Self::ProfileSideStreamTotalNitrogen => Some(TEXT_TONS),
            Self::ProfileSludgeBagsAreOpen => None,
            Self::ProfileSludgeStorageContainersAreOpen => None,
            Self::ProfileSludgeDisposal => Some(TEXT_TONS),
            Self::ProfileSludgeTransportDistance => Some(TEXT_KILOMETERS),
            Self::ProfileSludgeDigesterCount => None,
            Self::ProfileOperatingMaterialFeCl3
            | Self::ProfileOperatingMaterialFeClSO4
            | Self::ProfileOperatingMaterialCaOH2
            | Self::ProfileOperatingMaterialSyntheticPolymers => Some(TEXT_TONS),
            Self::SensitivityN2OCalculationMethod | Self::SensitivityCH4ChpCalculationMethod => {
                None
            }
            Self::SensitivityN2OCustomFactor
            | Self::SensitivityN2OSideStreamFactor
            | Self::SensitivityCH4ChpCustomFactor
            | Self::SensitivityCO2FossilCustomFactor
            | Self::SensitivitySludgeStorageCustomFactor => Some(TEXT_PERCENT),
            Self::SensitivitySludgeBagsCustomFactor => Some(TEXT_QUBICMETERS_PER_HOUR),
            // FIXME
            _ => None,
        }
    }
}

impl ValueUnit for ValueId {
    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            ValueId::Custom(_) => Some("t"), // Currently only tons are allowed
            ValueId::In(id) => id.unit_as_text(),
            ValueId::Out(id) => id.unit_as_text(),
        }
    }
}
