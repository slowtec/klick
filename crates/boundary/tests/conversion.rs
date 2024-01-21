use time::OffsetDateTime;

use klick_boundary::{
    export_to_string_pretty, import_from_str, Data, Project, ProjectData, SavedProject,
};
use klick_domain as domain;

#[test]
fn roundtrip() {
    // NOTE:
    // If you import a project
    // Please note that if you import a project in which the 'custom_factor' is defined
    // but not selected (e.g."tu-wien2016" is selected),
    // there will be a loss of data during the conversion.
    // This is to be expected and is not an error.
    let json = include_str!("example_data_v2.json");
    let Project::Unsaved(unsaved) = import_from_str(json).unwrap() else {
        panic!("Unexpected project data");
    };
    let ProjectData {
        title: _,
        optimization_scenario,
        plant_profile,
    } = unsaved;

    let id = domain::ProjectId::new().into();
    let title = Some("Test".to_string());
    let created_at = OffsetDateTime::now_utc();
    let data = ProjectData {
        title,
        optimization_scenario,
        plant_profile,
    };
    let saved = SavedProject {
        id,
        created_at,
        modified_at: None,
        data,
    };
    let boundary_project = Project::from(saved);
    let data = Data {
        project: boundary_project.clone(),
    };
    let json_string = export_to_string_pretty(&data);
    let re_imported_project = import_from_str(&json_string).unwrap();

    assert_eq!(boundary_project, re_imported_project);
}
