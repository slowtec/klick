use crate::{constants, units::*};

use klick_value_spec::value_spec as spec;

spec! {
    Value : InputValueId {
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
            default = 0.0;
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
            default = 0.0;
        },
        MethaneFraction {
            unit = Percent;
            optional;
            min = 0.0;
            max = 90.0;
            default = 62.0;
        },
        GasSupply {
            unit = Qubicmeters;
            optional;
            default = 0.0;
        },
        PurchaseOfBiogas {
            unit = bool;
            optional;
            default = false;
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
            default = 0.0;
        },
        EmissionFactorElectricityMix {
            unit = GramsPerKilowatthour;
            min = 0.0;
            max = 2500.0;
        },
        HeatingOil {
            unit = Liters;
            optional;
            default = 0.0;
        },

        SideStreamTreatmentTotalNitrogen {
            unit = Tons;
            optional;
            default = 0.0;
        },

        OperatingMaterialFeCl3 {
            unit = Tons;
            max = 500_000.0;
        },
        OperatingMaterialFeClSO4 {
            unit = Tons;
            optional;
            max = 100_000.0;
            default = 0.0;
        },
        OperatingMaterialCaOH2 {
            unit = Tons;
            optional;
            max = 500_000.0;
            default = 0.0;
        },
        OperatingMaterialSyntheticPolymers {
            unit = Tons;
            max = 50000.0;
        },

        SensitivityN2OCalculationMethod {
            unit = N2oEmissionFactorCalcMethod;
            optional;
            default = Default::default();
        },
        SensitivityN2OCustomFactor {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 2.0;
            default = constants::EMISSION_FACTOR_N2O_DEFAULT;
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
            default = Default::default();
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
        SludgeTreatmentBagsAreOpen {
            unit = bool;
            optional;
            default = true;
        },
        SludgeTreatmentStorageContainersAreOpen {
            unit = bool;
            optional;
            default = true;
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
            default = 0.0;
        },
        SludgeTreatmentDigesterCount {
            unit = Count;
            optional;
            min = 0.0;
            max = 9.0;
            default = 0;
        },

        ScenarioSludgeBagsAreOpen {
            unit = bool;
            optional;
            default = true;
        },
        ScenarioSludgeStorageContainersAreOpen {
            unit = bool;
            optional;
            default = true;
        },
        ScenarioN2OSideStreamFactor {
            unit = Factor; // TODO: should't this be Percent?
        },
        ScenarioN2OSideStreamCoverIsOpen {
            unit = bool;
            optional;
            default = true;
        },
        ScenarioProcessEnergySaving {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 0.0;
        },
        ScenarioFossilEnergySaving {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 0.0;
        },
        ScenarioDistrictHeating {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        ScenarioPhotovoltaicEnergyExpansion {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        ScenarioEstimatedSelfPhotovolaticUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 100.0;
        },
        ScenarioWindEnergyExpansion {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        ScenarioEstimatedSelfWindEnergyUsage {
            unit = Percent;
            optional;
            min = 0.0;
            max = 100.0;
            default = 100.0;
        },
        ScenarioWaterEnergyExpansion {
            unit = Kilowatthours;
            optional;
            default = 0.0;
        },
        ScenarioEstimatedSelfWaterEnergyUsage {
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
    }
}
