use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, ScenarioFieldId, SensitivityParameterId, SewageSludgeTreatmentId,
    SideStreamTreatmentId,
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

impl ValueUnit for ProfileValueId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::PlantName => None,
            Self::PopulationEquivalent => None,
            Self::Wastewater => Some(LATEX_QUBICMETERS),
            Self::InfluentAverage(id) => id.unit_as_latex(),
            Self::EffluentAverage(id) => id.unit_as_latex(),
            Self::EnergyConsumption(id) => id.unit_as_latex(),
            Self::SewageSludgeTreatment(id) => id.unit_as_latex(),
            Self::SideStreamTreatment(_id) => None, // FIXME id.unit_as_latex(),
            Self::OperatingMaterials(id) => id.unit_as_latex(),
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::PlantName => None,
            Self::PopulationEquivalent => None,
            Self::Wastewater => Some(TEXT_QUBICMETERS),
            Self::InfluentAverage(id) => id.unit_as_text(),
            Self::EffluentAverage(id) => id.unit_as_text(),
            Self::EnergyConsumption(id) => id.unit_as_text(),
            Self::SewageSludgeTreatment(id) => id.unit_as_text(),
            Self::SideStreamTreatment(_id) => None, // FIXME id.unit_as_latex(),
            Self::OperatingMaterials(id) => id.unit_as_text(),
        }
    }
}

impl ValueUnit for AnnualAverageInfluentId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::Nitrogen | Self::ChemicalOxygenDemand => Some(LATEX_MILLIGRAMSPERLITER),
            Self::TotalOrganicCarbohydrates => Some(LATEX_MILLIGRAMSPERLITER),
        }
    }
    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::Nitrogen | Self::ChemicalOxygenDemand => Some(TEXT_MILLIGRAMSPERLITER),
            Self::TotalOrganicCarbohydrates => Some(TEXT_MILLIGRAMSPERLITER),
        }
    }
}

impl ValueUnit for AnnualAverageEffluentId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::Nitrogen | Self::ChemicalOxygenDemand => Some(LATEX_MILLIGRAMSPERLITER),
        }
    }
    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::Nitrogen | Self::ChemicalOxygenDemand => Some(TEXT_MILLIGRAMSPERLITER),
        }
    }
}

impl ValueUnit for EnergyConsumptionId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::SewageGasProduced => Some(LATEX_QUBICMETERS),
            Self::MethaneFraction => Some(LATEX_PERCENT),
            Self::GasSupply => Some(LATEX_KILOWATTHOURS),
            Self::PurchaseOfBiogas => None,
            Self::TotalPowerConsumption => Some(LATEX_KILOWATTHOURS),
            Self::OnSitePowerGeneration => Some(LATEX_KILOWATTHOURS),
            Self::EmissionFactorElectricityMix => Some(LATEX_GRAMSPERKILOWATTHOUR),
            Self::HeatingOil => Some(LATEX_TONS),
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::SewageGasProduced => Some(TEXT_QUBICMETERS),
            Self::MethaneFraction => Some(TEXT_PERCENT),
            Self::GasSupply => Some(TEXT_KILOWATTHOURS),
            Self::PurchaseOfBiogas => None,
            Self::TotalPowerConsumption => Some(TEXT_KILOWATTHOURS),
            Self::OnSitePowerGeneration => Some(TEXT_KILOWATTHOURS),
            Self::EmissionFactorElectricityMix => Some(TEXT_GRAMSPERKILOWATTHOUR),
            Self::HeatingOil => Some(TEXT_TONS),
        }
    }
}

impl ValueUnit for SewageSludgeTreatmentId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::SludgeBags => None, // FIXME implement latex representation
            Self::SludgeStorageContainers => None, // FIXME implement latex representation
            Self::SewageSludgeForDisposal => Some(LATEX_TONS),
            Self::TransportDistance => Some(LATEX_KILOMETERS),
            Self::DigesterCount => None,
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::SludgeBags => None,
            Self::SludgeStorageContainers => None,
            Self::SewageSludgeForDisposal => Some(TEXT_TONS),
            Self::TransportDistance => Some(TEXT_KILOMETERS),
            Self::DigesterCount => None,
        }
    }
}
impl ValueUnit for OperatingMaterialId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::FeCl3 | Self::FeClSO4 | Self::CaOH2 | Self::SyntheticPolymers => Some(LATEX_TONS),
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::FeCl3 | Self::FeClSO4 | Self::CaOH2 | Self::SyntheticPolymers => Some(TEXT_TONS),
        }
    }
}

impl ValueUnit for SideStreamTreatmentId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            Self::TotalNitrogen => Some(LATEX_TONS),
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::TotalNitrogen => Some(TEXT_TONS),
        }
    }
}

impl ValueUnit for ScenarioFieldId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            // FIXME
            _ => None,
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            _ => None,
        }
    }
}

impl ValueUnit for SensitivityParameterId {
    fn unit_as_latex(&self) -> Option<&'static str> {
        match self {
            // FIXME
            _ => None,
        }
    }

    fn unit_as_text(&self) -> Option<&'static str> {
        match self {
            Self::N2OCalculationMethod | Self::CH4ChpCalculationMethod => None,
            Self::N2OCustomFactor
            | Self::N2OSideStreamFactor
            | Self::CH4ChpCustomFactor
            | Self::CO2FossilCustomFactor
            | Self::SludgeStorageCustomFactor => Some(TEXT_PERCENT),
            Self::SludgeBagsCustomFactor => Some(TEXT_QUBICMETERS_PER_HOUR),
        }
    }
}
