use klick_boundary::FormData;
use klick_domain::{units::*, InputValueId as Id, Value};

pub fn example_form_data() -> FormData {
    let mut data = FormData::default();

    let values = [
        (Id::ProfilePlantName, Value::text("Muster Klärwerk")),
        (Id::ProfilePopulationEquivalent, Value::count(50_000)),
        (Id::ProfileWastewater, Value::qubicmeters(2_135_250.0)),
        (
            Id::ProfileInfluentNitrogen,
            Value::milligrams_per_liter(94.0),
        ),
        (
            Id::ProfileInfluentChemicalOxygenDemand,
            Value::milligrams_per_liter(1_020.0),
        ),
        (
            Id::ProfileInfluentTotalOrganicCarbohydrates,
            Value::milligrams_per_liter(382.5),
        ),
        (
            Id::ProfileEffluentNitrogen,
            Value::milligrams_per_liter(15.77),
        ),
        (
            Id::ProfileEffluentChemicalOxygenDemand,
            Value::milligrams_per_liter(47.18),
        ),
        (Id::ProfileSewageGasProduced, Value::qubicmeters(420_000.0)),
        (Id::ProfileMethaneFraction, Value::percent(62.0)),
        (Id::ProfilePurchaseOfBiogas, Value::bool(false)),
        (
            Id::ProfileTotalPowerConsumption,
            Value::kilowatthours(1_665_000.0),
        ),
        (
            Id::ProfileOnSitePowerGeneration,
            Value::kilowatthours(810_000.0),
        ),
        (
            Id::ProfileEmissionFactorElectricityMix,
            Value::grams_per_kilowatthour(420.0),
        ),
        (Id::ProfileSludgeTreatmentBagsAreOpen, Value::bool(true)),
        (
            Id::ProfileSludgeTreatmentStorageContainersAreOpen,
            Value::bool(true),
        ),
        (Id::ProfileSludgeTreatmentDisposal, Value::tons(3016.5)),
        (
            Id::ProfileSludgeTreatmentTransportDistance,
            Value::kilometers(150.0),
        ),
        (Id::ProfileSludgeTreatmentDigesterCount, Value::count(1)),
        (Id::ProfileOperatingMaterialFeCl3, Value::tons(310.5)),
        (
            Id::ProfileOperatingMaterialSyntheticPolymers,
            Value::tons(12.0),
        ),
        (
            Id::SensitivityN2OCalculationMethod,
            Value::n2o_emission_factor_calc_method(N2oEmissionFactorCalcMethod::Ipcc2019),
        ),
        (
            Id::SensitivityCH4ChpCalculationMethod,
            Value::ch4_chp_emission_factor_calc_method(
                Ch4ChpEmissionFactorCalcMethod::GasolineEngine,
            ),
        ),
    ];

    for (id, v) in values {
        data.insert(id, v);
    }
    data
}
