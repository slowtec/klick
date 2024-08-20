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
        (In::PlantName, Value::text("Muster Klärwerk")),
        (In::PopulationEquivalent, Value::count(50_000)),
        (In::Wastewater, Value::qubicmeters(2_135_250.0)),
        (In::InfluentNitrogen, Value::milligrams_per_liter(94.0)),
        (
            In::InfluentChemicalOxygenDemand,
            Value::milligrams_per_liter(1_020.0),
        ),
        (
            In::InfluentTotalOrganicCarbohydrates,
            Value::milligrams_per_liter(0.0),
        ),
        (In::EffluentNitrogen, Value::milligrams_per_liter(15.77)),
        (
            In::EffluentChemicalOxygenDemand,
            Value::milligrams_per_liter(47.18),
        ),
        (In::OperatingMaterialFeCl3, Value::tons(310.5)),
        (In::OperatingMaterialFeClSO4, Value::tons(0.0)),
        (In::OperatingMaterialCaOH2, Value::tons(0.0)),
        (In::OperatingMaterialSyntheticPolymers, Value::tons(12.0)),
        (In::SewageGasProduced, Value::qubicmeters(420_000.0)),
        (In::MethaneFraction, Value::percent(62.0)),
        (In::PurchaseOfBiogas, Value::bool(true)),
        (In::TotalPowerConsumption, Value::kilowatthours(1_665_000.0)),
        (In::OnSitePowerGeneration, Value::kilowatthours(810_000.0)),
        (
            In::EmissionFactorElectricityMix,
            Value::grams_per_kilowatthour(420.0),
        ),
        (In::HeatingOil, Value::liters(0.0)),
        (In::SludgeTreatmentBagsAreOpen, Value::bool(true)),
        (
            In::SludgeTreatmentStorageContainersAreOpen,
            Value::bool(true),
        ),
        (In::SludgeTreatmentDisposal, Value::tons(3016.5)),
        (
            In::SludgeTreatmentTransportDistance,
            Value::kilometers(150.0),
        ),
        (In::SludgeTreatmentDigesterCount, Value::count(3)),
        (In::SideStreamTreatmentTotalNitrogen, Value::tons(0.0)),
        (In::SensitivityCO2FossilCustomFactor, Value::factor(0.0)),
        (In::ScenarioSludgeBagsAreOpen, Value::bool(true)),
        (
            In::ScenarioSludgeStorageContainersAreOpen,
            Value::bool(true),
        ),
        (In::ScenarioN2OSideStreamCoverIsOpen, Value::bool(true)),
    ]
    .into_iter()
    .map(|(id, value)| (id.into(), value))
    .collect()
}
