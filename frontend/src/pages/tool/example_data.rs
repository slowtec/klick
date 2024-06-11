use klick_boundary::FormData;
use klick_domain::{units::*, InputValueId as Id, Value};

pub fn example_form_data() -> FormData {
    let mut data = FormData::default();

    let values = [
        (Id::PlantName, Value::text("Muster Klärwerk")),
        (Id::PopulationEquivalent, Value::count(50_000)),
        (Id::Wastewater, Value::qubicmeters(2_135_250.0)),
        (Id::InfluentNitrogen, Value::milligrams_per_liter(94.0)),
        (
            Id::InfluentChemicalOxygenDemand,
            Value::milligrams_per_liter(1_020.0),
        ),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            Value::milligrams_per_liter(382.5),
        ),
        (Id::EffluentNitrogen, Value::milligrams_per_liter(15.77)),
        (
            Id::EffluentChemicalOxygenDemand,
            Value::milligrams_per_liter(47.18),
        ),
        (Id::SewageGasProduced, Value::qubicmeters(420_000.0)),
        (Id::MethaneFraction, Value::percent(62.0)),
        (Id::PurchaseOfBiogas, Value::bool(false)),
        (Id::TotalPowerConsumption, Value::kilowatthours(1_665_000.0)),
        (Id::OnSitePowerGeneration, Value::kilowatthours(810_000.0)),
        (
            Id::EmissionFactorElectricityMix,
            Value::grams_per_kilowatthour(420.0),
        ),
        (Id::SludgeTreatmentBagsAreOpen, Value::bool(true)),
        (
            Id::SludgeTreatmentStorageContainersAreOpen,
            Value::bool(true),
        ),
        (Id::SludgeTreatmentDisposal, Value::tons(3016.5)),
        (
            Id::SludgeTreatmentTransportDistance,
            Value::kilometers(150.0),
        ),
        (Id::SludgeTreatmentDigesterCount, Value::count(1)),
        (Id::OperatingMaterialFeCl3, Value::tons(310.5)),
        (Id::OperatingMaterialSyntheticPolymers, Value::tons(12.0)),
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
        data.set(id, Some(v));
    }
    data
}
