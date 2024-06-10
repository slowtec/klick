use crate::InputValueId;

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

impl ValueUnit for InputValueId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::PlantName => None,
            Self::PopulationEquivalent => None,
            Self::Wastewater => Some(LATEX_QUBICMETERS),
            Self::InfluentNitrogen | Self::InfluentChemicalOxygenDemand => {
                Some(LATEX_MILLIGRAMSPERLITER)
            }
            Self::InfluentTotalOrganicCarbohydrates => Some(LATEX_MILLIGRAMSPERLITER),
            Self::EffluentNitrogen | Self::EffluentChemicalOxygenDemand => {
                Some(LATEX_MILLIGRAMSPERLITER)
            }
            Self::SewageGasProduced => Some(LATEX_QUBICMETERS),
            Self::MethaneFraction => Some(LATEX_PERCENT),
            Self::GasSupply => Some(LATEX_KILOWATTHOURS),
            Self::PurchaseOfBiogas => None,
            Self::TotalPowerConsumption => Some(LATEX_KILOWATTHOURS),
            Self::OnSitePowerGeneration => Some(LATEX_KILOWATTHOURS),
            Self::EmissionFactorElectricityMix => Some(LATEX_GRAMSPERKILOWATTHOUR),
            Self::HeatingOil => Some(LATEX_TONS),
            Self::SideStreamTreatmentTotalNitrogen => Some(LATEX_TONS),
            Self::SludgeTreatmentBagsAreOpen => None, // FIXME implement latex representation
            Self::SludgeTreatmentStorageContainersAreOpen => None, // FIXME implement latex representation
            Self::SludgeTreatmentDisposal => Some(LATEX_TONS),
            Self::SludgeTreatmentTransportDistance => Some(LATEX_KILOMETERS),
            Self::SludgeTreatmentDigesterCount => None,
            Self::OperatingMaterialFeCl3
            | Self::OperatingMaterialFeClSO4
            | Self::OperatingMaterialCaOH2
            | Self::OperatingMaterialSyntheticPolymers => Some(LATEX_TONS),
            // FIXME
            _ => None,
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::PlantName => None,
            Self::PopulationEquivalent => None,
            Self::Wastewater => Some(TEXT_QUBICMETERS),
            Self::InfluentNitrogen | Self::InfluentChemicalOxygenDemand => {
                Some(TEXT_MILLIGRAMSPERLITER)
            }
            Self::InfluentTotalOrganicCarbohydrates => Some(TEXT_MILLIGRAMSPERLITER),
            Self::EffluentNitrogen | Self::EffluentChemicalOxygenDemand => {
                Some(TEXT_MILLIGRAMSPERLITER)
            }
            Self::SewageGasProduced => Some(TEXT_QUBICMETERS),
            Self::MethaneFraction => Some(TEXT_PERCENT),
            Self::GasSupply => Some(TEXT_KILOWATTHOURS),
            Self::PurchaseOfBiogas => None,
            Self::TotalPowerConsumption => Some(TEXT_KILOWATTHOURS),
            Self::OnSitePowerGeneration => Some(TEXT_KILOWATTHOURS),
            Self::EmissionFactorElectricityMix => Some(TEXT_GRAMSPERKILOWATTHOUR),
            Self::HeatingOil => Some(TEXT_TONS),
            Self::SideStreamTreatmentTotalNitrogen => Some(TEXT_TONS),
            Self::SludgeTreatmentBagsAreOpen => None,
            Self::SludgeTreatmentStorageContainersAreOpen => None,
            Self::SludgeTreatmentDisposal => Some(TEXT_TONS),
            Self::SludgeTreatmentTransportDistance => Some(TEXT_KILOMETERS),
            Self::SludgeTreatmentDigesterCount => None,
            Self::OperatingMaterialFeCl3
            | Self::OperatingMaterialFeClSO4
            | Self::OperatingMaterialCaOH2
            | Self::OperatingMaterialSyntheticPolymers => Some(TEXT_TONS),
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
