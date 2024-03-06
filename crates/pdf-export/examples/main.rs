use std::{fs::File, io::prelude::*};

use klick_boundary::{
    AnnualAverageEffluent, AnnualAverageInfluent, CustomEmissionFactors, EnergyConsumption,
    N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, OperatingMaterials,
    OptimizationScenario, PlantProfile, ProjectData, SewageSludgeTreatment, SideStreamTreatment,
};
use klick_pdf_export::export_to_pdf;

pub fn main() -> anyhow::Result<()> {
    let project = project_example_data();
    let bytes = export_to_pdf(project).unwrap();

    let mut file = File::create("example-report.pdf")?;
    file.write_all(&bytes)?;
    Ok(())
}

fn project_example_data() -> ProjectData {
    let plant_profile = PlantProfile {
        plant_name: Some("Muster Klärwerk".to_string()),
        population_equivalent: Some(50_000.0),
        wastewater: Some(2_135_250.0),
        influent_average: AnnualAverageInfluent {
            nitrogen: Some(94.0),
            chemical_oxygen_demand: Some(1_020.0),
            total_organic_carbohydrates: Some(0.0),
        },
        effluent_average: AnnualAverageEffluent {
            nitrogen: Some(15.77),
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
            sludge_bags_are_open: Some(true),
            sludge_bags_are_open_recommendation: Some(true),
            custom_sludge_bags_factor: None,
            sludge_storage_containers_are_open: Some(true),
            sludge_storage_containers_are_open_recommendation: Some(true),
            custom_sludge_storage_containers_factor: None,
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
            side_stream_cover_is_open: Some(true),
        },
        emission_factors: CustomEmissionFactors {
            co2_fossil: Some(0.0),
            n2o_side_stream: Some(0.0),
        },
    };

    let optimization_scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            custom_factor: None,
            calculation_method: N2oEmissionFactorCalcMethod::Ipcc2019,
        },
        ch4_chp_emission_factor: None,
    };

    ProjectData {
        title: None,
        plant_profile,
        optimization_scenario,
    }
}
