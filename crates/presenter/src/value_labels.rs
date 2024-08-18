use klick_domain as domain;

use crate::{InputValueId, OutputValueId};

// TODO: move to value_metadata

pub trait ValueLabel {
    fn label(&self) -> &'static str;
    fn label_latex(&self) -> &'static str {
        self.label()
    }
}

impl ValueLabel for InputValueId {
    fn label(&self) -> &'static str {
        match self {
            Self::ProjectName => "Projektname",
            Self::PlantName => "Name oder Ort",
            Self::PopulationEquivalent => "Angeschlossene Einwohner",
            Self::Wastewater => "Abwassermenge",
            Self::InfluentNitrogen => "Gesamtstickstoff",
            Self::InfluentChemicalOxygenDemand => "Chemischer Sauerstoffbedarf",
            Self::InfluentTotalOrganicCarbohydrates => "Gesamter organischer Kohlenstoff",
            Self::EffluentNitrogen => "Gesamtstickstoff",
            Self::EffluentChemicalOxygenDemand => "Chemischer Sauerstoffbedarf",
            Self::SewageGasProduced => "Erzeugtes Klärgas",
            Self::MethaneFraction => "Methangehalt",
            Self::GasSupply => "Gasbezug (Versorger)",
            Self::PurchaseOfBiogas => "Bezug von Biogas",
            Self::TotalPowerConsumption => "Strombedarf gesamt",
            Self::OnSitePowerGeneration => "Eigenstromerzeugung",
            Self::EmissionFactorElectricityMix => "Strommix-EF (Versorger)",
            Self::HeatingOil => "Heizölbezug",
            Self::SideStreamTreatmentTotalNitrogen => "Gesamtstickstoff",
            Self::SludgeTreatmentDisposal => "Klärschlamm zur Entsorgung",
            Self::SludgeTreatmentTransportDistance => "Transportdistanz",
            Self::SludgeTreatmentDigesterCount => "Anzahl Faultürme",
            Self::SludgeTreatmentBagsAreOpen | Self::ScenarioSludgeBagsAreOpen => {
                "Schlammtaschen sind offen"
            }
            Self::SludgeTreatmentStorageContainersAreOpen
            | Self::ScenarioSludgeStorageContainersAreOpen => "Schlammlagerung ist offen",
            Self::OperatingMaterialFeCl3 => "Eisen(III)-chlorid-Lösung",
            Self::OperatingMaterialFeClSO4 => "Eisenchloridsulfat-Lösung",
            Self::OperatingMaterialCaOH2 => "Kalkhydrat",
            Self::OperatingMaterialSyntheticPolymers => "Synthetische Polymere",
            Self::ScenarioN2OSideStreamFactor => "N₂O-EF Prozesswasser",
            Self::ScenarioN2OSideStreamCoverIsOpen => {
                "Abdeckung mit Abluftbehandlung Prozesswasserbehandlungsanlage"
            }
            Self::ScenarioProcessEnergySaving => "Energieeinsparung bei Prozessen",
            Self::ScenarioFossilEnergySaving => "Energieeinsparung bei fossilen Energiequellen",
            Self::ScenarioDistrictHeating => "Abgabe Fern-/Nahwärme (an Dritte)",
            Self::ScenarioPhotovoltaicEnergyExpansion => "Zubau PV",
            Self::ScenarioEstimatedSelfPhotovolaticUsage => "Geschätzte Eigennutzung",
            Self::ScenarioWindEnergyExpansion => "Zubau Wind",
            Self::ScenarioEstimatedSelfWindEnergyUsage => "Geschätzte Eigennutzung",
            Self::ScenarioWaterEnergyExpansion => "Zubau Wasserkraft",
            Self::ScenarioEstimatedSelfWaterEnergyUsage => "Geschätzte Eigennutzung",
            Self::SensitivityN2OCalculationMethod => "N₂O Berechnungsmethode",
            Self::SensitivityN2OCustomFactor => "N₂O-EF Benutzerdefiniert",
            Self::SensitivityN2OSideStreamFactor => "N₂O-EF Prozesswasser",
            Self::SensitivityCH4ChpCalculationMethod => "BHKW Berechnungsmethode",
            Self::SensitivityCH4ChpCustomFactor => "BHKW CH₄-EF benutzerdefiniert",
            Self::SensitivityCO2FossilCustomFactor => "CO₂-EF (fossil)",
            Self::SensitivitySludgeBagsCustomFactor => "CH₄-EF Schlammtaschen",
            Self::SensitivitySludgeStorageCustomFactor => "CH₄-EF Schlammlagerung",
        }
    }
    fn label_latex(&self) -> &'static str {
        match self {
            Self::ScenarioN2OSideStreamFactor => "$N_2O$-EF Prozesswasser",
            Self::SensitivityN2OCalculationMethod => "$N_2O$ Berechnungsmethode",
            Self::SensitivityN2OCustomFactor => "$N_2O$-EF Benutzerdefiniert",
            Self::SensitivityN2OSideStreamFactor => "$N_2O$-EF Prozesswasser",
            Self::SensitivityCH4ChpCalculationMethod => self.label(),
            Self::SensitivityCH4ChpCustomFactor => "BHKW $CH_4$-EF benutzerdefiniert",
            Self::SensitivityCO2FossilCustomFactor => "$CO_2$-EF (fossil)",
            Self::SensitivitySludgeBagsCustomFactor => "$CH_4$-EF Schlammtaschen",
            Self::SensitivitySludgeStorageCustomFactor => "$CH_4$-EF Schlammlagerung",
            _ => self.label(),
        }
    }
}

impl ValueLabel for domain::units::N2oEmissionFactorCalcMethod {
    fn label(&self) -> &'static str {
        match self {
            Self::TuWien2016 => "TU Wien 2016",
            Self::Optimistic => "Optimistisch",
            Self::Pesimistic => "Pessimistisch",
            Self::Ipcc2019 => "IPCC 2019",
            Self::Custom => "Benutzerdefiniert",
        }
    }
}

impl ValueLabel for domain::units::Ch4ChpEmissionFactorCalcMethod {
    fn label(&self) -> &'static str {
        match self {
            Self::MicroGasTurbines => "Mikrograsturbinen",
            Self::GasolineEngine => "Ottomotor",
            Self::JetEngine => "Zündstrahlmotor",
            Self::Custom => "Benutzerdefiniert",
        }
    }
}

impl ValueLabel for OutputValueId {
    fn label(&self) -> &'static str {
        match self {
            Self::N2oPlant => "N₂O Anlage",
            Self::N2oWater => "N₂O Gewässer",
            Self::N2oSideStream => "N₂O Prozesswasserbehandlung",
            Self::N2oEmissions => "Lachgasemissionen",
            Self::Ch4Plant => "CH₄ Anlage",
            Self::Ch4SludgeStorageContainers => "CH₄ Schlamm Lagerung",
            Self::Ch4SludgeBags => "CH₄ Schlammtasche",
            Self::Ch4Water => "CH₄ Gewässer",
            Self::Ch4CombinedHeatAndPowerPlant => "CH₄ BHKW",
            Self::Ch4Emissions => "Methanemissionen",
            Self::FossilEmissions => "Fossile CO₂-Emissionen",
            Self::Fecl3 => "Eisen(III)-chlorid-Lösung",
            Self::Feclso4 => "Eisenchloridsulfat-Lösung",
            Self::Caoh2 => "Kalkhydrat",
            Self::SyntheticPolymers => "Synthetische Polymere",
            Self::ElectricityMix => "Strommix",
            Self::OilEmissions => "Heizöl",
            Self::GasEmissions => "Gas",
            Self::OperatingMaterials => "Betriebsstoffe",
            Self::SewageSludgeTransport => "Klärschlamm Transport",
            Self::TotalEmissions => "Gesamtemissionen",
            Self::DirectEmissions => "Direkte Emissionen",
            Self::IndirectEmissions => "Indirekte Emissionen",
            Self::OtherIndirectEmissions => "Weitere Indirekte Emissionen",
            _ => todo!(),
        }
    }
}
