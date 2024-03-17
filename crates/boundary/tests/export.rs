use time::OffsetDateTime;
use uuid::Uuid;

use klick_boundary::*;

#[test]
fn export() {
    let plant_profile = PlantProfile {
        plant_name: Some("test export".to_string()),
        wastewater: Some(3456.889),
        ..Default::default()
    };

    let sensitivity_parameters = SensitivityParameters {
        n2o_emissions: N2OEmissionsSensitivity {
            custom_emission_factor: Some(0.013),
            calculation_method: Some(N2oEmissionFactorCalcMethod::CustomFactor),
            side_stream_emission_factor: None,
        },
        ch4_chp_emissions: CH4ChpEmissionsSensitivity {
            custom_emission_factor: Some(0.0345),
            calculation_method: Some(CH4ChpEmissionFactorCalcMethod::MicroGasTurbines),
        },
        ch4_sewage_sludge_emissions: SewageSludgeTreatmentEmissionsSensitivity::default(),
        co2_fossil_emissions: FossilEmissonsSensitivity::default(),
    };

    let id = ProjectId(Uuid::new_v4());
    let data = FormData {
        project_title: Some("Project".into()),
        plant_profile,
        sensitivity_parameters,
        optimization_scenario: Default::default(),
    };
    let project = SavedProject {
        id,
        created_at: OffsetDateTime::now_utc(),
        modified_at: None,
        data,
    }
    .into();
    let data = Data { project };

    let json_string = export_to_string_pretty(&data);
    assert!(json_string.starts_with(&format!("{{\n  \"version\": {CURRENT_VERSION}")));
    assert!(json_string.contains("\"wastewater\": 3456.889"));
    let json = serde_json::from_str::<serde_json::Value>(&json_string).unwrap();
    let project = &json["project"];

    assert_eq!(project["id"], id.0.to_string());
    assert_eq!(project["project_title"], "Project");

    assert_eq!(
        project["sensitivity_parameters"]["n2o_emissions"]["custom_emission_factor"],
        0.013
    );
    assert_eq!(
        project["sensitivity_parameters"]["n2o_emissions"]["calculation_method"],
        "custom-factor"
    );

    assert_eq!(
        project["sensitivity_parameters"]["ch4_chp_emissions"]["custom_emission_factor"],
        0.0345
    );
    assert_eq!(
        project["sensitivity_parameters"]["ch4_chp_emissions"]["calculation_method"],
        "micro-gas-turbines"
    );
}

#[test]
fn roundtrip() {
    let plant_profile = PlantProfile {
        plant_name: Some("test export".to_string()),
        wastewater: Some(3456.889),
        influent_average: AnnualAverageInfluent {
            total_nitrogen: Some(1.2345000000000000000000000001),
            ..Default::default()
        },
        ..Default::default()
    };
    let sensitivity_parameters = SensitivityParameters {
        n2o_emissions: N2OEmissionsSensitivity {
            custom_emission_factor: Some(0.013),
            calculation_method: Some(N2oEmissionFactorCalcMethod::Pesimistic),
            side_stream_emission_factor: None,
        },
        ch4_chp_emissions: CH4ChpEmissionsSensitivity {
            custom_emission_factor: Some(0.013),
            calculation_method: Some(CH4ChpEmissionFactorCalcMethod::GasolineEngine),
        },
        ch4_sewage_sludge_emissions: SewageSludgeTreatmentEmissionsSensitivity::default(),
        co2_fossil_emissions: FossilEmissonsSensitivity::default(),
    };

    let id = ProjectId(Uuid::new_v4());
    let project = Project::Saved(SavedProject {
        id,
        created_at: OffsetDateTime::now_utc(),
        modified_at: None,
        data: FormData {
            project_title: Some("Project".into()),
            plant_profile,
            sensitivity_parameters,
            optimization_scenario: Default::default(),
        },
    });
    let data = Data {
        project: project.clone(),
    };
    let json_string = export_to_string_pretty(&data);
    let imported = import_from_str(&json_string).unwrap();
    assert_eq!(imported, project);
}
