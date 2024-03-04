use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, SewageSludgeTreatmentId, SideStreamTreatmentId,
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
            Self::SideStreamTreatment(id) => id.label(),
            Self::OperatingMaterials(id) => id.label(),
        }
    }
}

impl ValueLabel for AnnualAverageInfluentId {
    fn label(&self) -> &str {
        match self {
            Self::Nitrogen => "Gesamtstickstoff",
            Self::ChemicalOxygenDemand => "Chemischer Sauerstoffbedarf",
            Self::TotalOrganicCarbohydrates => "Gesamter Organischer Stickstoff",
        }
    }
}

impl ValueLabel for AnnualAverageEffluentId {
    fn label(&self) -> &str {
        match self {
            Self::Nitrogen => "Gesamtstickstoff",
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
            Self::HeatingOil => "Heizölbezug (Versorger)",
        }
    }
}

impl ValueLabel for SideStreamTreatmentId {
    fn label(&self) -> &str {
        match self {
            Self::TotalNitrogen => "Gesamtstickstoff",
        }
    }
}

impl ValueLabel for SewageSludgeTreatmentId {
    fn label(&self) -> &str {
        match self {
            Self::SewageSludgeForDisposal => "Klärschlamm zur Entsorgung",
            Self::TransportDistance => "Transportdistanz",
            Self::DigesterCount => "Anzahl Faultürme",
            Self::SludgeBags => "Schließen der Schlammtaschen",
            Self::SludgeBagsRecommended => "Schließen der Schlammtaschen",
            Self::SludgeStorageContainers => "Schließen der Schlammlagerung",
            Self::SludgeStorageContainersRecommended => "Schließen der Schlammlagerung",
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
