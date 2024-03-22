use std::{fs::File, io::prelude::*};

use klick_boundary::*;

use klick_pdf_export::export_to_pdf;

pub fn main() -> anyhow::Result<()> {
    let project = project_example_data();
    let bytes = export_to_pdf(project).unwrap();

    let mut file = File::create("example-report.pdf")?;
    file.write_all(&bytes)?;
    Ok(())
}

fn project_example_data() -> FormData {
    let plant_profile = PlantProfile {
        plant_name: Some("Muster Klärwerk".to_string()),
        population_equivalent: Some(50_000.0),
        wastewater: Some(2_135_250.0),
        influent_average: AnnualAverageInfluent {
            total_nitrogen: Some(94.0),
            chemical_oxygen_demand: Some(1_020.0),
            total_organic_carbohydrates: Some(0.0),
        },
        effluent_average: AnnualAverageEffluent {
            total_nitrogen: Some(15.77),
            chemical_oxygen_demand: Some(47.18),
        },
        energy_consumption: EnergyConsumption {
            sewage_gas_produced: Some(420_000.0),
            methane_fraction: Some(62.0),
            gas_supply: None,
            purchase_of_biogas: Some(true),
            total_power_consumption: Some(1_665_000.0),
            on_site_power_generation: Some(810_000.0),
            emission_factor_electricity_mix: Some(420.0),
            heating_oil: Some(0.0),
        },
        sewage_sludge_treatment: SewageSludgeTreatment {
            sludge_bags_are_closed: Some(false),
            sludge_storage_containers_are_closed: Some(false),
            sewage_sludge_for_disposal: Some(3016.5),
            transport_distance: Some(150.0),
            digester_count: Some(3),
        },
        operating_materials: OperatingMaterials {
            fecl3: Some(310.5),
            feclso4: Some(0.0),
            caoh2: Some(0.0),
            synthetic_polymers: Some(12.0),
        },
        side_stream_treatment: SideStreamTreatment {
            total_nitrogen: Some(0.0),
        },
    };

    let sensitivity_parameters = SensitivityParameters {
        n2o_emissions: Default::default(),
        ch4_chp_emissions: Default::default(),
        ch4_sewage_sludge_emissions: Default::default(),
        co2_fossil_emissions: FossilEmissonsSensitivity {
            emission_factor: Some(0.0),
        },
    };

    let optimization_scenario = OptimizationScenario {
        sewage_sludge_treatment: SewageSludgeTreatmentScenario {
            sludge_bags_are_closed: Some(false),
            sludge_storage_containers_are_closed: Some(false),
        },
        energy_emissions: Default::default(),
        side_stream_treatment: SideStreamTreatmentScenario {
            side_stream_cover_is_closed: Some(false),
        },
    };

    FormData {
        project_title: Some("A Project Name".to_string()),
        plant_profile,
        sensitivity_parameters,
        optimization_scenario,
    }
}
