use fluent_templates::Loader;

use klick_domain::InputValueId as Id;

use crate::{Lng, LOCALES};

#[must_use]
pub fn metadata_of(id: &Id) -> FieldMetaData {
    metadata()
        .into_iter()
        .find(|(x, _)| x == id)
        .map(|(_, m)| m)
        .unwrap()
}

#[derive(Debug)]
pub enum Placeholder {
    /// Use the label of the value
    Label,

    /// Custom text
    Text(&'static str),

    /// If there is a default value, render it as string
    DefaultValue,

    // Nothing
    None,
}

impl Placeholder {
    #[must_use]
    pub const fn label() -> Self {
        Self::Label
    }

    #[must_use]
    pub const fn text(txt: &'static str) -> Self {
        Self::Text(txt)
    }

    #[must_use]
    pub const fn default_value() -> Self {
        Self::DefaultValue
    }

    #[must_use]
    pub const fn none() -> Self {
        Self::None
    }
}

fn metadata() -> [(Id, FieldMetaData); 43] {
    use FieldMetaData as M;
    use Placeholder as P;
    [
        (
            Id::ProjectName,
            M {
                placeholder: P::text("project-name"),
                description: "description-project-name",
            },
        ),
        (
            Id::PlantName,
            M {
                placeholder: P::Label,
                description: "description-plant-name",
            },
        ),
        (
            Id::PopulationEquivalent,
            M {
                placeholder: P::text("connected-inhabitants"),
                description: "description-connected-inhabitants",
            },
        ),
        (
            Id::Wastewater,
            M {
                placeholder: P::text("wastewater-quantity"),
                description: "description-wastewater-quantity",
            },
        ),
        (
            Id::InfluentChemicalOxygenDemand,
            M {
                placeholder: P::text("cod"),
                description: "description-cod",
            },
        ),
        (
            Id::InfluentNitrogen,
            M {
                placeholder: P::text("total-nitrogen"),
                description: "description-total-nitrogen",
            },
        ),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            M {
                placeholder: P::text("toc"),
                description: "description-toc",
            },
        ),
        (
            Id::EffluentNitrogen,
            M {
                placeholder: P::text("total-nitrogen"),
                description: "description-total-nitrogen-effluent",
            },
        ),
        (
            Id::EffluentChemicalOxygenDemand,
            M {
                placeholder: P::text("cod"),
                description: "description-cod-effluent",
            },
        ),
        (
            Id::TotalPowerConsumption,
            M {
                placeholder: P::text("total-power-consumption"),
                description: "description-total-power-consumption",
            },
        ),
        (
            Id::OnSitePowerGeneration,
            M {
                placeholder: P::text("self-generated-power"),
                description: "description-self-generated-power",
            },
        ),
        (
            Id::EmissionFactorElectricityMix,
            M {
                placeholder: P::text("emission-factor-electricity-mix"),
                description: "description-emission-factor-electricity-mix",
            },
        ),
        (
            Id::GasSupply,
            M {
                placeholder: P::text("gas-supply"),
                description: "description-gas-supply",
            },
        ),
        (
            Id::PurchaseOfBiogas,
            M {
                placeholder: P::text("biogas-purchase"),
                description: "description-biogas-purchase",
            },
        ),
        (
            Id::HeatingOil,
            M {
                placeholder: P::text("heating-oil-purchase"),
                description: "description-heating-oil-purchase",
            },
        ),
        (
            Id::SewageGasProduced,
            M {
                placeholder: P::text("sewage-gas-produced"),
                description: "description-sewage-gas-produced",
            },
        ),
        (
            Id::MethaneFraction,
            M {
                placeholder: P::text("methane-percentage"),
                description: "description-methane-percentage",
            },
        ),
        (
            Id::SludgeTreatmentDigesterCount,
            M {
                placeholder: P::text("number-of-digesters"),
                description: "description-number-of-digesters",
            },
        ),
        (
            Id::SludgeTreatmentBagsAreOpen,
            M {
                placeholder: P::none(),
                description: "description-sludge-treatment-bags-are-open",
            },
        ),
        (
            Id::SludgeTreatmentStorageContainersAreOpen,
            M {
                placeholder: P::none(),
                description: "description-sludge-treatment-storage-containers-are-open",
            },
        ),
        (
            Id::SludgeTreatmentDisposal,
            M {
                placeholder: P::text("mass-dewatered"),
                description: "description-mass-dewatered",
            },
        ),
        (
            Id::SludgeTreatmentTransportDistance,
            M {
                placeholder: P::text("distance"),
                description: "description-distance",
            },
        ),
        (
            Id::SideStreamTreatmentTotalNitrogen,
            M {
                placeholder: P::text("total-nitrogen"),
                description: "description-total-nitrogen-side-stream",
            },
        ),
        (
            Id::OperatingMaterialFeCl3,
            M {
                placeholder: P::text("ferrous-chloride"),
                description: "description-ferrous-chloride",
            },
        ),
        (
            Id::OperatingMaterialFeClSO4,
            M {
                placeholder: P::text("ferrous-chloride-sulfate"),
                description: "description-ferrous-chloride-sulfate",
            },
        ),
        (
            Id::OperatingMaterialCaOH2,
            M {
                placeholder: P::text("calcium-hydroxide"),
                description: "description-calcium-hydroxide",
            },
        ),
        (
            Id::OperatingMaterialSyntheticPolymers,
            M {
                placeholder: P::text("synthetic-polymers"),
                description: "description-synthetic-polymers",
            },
        ),
        (
            Id::SensitivitySludgeBagsCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "description-sensitivity-sludge-bags-custom-factor",
            },
        ),
        (
            Id::SensitivitySludgeStorageCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "description-sensitivity-sludge-storage-custom-factor",
            },
        ),
        (
            Id::SensitivityCH4ChpCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "description-sensitivity-CH4-chp-custom-factor",
            },
        ),
        (
            Id::SensitivityCO2FossilCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "description-sensitivity-CO2-fossil-custom-factor",
            },
        ),
        (
            Id::SensitivityN2OCustomFactor,
            M {
                placeholder: P::default_value(),
                description: "description-sensitivity-N2O-custom-factor",
            },
        ),
        (
            Id::SensitivityN2OSideStreamFactor,
            M {
                placeholder: P::default_value(),
                description: "description-sensitivity-N2O-side-stream-factor",
            },
        ),
        (
            Id::ScenarioDistrictHeating,
            M {
                placeholder: P::text("district-heating"),
                description: "description-district-heating",
            },
        ),
        (
            Id::ScenarioEstimatedSelfWaterEnergyUsage,
            M {
                placeholder: P::default_value(),
                description: "description-scenario-estimated-self-water-energy-usage",
            },
        ),
        (
            Id::ScenarioWaterEnergyExpansion,
            M {
                placeholder: P::text("self-use-water-energy"),
                description: "description-water-energy-expansion",
            },
        ),
        (
            Id::ScenarioEstimatedSelfWindEnergyUsage,
            M {
                placeholder: P::default_value(),
                description: "description-scenario-estimated-self-wind-energy-usage",
            },
        ),
        (
            Id::ScenarioWindEnergyExpansion,
            M {
                placeholder: P::text("self-use-wind-energy"),
                description: "description-wind-energy-expansion",
            },
        ),
        (
            Id::ScenarioEstimatedSelfPhotovolaticUsage,
            M {
                placeholder: P::default_value(),
                description: "description-scenario-estimated-self-photovolatic-usage",
            },
        ),
        (
            Id::ScenarioPhotovoltaicEnergyExpansion,
            M {
                placeholder: P::text("self-use-pv-energy"),
                description: "description-photovoltaic-energy-expansion",
            },
        ),
        (
            Id::ScenarioFossilEnergySaving,
            M {
                placeholder: P::text("fossil-energy-saving"),
                description: "description-fossil-energy-saving",
            },
        ),
        (
            Id::ScenarioProcessEnergySaving,
            M {
                placeholder: P::text("process-energy-saving"),
                description: "description-process-energy-saving",
            },
        ),
        (
            Id::AdditionalCustomEmissions,
            M {
                placeholder: P::none(),
                description: "description-additional-custom-emissions",
            },
        ),
    ]
}

pub struct FieldMetaData {
    pub description: &'static str,
    pub placeholder: Placeholder,
}

impl FieldMetaData {
    #[must_use]
    pub fn lookup(lng: Lng, key: &'static str) -> String {
        LOCALES.lookup(&lng.id(), key)
    }
}

#[test]
fn validate_fluent_file() {
    use super::*;

    for lang in [Lng::De.id(), Lng::En.id()] {
        for (
            _,
            FieldMetaData {
                placeholder,
                description,
            },
        ) in metadata()
        {
            assert!(!LOCALES.lookup(&lang, description).is_empty());
            match placeholder {
                Placeholder::Text(key) => {
                    assert!(!LOCALES.lookup(&lang, key).is_empty());
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
