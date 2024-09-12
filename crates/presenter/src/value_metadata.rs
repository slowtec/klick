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

fn metadata() -> [(Id, FieldMetaData); 48] {
    use FieldMetaData as M;
    use Placeholder as P;
    [
        // ------    ------ //
        //      Project     //
        // ------    ------ //
        (
            Id::ProjectName,
            M {
                placeholder: P::text("project-name"),
                description: "description-project-name",
            },
        ),
        // ------    ------ //
        //      Profile     //
        // ------    ------ //
        (
            Id::ProfilePlantName,
            M {
                placeholder: P::Label,
                description: "description-plant-name",
            },
        ),
        (
            Id::ProfilePopulationEquivalent,
            M {
                placeholder: P::text("connected-inhabitants"),
                description: "description-connected-inhabitants",
            },
        ),
        (
            Id::ProfileWastewater,
            M {
                placeholder: P::text("wastewater-quantity"),
                description: "description-wastewater-quantity",
            },
        ),
        (
            Id::ProfileInfluentChemicalOxygenDemand,
            M {
                placeholder: P::text("cod"),
                description: "description-cod",
            },
        ),
        (
            Id::ProfileInfluentNitrogen,
            M {
                placeholder: P::text("total-nitrogen"),
                description: "description-total-nitrogen",
            },
        ),
        (
            Id::ProfileInfluentTotalOrganicCarbohydrates,
            M {
                placeholder: P::text("toc"),
                description: "description-toc",
            },
        ),
        (
            Id::ProfileEffluentNitrogen,
            M {
                placeholder: P::text("total-nitrogen"),
                description: "description-total-nitrogen-effluent",
            },
        ),
        (
            Id::ProfileEffluentChemicalOxygenDemand,
            M {
                placeholder: P::text("cod"),
                description: "description-cod-effluent",
            },
        ),
        (
            Id::ProfileTotalPowerConsumption,
            M {
                placeholder: P::text("total-power-consumption"),
                description: "description-total-power-consumption",
            },
        ),
        (
            Id::ProfileOnSitePowerGeneration,
            M {
                placeholder: P::text("self-generated-power"),
                description: "description-self-generated-power",
            },
        ),
        (
            Id::ProfileEmissionFactorElectricityMix,
            M {
                placeholder: P::default_value(),
                description: "description-emission-factor-electricity-mix",
            },
        ),
        (
            Id::ProfileGasSupply,
            M {
                placeholder: P::text("gas-supply"),
                description: "description-gas-supply",
            },
        ),
        (
            Id::ProfilePurchaseOfBiogas,
            M {
                placeholder: P::text("biogas-purchase"),
                description: "description-biogas-purchase",
            },
        ),
        (
            Id::ProfileHeatingOil,
            M {
                placeholder: P::text("heating-oil-purchase"),
                description: "description-heating-oil-purchase",
            },
        ),
        (
            Id::ProfileSewageGasProduced,
            M {
                placeholder: P::text("sewage-gas-produced"),
                description: "description-sewage-gas-produced",
            },
        ),
        (
            Id::ProfileMethaneFraction,
            M {
                placeholder: P::default_value(),
                description: "description-methane-percentage",
            },
        ),
        (
            Id::ProfileSludgeDigesterCount,
            M {
                placeholder: P::text("number-of-digesters"),
                description: "description-number-of-digesters",
            },
        ),
        (
            Id::ProfileSludgeBagsAreOpen,
            M {
                placeholder: P::none(),
                description: "description-sludge-treatment-bags-are-open",
            },
        ),
        (
            Id::ProfileSludgeStorageContainersAreOpen,
            M {
                placeholder: P::none(),
                description: "description-sludge-treatment-storage-containers-are-open",
            },
        ),
        (
            Id::ProfileSludgeDisposal,
            M {
                placeholder: P::text("mass-dewatered"),
                description: "description-mass-dewatered",
            },
        ),
        (
            Id::ProfileSludgeTransportDistance,
            M {
                placeholder: P::text("distance"),
                description: "description-distance",
            },
        ),
        (
            Id::ProfileSideStreamTotalNitrogen,
            M {
                placeholder: P::text("total-nitrogen"),
                description: "description-total-nitrogen-side-stream",
            },
        ),
        (
            Id::ProfileOperatingMaterialFeCl3,
            M {
                placeholder: P::text("ferrous-chloride"),
                description: "description-ferrous-chloride",
            },
        ),
        (
            Id::ProfileOperatingMaterialFeClSO4,
            M {
                placeholder: P::text("ferrous-chloride-sulfate"),
                description: "description-ferrous-chloride-sulfate",
            },
        ),
        (
            Id::ProfileOperatingMaterialCaOH2,
            M {
                placeholder: P::text("calcium-hydroxide"),
                description: "description-calcium-hydroxide",
            },
        ),
        (
            Id::ProfileOperatingMaterialSyntheticPolymers,
            M {
                placeholder: P::text("synthetic-polymers"),
                description: "description-synthetic-polymers",
            },
        ),
        // ------    ------ //
        //    Sensitivity   //
        // ------    ------ //
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
            Id::SensitivityN2OCalculationMethod,
            M {
                placeholder: P::none(),
                description: "description-sensitivity-N2O-calculation-method",
            },
        ),
        (
            Id::SensitivityCH4ChpCalculationMethod,
            M {
                placeholder: P::none(),
                description: "description-sensitivity-ch4-chp-calculation-method",
            },
        ),
        // ------    ------ //
        //  Recommendation  //
        // ------    ------ //
        (
            Id::RecommendationDistrictHeating,
            M {
                placeholder: P::text("district-heating"),
                description: "description-district-heating",
            },
        ),
        (
            Id::RecommendationEstimatedSelfWaterEnergyUsage,
            M {
                placeholder: P::default_value(),
                description: "description-scenario-estimated-self-water-energy-usage",
            },
        ),
        (
            Id::RecommendationWaterEnergyExpansion,
            M {
                placeholder: P::text("self-use-water-energy"),
                description: "description-water-energy-expansion",
            },
        ),
        (
            Id::RecommendationEstimatedSelfWindEnergyUsage,
            M {
                placeholder: P::default_value(),
                description: "description-scenario-estimated-self-wind-energy-usage",
            },
        ),
        (
            Id::RecommendationWindEnergyExpansion,
            M {
                placeholder: P::text("self-use-wind-energy"),
                description: "description-wind-energy-expansion",
            },
        ),
        (
            Id::RecommendationEstimatedSelfPhotovolaticUsage,
            M {
                placeholder: P::default_value(),
                description: "description-scenario-estimated-self-photovolatic-usage",
            },
        ),
        (
            Id::RecommendationPhotovoltaicEnergyExpansion,
            M {
                placeholder: P::text("self-use-pv-energy"),
                description: "description-photovoltaic-energy-expansion",
            },
        ),
        (
            Id::RecommendationFossilEnergySaving,
            M {
                placeholder: P::text("fossil-energy-saving"),
                description: "description-fossil-energy-saving",
            },
        ),
        (
            Id::RecommendationProcessEnergySaving,
            M {
                placeholder: P::text("process-energy-saving"),
                description: "description-process-energy-saving",
            },
        ),
        (
            Id::SensitivityAdditionalCustomEmissions,
            M {
                placeholder: P::none(),
                description: "description-additional-custom-emissions",
            },
        ),
        (
            Id::RecommendationSludgeBagsAreOpen,
            M {
                placeholder: P::none(),
                description: "description-recommendation-sludge-treatment-bags-are-open",
            },
        ),
        (
            Id::RecommendationSludgeStorageContainersAreOpen,
            M {
                placeholder: P::none(),
                description:
                    "description-recommendation-sludge-treatment-storage-containers-are-open",
            },
        ),
        (
            Id::RecommendationN2OSideStreamCoverIsOpen,
            M {
                placeholder: P::none(),
                description:
                    "description-recommendation-recommendation-n2o-side-stream-cover-is-open",
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
    use colored::*;

    for lang in [Lng::De.id(), Lng::En.id()] {
        for (
            id,
            FieldMetaData {
                placeholder,
                description,
            },
        ) in metadata()
        {
            let desc_txt = LOCALES.lookup(&lang, description);
            assert!(!desc_txt.is_empty());
            assert_ne!(desc_txt, description);
            assert!(
                !desc_txt.contains("Unknown localization"),
                "No description {lang} translation found for {id} (key = {key})",
                id = format!("{id:?}").yellow(),
                lang = lang.to_string().to_uppercase().yellow().bold(),
                key = description.yellow().bold()
            );
            match placeholder {
                Placeholder::Text(key) => {
                    let placeholder_txt = LOCALES.lookup(&lang, key);
                    assert!(!placeholder_txt.is_empty());
                    assert!(
                        !placeholder_txt.contains("Unknown localization"),
                        "No placeholder {lang} translation found for {id} (key = {key})",
                        id = format!("{id:?}").yellow(),
                        lang = lang.to_string().to_uppercase().yellow().bold(),
                        key = key.yellow().bold()
                    );
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

#[test]
fn all_input_value_id_meta_data_are_defined() {
    use colored::*;
    use strum::IntoEnumIterator;

    let d = metadata();

    for id in Id::iter() {
        if !d.iter().any(|(field_id, _)| *field_id == id) {
            let variant = format!("{id:?}").yellow().bold();
            panic!("Metadata of {variant} was not defined");
        }
    }
    assert_eq!(Id::iter().count(), metadata().len());
}
