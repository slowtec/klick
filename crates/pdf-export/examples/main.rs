use std::{collections::HashMap, fs::File, io::prelude::*};

use klick_domain::{Id, InputValueId as In, Value};

use klick_pdf_export::export_to_pdf;

pub fn main() -> anyhow::Result<()> {
    let project = project_example_data();
    let bytes = export_to_pdf(&project).unwrap();

    let mut file = File::create("example-report.pdf")?;
    file.write_all(&bytes)?;
    Ok(())
}

fn project_example_data() -> HashMap<Id, Value> {
    [
        (In::ProjectName, Value::text("A Project Name")),
        (In::ProfilePlantName, Value::text("Muster Klärwerk")),
        (In::ProfilePopulationEquivalent, Value::count(50_000)),
        (In::ProfileWastewater, Value::qubicmeters(2_135_250.0)),
        (
            In::ProfileInfluentNitrogen,
            Value::milligrams_per_liter(94.0),
        ),
        (
            In::ProfileInfluentChemicalOxygenDemand,
            Value::milligrams_per_liter(1_020.0),
        ),
        (
            In::ProfileEffluentNitrogen,
            Value::milligrams_per_liter(15.77),
        ),
        (
            In::ProfileEffluentChemicalOxygenDemand,
            Value::milligrams_per_liter(47.18),
        ),
        (In::ProfileOperatingMaterialFeCl3, Value::tons(310.5)),
        (
            In::ProfileOperatingMaterialSyntheticPolymers,
            Value::tons(12.0),
        ),
        (In::ProfileSewageGasProduced, Value::qubicmeters(420_000.0)),
        (In::ProfileMethaneFraction, Value::percent(62.0)),
        (In::ProfilePurchaseOfBiogas, Value::bool(true)),
        (
            In::ProfileTotalPowerConsumption,
            Value::kilowatthours(1_665_000.0),
        ),
        (
            In::ProfileOnSitePowerGeneration,
            Value::kilowatthours(810_000.0),
        ),
        (
            In::ProfileEmissionFactorElectricityMix,
            Value::grams_per_kilowatthour(420.0),
        ),
        (In::ProfileHeatingOil, Value::liters(0.0)),
        (In::ProfileSludgeTreatmentBagsAreOpen, Value::bool(true)),
        (
            In::ProfileSludgeTreatmentStorageContainersAreOpen,
            Value::bool(true),
        ),
        (In::ProfileSludgeTreatmentDisposal, Value::tons(3016.5)),
        (
            In::ProfileSludgeTreatmentTransportDistance,
            Value::kilometers(150.0),
        ),
        (In::ProfileSludgeTreatmentDigesterCount, Value::count(3)),
        (In::RecommendationSludgeBagsAreOpen, Value::bool(true)),
        (
            In::RecommendationSludgeStorageContainersAreOpen,
            Value::bool(true),
        ),
        (
            In::RecommendationN2OSideStreamCoverIsOpen,
            Value::bool(true),
        ),
    ]
    .into_iter()
    .map(|(id, value)| (id.into(), value))
    .collect()
}
