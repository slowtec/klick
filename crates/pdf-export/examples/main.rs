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
    let values = [
        (Id::ProjectName, Some(Value::text("A Project Name"))),
        (Id::PlantName, Some(Value::text("Muster Klärwerk"))),
        (Id::PopulationEquivalent, Some(Value::count(50_000))),
        (Id::Wastewater, Some(Value::qubicmeters(2_135_250.0))),
        (
            Id::InfluentNitrogen,
            Some(Value::milligrams_per_liter(94.0)),
        ),
        (
            Id::InfluentChemicalOxygenDemand,
            Some(Value::milligrams_per_liter(1_020.0)),
        ),
        (
            Id::InfluentTotalOrganicCarbohydrates,
            Some(Value::milligrams_per_liter(0.0)),
        ),
        (
            Id::EffluentNitrogen,
            Some(Value::milligrams_per_liter(15.77)),
        ),
        (
            Id::EffluentChemicalOxygenDemand,
            Some(Value::milligrams_per_liter(47.18)),
        ),
        (Id::OperatingMaterialFeCl3, Some(Value::tons(310.5))),
        (Id::OperatingMaterialFeClSO4, Some(Value::tons(0.0))),
        (Id::OperatingMaterialCaOH2, Some(Value::tons(0.0))),
        (
            Id::OperatingMaterialSyntheticPolymers,
            Some(Value::tons(12.0)),
        ),
        (Id::SewageGasProduced, Some(Value::qubicmeters(420_000.0))),
        (Id::MethaneFraction, Some(Value::percent(62.0))),
        (Id::PurchaseOfBiogas, Some(Value::bool(true))),
        (
            Id::TotalPowerConsumption,
            Some(Value::kilowatthours(1_665_000.0)),
        ),
        (
            Id::OnSitePowerGeneration,
            Some(Value::kilowatthours(810_000.0)),
        ),
        (
            Id::EmissionFactorElectricityMix,
            Some(Value::grams_per_kilowatthour(420.0)),
        ),
        (Id::HeatingOil, Some(Value::liters(0.0))),
        (Id::SludgeTreatmentBagsAreOpen, Some(Value::bool(true))),
        (
            Id::SludgeTreatmentStorageContainersAreOpen,
            Some(Value::bool(true)),
        ),
        (Id::SludgeTreatmentDisposal, Some(Value::tons(3016.5))),
        (
            Id::SludgeTreatmentTransportDistance,
            Some(Value::kilometers(150.0)),
        ),
        (Id::SludgeTreatmentDigesterCount, Some(Value::count(3))),
        (Id::SideStreamTreatmentTotalNitrogen, Some(Value::tons(0.0))),
        (
            Id::SensitivityCO2FossilCustomFactor,
            Some(Value::factor(0.0)),
        ),
        (Id::ScenarioSludgeBagsAreOpen, Some(Value::bool(true))),
        (
            Id::ScenarioSludgeStorageContainersAreOpen,
            Some(Value::bool(true)),
        ),
        (
            Id::ScenarioN2OSideStreamCoverIsOpen,
            Some(Value::bool(true)),
        ),
    ];

    let mut data = FormData::default();
    for (k, v) in values {
        data.set(k, v);
    }

    data
}
