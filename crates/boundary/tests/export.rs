use std::collections::HashMap;

use serde_json::json;
use time::OffsetDateTime;

use klick_boundary::{self as boundary, InputValueId as In, *};
use klick_domain::ProjectId;

#[test]
fn export() {
    let id = ProjectId::new();

    let form_data: HashMap<In, serde_json::Value> = [
        (In::ProjectName, json!("Project")),
        (In::PlantName, json!("test export")),
        (In::Wastewater, json!(3456.889)),
        (
            In::SensitivityN2OCalculationMethod,
            serde_json::to_value(N2oEmissionFactorCalcMethod::CustomFactor).unwrap(),
        ),
        (In::SensitivityN2OCustomFactor, json!(1.5)),
        (
            In::SensitivityCH4ChpCalculationMethod,
            serde_json::to_value(CH4ChpEmissionFactorCalcMethod::MicroGasTurbines).unwrap(),
        ),
        (In::SensitivityCH4ChpCustomFactor, json!(3.45)),
    ]
    .iter()
    .cloned()
    .collect();

    let project = SavedProject {
        id: id.into(),
        created_at: OffsetDateTime::now_utc(),
        modified_at: None,
        form_data: form_data.into(),
    };

    let json_string = export_to_string_pretty(&project.into());

    assert!(json_string.starts_with(&format!("{{\n  \"version\": {CURRENT_VERSION}")));
    assert!(json_string.contains("\"wastewater\": 3456.889"));

    let json = serde_json::from_str::<serde_json::Value>(&json_string).unwrap();
    let p_id = serde_json::from_value::<boundary::ProjectId>(json["id"].clone()).unwrap();

    assert_eq!(ProjectId::from(p_id), id);

    let form_data = &json["form_data"];
    assert_eq!(form_data["project-name"], "Project");
    assert_eq!(form_data["plant-name"], "test export");

    assert_eq!(form_data["sensitivity-n2o-custom-factor"], 1.5);
    assert_eq!(
        form_data["sensitivity-n2o-calculation-method"],
        "custom-factor"
    );

    assert_eq!(form_data["sensitivity-ch4-chp-custom-factor"], 3.45);
    assert_eq!(
        form_data["sensitivity-ch4-chp-calculation-method"],
        "micro-gas-turbines"
    );

    assert_eq!(form_data["wastewater"], 3456.889);
}

#[test]
fn roundtrip() {
    let id = ProjectId::new().into();

    let form_data: HashMap<In, serde_json::Value> = [
        (In::ProjectName, json!("Project")),
        (In::PlantName, json!("test export")),
        (In::Wastewater, json!(3456.889)),
        (In::InfluentNitrogen, json!(1.234_5)),
        (
            In::SensitivityN2OCalculationMethod,
            serde_json::to_value(N2oEmissionFactorCalcMethod::Pesimistic).unwrap(),
        ),
        (In::SensitivityN2OCustomFactor, json!(1.3)),
        (
            In::SensitivityCH4ChpCalculationMethod,
            serde_json::to_value(CH4ChpEmissionFactorCalcMethod::GasolineEngine).unwrap(),
        ),
        (In::SensitivityCH4ChpCustomFactor, json!(3.45)),
    ]
    .iter()
    .cloned()
    .collect();

    let project = SavedProject {
        id,
        created_at: OffsetDateTime::now_utc(),
        modified_at: None,
        form_data: form_data.into(),
    }
    .into();
    let json_string = export_to_string_pretty(&project);
    let imported = import_from_str(&json_string).unwrap();

    assert_eq!(imported, project);
}
