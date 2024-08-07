use crate::{constants, units::*, InputValueId as Id, Value as V};

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
                .with_min(1.0) // must not be 0.0 to prevent division by 0
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
            S::new_with_default(V::milligrams_per_liter(0.0))
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
            S::new_with_default(V::tons(0.0)),
        ),
        (
            Id::OperatingMaterialFeCl3,
            S::new(T::tons()).with_max(500_000.0),
        ),
        (
            Id::OperatingMaterialFeClSO4,
            S::new_with_default(V::tons(0.0)).with_max(100_000.0),
        ),
        (
            Id::OperatingMaterialCaOH2,
            S::new_with_default(V::tons(0.0)).with_max(500_000.0),
        ),
        (
            Id::OperatingMaterialSyntheticPolymers,
            S::new(T::tons()).with_max(50000.0),
        ),
        (
            Id::SensitivityN2OCalculationMethod,
            S::new_with_default(V::n2o_emission_factor_calc_method(Default::default())),
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
            S::new_with_default(V::ch4_chp_emission_factor_calc_method(Default::default())),
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
        (
            Id::SludgeTreatmentBagsAreOpen,
            S::new_with_default(V::bool(true)),
        ),
        (
            Id::SludgeTreatmentStorageContainersAreOpen,
            S::new_with_default(V::bool(true)),
        ),
        (
            Id::SludgeTreatmentDisposal,
            S::new(T::tons()).with_max(500_000.0),
        ),
        (
            Id::SludgeTreatmentTransportDistance,
            S::new_with_default(V::kilometers(0.0))
                .with_min(0.0)
                .with_max(2000.0),
        ),
        (
            Id::SludgeTreatmentDigesterCount,
            S::new_with_default(V::count(0)).with_min(0.0).with_max(9.0),
        ),
        (
            Id::SewageGasProduced,
            S::new_with_default(V::qubicmeters(0.0))
                .with_min(0.0)
                .with_max(100_000_000.0),
        ),
        (
            Id::MethaneFraction,
            S::new_with_default(V::percent(62.0))
                .with_min(0.0)
                .with_max(90.0),
        ),
        (Id::GasSupply, S::new_with_default(V::qubicmeters(0.0))),
        (Id::PurchaseOfBiogas, S::new_with_default(V::bool(false))),
        (
            Id::TotalPowerConsumption,
            S::new(T::kilowatthours())
                .with_min(0.0)
                .with_max(1_000_000_000.0),
        ),
        (
            Id::OnSitePowerGeneration,
            S::new_with_default(V::kilowatthours(0.0))
                .with_min(0.0)
                .with_max(50_000_000.0),
        ),
        (
            Id::EmissionFactorElectricityMix,
            S::new(T::grams_per_kilowatthour())
                .with_min(0.0)
                .with_max(2500.0),
        ),
        (Id::HeatingOil, S::new_with_default(V::liters(0.0))),
        (
            Id::ScenarioSludgeBagsAreOpen,
            S::new_with_default(V::bool(true)),
        ),
        (
            Id::ScenarioSludgeStorageContainersAreOpen,
            S::new_with_default(V::bool(true)),
        ),
        (Id::ScenarioN2OSideStreamFactor, S::new(T::factor())),
        (
            Id::ScenarioN2OSideStreamCoverIsOpen,
            S::new_with_default(V::bool(true)),
        ),
        (
            Id::ScenarioProcessEnergySaving,
            S::new_with_default(V::percent(0.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::ScenarioFossilEnergySaving,
            S::new_with_default(V::percent(0.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::ScenarioDistrictHeating,
            S::new_with_default(V::kilowatthours(0.0)),
        ),
        (
            Id::ScenarioPhotovoltaicEnergyExpansion,
            S::new_with_default(V::kilowatthours(0.0)),
        ),
        (
            Id::ScenarioEstimatedSelfPhotovolaticUsage,
            S::new_with_default(V::percent(100.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::ScenarioWindEnergyExpansion,
            S::new_with_default(V::kilowatthours(0.0)),
        ),
        (
            Id::ScenarioEstimatedSelfWindEnergyUsage,
            S::new_with_default(V::percent(100.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
        (
            Id::ScenarioWaterEnergyExpansion,
            S::new_with_default(V::kilowatthours(0.0)),
        ),
        (
            Id::ScenarioEstimatedSelfWaterEnergyUsage,
            S::new_with_default(V::percent(100.0))
                .with_min(0.0)
                .with_max(100.0),
        ),
    ]
}
