use klick_boundary::{
    export_to_string_pretty, import_from_str, AnnualAverage, CH4ChpEmissionFactorCalcMethod,
    CH4ChpEmissionFactorScenario, InputData, N2oEmissionFactorCalcMethod,
    N2oEmissionFactorScenario, Scenario, CURRENT_VERSION,
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
        ch4_chp_emission_factor: Some(CH4ChpEmissionFactorScenario {
            custom_factor: Some(0.0345),
            calculation_method: CH4ChpEmissionFactorCalcMethod::MicroGasTurbines,
        }),
    };

    let json_string = export_to_string_pretty(&data, &scenario);
    assert!(json_string.starts_with(&format!("{{\n  \"version\": {CURRENT_VERSION}")));
    assert!(json_string.contains("\"wastewater\": 3456.889"));
    let json = serde_json::from_str::<serde_json::Value>(&json_string).unwrap();

    assert_eq!(
        json["scenario"]["n2o_emission_factor"]["custom_factor"],
        0.013
    );
    assert_eq!(
        json["scenario"]["n2o_emission_factor"]["calculation_method"],
        "custom-factor"
    );

    assert_eq!(
        json["scenario"]["ch4_chp_emission_factor"]["custom_factor"],
        0.0345
    );
    assert_eq!(
        json["scenario"]["ch4_chp_emission_factor"]["calculation_method"],
        "micro-gas-turbines"
    );
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
        ch4_chp_emission_factor: Some(CH4ChpEmissionFactorScenario {
            custom_factor: Some(0.013),
            calculation_method: CH4ChpEmissionFactorCalcMethod::GasolineEngine,
        }),
    };
    let json_string = export_to_string_pretty(&original_input, &original_scenario);
    let (imported_input, imported_scenario) = import_from_str(&json_string).unwrap();
    assert_eq!(original_input, imported_input);
    assert_eq!(original_scenario, imported_scenario);
}
