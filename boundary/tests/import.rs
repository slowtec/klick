use klick_boundary::{import_from_str, ImportError};

#[test]
fn check_version() {
    let data = r#"{"version":0}"#;
    let err = import_from_str(data).err().unwrap();
    assert!(matches!(
        err,
        ImportError::Version {
            actual: 0,
            expected: 1
        }
    ));
}

#[test]
fn import_v1() {
    let data = include_str!("example_data_v1.json");
    assert!(import_from_str(data).is_ok());
}
