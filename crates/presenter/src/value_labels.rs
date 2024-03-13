use klick_boundary as boundary;
use klick_domain as domain;

use crate::{
    AnnualAverageEffluentId, AnnualAverageInfluentId, EnergyConsumptionId, OperatingMaterialId,
    ProfileValueId, ScenarioFieldId, SewageSludgeTreatmentId, SideStreamTreatmentId,
};

pub trait ValueLabel {
    fn label(&self) -> &'static str;
}

impl ValueLabel for ProfileValueId {
    fn label(&self) -> &'static str {
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
    fn label(&self) -> &'static str {
        match self {
            Self::Nitrogen => "Gesamtstickstoff",
            Self::ChemicalOxygenDemand => "Chemischer Sauerstoffbedarf",
            Self::TotalOrganicCarbohydrates => "Gesamter organischer Kohlenstoff",
        }
    }
}

impl ValueLabel for AnnualAverageEffluentId {
    fn label(&self) -> &'static str {
        match self {
            Self::Nitrogen => "Gesamtstickstoff",
            Self::ChemicalOxygenDemand => "Chemischer Sauerstoffbedarf",
        }
    }
}

impl ValueLabel for EnergyConsumptionId {
    fn label(&self) -> &'static str {
        match self {
            Self::SewageGasProduced => "Erzeugtes Klärgas",
            Self::MethaneFraction => "Methangehalt",
            Self::GasSupply => "Gasbezug (Versorger)",
            Self::PurchaseOfBiogas => "Bezug von Biogas",
            Self::TotalPowerConsumption => "Strombedarf gesamt",
            Self::OnSitePowerGeneration => "Eigenstromerzeugung",
            Self::EmissionFactorElectricityMix => "Strommix-EF (Versorger)",
            Self::HeatingOil => "Heizölbezug",
        }
    }
}

impl ValueLabel for SideStreamTreatmentId {
    fn label(&self) -> &'static str {
        match self {
            Self::TotalNitrogen => "Gesamtstickstoff",
        }
    }
}

impl ValueLabel for SewageSludgeTreatmentId {
    fn label(&self) -> &'static str {
        match self {
            Self::SewageSludgeForDisposal => "Klärschlamm zur Entsorgung",
            Self::TransportDistance => "Transportdistanz",
            Self::DigesterCount => "Anzahl Faultürme",
            Self::SludgeBags => "Schlammtaschen sind geschlossen",
            Self::SludgeBagsRecommended => "Schließen der Schlammtaschen",
            Self::SludgeStorageContainers => "Schlammlagerung ist geschlossen",
            Self::SludgeStorageContainersRecommended => "Schließen der Schlammlagerung",
        }
    }
}

impl ValueLabel for OperatingMaterialId {
    fn label(&self) -> &'static str {
        match self {
            Self::FeCl3 => "Eisen(III)-chlorid-Lösung",
            Self::FeClSO4 => "Eisenchloridsulfat-Lösung",
            Self::CaOH2 => "Kalkhydrat",
            Self::SyntheticPolymers => "Synthetische Polymere",
        }
    }
}

impl ValueLabel for ScenarioFieldId {
    fn label(&self) -> &'static str {
        match self {
            Self::N2OCustomFactor => "N₂O-EF Benutzerdefiniert",
            Self::N2OSideStreamFactor => "N₂O-EF Nebenstrom",
            Self::N2OSideStreamCoverIsOpen => {
                "Abdeckung mit Abluftbehandlung Prozesswasserbehandlungsanlage"
            }
            Self::CH4ChpCalculationMethod => "BHKW Emmisionsfaktor",
            Self::CH4ChpCustomFactor => "BHKW CH₄-EF benutzerdefiniert",
            Self::CO2FossilCustomFactor => "CO₂-EF (fossil)",
            Self::SludgeBagsCustomFactor => "CH₄-EF Schlammtaschen",
            Self::SludgeStorageCustomFactor => "CH₄-EF Schlammlagerung",

            Self::ProcessEnergySaving => "Energieeinsparung bei Prozessen",
            Self::FossilEnergySaving => "Energieeinsparung bei fossilen Energiequellen",
            Self::DistrictHeating => "Abgabe Fern-/Nahwärme (an Dritte)",
            Self::PhotovoltaicEnergyExpansion => "Zubau PV",
            Self::EstimatedSelfPhotovolaticUsage => "Geschätzte Eigennutzung",
            Self::WindEnergyExpansion => "Zubau Wind",
            Self::EstimatedSelfWindEnergyUsage => "Geschätzte Eigennutzung",
            Self::WaterEnergyExpansion => "Zubau Wasserkraft",
            Self::EstimatedSelfWaterEnergyUsage => "Geschätzte Eigennutzung",
        }
    }
}

impl ValueLabel for domain::N2oEmissionFactorCalcMethod {
    fn label(&self) -> &'static str {
        match self {
            Self::TuWien2016 => "TU Wien 2016",
            Self::Optimistic => "Optimistisch",
            Self::Pesimistic => "Pessimistisch",
            Self::Ipcc2019 => "IPCC 2019",
            Self::Custom(_) => "Benutzerdefiniert",
        }
    }
}

impl ValueLabel for boundary::N2oEmissionFactorCalcMethod {
    fn label(&self) -> &'static str {
        match self {
            Self::TuWien2016 => "TU Wien 2016",
            Self::Optimistic => "Optimistisch",
            Self::Pesimistic => "Pessimistisch",
            Self::Ipcc2019 => "IPCC 2019",
            Self::CustomFactor => "Benutzerdefiniert",
        }
    }
}

impl ValueLabel for domain::CH4ChpEmissionFactorCalcMethod {
    fn label(&self) -> &'static str {
        match self {
            Self::MicroGasTurbines => "Mikrograsturbinen",
            Self::GasolineEngine => "Ottomotor",
            Self::JetEngine => "Zündstrahlmotor",
            Self::Custom(_) => "Benutzerdefiniert",
        }
    }
}

impl ValueLabel for boundary::CH4ChpEmissionFactorCalcMethod {
    fn label(&self) -> &'static str {
        match self {
            Self::MicroGasTurbines => "Mikrograsturbinen",
            Self::GasolineEngine => "Ottomotor",
            Self::JetEngine => "Zündstrahlmotor",
            Self::CustomFactor => "Benutzerdefiniert",
        }
    }
}
