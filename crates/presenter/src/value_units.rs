use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, SewageSludgeTreatmentId,
};

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
    fn unit_as_latex(&self) -> Option<&str> {
        None
    }
}

const LATEX_PERCENT: &str = "\\%";
const LATEX_QUBICMETERS: &str = r#"\si{\metre}^3"#;
const LATEX_KILOMETERS: &str = r#"\si{\kilo\metre}"#;
const LATEX_TONS: &str = r#"\si{\tonne}"#;
const LATEX_MILLIGRAMSPERLITER: &str = r#"\si{\milli\gram\per\liter}"#;
const LATEX_KILOWATTHOURS: &str = r#"\si{\kilo\watt\hour}"#;
const LATEX_GRAMSPERKILOWATTHOUR: &str = r#"\si{\gram\per\kilo\watt\hour}"#;

impl ValueUnit for ProfileValueId {
    fn unit_as_latex(&self) -> Option<&str> {
        match self {
            Self::PlantName => None,
            Self::PopulationEquivalent => None,
            Self::Wastewater => Some(LATEX_QUBICMETERS),
            Self::InfluentAverage(id) => id.unit_as_latex(),
            Self::EffluentAverage(id) => id.unit_as_latex(),
            Self::EnergyConsumption(id) => id.unit_as_latex(),
            Self::SewageSludgeTreatment(id) => id.unit_as_latex(),
            Self::OperatingMaterials(id) => id.unit_as_latex(),
        }
    }
}

impl ValueUnit for AnnualAverageInfluentId {
    fn unit_as_latex(&self) -> Option<&str> {
        match self {
            Self::Nitrogen | Self::Phosphorus | Self::ChemicalOxygenDemand => {
                Some(LATEX_MILLIGRAMSPERLITER)
            }
        }
    }
}

impl ValueUnit for AnnualAverageEffluentId {
    fn unit_as_latex(&self) -> Option<&str> {
        match self {
            Self::Nitrogen | Self::Phosphorus | Self::ChemicalOxygenDemand => {
                Some(LATEX_MILLIGRAMSPERLITER)
            }
        }
    }
}

impl ValueUnit for EnergyConsumptionId {
    fn unit_as_latex(&self) -> Option<&str> {
        match self {
            Self::SewageGasProduced => Some(LATEX_QUBICMETERS),
            Self::MethaneFraction => Some(LATEX_PERCENT),
            Self::GasSupply => Some(LATEX_KILOWATTHOURS),
            Self::PurchaseOfBiogas => None,
            Self::TotalPowerConsumption => Some(LATEX_KILOWATTHOURS),
            Self::OnSitePowerGeneration => Some(LATEX_KILOWATTHOURS),
            Self::EmissionFactorElectricityMix => Some(LATEX_GRAMSPERKILOWATTHOUR),
        }
    }
}

impl ValueUnit for SewageSludgeTreatmentId {
    fn unit_as_latex(&self) -> Option<&str> {
        match self {
            Self::SewageSludgeForDisposal => Some(LATEX_TONS),
            Self::TransportDistance => Some(LATEX_KILOMETERS),
            Self::DigesterCount => None,
        }
    }
}
impl ValueUnit for OperatingMaterialId {
    fn unit_as_latex(&self) -> Option<&str> {
        match self {
            Self::FeCl3 | Self::FeClSO4 | Self::CaOH2 | Self::SyntheticPolymers => Some(LATEX_TONS),
        }
    }
}
