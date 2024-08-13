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
    optional: bool,
    min: Option<f64>, // TODO: use Scalar
    max: Option<f64>, // TODO: use Scalar
    default: Option<Value>,
}

impl ValueSpec {
    #[must_use]
    const fn new() -> Self {
        Self {
            optional: false,
            min: None,
            max: None,
            default: None,
        }
    }

    #[must_use]
    const fn new_optional() -> Self {
        Self {
            optional: true,
            min: None,
            max: None,
            default: None,
        }
    }

    #[must_use]
    const fn new_with_default(default_value: Value) -> Self {
        let default = Some(default_value);
        Self {
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

    [
        (Id::ProjectName, S::new_optional()),
        (Id::PlantName, S::new_optional()),
        (
            Id::PopulationEquivalent,
            S::new().with_min(0.0).with_max(5_000_000.0),
        ),
        (
            Id::Wastewater,
            S::new().with_min(0.0).with_max(1_000_000_000.0),
        ),
        (
            Id::InfluentNitrogen,
            S::new()
                .with_min(1.0) // must not be 0.0 to prevent division by 0
                .with_max(5000.0),
        ),
        (
            Id::InfluentChemicalOxygenDemand,
            S::new().with_min(0.0).with_max(5000.0),
        ),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            S::new_with_default(V::milligrams_per_liter(0.0))
                .with_min(0.0)
                .with_max(2000.0),
        ),
        (
            Id::EffluentNitrogen,
            S::new().with_min(0.0).with_max(1000.0),
        ),
        (
            Id::EffluentChemicalOxygenDemand,
            S::new().with_min(0.0).with_max(1000.0),
        ),
        (
            Id::SideStreamTreatmentTotalNitrogen,
            S::new_with_default(V::tons(0.0)),
        ),
        (Id::OperatingMaterialFeCl3, S::new().with_max(500_000.0)),
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
            S::new().with_max(50000.0),
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
        (Id::SludgeTreatmentDisposal, S::new().with_max(500_000.0)),
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
            S::new().with_min(0.0).with_max(1_000_000_000.0),
        ),
        (
            Id::OnSitePowerGeneration,
            S::new_with_default(V::kilowatthours(0.0))
                .with_min(0.0)
                .with_max(50_000_000.0),
        ),
        (
            Id::EmissionFactorElectricityMix,
            S::new().with_min(0.0).with_max(2500.0),
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
        (Id::ScenarioN2OSideStreamFactor, S::new()),
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

macro_rules! specs {
    (
        $(
            $enum_name:ident {
                $(
                    $variant:ident = {
                        unit = $unit:ident;
                        $(optional = $optional:tt;)?
                        $(default = $default:expr;)?
                        $(min = $min:expr;)?
                        $(max = $max:expr;)?
                    }$(,)?
                )+
            }$(,)?
        )+
    ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum $enum_name {
                $(
                    $variant,
                )*
            }

            impl $enum_name {
                #[must_use]
                pub fn is_optional(&self) -> bool {
                    match self {
                        $(
                            Self::$variant => {
                                // match stringify!($($optional)?) {
                                //   "true" => true,
                                //   _ => false,
                                // }
                                todo!("Don't use this yet!")
                            },
                        )*
                    }
                }

                #[must_use]
                pub fn value_type(&self) -> ValueType {
                    match self {
                        $(
                            Self::$variant => {
                                $unit::VALUE_TYPE
                            }
                        )*
                    }
                }

            }
        )*


    };
}

specs! {
    InputValueId {
        ProjectName = {
           unit = String;
           optional = true;
        },
        PlantName = {
           unit = String;
           optional = true;
        },

        PopulationEquivalent = {
            unit = Count;
        },
        Wastewater = {
            unit = Qubicmeters;
        },

        InfluentNitrogen = {
            unit = MilligramsPerLiter;
        },
        InfluentChemicalOxygenDemand = {
            unit = MilligramsPerLiter;
        },
        InfluentTotalOrganicCarbohydrates = {
            unit = MilligramsPerLiter;
        },

        EffluentNitrogen = {
            unit = MilligramsPerLiter;

        },
        EffluentChemicalOxygenDemand = {
            unit = MilligramsPerLiter;
        },

        SewageGasProduced = {
            unit = Qubicmeters;
        },
        MethaneFraction = {
            unit = Percent;
        },
        GasSupply = {
            unit = Qubicmeters;
        },
        PurchaseOfBiogas = {
            unit = bool;
        },
        TotalPowerConsumption = {
            unit = Kilowatthours;
        },
        OnSitePowerGeneration = {
            unit = Kilowatthours;
        },
        EmissionFactorElectricityMix = {
            unit = GramsPerKilowatthour;
        },
        HeatingOil = {
            unit = Liters;
        },

        SideStreamTreatmentTotalNitrogen = {
            unit = Tons;
        },

        OperatingMaterialFeCl3 = {
            unit = Tons;
        },
        OperatingMaterialFeClSO4 = {
            unit = Tons;
        },
        OperatingMaterialCaOH2 = {
            unit = Tons;
        },
        OperatingMaterialSyntheticPolymers = {
            unit = Tons;
        },

        SensitivityN2OCalculationMethod = {
            unit = N2oEmissionFactorCalcMethod;

        },
        SensitivityN2OCustomFactor = {
            unit = Percent;
        },
        SensitivityN2OSideStreamFactor = {
            unit = Percent;
        },
        SensitivityCH4ChpCalculationMethod = {
            unit = Ch4ChpEmissionFactorCalcMethod;
        },
        SensitivityCH4ChpCustomFactor = {
            unit = Percent;
        },
        SensitivityCO2FossilCustomFactor = {
            unit = Percent;
        },
        SensitivitySludgeBagsCustomFactor = {
            unit = QubicmetersPerHour;
        },
        SensitivitySludgeStorageCustomFactor = {
            unit = Percent;
        },
        SludgeTreatmentBagsAreOpen = {
            unit = bool;
        },
        SludgeTreatmentStorageContainersAreOpen = {
            unit = bool;
        },
        SludgeTreatmentDisposal = {
            unit = Tons;
        },
        SludgeTreatmentTransportDistance = {
            unit = Kilometers;
        },
        SludgeTreatmentDigesterCount = {
            unit = Count;
        },

        ScenarioSludgeBagsAreOpen = {
            unit = bool;
        },
        ScenarioSludgeStorageContainersAreOpen = {
            unit = bool;
        },
        ScenarioN2OSideStreamFactor = {
            unit = Percent;
        },
        ScenarioN2OSideStreamCoverIsOpen = {
            unit = bool;
        },
        ScenarioProcessEnergySaving = {
            unit = Percent;
        },
        ScenarioFossilEnergySaving = {
            unit = Percent;
        },
        ScenarioDistrictHeating = {
            unit = Kilowatthours;
        },
        ScenarioPhotovoltaicEnergyExpansion = {
            unit = Kilowatthours;
        },
        ScenarioEstimatedSelfPhotovolaticUsage = {
            unit = Percent;
        },
        ScenarioWindEnergyExpansion = {
            unit = Kilowatthours;
        },
        ScenarioEstimatedSelfWindEnergyUsage = {
            unit = Percent;
        },
        ScenarioWaterEnergyExpansion = {
            unit = Kilowatthours;
        },
        ScenarioEstimatedSelfWaterEnergyUsage = {
            unit = Percent;
        },
    }
}
