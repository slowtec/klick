use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, SewageSludgeTreatmentId,
};

pub trait ValueLabel {
    fn label(&self) -> &str;
}

impl ValueLabel for ProfileValueId {
    fn label(&self) -> &str {
        match self {
            Self::PlantName => "Name oder Ort",
            Self::PopulationEquivalent => "Ausbaugröße",
            Self::Wastewater => "Abwassermenge",
            Self::InfluentAverage(id) => id.label(),
            Self::EffluentAverage(id) => id.label(),
            Self::EnergyConsumption(id) => id.label(),
            Self::SewageSludgeTreatment(id) => id.label(),
            Self::OperatingMaterials(id) => id.label(),
        }
    }
}

impl ValueLabel for AnnualAverageInfluentId {
    fn label(&self) -> &str {
        match self {
            Self::Nitrogen => "Gesamtstickstoff",
            Self::Phosphorus => "Phosphor",
            Self::ChemicalOxygenDemand => "Chemischer Sauerstoffbedarf",
        }
    }
}

impl ValueLabel for AnnualAverageEffluentId {
    fn label(&self) -> &str {
        match self {
            Self::Nitrogen => "Gesamtstickstoff",
            Self::Phosphorus => "Phosphor",
            Self::ChemicalOxygenDemand => "Chemischer Sauerstoffbedarf",
        }
    }
}

impl ValueLabel for EnergyConsumptionId {
    fn label(&self) -> &str {
        match self {
            Self::SewageGasProduced => "Erzeugtes Klärgas",
            Self::MethaneFraction => "Methangehalt",
            Self::GasSupply => "Gasbezug (Versorger)",
            Self::PurchaseOfBiogas => "Bezug von Biogas",
            Self::TotalPowerConsumption => "Strombedarf gesamt",
            Self::OnSitePowerGeneration => "Eigenstromerzeugung",
            Self::EmissionFactorElectricityMix => "Emissionsfaktor Strommix (Versorger)",
        }
    }
}
impl ValueLabel for SewageSludgeTreatmentId {
    fn label(&self) -> &str {
        match self {
            Self::SewageSludgeForDisposal => "Klärschlamm zur Entsorgung",
            Self::TransportDistance => "Transportdistanz",
            Self::DigesterCount => "Anzahl Faultürme",
        }
    }
}
impl ValueLabel for OperatingMaterialId {
    fn label(&self) -> &str {
        match self {
            Self::FeCl3 => "Eisen(III)-chlorid-Lösung",
            Self::FeClSO4 => "Eisenchloridsulfat-Lösung",
            Self::CaOH2 => "Kalkhydrat",
            Self::SyntheticPolymers => "Synthetische Polymere",
        }
    }
}
