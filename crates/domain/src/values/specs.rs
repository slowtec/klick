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
    default: Option<Value>,
}

impl ValueSpec {
    #[must_use]
    const fn new() -> Self {
        Self { default: None }
    }

    #[must_use]
    const fn new_with_default(default_value: Value) -> Self {
        let default = Some(default_value);
        Self { default }
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
        (Id::ProjectName, S::new()),
        (Id::PlantName, S::new()),
        (Id::PopulationEquivalent, S::new()),
        (Id::Wastewater, S::new()),
        (Id::InfluentNitrogen, S::new()),
        (Id::InfluentChemicalOxygenDemand, S::new()),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            S::new_with_default(V::milligrams_per_liter(0.0)),
        ),
        (Id::EffluentNitrogen, S::new()),
        (Id::EffluentChemicalOxygenDemand, S::new()),
        (
            Id::SideStreamTreatmentTotalNitrogen,
            S::new_with_default(V::tons(0.0)),
        ),
        (Id::OperatingMaterialFeCl3, S::new()),
        (
            Id::OperatingMaterialFeClSO4,
            S::new_with_default(V::tons(0.0)),
        ),
        (
            Id::OperatingMaterialCaOH2,
            S::new_with_default(V::tons(0.0)),
        ),
        (Id::OperatingMaterialSyntheticPolymers, S::new()),
        (
            Id::SensitivityN2OCalculationMethod,
            S::new_with_default(V::n2o_emission_factor_calc_method(Default::default())),
        ),
        (
            Id::SensitivityN2OCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_N2O_DEFAULT.into()),
        ),
        (
            Id::SensitivityN2OSideStreamFactor,
            S::new_with_default(constants::EMISSION_FACTOR_N2O_DEFAULT.into()),
        ),
        (
            Id::SensitivityCH4ChpCalculationMethod,
            S::new_with_default(V::ch4_chp_emission_factor_calc_method(Default::default())),
        ),
        (
            Id::SensitivityCH4ChpCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_CH4_CHP_DEFAULT.into()),
        ),
        (
            Id::SensitivityCO2FossilCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_CO2_DEFAULT.into()),
        ),
        (
            Id::SensitivitySludgeBagsCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_SLUDGE_BAGS.into()),
        ),
        (
            Id::SensitivitySludgeStorageCustomFactor,
            S::new_with_default(constants::EMISSION_FACTOR_SLUDGE_STORAGE.into()),
        ),
        (
            Id::SludgeTreatmentBagsAreOpen,
            S::new_with_default(V::bool(true)),
        ),
        (
            Id::SludgeTreatmentStorageContainersAreOpen,
            S::new_with_default(V::bool(true)),
        ),
        (Id::SludgeTreatmentDisposal, S::new()),
        (
            Id::SludgeTreatmentTransportDistance,
            S::new_with_default(V::kilometers(0.0)),
        ),
        (
            Id::SludgeTreatmentDigesterCount,
            S::new_with_default(V::count(0)),
        ),
        (
            Id::SewageGasProduced,
            S::new_with_default(V::qubicmeters(0.0)),
        ),
        (Id::MethaneFraction, S::new_with_default(V::percent(62.0))),
        (Id::GasSupply, S::new_with_default(V::qubicmeters(0.0))),
        (Id::PurchaseOfBiogas, S::new_with_default(V::bool(false))),
        (Id::TotalPowerConsumption, S::new()),
        (
            Id::OnSitePowerGeneration,
            S::new_with_default(V::kilowatthours(0.0)),
        ),
        (Id::EmissionFactorElectricityMix, S::new()),
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
            S::new_with_default(V::percent(0.0)),
        ),
        (
            Id::ScenarioFossilEnergySaving,
            S::new_with_default(V::percent(0.0)),
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
            S::new_with_default(V::percent(100.0)),
        ),
        (
            Id::ScenarioWindEnergyExpansion,
            S::new_with_default(V::kilowatthours(0.0)),
        ),
        (
            Id::ScenarioEstimatedSelfWindEnergyUsage,
            S::new_with_default(V::percent(100.0)),
        ),
        (
            Id::ScenarioWaterEnergyExpansion,
            S::new_with_default(V::kilowatthours(0.0)),
        ),
        (
            Id::ScenarioEstimatedSelfWaterEnergyUsage,
            S::new_with_default(V::percent(100.0)),
        ),
    ]
}

//macro_rules! specs {
//    (
//        $(
//            $enum_name:ident {
//                $(
//                    $variant:ident = {
//                        unit = $unit:ident;
//                        $(optional = $optional:tt;)?
//                        $(default = $default:expr;)?
//                        $(min = $min:expr;)?
//                        $(max = $max:expr;)?
//                    }$(,)?
//                )+
//            }$(,)?
//        )+
//    ) => {
//        $(
//            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//            pub enum $enum_name {
//                $(
//                    $variant,
//                )*
//            }
//
//            impl $enum_name {
//                #[must_use]
//                pub fn is_optional(&self) -> bool {
//                    match self {
//                        $(
//                            Self::$variant => {
//                                match stringify!($($optional)?) {
//                                  "true" => true,
//                                  _ => false,
//                                }
//                            },
//                        )*
//                    }
//                }
//
//                #[must_use]
//                pub fn value_type(&self) -> ValueType {
//                    match self {
//                        $(
//                            Self::$variant => {
//                                $unit::VALUE_TYPE
//                            }
//                        )*
//                    }
//                }
//
//            }
//        )*
//
//
//    };
//}

macro_rules! spec {
    (
        $name:ident {
            $(
                $variant:ident {
                    unit = $unit_type:ident;
                    $( $key:ident $( = $value:expr )?; )*
                }
            ),* $(,)?
        }
    ) => {
        #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn min(&self) -> Option<f64> {
                match self {
                    $(
                        Self::$variant => {
                            $(
                                $(
                                    if stringify!($key) == "min" {
                                        return Some($value);
                                    }
                                )?
                            )*
                            None
                        },
                    )*
                }
            }

            pub fn max(&self) -> Option<f64> {
                match self {
                    $(
                        Self::$variant => {
                            $(
                                $(
                                    if stringify!($key) == "max" {
                                        return Some($value);
                                    }
                                )?
                            )*
                            None
                        },
                    )*
                }
            }

            // TODO:
            // pub fn default(&self) -> Option<f64> {
            //     match self {
            //         $(
            //             Self::$variant => {
            //                 $(
            //                     $(
            //                         if stringify!($key) == "default" {
            //                             return Some($value);
            //                         }
            //                     )?
            //                 )*
            //                 None
            //             },
            //         )*
            //     }
            // }

            pub fn is_optional(&self) -> bool {
                match self {
                    $(
                        Self::$variant => {
                            $(
                                if stringify!($key) == "optional" {
                                    return true;
                                }
                            )*
                            false
                        },
                    )*
                }
            }

            #[must_use]
            pub fn value_type(&self) -> ValueType {
                match self {
                    $(
                        Self::$variant => {
                            $unit_type::VALUE_TYPE
                        }
                    )*
                }
            }
        }
    };
}

spec! {
    InputValueId {
        ProjectName {
           unit = String;
           optional;
        },
        PlantName {
           unit = String;
           optional;
        },

        PopulationEquivalent {
            unit = Count;
            min = 0.0;
            max = 5_000_000.0;
        },
        Wastewater {
            unit = Qubicmeters;
            min = 0.0;
            max = 1_000_000_000.0;
        },

        InfluentNitrogen {
            unit = MilligramsPerLiter;
            min = 1.0; // must not be 0.0 to prevent division by 0
            max = 5000.0;
        },
        InfluentChemicalOxygenDemand {
            unit = MilligramsPerLiter;
            min = 0.0;
            max = 5000.0;
        },
        InfluentTotalOrganicCarbohydrates {
            unit = MilligramsPerLiter;
            optional;
            min = 0.0;
            max = 2000.0;
        },

        EffluentNitrogen {
            unit = MilligramsPerLiter;
            min = 0.0;
            max = 1000.0;

        },
        EffluentChemicalOxygenDemand {
            unit = MilligramsPerLiter;
            min = 0.0;
            max = 1000.0;
        },

        SewageGasProduced {
            unit = Qubicmeters;
            optional;
            min = 0.0;
            max = 100_000_000.0;
        },
        MethaneFraction {
            unit = Percent;
            optional;
            min = 0.0;
            max = 90.0;
        },
        GasSupply {
            unit = Qubicmeters;
            optional;
        },
        PurchaseOfBiogas {
            unit = bool;
            optional;
        },
        TotalPowerConsumption {
            unit = Kilowatthours;
            min = 0.0;
            max = 1_000_000_000.0;
        },
        OnSitePowerGeneration {
            unit = Kilowatthours;
            optional;
            min = 0.0;
            max = 50_000_000.0;
        },
        EmissionFactorElectricityMix {
            unit = GramsPerKilowatthour;
            min = 0.0;
            max = 2500.0;
        },
        HeatingOil {
            unit = Liters;
            optional;
        },

        SideStreamTreatmentTotalNitrogen {
            unit = Tons;
            optional;
        },

        OperatingMaterialFeCl3 {
            unit = Tons;
            max = 500_000.0;
        },
        OperatingMaterialFeClSO4 {
            unit = Tons;
            optional;
            max = 100_000.0;
        },
        OperatingMaterialCaOH2 {
            unit = Tons;
            optional;
            max = 500_000.0;
        },
        OperatingMaterialSyntheticPolymers {
            unit = Tons;
            max = 50000.0;
        },

        SensitivityN2OCalculationMethod {
            unit = N2oEmissionFactorCalcMethod;
            optional;
        },
        SensitivityN2OCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        SensitivityN2OSideStreamFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        SensitivityCH4ChpCalculationMethod {
            unit = Ch4ChpEmissionFactorCalcMethod;
            optional;
        },
        SensitivityCH4ChpCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        SensitivityCO2FossilCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        SensitivitySludgeBagsCustomFactor {
            unit = QubicmetersPerHour;
            optional;
            min = 0.0;
            max = 100.0;
        },
        SensitivitySludgeStorageCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        SludgeTreatmentBagsAreOpen {
            unit = bool;
            optional;
        },
        SludgeTreatmentStorageContainersAreOpen {
            unit = bool;
            optional;
        },
        SludgeTreatmentDisposal {
            unit = Tons;
            max = 500_000.0;
        },
        SludgeTreatmentTransportDistance {
            unit = Kilometers;
            optional;
            min = 0.0;
            max = 2000.0;
        },
        SludgeTreatmentDigesterCount {
            unit = Count;
            optional;
            min = 0.0;
            max = 9.0;
        },

        ScenarioSludgeBagsAreOpen {
            unit = bool;
            optional;
        },
        ScenarioSludgeStorageContainersAreOpen {
            unit = bool;
            optional;
        },
        ScenarioN2OSideStreamFactor {
            unit = Factor; // TODO: should't this be Percent?
        },
        ScenarioN2OSideStreamCoverIsOpen {
           unit = bool;
           optional;
        },
        ScenarioProcessEnergySaving {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        ScenarioFossilEnergySaving {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        ScenarioDistrictHeating {
            unit = Kilowatthours;
            optional;
        },
        ScenarioPhotovoltaicEnergyExpansion {
            unit = Kilowatthours;
            optional;
        },
        ScenarioEstimatedSelfPhotovolaticUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        ScenarioWindEnergyExpansion {
            unit = Kilowatthours;
            optional;
        },
        ScenarioEstimatedSelfWindEnergyUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
        ScenarioWaterEnergyExpansion {
            unit = Kilowatthours;
            optional;
        },
        ScenarioEstimatedSelfWaterEnergyUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
        },
    }
}

// TODO:
// #[test]
// fn check_specs() {
//   for (id, spec) in specs() {
//       assert_eq!(id.default(), spec.default(), "{id:?}");
//   }
// }
