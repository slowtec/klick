use klick_boundary::{
    export_to_string_pretty, import_from_str, AnnualAverages, InputData, CURRENT_VERSION,
};

#[test]
fn export() {
    let data = InputData {
        name: Some("test export".to_string()),
        waste_water: Some(3456.889),
        ..Default::default()
    };
    let json_string = export_to_string_pretty(&data);
    assert!(json_string.starts_with(&format!("{{\n  \"version\": {CURRENT_VERSION}")));
    assert!(json_string.contains("\"waste_water\": 3456.889"));
}

#[test]
fn roundtrip() {
    let original = InputData {
        name: Some("test export".to_string()),
        waste_water: Some(3456.889),
        inflow_averages: AnnualAverages {
            nitrogen: Some(1.2345000000000000000000000001),
            ..Default::default()
        },
        ..Default::default()
    };
    let json_string = export_to_string_pretty(&original);
    let imported = import_from_str(&json_string).unwrap();
    assert_eq!(original, imported);
}
