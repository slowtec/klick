use time::OffsetDateTime;
use uuid::Uuid;

use klick_boundary::*;
use klick_domain::{units, InputValueId as Id, Value};

#[test]
fn export() {
    let id = ProjectId(Uuid::new_v4());
    let mut data = FormData::default();

    data.set(Id::ProjectName, Some(Value::text("Project")));
    data.set(Id::PlantName, Some("test export".to_string().into()));
    data.set(Id::Wastewater, Some(Value::qubicmeters(3456.889)));
    data.set(
        Id::SensitivityN2OCalculationMethod,
        Some(Value::n2o_emission_factor_calc_method(
            units::N2oEmissionFactorCalcMethod::Custom,
        )),
    );
    data.set(Id::SensitivityN2OCustomFactor, Some(Value::percent(01.5)));
    data.set(
        Id::SensitivityCH4ChpCalculationMethod,
        Some(Value::ch4_chp_emission_factor_calc_method(
            units::Ch4ChpEmissionFactorCalcMethod::MicroGasTurbines,
        )),
    );
    data.set(
        Id::SensitivityCH4ChpCustomFactor,
        Some(Value::percent(3.45)),
    );

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
        1.5
    );
    assert_eq!(
        project["sensitivity_parameters"]["n2o_emissions"]["calculation_method"],
        "custom-factor"
    );

    assert_eq!(
        project["sensitivity_parameters"]["ch4_chp_emissions"]["custom_emission_factor"],
        3.45
    );
    assert_eq!(
        project["sensitivity_parameters"]["ch4_chp_emissions"]["calculation_method"],
        "micro-gas-turbines"
    );
}

#[test]
fn roundtrip() {
    let id = ProjectId(Uuid::new_v4());
    let mut data = FormData::default();

    data.set(Id::ProjectName, Some(Value::text("Project")));
    data.set(Id::PlantName, Some("test export".to_string().into()));
    data.set(Id::Wastewater, Some(Value::qubicmeters(3456.889)));
    data.set(
        Id::InfluentNitrogen,
        Some(Value::milligrams_per_liter(1.234_5)),
    );
    data.set(
        Id::SensitivityN2OCalculationMethod,
        Some(Value::n2o_emission_factor_calc_method(
            units::N2oEmissionFactorCalcMethod::Pesimistic,
        )),
    );
    data.set(Id::SensitivityN2OCustomFactor, Some(Value::percent(1.3)));

    data.set(
        Id::SensitivityCH4ChpCalculationMethod,
        Some(Value::ch4_chp_emission_factor_calc_method(
            units::Ch4ChpEmissionFactorCalcMethod::GasolineEngine,
        )),
    );
    data.set(
        Id::SensitivityCH4ChpCustomFactor,
        Some(Value::percent(3.45)),
    );

    let project = Project::Saved(SavedProject {
        id,
        created_at: OffsetDateTime::now_utc(),
        modified_at: None,
        data,
    });
    let data = Data {
        project: project.clone(),
    };
    let json_string = export_to_string_pretty(&data);
    let imported = import_from_str(&json_string).unwrap();
    assert_eq!(imported, project);
}
