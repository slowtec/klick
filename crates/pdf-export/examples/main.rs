use std::{fs::File, io::prelude::*};

use klick_boundary::FormData;
use klick_domain::{InputValueId as Id, Value};

use klick_pdf_export::export_to_pdf;

pub fn main() -> anyhow::Result<()> {
    let project = project_example_data();
    let bytes = export_to_pdf(project).unwrap();

    let mut file = File::create("example-report.pdf")?;
    file.write_all(&bytes)?;
    Ok(())
}

fn project_example_data() -> FormData {
    [
        (Id::ProjectName, Value::text("A Project Name")),
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
            Value::milligrams_per_liter(0.0),
        ),
        (Id::EffluentNitrogen, Value::milligrams_per_liter(15.77)),
        (
            Id::EffluentChemicalOxygenDemand,
            Value::milligrams_per_liter(47.18),
        ),
        (Id::OperatingMaterialFeCl3, Value::tons(310.5)),
        (Id::OperatingMaterialFeClSO4, Value::tons(0.0)),
        (Id::OperatingMaterialCaOH2, Value::tons(0.0)),
        (Id::OperatingMaterialSyntheticPolymers, Value::tons(12.0)),
        (Id::SewageGasProduced, Value::qubicmeters(420_000.0)),
        (Id::MethaneFraction, Value::percent(62.0)),
        (Id::PurchaseOfBiogas, Value::bool(true)),
        (Id::TotalPowerConsumption, Value::kilowatthours(1_665_000.0)),
        (Id::OnSitePowerGeneration, Value::kilowatthours(810_000.0)),
        (
            Id::EmissionFactorElectricityMix,
            Value::grams_per_kilowatthour(420.0),
        ),
        (Id::HeatingOil, Value::liters(0.0)),
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
        (Id::SludgeTreatmentDigesterCount, Value::count(3)),
        (Id::SideStreamTreatmentTotalNitrogen, Value::tons(0.0)),
        (Id::SensitivityCO2FossilCustomFactor, Value::factor(0.0)),
        (Id::ScenarioSludgeBagsAreOpen, Value::bool(true)),
        (
            Id::ScenarioSludgeStorageContainersAreOpen,
            Value::bool(true),
        ),
        (Id::ScenarioN2OSideStreamCoverIsOpen, Value::bool(true)),
    ]
    .into_iter()
    .collect()
}
