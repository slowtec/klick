use time::OffsetDateTime;
use uuid::Uuid;

use klick_boundary::{
    export_to_string_pretty, import_from_str, AnnualAverageInfluent,
    CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, Data,
    N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, OptimizationScenario, PlantProfile,
    Project, ProjectData, ProjectId, SavedProject, CURRENT_VERSION,
};

#[test]
fn export() {
    let plant_profile = PlantProfile {
        plant_name: Some("test export".to_string()),
        wastewater: Some(3456.889),
        ..Default::default()
    };
    let optimization_scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            custom_factor: Some(0.013),
            calculation_method: N2oEmissionFactorCalcMethod::CustomFactor,
        },
        ch4_chp_emission_factor: Some(CH4ChpEmissionFactorScenario {
            custom_factor: Some(0.0345),
            calculation_method: CH4ChpEmissionFactorCalcMethod::MicroGasTurbines,
        }),
    };

    let id = ProjectId(Uuid::new_v4());
    let data = ProjectData {
        title: Some("Project".into()),
        plant_profile,
        optimization_scenario,
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
    assert_eq!(project["title"], "Project");

    assert_eq!(
        project["optimization_scenario"]["n2o_emission_factor"]["custom_factor"],
        0.013
    );
    assert_eq!(
        project["optimization_scenario"]["n2o_emission_factor"]["calculation_method"],
        "custom-factor"
    );

    assert_eq!(
        project["optimization_scenario"]["ch4_chp_emission_factor"]["custom_factor"],
        0.0345
    );
    assert_eq!(
        project["optimization_scenario"]["ch4_chp_emission_factor"]["calculation_method"],
        "micro-gas-turbines"
    );
}

#[test]
fn roundtrip() {
    let plant_profile = PlantProfile {
        plant_name: Some("test export".to_string()),
        wastewater: Some(3456.889),
        influent_average: AnnualAverageInfluent {
            nitrogen: Some(1.2345000000000000000000000001),
            ..Default::default()
        },
        ..Default::default()
    };
    let optimization_scenario = OptimizationScenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            custom_factor: Some(0.013),
            calculation_method: N2oEmissionFactorCalcMethod::Pesimistic,
        },
        ch4_chp_emission_factor: Some(CH4ChpEmissionFactorScenario {
            custom_factor: Some(0.013),
            calculation_method: CH4ChpEmissionFactorCalcMethod::GasolineEngine,
        }),
    };

    let id = ProjectId(Uuid::new_v4());
    let project = Project::Saved(SavedProject {
        id,
        created_at: OffsetDateTime::now_utc(),
        modified_at: None,
        data: ProjectData {
            title: Some("Project".into()),
            plant_profile,
            optimization_scenario,
        },
    });
    let data = Data {
        project: project.clone(),
    };
    let json_string = export_to_string_pretty(&data);
    let imported = import_from_str(&json_string).unwrap();
    assert_eq!(imported, project);
}
