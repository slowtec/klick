use klick_boundary::{export_to_string_pretty, import_from_str};

#[test]
fn roundtrip() {
    // NOTE:
    // If you import a project
    // Please note that if you import a project in which the 'custom_factor' is defined
    // but not selected (e.g."tu-wien2016" is selected),
    // there will be a loss of data during the conversion.
    // This is to be expected and is not an error.
    let json = include_str!("example_data_v2.json");
    let project = import_from_str(json).unwrap();

    let json_string = export_to_string_pretty(&project);
    let re_imported_project = import_from_str(&json_string).unwrap();

    assert_eq!(project, re_imported_project);
}
