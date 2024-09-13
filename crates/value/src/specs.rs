use crate::{constants, units::*};
use klick_value_spec::value_spec as spec;

spec! {
    Value : InputValueId {

        // ------    ------ //
        //     Project      //
        // ------    ------ //

        ProjectName {
           unit = String;
           optional;
        },

        // ------    ------ //
        //     Profile      //
        // ------    ------ //

        ProfilePlantName {
           unit = String;
           optional;
        },
        ProfilePopulationEquivalent {
            unit = Count;
            min = 0.0;
            max = 5_000_000.0;
        },
        ProfileWastewater {
            unit = Qubicmeters;
            min = 0.0;
            max = 1_000_000_000.0;
        },
        ProfileInfluentNitrogen {
            unit = MilligramsPerLiter;
            min = 1.0; // must not be 0.0 to prevent division by 0
            max = 5000.0;
        },
        ProfileInfluentChemicalOxygenDemand {
            unit = MilligramsPerLiter;
            min = 0.0;
            max = 5000.0;
        },
        ProfileInfluentTotalOrganicCarbohydrates {
            unit = MilligramsPerLiter;
            optional;
            min = 0.0;
            max = 2000.0;
            default = 0.0;
        },
        ProfileEffluentNitrogen {
            unit = MilligramsPerLiter;
            min = 0.0;
            max = 1000.0;

        },
        ProfileEffluentChemicalOxygenDemand {
            unit = MilligramsPerLiter;
            min = 0.0;
            max = 1000.0;
        },
        ProfileSewageGasProduced {
            unit = Qubicmeters;
            optional;
            min = 0.0;
            max = 100_000_000.0;
            default = 0.0;
        },
        ProfileMethaneFraction {
            unit = Percent;
            optional;
            min = 0.0;
            max = 90.0;
            default = 62.0;
        },
        ProfileGasSupply {
            unit = Qubicmeters;
            optional;
            default = 0.0;
        },
        ProfilePurchaseOfBiogas {
            unit = bool;
            optional;
            default = false;
        },
        ProfileTotalPowerConsumption {
            unit = Kilowatthours;
            min = 0.0;
            max = 1_000_000_000.0;
        },
        ProfileOnSitePowerGeneration {
            unit = Kilowatthours;
            optional;
            min = 0.0;
            max = 50_000_000.0;
            default = 0.0;
        },
        ProfileEmissionFactorElectricityMix {
            unit = GramsPerKilowatthour;
            optional;
            min = 0.0;
            max = 2500.0;
            default = 485.0;
        },
        ProfileHeatingOil {
            unit = Liters;
            optional;
            default = 0.0;
        },
        ProfileSideStreamTotalNitrogen {
            unit = Tons;
            optional;
            default = 0.0;
        },
        ProfileOperatingMaterialFeCl3 {
            unit = Tons;
            max = 500_000.0;
        },
        ProfileOperatingMaterialFeClSO4 {
            unit = Tons;
            optional;
            max = 100_000.0;
            default = 0.0;
        },
        ProfileOperatingMaterialCaOH2 {
            unit = Tons;
            optional;
            max = 500_000.0;
            default = 0.0;
        },
        ProfileOperatingMaterialSyntheticPolymers {
            unit = Tons;
            max = 50000.0;
        },
        ProfileSludgeBagsAreOpen{
            unit = bool;
            optional;
            default = true;
        },
        ProfileSludgeStorageContainersAreOpen {
            unit = bool;
            optional;
            default = true;
        },
        ProfileSludgeDisposal {
            unit = Tons;
            max = 500_000.0;
        },
        ProfileSludgeTransportDistance {
            unit = Kilometers;
            optional;
            min = 0.0;
            max = 2000.0;
            default = 0.0;
        },
        ProfileSludgeDigesterCount {
            unit = Count;
            optional;
            min = 0.0;
            max = 9.0;
            default = 0;
        },

        // ------    ------ //
        //   Sensitivity    //
        // ------    ------ //

        SensitivityN2OCalculationMethod {
            unit = N2oEmissionFactorCalcMethod;
            optional;
            default = N2oEmissionFactorCalcMethod::default();
        },
        SensitivityN2OCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = constants::EMISSION_FACTOR_N2O_DEFAULT.into();
        },
        SensitivityN2OSideStreamFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = constants::EMISSION_FACTOR_N2O_DEFAULT.into();
        },
        SensitivityCH4ChpCalculationMethod {
            unit = Ch4ChpEmissionFactorCalcMethod;
            optional;
            default = Ch4ChpEmissionFactorCalcMethod::default();
        },
        SensitivityCH4ChpCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = constants::EMISSION_FACTOR_CH4_CHP_DEFAULT.into();
        },
        SensitivityCO2FossilCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = constants::EMISSION_FACTOR_CO2_DEFAULT.into();
        },
        SensitivitySludgeBagsCustomFactor {
            unit = QubicmetersPerHour;
            optional;
            min = 0.0;
            max = 100.0;
            default = constants::EMISSION_FACTOR_SLUDGE_BAGS.into();
        },
        SensitivitySludgeStorageCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = constants::EMISSION_FACTOR_SLUDGE_STORAGE.into();
        },
        SensitivityAdditionalCustomEmissions {
          unit = String;
          optional;
        },

        // ------    ------ //
        //  Recommendation  //
        // ------    ------ //

        RecommendationSludgeBagsAreOpen {
            unit = bool;
            optional;
            default = true;
        },
        RecommendationSludgeStorageContainersAreOpen {
            unit = bool;
            optional;
            default = true;
        },
        RecommendationN2OSideStreamCoverIsOpen {
            unit = bool;
            optional;
            default = true;
        },
        RecommendationProcessEnergySaving {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 0.0;
        },
        RecommendationFossilEnergySaving {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 0.0;
        },
        RecommendationDistrictHeating {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        RecommendationPhotovoltaicEnergyExpansion {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        RecommendationEstimatedSelfPhotovolaticUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 100.0;
        },
        RecommendationWindEnergyExpansion {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        RecommendationEstimatedSelfWindEnergyUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 100.0;
        },
        RecommendationWaterEnergyExpansion {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        RecommendationEstimatedSelfWaterEnergyUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 100.0;
        },
    }
}

spec! {
    Value : OutputValueId {
        N2oPlant {
          unit = Tons;
        },
        N2oWater {
          unit = Tons;
        },
        N2oSideStream {
          unit = Tons;
        },
        N2oEmissions {
          unit = Tons;
        },
        Ch4Plant {
          unit = Tons;
        },
        Ch4SludgeStorageContainers {
          unit = Tons;
        },
        Ch4SludgeBags {
          unit = Tons;
        },
        Ch4Water {
          unit = Tons;
        },
        Ch4CombinedHeatAndPowerPlant {
          unit = Tons;
        },
        Ch4Emissions {
          unit = Tons;
        },
        FossilEmissions {
          unit = Tons;
        },
        Fecl3 {
          unit = Tons;
        },
        Feclso4 {
          unit = Tons;
        },
        Caoh2 {
          unit = Tons;
        },
        SyntheticPolymers {
          unit = Tons;
        },
        ElectricityMix {
          unit = Tons;
        },
        OilEmissions {
          unit = Tons;
        },
        GasEmissions {
          unit = Tons;
        },
        OperatingMaterials {
          unit = Tons;
        },
        SewageSludgeTransport {
          unit = Tons;
        },
        TotalEmissions {
          unit = Tons;
        },
        DirectEmissions {
          unit = Tons;
        },
        ProcessEnergySavings {
          unit = Tons;
        },
        PhotovoltaicExpansionSavings {
          unit = Tons;
        },
        WindExpansionSavings {
          unit = Tons;
        },
        WaterExpansionSavings {
          unit = Tons;
        },
        DistrictHeatingSavings {
          unit = Tons;
        },
        FossilEnergySavings {
          unit = Tons;
        },
        IndirectEmissions {
          unit = Tons;
        },
        OtherIndirectEmissions {
          unit = Tons;
        },
        ExcessEnergyCo2Equivalent {
          unit = Tons;
        },
        N2oCalculatedEmissionFactor {
          unit = Factor;
        },
        Ch4ChpCalculatedEmissionFactor {
          unit = Factor;
        },
        N2oEmissionFactorCalcMethod {
          unit = N2oEmissionFactorCalcMethod;
        },
        Ch4ChpEmissionFactorCalcMethod {
          unit = Ch4ChpEmissionFactorCalcMethod;
        },
        N2oEmissionCustomFactor {
          unit = Factor;
          optional;
        },
        Ch4ChpEmissionCustomFactor {
          unit = Factor;
          optional;
        },
        AdditionalCustomEmissions {
          unit = Tons;
          optional;
        }
    }
}
