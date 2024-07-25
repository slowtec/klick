use crate::{constants, units::*, InputValueId as Id};

#[must_use]
pub fn value_spec(id: &Id) -> ValueSpec {
    specs()
        .into_iter()
        .find(|(k, _)| k == id)
        .map(|(_, s)| s)
        .unwrap()
}

#[derive(Debug)]
pub struct ValueSpec {
    value_type: ValueType,
    optional: bool,
    min: Option<f64>, // TODO: use Scalar
    max: Option<f64>, // TODO: use Scalar
    default: Option<Value>,
}

impl ValueSpec {
    #[must_use]
    const fn new(value_type: ValueType) -> Self {
        Self {
            value_type,
            optional: false,
            min: None,
            max: None,
            default: None,
        }
    }

    #[must_use]
    const fn new_optional(value_type: ValueType) -> Self {
        Self {
            value_type,
            optional: true,
            min: None,
            max: None,
            default: None,
        }
    }

    #[must_use]
    const fn new_with_default(default_value: Value) -> Self {
        let value_type = default_value.value_type();
        let default = Some(default_value);
        Self {
            value_type,
            optional: true,
            min: None,
            max: None,
            default,
        }
    }

    #[must_use]
    const fn with_min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    #[must_use]
    const fn with_max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        self.value_type
    }

    #[must_use]
    pub const fn optional(&self) -> bool {
        self.optional
    }

    #[must_use]
    pub const fn min(&self) -> Option<f64> {
        self.min
    }

    #[must_use]
    pub const fn max(&self) -> Option<f64> {
        self.max
    }

    #[must_use]
    pub const fn default_value(&self) -> Option<&Value> {
        self.default.as_ref()
    }
}

// TODO: check optional values
fn specs() -> [(Id, ValueSpec); 48] {
    use ValueSpec as S;
    use ValueType as T;

    [
        (Id::ProjectName, S::new_optional(T::text())),
        (Id::PlantName, S::new_optional(T::text())),
        (
            Id::PopulationEquivalent,
            S::new(T::count()).with_min(0.0).with_max(5_000_000.0),
        ),
        (
            Id::Wastewater,
            S::new(T::qubicmeters())
                .with_min(0.0)
                .with_max(1_000_000_000.0),
        ),
        (
            Id::InfluentNitrogen,
            S::new(T::milligrams_per_liter())
                .with_min(0.0)
                .with_max(5000.0),
        ),
        (
            Id::InfluentChemicalOxygenDemand,
            S::new(T::milligrams_per_liter())
                .with_min(0.0)
                .with_max(5000.0),
        ),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            S::new_optional(T::milligrams_per_liter())
                .with_min(0.0)
                .with_max(2000.0),
        ),
        (
            Id::EffluentNitrogen,
            S::new(T::milligrams_per_liter())
                .with_min(0.0)
                .with_max(1000.0),
        ),
        (
            Id::EffluentChemicalOxygenDemand,
            S::new(T::milligrams_per_liter())
                .with_min(0.0)
                .with_max(1000.0),
        ),
        (
            Id::SideStreamTreatmentTotalNitrogen,
            S::new_optional(T::tons()),
        ),
        (
            Id::OperatingMaterialFeCl3,
            S::new(T::tons()).with_max(500_000.0),
        ),
        (
            Id::OperatingMaterialFeClSO4,
            S::new_optional(T::tons()).with_max(100_000.0),
        ),
        (
            Id::OperatingMaterialCaOH2,
            S::new_optional(T::tons()).with_max(500_000.0),
        ),
        (
            Id::OperatingMaterialSyntheticPolymers,
            S::new(T::tons()).with_max(50000.0),
        ),
        (
            Id::SensitivityN2OCalculationMethod,
            S::new(T::n2o_emission_factor_calc_method()),
        ),
        (
            Id::SensitivityN2OCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_N2O_DEFAULT.into())
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::SensitivityN2OSideStreamFactor,
            S::new_with_default(constants::EMISSION_FACTOR_N2O_DEFAULT.into())
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::SensitivityCH4ChpCalculationMethod,
            S::new(T::ch4_chp_emission_factor_calc_method()),
        ),
        (
            Id::SensitivityCH4ChpCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_CH4_CHP_DEFAULT.into())
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::SensitivityCO2FossilCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_CO2_DEFAULT.into())
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::SensitivitySludgeBagsCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_SLUDGE_BAGS.into())
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::SensitivitySludgeStorageCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_SLUDGE_STORAGE.into())
                .with_min(0.0)
                .with_max(100.0),
        ),
        (Id::SludgeTreatmentBagsAreOpen, S::new(T::bool())),
        (
            Id::SludgeTreatmentStorageContainersAreOpen,
            S::new(T::bool()),
        ),
        (
            Id::SludgeTreatmentDisposal,
            S::new(T::tons()).with_max(500_000.0),
        ),
        (
            Id::SludgeTreatmentTransportDistance,
            S::new_optional(T::kilometers())
                .with_min(0.0)
                .with_max(2000.0),
        ),
        (
            Id::SludgeTreatmentDigesterCount,
            S::new_optional(T::count()).with_max(9.0),
        ),
        (
            Id::SewageGasProduced,
            S::new_optional(T::qubicmeters())
                .with_min(0.0)
                .with_max(100_000_000.0),
        ),
        (
            Id::MethaneFraction,
            S::new_optional(T::percent()).with_min(0.0).with_max(90.0),
        ),
        (Id::GasSupply, S::new_optional(T::qubicmeters())),
        (Id::PurchaseOfBiogas, S::new(T::bool())),
        (
            Id::TotalPowerConsumption,
            S::new(T::kilowatthours())
                .with_min(0.0)
                .with_max(1_000_000_000.0),
        ),
        (
            Id::OnSitePowerGeneration,
            S::new_optional(T::kilowatthours())
                .with_min(0.0)
                .with_max(50_000_000.0),
        ),
        (
            Id::EmissionFactorElectricityMix,
            S::new(T::grams_per_kilowatthour())
                .with_min(0.0)
                .with_max(2500.0),
        ),
        (Id::HeatingOil, S::new_optional(T::liters())),
        (Id::ScenarioSludgeBagsAreOpen, S::new(T::bool())),
        (
            Id::ScenarioSludgeStorageContainersAreOpen,
            S::new(T::bool()),
        ),
        (Id::ScenarioN2OSideStreamFactor, S::new(T::factor())),
        (Id::ScenarioN2OSideStreamCoverIsOpen, S::new(T::bool())),
        (
            Id::ScenarioProcessEnergySaving,
            S::new_optional(T::percent()).with_min(0.0).with_max(100.0),
        ),
        (
            Id::ScenarioFossilEnergySaving,
            S::new_optional(T::percent()).with_min(0.0).with_max(100.0),
        ),
        (
            Id::ScenarioDistrictHeating,
            S::new_optional(T::kilowatthours()),
        ),
        (
            Id::ScenarioPhotovoltaicEnergyExpansion,
            S::new_optional(T::kilowatthours()),
        ),
        (
            Id::ScenarioEstimatedSelfPhotovolaticUsage,
            S::new_with_default(Value::percent(100.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::ScenarioWindEnergyExpansion,
            S::new_optional(T::kilowatthours()),
        ),
        (
            Id::ScenarioEstimatedSelfWindEnergyUsage,
            S::new_with_default(Value::percent(100.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::ScenarioWaterEnergyExpansion,
            S::new_optional(T::kilowatthours()),
        ),
        (
            Id::ScenarioEstimatedSelfWaterEnergyUsage,
            S::new_with_default(Value::percent(100.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
    ]
}
