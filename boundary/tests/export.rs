use klick_boundary::{
    export_to_string_pretty, import_from_str, AnnualAverage, InputData,
    N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, Scenario, CURRENT_VERSION,
};

#[test]
fn export() {
    let data = InputData {
        plant_name: Some("test export".to_string()),
        wastewater: Some(3456.889),
        ..Default::default()
    };
    let scenario = Scenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            custom_factor: Some(0.013),
            calculation_method: N2oEmissionFactorCalcMethod::CustomFactor,
        },
    };

    let json_string = export_to_string_pretty(&data, &scenario);
    assert!(json_string.starts_with(&format!("{{\n  \"version\": {CURRENT_VERSION}")));
    assert!(json_string.contains("\"wastewater\": 3456.889"));
}

#[test]
fn roundtrip() {
    let original_input = InputData {
        plant_name: Some("test export".to_string()),
        wastewater: Some(3456.889),
        influent_average: AnnualAverage {
            nitrogen: Some(1.2345000000000000000000000001),
            ..Default::default()
        },
        ..Default::default()
    };
    let original_scenario = Scenario {
        n2o_emission_factor: N2oEmissionFactorScenario {
            custom_factor: Some(0.013),
            calculation_method: N2oEmissionFactorCalcMethod::Pesimistic,
        },
    };
    let json_string = export_to_string_pretty(&original_input, &original_scenario);
    let (imported_input, imported_scenario) = import_from_str(&json_string).unwrap();
    assert_eq!(original_input, imported_input);
    assert_eq!(original_scenario, imported_scenario);
}
