use crate::{constants, units::*, InputValueId as Id};

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
    const fn new(value_type: ValueType) -> Self {
        Self {
            value_type,
            optional: false,
            min: None,
            max: None,
            default: None,
        }
    }

    const fn new_optional(value_type: ValueType) -> Self {
        Self {
            value_type,
            optional: true,
            min: None,
            max: None,
            default: None,
        }
    }

    const fn with_min(mut self, min: f64) -> Self {
        self.min = Some(min);
        self
    }

    const fn with_max(mut self, max: f64) -> Self {
        self.max = Some(max);
        self
    }

    fn with_default(mut self, default: Value) -> Self {
        self.default = Some(default);
        self
    }

    pub const fn value_type(&self) -> ValueType {
        self.value_type
    }

    pub const fn optional(&self) -> bool {
        self.optional
    }

    pub const fn min(&self) -> Option<f64> {
        self.min
    }

    pub const fn max(&self) -> Option<f64> {
        self.max
    }

    pub const fn default(&self) -> Option<&Value> {
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
            S::new(T::milligrams_per_liter())
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
        (Id::SideStreamTreatmentTotalNitrogen, S::new(T::tons())),
        (
            Id::OperatingMaterialFeCl3,
            S::new(T::tons()).with_max(500_000.0),
        ),
        (
            Id::OperatingMaterialFeClSO4,
            S::new(T::tons()).with_max(100_000.0),
        ),
        (
            Id::OperatingMaterialCaOH2,
            S::new(T::tons()).with_max(500_000.0),
        ),
        (
            Id::OperatingMaterialSyntheticPolymers,
            S::new(T::tons()).with_max(50000.0),
        ),
        (
            Id::SensitivityN2OCalculationMethod,
            S::new(T::n2o_emission_factor_calc_method()),
        ),
        (Id::SensitivityN2OCustomFactor, S::new(T::factor())),
        (Id::SensitivityN2OSideStreamFactor, S::new(T::factor())),
        (
            Id::SensitivityCH4ChpCalculationMethod,
            S::new(T::ch4_chp_emission_factor_calc_method()),
        ),
        (
            Id::SensitivityCH4ChpCustomFactor,
            S::new(T::factor())
                .with_min(0.0)
                .with_max(1.0)
                .with_default(constants::EMISSION_FACTOR_CH4_CHP_CUSTOM_FACTOR_DEFAULT.into()),
        ),
        (Id::SensitivityCO2FossilCustomFactor, S::new(T::factor())),
        (
            Id::SensitivitySludgeBagsCustomFactor,
            S::new(T::factor())
                .with_min(0.0)
                .with_max(1.0)
                .with_default(constants::EMISSION_FACTOR_SLUDGE_BAGS.into()),
        ),
        (
            Id::SensitivitySludgeStorageCustomFactor,
            S::new(T::factor())
                .with_min(0.0)
                .with_max(1.0)
                .with_default(constants::EMISSION_FACTOR_SLUDGE_STORAGE.into()),
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
            S::new(T::kilometers()).with_min(0.0).with_max(2000.0),
        ),
        (
            Id::SludgeTreatmentDigesterCount,
            S::new(T::count()).with_max(9.0),
        ),
        (
            Id::SewageGasProduced,
            S::new(T::qubicmeters())
                .with_min(0.0)
                .with_max(100_000_000.0),
        ),
        (
            Id::MethaneFraction,
            S::new(T::percent()).with_min(0.0).with_max(90.0),
        ),
        (Id::GasSupply, S::new(T::qubicmeters())),
        (Id::PurchaseOfBiogas, S::new(T::bool())),
        (
            Id::TotalPowerConsumption,
            S::new(T::kilowatthours())
                .with_min(0.0)
                .with_max(1_000_000_000.0),
        ),
        (
            Id::OnSitePowerGeneration,
            S::new(T::kilowatthours())
                .with_min(0.0)
                .with_max(50_000_000.0),
        ),
        (
            Id::EmissionFactorElectricityMix,
            S::new(T::grams_per_kilowatthour())
                .with_min(0.0)
                .with_max(2500.0),
        ),
        (Id::HeatingOil, S::new(T::liters())),
        (Id::ScenarioSludgeBagsAreOpen, S::new(T::bool())),
        (
            Id::ScenarioSludgeStorageContainersAreOpen,
            S::new(T::bool()),
        ),
        (Id::ScenarioN2OSideStreamFactor, S::new(T::factor())),
        (Id::ScenarioN2OSideStreamCoverIsOpen, S::new(T::bool())),
        (Id::ScenarioProcessEnergySaving, S::new(T::percent())),
        (Id::ScenarioFossilEnergySaving, S::new(T::percent())),
        (Id::ScenarioDistrictHeating, S::new(T::kilowatthours())),
        (
            Id::ScenarioPhotovoltaicEnergyExpansion,
            S::new(T::kilowatthours()),
        ),
        (
            Id::ScenarioEstimatedSelfPhotovolaticUsage,
            S::new(T::percent()),
        ),
        (Id::ScenarioWindEnergyExpansion, S::new(T::kilowatthours())),
        (
            Id::ScenarioEstimatedSelfWindEnergyUsage,
            S::new(T::percent()),
        ),
        (Id::ScenarioWaterEnergyExpansion, S::new(T::kilowatthours())),
        (
            Id::ScenarioEstimatedSelfWaterEnergyUsage,
            S::new(T::percent()),
        ),
    ]
}
