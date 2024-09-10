use std::collections::HashMap;

use anyhow::anyhow;
use derive_more::From;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use time::{serde::iso8601, OffsetDateTime};

use klick_domain::Value;

pub use crate::v8::{
    CH4ChpEmissionFactorCalcMethod, FormData, N2oEmissionFactorCalcMethod, ProjectId,
};

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, From)]
pub struct JsonFormData(pub(crate) HashMap<InputValueId, JsonValue>);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, From)]
#[serde(untagged)]
pub enum Project {
    Saved(SavedProject),
    Unsaved(UnsavedProject),
}

impl Project {
    #[must_use]
    pub const fn form_data(&self) -> &JsonFormData {
        match self {
            Self::Saved(SavedProject { form_data, .. })
            | Self::Unsaved(UnsavedProject { form_data }) => form_data,
        }
    }

    #[must_use]
    pub fn into_form_data(self) -> JsonFormData {
        match self {
            Self::Saved(SavedProject { form_data, .. })
            | Self::Unsaved(UnsavedProject { form_data }) => form_data,
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        UnsavedProject::default().into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct UnsavedProject {
    pub form_data: JsonFormData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SavedProject {
    pub id: ProjectId,
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        with = "iso8601::option"
    )]
    pub modified_at: Option<OffsetDateTime>,
    pub form_data: JsonFormData,
}

// Since the IDs in the domain layer may change in the future,
// we need a stable ID for serialization and deserialization at this point,
// which can always be resolved for v9.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum InputValueId {
    // --- Project ---//
    ProjectName,

    // --- Profile ---//
    ProfilePlantName,
    ProfilePopulationEquivalent,
    ProfileWastewater,
    ProfileInfluentNitrogen,
    ProfileInfluentChemicalOxygenDemand,
    ProfileInfluentTotalOrganicCarbohydrates,
    ProfileEffluentNitrogen,
    ProfileEffluentChemicalOxygenDemand,
    ProfileSewageGasProduced,
    ProfileMethaneFraction,
    ProfileGasSupply,
    ProfilePurchaseOfBiogas,
    ProfileTotalPowerConsumption,
    ProfileOnSitePowerGeneration,
    ProfileEmissionFactorElectricityMix,
    ProfileHeatingOil,
    ProfileSideStreamTreatmentTotalNitrogen,
    #[serde(rename = "profile-operating-material-fe-cl3")]
    ProfileOperatingMaterialFeCl3,
    #[serde(rename = "profile-operating-material-fe-cl-so4")]
    ProfileOperatingMaterialFeClSO4,
    #[serde(rename = "profile-operating-material-ca-oh2")]
    ProfileOperatingMaterialCaOH2,
    ProfileOperatingMaterialSyntheticPolymers,
    ProfileSludgeTreatmentBagsAreOpen,
    ProfileSludgeTreatmentStorageContainersAreOpen,
    ProfileSludgeTreatmentDisposal,
    ProfileSludgeTreatmentTransportDistance,
    ProfileSludgeTreatmentDigesterCount,

    // --- Sensitivity ---//
    #[serde(rename = "sensitivity-n2o-calculation-method")]
    SensitivityN2OCalculationMethod,
    #[serde(rename = "sensitivity-n2o-custom-factor")]
    SensitivityN2OCustomFactor,
    #[serde(rename = "sensitivity-n2o-side-stream-factor")]
    SensitivityN2OSideStreamFactor,
    #[serde(rename = "sensitivity-ch4-chp-calculation-method")]
    SensitivityCH4ChpCalculationMethod,
    #[serde(rename = "sensitivity-ch4-chp-custom-factor")]
    SensitivityCH4ChpCustomFactor,
    #[serde(rename = "sensitivity-co2-fossil-custom-factor")]
    SensitivityCO2FossilCustomFactor,
    SensitivitySludgeBagsCustomFactor,
    SensitivitySludgeStorageCustomFactor,
    SensitivityAdditionalCustomEmissions,

    // --- Recommendation ---//
    RecommendationSludgeBagsAreOpen,
    RecommendationSludgeStorageContainersAreOpen,
    #[serde(rename = "recommendation-n2o-side-stream-factor")]
    RecommendationN2OSideStreamFactor,
    #[serde(rename = "recommendation-n2o-side-stream-cover-is-open")]
    RecommendationN2OSideStreamCoverIsOpen,
    RecommendationProcessEnergySaving,
    RecommendationFossilEnergySaving,
    RecommendationDistrictHeating,
    RecommendationPhotovoltaicEnergyExpansion,
    RecommendationEstimatedSelfPhotovolaticUsage,
    RecommendationWindEnergyExpansion,
    RecommendationEstimatedSelfWindEnergyUsage,
    RecommendationWaterEnergyExpansion,
    RecommendationEstimatedSelfWaterEnergyUsage,
}

impl InputValueId {
    /// Convert JSON values to domain values
    // Since the data types in the domain layer may change in the future,
    // we need a clear assignment for v9 at this point.
    pub fn value_from_json(&self, v: JsonValue) -> anyhow::Result<Value> {
        let v = match self {
            Self::ProjectName | Self::ProfilePlantName => {
                let v = v
                    .as_str()
                    .ok_or_else(|| anyhow!("Expected text value for {self:?}, got {v:?}"))?;
                Value::text(v)
            }

            // Boolean values
            Self::RecommendationSludgeStorageContainersAreOpen
            | Self::ProfilePurchaseOfBiogas
            | Self::ProfileSludgeTreatmentBagsAreOpen
            | Self::RecommendationSludgeBagsAreOpen
            | Self::RecommendationN2OSideStreamCoverIsOpen
            | Self::ProfileSludgeTreatmentStorageContainersAreOpen => {
                let v = v
                    .as_bool()
                    .ok_or_else(|| anyhow!("Expected bool value for {self:?}, got {v:?}"))?;
                Value::bool(v)
            }

            // Count values
            Self::ProfilePopulationEquivalent | Self::ProfileSludgeTreatmentDigesterCount => {
                let v = v
                    .as_u64()
                    .ok_or_else(|| anyhow!("Expected count value for {self:?}, got {v:?}"))?;
                Value::count(v)
            }

            // Qubicmeters values
            Self::ProfileWastewater | Self::ProfileSewageGasProduced | Self::ProfileGasSupply => {
                let v = v
                    .as_f64()
                    .ok_or_else(|| anyhow!("Expected qubicmeters value for {self:?}, got {v:?}"))?;
                Value::qubicmeters(v)
            }

            // MilligramsPerLiter values
            Self::ProfileInfluentNitrogen
            | Self::ProfileInfluentChemicalOxygenDemand
            | Self::ProfileInfluentTotalOrganicCarbohydrates
            | Self::ProfileEffluentNitrogen
            | Self::ProfileEffluentChemicalOxygenDemand => {
                let v = v.as_f64().ok_or_else(|| {
                    anyhow!("Expected milligrams_per_liter value for {self:?}, got {v:?}")
                })?;
                Value::milligrams_per_liter(v)
            }

            // Kilowatthours values
            Self::ProfileTotalPowerConsumption
            | Self::ProfileOnSitePowerGeneration
            | Self::RecommendationDistrictHeating
            | Self::RecommendationPhotovoltaicEnergyExpansion
            | Self::RecommendationWindEnergyExpansion
            | Self::RecommendationWaterEnergyExpansion => {
                let v = v.as_f64().ok_or_else(|| {
                    anyhow!("Expected kilowatthours value for {self:?}, got {v:?}")
                })?;
                Value::kilowatthours(v)
            }

            // GramsPerKilowatthour values
            Self::ProfileEmissionFactorElectricityMix => {
                let v = v.as_f64().ok_or_else(|| {
                    anyhow!("Expected grams_per_kilowatthour value for {self:?}, got {v:?}")
                })?;
                Value::grams_per_kilowatthour(v)
            }

            // Liters values
            Self::ProfileHeatingOil => {
                let v = v
                    .as_f64()
                    .ok_or_else(|| anyhow!("Expected liters value for {self:?}, got {v:?}"))?;
                Value::liters(v)
            }

            // Tons values
            Self::ProfileSideStreamTreatmentTotalNitrogen
            | Self::ProfileOperatingMaterialFeCl3
            | Self::ProfileOperatingMaterialFeClSO4
            | Self::ProfileOperatingMaterialCaOH2
            | Self::ProfileOperatingMaterialSyntheticPolymers
            | Self::ProfileSludgeTreatmentDisposal => {
                let v = v
                    .as_f64()
                    .ok_or_else(|| anyhow!("Expected tons value for {self:?}, got {v:?}"))?;
                Value::tons(v)
            }

            // Kilometers values
            Self::ProfileSludgeTreatmentTransportDistance => {
                let v = v
                    .as_f64()
                    .ok_or_else(|| anyhow!("Expected kilometers value for {self:?}, got {v:?}"))?;
                Value::kilometers(v)
            }

            // Percent values
            Self::ProfileMethaneFraction
            | Self::SensitivityN2OCustomFactor
            | Self::SensitivityN2OSideStreamFactor
            | Self::SensitivityCH4ChpCustomFactor
            | Self::SensitivityCO2FossilCustomFactor
            | Self::SensitivitySludgeStorageCustomFactor
            | Self::RecommendationProcessEnergySaving
            | Self::RecommendationFossilEnergySaving
            | Self::RecommendationEstimatedSelfPhotovolaticUsage
            | Self::RecommendationEstimatedSelfWindEnergyUsage
            | Self::RecommendationEstimatedSelfWaterEnergyUsage => {
                let v = v
                    .as_f64()
                    .ok_or_else(|| anyhow!("Expected percent value for {self:?}, got {v:?}"))?;
                Value::percent(v)
            }

            // QubicmetersPerHour values
            Self::SensitivitySludgeBagsCustomFactor => {
                let v = v.as_f64().ok_or_else(|| {
                    anyhow!("Expected qubicmeters_per_hour value for {self:?}, got {v:?}")
                })?;
                Value::qubicmeters_per_hour(v)
            }

            // N2oEmissionFactorCalcMethod values
            Self::SensitivityN2OCalculationMethod => {
                let method: N2oEmissionFactorCalcMethod = serde_json::from_value(v)?;
                Value::n2o_emission_factor_calc_method(method.into())
            }

            // CH4ChpEmissionFactorCalcMethod values
            Self::SensitivityCH4ChpCalculationMethod => {
                let method: CH4ChpEmissionFactorCalcMethod = serde_json::from_value(v)?;
                Value::ch4_chp_emission_factor_calc_method(method.into())
            }

            // Factor values
            Self::RecommendationN2OSideStreamFactor => {
                let v = v
                    .as_f64()
                    .ok_or_else(|| anyhow!("Expected factor value for {self:?}, got {v:?}"))?;
                Value::factor(v)
            }

            // Text values
            Self::SensitivityAdditionalCustomEmissions => {
                let v = v
                    .as_str()
                    .ok_or_else(|| anyhow!("Expected text value for {self:?}, got {v:?}"))?;
                Value::text(v)
            }
        };
        Ok(v)
    }

    /// Convert domain values to JSON
    // Since the data types in the domain layer may change in the future,
    // we need a clear assignment for v9 at this point.
    pub fn value_to_json(&self, value: Value) -> anyhow::Result<JsonValue> {
        let value_clone = value.clone(); // used in error messages

        match self {
            // Text values
            Self::ProjectName
            | Self::ProfilePlantName
            | Self::SensitivityAdditionalCustomEmissions => {
                let text_value = value
                    .as_text()
                    .ok_or_else(|| {
                        anyhow!("Expected text value for {self:?}, got {value_clone:?}")
                    })?
                    .to_string();
                Ok(JsonValue::String(text_value))
            }

            // Boolean values
            Self::RecommendationSludgeStorageContainersAreOpen
            | Self::ProfilePurchaseOfBiogas
            | Self::ProfileSludgeTreatmentBagsAreOpen
            | Self::RecommendationSludgeBagsAreOpen
            | Self::RecommendationN2OSideStreamCoverIsOpen
            | Self::ProfileSludgeTreatmentStorageContainersAreOpen => {
                let bool_value = value.as_bool().ok_or_else(|| {
                    anyhow!("Expected bool value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::Bool(bool_value))
            }

            // Count values
            Self::ProfilePopulationEquivalent | Self::ProfileSludgeTreatmentDigesterCount => {
                let count_value = value.as_count().ok_or_else(|| {
                    anyhow!("Expected count value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(u64::from(count_value)))
            }

            // Qubicmeters values
            Self::ProfileWastewater | Self::ProfileSewageGasProduced | Self::ProfileGasSupply => {
                let qubicmeters_value = value.as_qubicmeters().ok_or_else(|| {
                    anyhow!("Expected qubicmeters value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(qubicmeters_value)))
            }

            // MilligramsPerLiter values
            Self::ProfileInfluentNitrogen
            | Self::ProfileInfluentChemicalOxygenDemand
            | Self::ProfileInfluentTotalOrganicCarbohydrates
            | Self::ProfileEffluentNitrogen
            | Self::ProfileEffluentChemicalOxygenDemand => {
                let milligrams_value = value.as_milligrams_per_liter().ok_or_else(|| {
                    anyhow!("Expected milligrams_per_liter value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(milligrams_value)))
            }

            // Kilowatthours values
            Self::ProfileTotalPowerConsumption
            | Self::ProfileOnSitePowerGeneration
            | Self::RecommendationDistrictHeating
            | Self::RecommendationPhotovoltaicEnergyExpansion
            | Self::RecommendationWindEnergyExpansion
            | Self::RecommendationWaterEnergyExpansion => {
                let kilowatt_value = value.as_kilowatthours().ok_or_else(|| {
                    anyhow!("Expected kilowatthours value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(kilowatt_value)))
            }

            // GramsPerKilowatthour values
            Self::ProfileEmissionFactorElectricityMix => {
                let grams_value = value.as_grams_per_kilowatthour().ok_or_else(|| {
                    anyhow!(
                        "Expected grams_per_kilowatthour value for {self:?}, got {value_clone:?}"
                    )
                })?;
                Ok(JsonValue::from(f64::from(grams_value)))
            }

            // Liters values
            Self::ProfileHeatingOil => {
                let liters_value = value.as_liters().ok_or_else(|| {
                    anyhow!("Expected liters value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(liters_value)))
            }

            // Tons values
            Self::ProfileSideStreamTreatmentTotalNitrogen
            | Self::ProfileOperatingMaterialFeCl3
            | Self::ProfileOperatingMaterialFeClSO4
            | Self::ProfileOperatingMaterialCaOH2
            | Self::ProfileOperatingMaterialSyntheticPolymers
            | Self::ProfileSludgeTreatmentDisposal => {
                let tons_value = value.as_tons().ok_or_else(|| {
                    anyhow!("Expected tons value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(tons_value)))
            }

            // Kilometers values
            Self::ProfileSludgeTreatmentTransportDistance => {
                let kilometers_value = value.as_kilometers().ok_or_else(|| {
                    anyhow!("Expected kilometers value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(kilometers_value)))
            }

            // Percent values
            Self::ProfileMethaneFraction
            | Self::SensitivityN2OCustomFactor
            | Self::SensitivityN2OSideStreamFactor
            | Self::SensitivityCH4ChpCustomFactor
            | Self::SensitivityCO2FossilCustomFactor
            | Self::SensitivitySludgeStorageCustomFactor
            | Self::RecommendationProcessEnergySaving
            | Self::RecommendationFossilEnergySaving
            | Self::RecommendationEstimatedSelfPhotovolaticUsage
            | Self::RecommendationEstimatedSelfWindEnergyUsage
            | Self::RecommendationEstimatedSelfWaterEnergyUsage => {
                let percent_value = value.as_percent().ok_or_else(|| {
                    anyhow!("Expected percent value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(percent_value)))
            }

            // QubicmetersPerHour values
            Self::SensitivitySludgeBagsCustomFactor => {
                let qubicmeters_per_hour_value =
                    value.as_qubicmeters_per_hour().ok_or_else(|| {
                        anyhow!(
                            "Expected qubicmeters_per_hour value for {self:?}, got {value_clone:?}"
                        )
                    })?;
                Ok(JsonValue::from(f64::from(qubicmeters_per_hour_value)))
            }

            // N2oEmissionFactorCalcMethod values
            Self::SensitivityN2OCalculationMethod => {
                let method : N2oEmissionFactorCalcMethod= value
                    .as_n2o_emission_factor_calc_method()
                    .ok_or_else(|| {
                        anyhow!("Expected N2oEmissionFactorCalcMethod value for {self:?}, got {value_clone:?}")
                    })?
                    .into();
                Ok(serde_json::to_value(method)?)
            }

            // CH4ChpEmissionFactorCalcMethod values
            Self::SensitivityCH4ChpCalculationMethod => {
                let method : CH4ChpEmissionFactorCalcMethod= value
                    .as_ch4_chp_emission_factor_calc_method()
                    .ok_or_else(|| {
                        anyhow!("Expected Ch4ChpEmissionFactorCalcMethod value for {self:?}, got {value_clone:?}")
                    })?
                    .into();
                Ok(serde_json::to_value(method)?)
            }

            // Factor values
            Self::RecommendationN2OSideStreamFactor => {
                let factor_value = value.as_factor().ok_or_else(|| {
                    anyhow!("Expected factor value for {self:?}, got {value_clone:?}")
                })?;
                Ok(JsonValue::from(f64::from(factor_value)))
            }
        }
    }
}
